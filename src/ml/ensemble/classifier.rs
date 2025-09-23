use crate::ml::cross_validation::{Classifier, ClassifierFactory};
use crate::ml::training_data::TrainingDataset;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

use super::config::*;
use super::meta_learners::*;

/// Training history for ensemble
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleTrainingHistory {
    /// Performance of individual models
    pub model_performances: Vec<ModelPerformance>,
    /// Ensemble performance over training
    pub ensemble_performance: Vec<f64>,
    /// Diversity metrics over training
    pub diversity_metrics: Vec<DiversityMetrics>,
    /// Training times
    pub training_times: Vec<std::time::Duration>,
}

/// Individual model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformance {
    pub model_id: usize,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub training_time: std::time::Duration,
    pub feature_importance: Vec<f64>,
}

/// Diversity metrics for ensemble
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityMetrics {
    /// Pairwise diversity between models
    pub pairwise_diversity: f64,
    /// Q-statistic diversity measure
    pub q_statistic: f64,
    /// Disagreement measure
    pub disagreement: f64,
    /// Double fault measure
    pub double_fault: f64,
}

/// Ensemble prediction result
#[derive(Debug, Clone)]
pub struct EnsemblePrediction {
    /// Final ensemble prediction
    pub prediction: f32,
    /// Individual model predictions
    pub individual_predictions: Vec<f32>,
    /// Model confidence scores
    pub confidence_scores: Vec<f64>,
    /// Prediction uncertainty
    pub uncertainty: f64,
    /// Explanation of ensemble decision
    pub explanation: EnsembleExplanation,
}

/// Explanation of ensemble decision
#[derive(Debug, Clone)]
pub struct EnsembleExplanation {
    /// Which models agreed/disagreed
    pub model_agreement: HashMap<usize, bool>,
    /// Contribution of each model to final decision
    pub model_contributions: Vec<f64>,
    /// Uncertainty sources
    pub uncertainty_sources: Vec<String>,
    /// Confidence level in prediction
    pub confidence_level: f64,
}

/// Main ensemble classifier
pub struct EnsembleClassifier<T: Classifier + Clone + Send + Sync> {
    /// Base models in the ensemble
    base_models: Vec<T>,
    /// Model weights for weighted voting
    model_weights: Vec<f64>,
    /// Ensemble configuration
    config: EnsembleConfig,
    /// Meta-learner for stacking (if applicable)
    meta_learner: Option<Box<dyn Classifier + Send + Sync>>,
    /// Training history and metrics
    training_history: EnsembleTrainingHistory,
}

impl<T: Classifier + Clone + Send + Sync> EnsembleClassifier<T> {
    /// Create a new ensemble classifier
    pub fn new(config: EnsembleConfig) -> Self {
        Self {
            base_models: Vec::new(),
            model_weights: vec![1.0; config.n_models],
            config,
            meta_learner: None,
            training_history: EnsembleTrainingHistory {
                model_performances: Vec::new(),
                ensemble_performance: Vec::new(),
                diversity_metrics: Vec::new(),
                training_times: Vec::new(),
            },
        }
    }

    /// Train the ensemble with a factory for creating base models
    pub async fn train(
        &mut self,
        factory: &dyn ClassifierFactory<T>,
        dataset: &TrainingDataset,
    ) -> Result<()> {
        // Input validation for security
        if self.config.n_models == 0 {
            return Err(anyhow!("Number of models must be greater than 0"));
        }
        if self.config.n_models > 100 {
            return Err(anyhow!("Number of models exceeds maximum allowed (100)"));
        }
        if dataset.len() == 0 {
            return Err(anyhow!("Training dataset is empty"));
        }

        let start_time = std::time::Instant::now();
        info!(
            "Starting ensemble training with {} models",
            self.config.n_models
        );

        // Clear previous models
        self.base_models.clear();
        self.training_history.model_performances.clear();

        // Create diverse training datasets
        let training_sets = self.create_diverse_datasets(dataset)?;

        // Train base models
        if self.config.optimization.parallel_training {
            self.train_models_parallel(factory, &training_sets).await?;
        } else {
            self.train_models_sequential(factory, &training_sets)
                .await?;
        }

        // Calculate model weights based on performance
        self.calculate_model_weights(dataset).await?;

        // Train meta-learner if using stacking
        if let EnsembleStrategy::Stacking {
            meta_learner_config,
        } = &self.config.strategy
        {
            self.train_meta_learner(meta_learner_config, dataset)
                .await?;
        }

        // Calculate diversity metrics
        let diversity = self.calculate_diversity_metrics(dataset).await?;
        self.training_history.diversity_metrics.push(diversity);

        let training_time = start_time.elapsed();
        self.training_history.training_times.push(training_time);

        info!(
            "Ensemble training completed in {:.2}s",
            training_time.as_secs_f64()
        );
        Ok(())
    }

    /// Create diverse datasets for training different models
    fn create_diverse_datasets(&self, dataset: &TrainingDataset) -> Result<Vec<TrainingDataset>> {
        let mut datasets = Vec::new();

        for i in 0..self.config.n_models {
            let mut subset = TrainingDataset::new();

            if self.config.diversity.data_bagging {
                // Bootstrap sampling
                let sample_size =
                    (dataset.len() as f64 * self.config.diversity.bagging_fraction) as usize;
                let indices = self.bootstrap_sample(dataset.len(), sample_size);

                for idx in indices {
                    let (features, label) = dataset.get_sample(idx)?;
                    let is_positive = label > 0.5;
                    subset.add_synthetic_example(
                        &format!("model_{}_sample_{}", i, idx),
                        features,
                        is_positive,
                    );
                }
            } else {
                // Use full dataset
                for idx in 0..dataset.len() {
                    let (features, label) = dataset.get_sample(idx)?;
                    let is_positive = label > 0.5;
                    subset.add_synthetic_example(
                        &format!("model_{}_sample_{}", i, idx),
                        features,
                        is_positive,
                    );
                }
            }

            datasets.push(subset);
        }

        Ok(datasets)
    }

    /// Bootstrap sampling with replacement
    fn bootstrap_sample(&self, population_size: usize, sample_size: usize) -> Vec<usize> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..sample_size)
            .map(|_| rng.gen_range(0..population_size))
            .collect()
    }

    /// Train models sequentially
    async fn train_models_sequential(
        &mut self,
        factory: &dyn ClassifierFactory<T>,
        training_sets: &[TrainingDataset],
    ) -> Result<()> {
        for (i, training_set) in training_sets.iter().enumerate() {
            info!("Training model {}/{}", i + 1, self.config.n_models);

            let model_start = std::time::Instant::now();
            let mut model = factory.create()?;

            let training_data = training_set.get_training_pairs();
            model.train(&training_data).await?;

            let training_time = model_start.elapsed();

            // Evaluate model performance
            let performance = self
                .evaluate_model_performance(&model, training_set, i, training_time)
                .await?;
            self.training_history.model_performances.push(performance);

            self.base_models.push(model);
        }

        Ok(())
    }

    /// Train models in parallel
    async fn train_models_parallel(
        &mut self,
        factory: &dyn ClassifierFactory<T>,
        training_sets: &[TrainingDataset],
    ) -> Result<()> {
        use tokio::task;

        let max_concurrent = self
            .config
            .optimization
            .max_threads
            .min(self.config.n_models);
        let mut handles = Vec::new();

        for (i, training_set) in training_sets.iter().enumerate() {
            if handles.len() >= max_concurrent {
                // Wait for one task to complete
                if let Some(handle) = handles.pop() {
                    let (model, performance) = handle.await??;
                    self.base_models.push(model);
                    self.training_history.model_performances.push(performance);
                }
            }

            let training_data = training_set.get_training_pairs();
            let mut model = factory.create()?;
            let training_set_clone = training_set.clone();

            let handle = task::spawn(async move {
                Self::train_single_model(factory, training_set_clone, i).await
            });

            handles.push(handle);
        }

        // Wait for remaining tasks
        for handle in handles {
            let (model, performance) = handle.await??;
            self.base_models.push(model);
            self.training_history.model_performances.push(performance);
        }

        Ok(())
    }

    /// Train a single model and evaluate its performance
    async fn train_single_model(
        factory: &dyn ClassifierFactory<T>,
        training_set: TrainingDataset,
        model_id: usize,
    ) -> Result<(T, ModelPerformance)> {
        let model_start = std::time::Instant::now();
        let mut model = factory.create()?;
        let training_data = training_set.get_training_pairs();
        model.train(&training_data).await?;
        let training_time = model_start.elapsed();

        // Evaluate model performance
        let performance =
            Self::evaluate_model_performance_static(&model, &training_set, model_id, training_time)
                .await?;

        Ok((model, performance))
    }

    /// Evaluate individual model performance
    async fn evaluate_model_performance(
        &self,
        model: &T,
        dataset: &TrainingDataset,
        model_id: usize,
        training_time: std::time::Duration,
    ) -> Result<ModelPerformance> {
        Self::evaluate_model_performance_static(model, dataset, model_id, training_time).await
    }

    /// Static version for use in async contexts
    async fn evaluate_model_performance_static(
        model: &T,
        dataset: &TrainingDataset,
        model_id: usize,
        training_time: std::time::Duration,
    ) -> Result<ModelPerformance> {
        let mut correct = 0;
        let mut total = 0;
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for i in 0..dataset.len() {
            let (features, actual_label) = dataset.get_sample(i)?;
            let prediction = model.predict(&features).await?;
            let predicted_label = prediction > 0.5;
            let actual_bool = actual_label > 0.5;

            total += 1;
            if predicted_label == actual_bool {
                correct += 1;
            }

            if actual_bool && predicted_label {
                true_positives += 1;
            } else if !actual_bool && predicted_label {
                false_positives += 1;
            } else if actual_bool && !predicted_label {
                false_negatives += 1;
            }
        }

        let accuracy = correct as f64 / total as f64;
        let precision = if true_positives + false_positives > 0 {
            true_positives as f64 / (true_positives + false_positives) as f64
        } else {
            0.0
        };
        let recall = if true_positives + false_negatives > 0 {
            true_positives as f64 / (true_positives + false_negatives) as f64
        } else {
            0.0
        };
        let f1_score = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        Ok(ModelPerformance {
            model_id,
            accuracy,
            precision,
            recall,
            f1_score,
            training_time,
            feature_importance: vec![1.0 / features.len() as f64; features.len()], // Simplified
        })
    }

    /// Calculate model weights based on performance
    async fn calculate_model_weights(&mut self, dataset: &TrainingDataset) -> Result<()> {
        match &self.config.strategy {
            EnsembleStrategy::MajorityVoting => {
                // Equal weights for majority voting
                self.model_weights = vec![1.0; self.base_models.len()];
            }
            EnsembleStrategy::WeightedVoting { weights } => {
                if weights.len() != self.base_models.len() {
                    // Calculate weights based on performance
                    self.model_weights = self
                        .training_history
                        .model_performances
                        .iter()
                        .map(|p| p.f1_score)
                        .collect();

                    // Normalize weights
                    let sum: f64 = self.model_weights.iter().sum();
                    if sum > 0.0 {
                        self.model_weights.iter_mut().for_each(|w| *w /= sum);
                    }
                } else {
                    self.model_weights = weights.clone();
                }
            }
            EnsembleStrategy::AdaBoost { learning_rate, .. } => {
                self.calculate_adaboost_weights(*learning_rate, dataset)
                    .await?;
            }
            _ => {
                // Default to performance-based weights
                self.model_weights = self
                    .training_history
                    .model_performances
                    .iter()
                    .map(|p| p.f1_score)
                    .collect();
            }
        }

        Ok(())
    }

    /// Calculate AdaBoost weights
    async fn calculate_adaboost_weights(
        &mut self,
        learning_rate: f64,
        dataset: &TrainingDataset,
    ) -> Result<()> {
        let n_samples = dataset.len();
        let mut sample_weights = vec![1.0 / n_samples as f64; n_samples];
        self.model_weights.clear();

        for (model_idx, model) in self.base_models.iter().enumerate() {
            let mut error = 0.0;
            let mut predictions = Vec::new();

            // Calculate weighted error
            for i in 0..n_samples {
                let (features, actual_label) = dataset.get_sample(i)?;
                let prediction = model.predict(&features).await?;
                let predicted_label = prediction > 0.5;
                let actual_bool = actual_label > 0.5;

                predictions.push(predicted_label);

                if predicted_label != actual_bool {
                    error += sample_weights[i];
                }
            }

            if error == 0.0 {
                // Perfect classifier
                self.model_weights.push(1.0);
                break;
            } else if error >= 0.5 {
                // Worse than random
                self.model_weights.push(0.0);
                continue;
            }

            // Calculate model weight
            let model_weight = learning_rate * (((1.0 - error) / error).ln());
            self.model_weights.push(model_weight);

            // Update sample weights
            for i in 0..n_samples {
                let (_, actual_label) = dataset.get_sample(i)?;
                let actual_bool = actual_label > 0.5;

                if predictions[i] != actual_bool {
                    sample_weights[i] *= (error / (1.0 - error)).exp();
                }
            }

            // Normalize sample weights
            let sum: f64 = sample_weights.iter().sum();
            sample_weights.iter_mut().for_each(|w| *w /= sum);
        }

        Ok(())
    }

    /// Train meta-learner for stacking
    async fn train_meta_learner(
        &mut self,
        meta_config: &MetaLearnerConfig,
        dataset: &TrainingDataset,
    ) -> Result<()> {
        info!("Training meta-learner for stacking ensemble");

        // Generate meta-features using cross-validation
        let meta_features = self
            .generate_meta_features(dataset, meta_config.cv_folds)
            .await?;

        // Create meta-learner
        self.meta_learner = Some(self.create_meta_learner(meta_config)?);

        // Train meta-learner
        if let Some(ref mut meta_learner) = self.meta_learner {
            meta_learner.train(&meta_features).await?;
        }

        Ok(())
    }

    /// Generate meta-features for stacking
    async fn generate_meta_features(
        &self,
        dataset: &TrainingDataset,
        cv_folds: usize,
    ) -> Result<Vec<(Vec<f32>, f32)>> {
        let mut meta_features = Vec::new();
        let fold_size = dataset.len() / cv_folds;

        for fold in 0..cv_folds {
            let test_start = fold * fold_size;
            let test_end = if fold == cv_folds - 1 {
                dataset.len()
            } else {
                (fold + 1) * fold_size
            };

            // Get fold predictions from each base model
            for test_idx in test_start..test_end {
                let (features, label) = dataset.get_sample(test_idx)?;
                let mut model_predictions = Vec::new();

                for model in &self.base_models {
                    let prediction = model.predict(&features).await?;
                    model_predictions.push(prediction);
                }

                meta_features.push((model_predictions, label));
            }
        }

        Ok(meta_features)
    }

    /// Create meta-learner based on configuration
    fn create_meta_learner(
        &self,
        config: &MetaLearnerConfig,
    ) -> Result<Box<dyn Classifier + Send + Sync>> {
        match config.learner_type {
            MetaLearnerType::LinearRegression => Ok(Box::new(SimpleLinearRegression::new())),
            MetaLearnerType::LogisticRegression => Ok(Box::new(SimpleLogisticRegression::new())),
            MetaLearnerType::DecisionTree => {
                Err(anyhow!("Decision tree meta-learner not implemented yet"))
            }
            MetaLearnerType::NeuralNetwork => {
                Err(anyhow!("Neural network meta-learner not implemented yet"))
            }
        }
    }

    /// Make ensemble prediction
    pub async fn predict(&self, features: &[f32]) -> Result<EnsemblePrediction> {
        // Input validation for security
        if features.is_empty() {
            return Err(anyhow!("Features array is empty"));
        }
        if features.len() > 10000 {
            return Err(anyhow!("Features array too large (max 10000)"));
        }
        // Check for invalid values
        if features.iter().any(|&f| !f.is_finite()) {
            return Err(anyhow!("Features contain invalid values (NaN or infinite)"));
        }

        if self.base_models.is_empty() {
            return Err(anyhow!("Ensemble not trained"));
        }

        // Get predictions from all base models
        let mut individual_predictions = Vec::new();
        let mut confidence_scores = Vec::new();

        for model in &self.base_models {
            let prediction = model.predict(features).await?;
            individual_predictions.push(prediction);
            confidence_scores.push(self.calculate_prediction_confidence(prediction));
        }

        // Combine predictions based on strategy
        let final_prediction = match &self.config.strategy {
            EnsembleStrategy::MajorityVoting => self.majority_voting(&individual_predictions),
            EnsembleStrategy::WeightedVoting { .. } => {
                self.weighted_voting(&individual_predictions)
            }
            EnsembleStrategy::Stacking { .. } => {
                self.stacking_prediction(&individual_predictions).await?
            }
            _ => self.weighted_voting(&individual_predictions),
        };

        // Calculate prediction uncertainty
        let uncertainty = self.calculate_prediction_uncertainty(&individual_predictions);

        // Generate explanation
        let explanation = self.generate_prediction_explanation(
            &individual_predictions,
            final_prediction,
            uncertainty,
        );

        Ok(EnsemblePrediction {
            prediction: final_prediction,
            individual_predictions,
            confidence_scores,
            uncertainty,
            explanation,
        })
    }

    /// Majority voting strategy
    fn majority_voting(&self, predictions: &[f32]) -> f32 {
        let votes: usize = predictions
            .iter()
            .map(|&p| if p > 0.5 { 1 } else { 0 })
            .sum();
        if votes as f32 > predictions.len() as f32 / 2.0 {
            1.0
        } else {
            0.0
        }
    }

    /// Weighted voting strategy
    fn weighted_voting(&self, predictions: &[f32]) -> f32 {
        if self.model_weights.len() != predictions.len() {
            // Fallback to simple average
            return predictions.iter().sum::<f32>() / predictions.len() as f32;
        }

        let weighted_sum: f32 = predictions
            .iter()
            .zip(self.model_weights.iter())
            .map(|(pred, weight)| pred * (*weight as f32))
            .sum();

        let weight_sum: f32 = self.model_weights.iter().map(|w| *w as f32).sum();

        if weight_sum > 0.0 {
            weighted_sum / weight_sum
        } else {
            predictions.iter().sum::<f32>() / predictions.len() as f32
        }
    }

    /// Stacking prediction using meta-learner
    async fn stacking_prediction(&self, base_predictions: &[f32]) -> Result<f32> {
        if let Some(ref meta_learner) = self.meta_learner {
            meta_learner.predict(base_predictions).await
        } else {
            Err(anyhow!("Meta-learner not trained for stacking"))
        }
    }

    /// Calculate prediction confidence
    fn calculate_prediction_confidence(&self, prediction: f32) -> f64 {
        // Distance from decision boundary (0.5)
        (prediction - 0.5).abs() as f64 * 2.0
    }

    /// Calculate prediction uncertainty
    fn calculate_prediction_uncertainty(&self, predictions: &[f32]) -> f64 {
        if predictions.len() <= 1 {
            return 1.0;
        }

        // Calculate variance as uncertainty measure
        let mean = predictions.iter().sum::<f32>() / predictions.len() as f32;
        let variance =
            predictions.iter().map(|p| (p - mean).powi(2)).sum::<f32>() / predictions.len() as f32;

        variance.sqrt() as f64
    }

    /// Generate explanation for ensemble prediction
    fn generate_prediction_explanation(
        &self,
        predictions: &[f32],
        final_prediction: f32,
        uncertainty: f64,
    ) -> EnsembleExplanation {
        let mut model_agreement = HashMap::new();
        let mut model_contributions = Vec::new();
        let mut uncertainty_sources = Vec::new();

        // Analyze model agreement
        for (i, &pred) in predictions.iter().enumerate() {
            let agrees = (pred > 0.5) == (final_prediction > 0.5);
            model_agreement.insert(i, agrees);

            // Calculate contribution (simplified)
            let weight = self.model_weights.get(i).copied().unwrap_or(1.0);
            model_contributions.push(weight * pred as f64);
        }

        // Identify uncertainty sources
        if uncertainty > 0.3 {
            uncertainty_sources.push("High disagreement between models".to_string());
        }
        if predictions.iter().any(|&p| (p - 0.5).abs() < 0.1) {
            uncertainty_sources.push("Some models near decision boundary".to_string());
        }

        let confidence_level = 1.0 - uncertainty;

        EnsembleExplanation {
            model_agreement,
            model_contributions,
            uncertainty_sources,
            confidence_level,
        }
    }

    /// Calculate diversity metrics
    async fn calculate_diversity_metrics(
        &self,
        dataset: &TrainingDataset,
    ) -> Result<DiversityMetrics> {
        if self.base_models.len() < 2 {
            return Ok(DiversityMetrics {
                pairwise_diversity: 0.0,
                q_statistic: 0.0,
                disagreement: 0.0,
                double_fault: 0.0,
            });
        }

        let mut all_predictions = Vec::new();

        // Get predictions from all models
        for model in &self.base_models {
            let mut model_predictions = Vec::new();
            for i in 0..dataset.len() {
                let (features, _) = dataset.get_sample(i)?;
                let prediction = model.predict(&features).await?;
                model_predictions.push(prediction > 0.5);
            }
            all_predictions.push(model_predictions);
        }

        // Calculate pairwise diversity
        let mut diversity_sum = 0.0;
        let mut pair_count = 0;

        for i in 0..all_predictions.len() {
            for j in (i + 1)..all_predictions.len() {
                let diversity = self
                    .calculate_pairwise_diversity(&all_predictions[i], &all_predictions[j])
                    .await?;
                diversity_sum += diversity;
                pair_count += 1;
            }
        }

        let pairwise_diversity = if pair_count > 0 {
            diversity_sum / pair_count as f64
        } else {
            0.0
        };

        // Calculate other diversity metrics (simplified)
        let disagreement = self.calculate_disagreement(&all_predictions);
        let q_statistic = self
            .calculate_q_statistic(&all_predictions, dataset)
            .await?;
        let double_fault = self
            .calculate_double_fault(&all_predictions, dataset)
            .await?;

        Ok(DiversityMetrics {
            pairwise_diversity,
            q_statistic,
            disagreement,
            double_fault,
        })
    }

    /// Calculate pairwise diversity between two models
    async fn calculate_pairwise_diversity(
        &self,
        predictions1: &[bool],
        predictions2: &[bool],
    ) -> Result<f64> {
        let mut disagreements = 0;
        let total = predictions1.len();

        for i in 0..total {
            if predictions1[i] != predictions2[i] {
                disagreements += 1;
            }
        }

        Ok(disagreements as f64 / total as f64)
    }

    /// Calculate disagreement measure
    fn calculate_disagreement(&self, all_predictions: &[Vec<bool>]) -> f64 {
        if all_predictions.is_empty() || all_predictions[0].is_empty() {
            return 0.0;
        }

        let n_models = all_predictions.len();
        let n_samples = all_predictions[0].len();
        let mut total_disagreement = 0.0;

        for i in 0..n_samples {
            let positive_votes: usize = all_predictions
                .iter()
                .map(|preds| if preds[i] { 1 } else { 0 })
                .sum();
            let negative_votes = n_models - positive_votes;
            let disagreement =
                (positive_votes * negative_votes) as f64 / (n_models * n_models) as f64;
            total_disagreement += disagreement;
        }

        total_disagreement / n_samples as f64
    }

    /// Calculate Q-statistic
    async fn calculate_q_statistic(
        &self,
        all_predictions: &[Vec<bool>],
        dataset: &TrainingDataset,
    ) -> Result<f64> {
        // Simplified Q-statistic calculation
        if all_predictions.len() < 2 {
            return Ok(0.0);
        }

        let mut q_sum = 0.0;
        let mut pair_count = 0;

        for i in 0..all_predictions.len() {
            for j in (i + 1)..all_predictions.len() {
                let q = self
                    .calculate_q_statistic_pair(&all_predictions[i], &all_predictions[j], dataset)
                    .await?;
                q_sum += q;
                pair_count += 1;
            }
        }

        Ok(if pair_count > 0 {
            q_sum / pair_count as f64
        } else {
            0.0
        })
    }

    /// Calculate Q-statistic for a pair of models
    async fn calculate_q_statistic_pair(
        &self,
        pred1: &[bool],
        pred2: &[bool],
        dataset: &TrainingDataset,
    ) -> Result<f64> {
        let mut n11 = 0; // Both correct
        let mut n10 = 0; // First correct, second wrong
        let mut n01 = 0; // First wrong, second correct
        let mut n00 = 0; // Both wrong

        for i in 0..pred1.len() {
            let (_, actual_label) = dataset.get_sample(i)?;
            let actual = actual_label > 0.5;

            let correct1 = pred1[i] == actual;
            let correct2 = pred2[i] == actual;

            match (correct1, correct2) {
                (true, true) => n11 += 1,
                (true, false) => n10 += 1,
                (false, true) => n01 += 1,
                (false, false) => n00 += 1,
            }
        }

        let numerator = (n11 * n00) as f64 - (n01 * n10) as f64;
        let denominator = (n11 * n00) as f64 + (n01 * n10) as f64;

        Ok(if denominator != 0.0 {
            numerator / denominator
        } else {
            0.0
        })
    }

    /// Calculate double fault measure
    async fn calculate_double_fault(
        &self,
        all_predictions: &[Vec<bool>],
        dataset: &TrainingDataset,
    ) -> Result<f64> {
        if all_predictions.len() < 2 {
            return Ok(0.0);
        }

        let mut total_double_faults = 0;
        let mut total_samples = 0;

        for i in 0..all_predictions[0].len() {
            let (_, actual_label) = dataset.get_sample(i)?;
            let actual = actual_label > 0.5;

            let wrong_count = all_predictions
                .iter()
                .filter(|preds| preds[i] != actual)
                .count();

            if wrong_count >= 2 {
                total_double_faults += 1;
            }
            total_samples += 1;
        }

        Ok(total_double_faults as f64 / total_samples as f64)
    }

    /// Get ensemble training history
    pub fn get_training_history(&self) -> &EnsembleTrainingHistory {
        &self.training_history
    }

    /// Get model weights
    pub fn get_model_weights(&self) -> &[f64] {
        &self.model_weights
    }

    /// Get number of base models
    pub fn get_num_models(&self) -> usize {
        self.base_models.len()
    }
}
