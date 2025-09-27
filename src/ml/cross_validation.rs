//! # K-Fold Cross-Validation System
//!
//! Advanced cross-validation framework for robust model evaluation and validation.
//! Supports stratified K-fold, time-series validation, and statistical significance testing.

use crate::ml::training_data::TrainingDataset;
use crate::types::{Finding, Severity};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info, warn};

/// Cross-validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossValidationConfig {
    /// Number of folds (default: 5)
    pub k_folds: usize,
    /// Whether to use stratified sampling
    pub stratified: bool,
    /// Random seed for reproducibility
    pub random_state: Option<u64>,
    /// Whether to shuffle data before splitting
    pub shuffle: bool,
    /// Validation strategy type
    pub strategy: ValidationStrategy,
    /// Statistical significance threshold
    pub significance_threshold: f64,
}

/// Different validation strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStrategy {
    /// Standard K-fold cross-validation
    KFold,
    /// Stratified K-fold (maintains class distribution)
    StratifiedKFold,
    /// Time-series specific validation
    TimeSeriesSplit { gap: usize },
    /// Leave-one-out cross-validation
    LeaveOneOut,
    /// Custom validation with specific train/test splits
    Custom {
        splits: Vec<(Vec<usize>, Vec<usize>)>,
    },
}

impl Default for CrossValidationConfig {
    fn default() -> Self {
        Self {
            k_folds: 5,
            stratified: true,
            random_state: Some(42),
            shuffle: true,
            strategy: ValidationStrategy::StratifiedKFold,
            significance_threshold: 0.05,
        }
    }
}

/// Cross-validation results for a single fold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldResult {
    pub fold_id: usize,
    pub train_size: usize,
    pub test_size: usize,
    pub metrics: ValidationMetrics,
    pub training_time: Duration,
    pub inference_time: Duration,
    pub confusion_matrix: ConfusionMatrix,
}

/// Comprehensive validation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub specificity: f64,
    pub sensitivity: f64,
    pub auc_roc: f64,
    pub auc_pr: f64,
    pub log_loss: f64,
    pub matthews_correlation: f64,
}

/// Confusion matrix representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfusionMatrix {
    pub true_positives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub false_negatives: usize,
}

impl ConfusionMatrix {
    pub fn new() -> Self {
        Self {
            true_positives: 0,
            false_positives: 0,
            true_negatives: 0,
            false_negatives: 0,
        }
    }

    pub fn add_prediction(&mut self, actual: bool, predicted: bool) {
        match (actual, predicted) {
            (true, true) => self.true_positives += 1,
            (false, true) => self.false_positives += 1,
            (false, false) => self.true_negatives += 1,
            (true, false) => self.false_negatives += 1,
        }
    }

    pub fn total(&self) -> usize {
        self.true_positives + self.false_positives + self.true_negatives + self.false_negatives
    }
}

impl Default for ConfusionMatrix {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregated cross-validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossValidationResults {
    pub config: CrossValidationConfig,
    pub fold_results: Vec<FoldResult>,
    pub aggregated_metrics: AggregatedMetrics,
    pub statistical_tests: StatisticalTests,
    pub total_duration: Duration,
    pub recommendation: ValidationRecommendation,
}

/// Aggregated metrics across all folds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub mean_metrics: ValidationMetrics,
    pub std_metrics: ValidationMetrics,
    pub confidence_intervals: ConfidenceIntervals,
    pub best_fold: usize,
    pub worst_fold: usize,
}

/// Confidence intervals for metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceIntervals {
    pub accuracy: (f64, f64),
    pub precision: (f64, f64),
    pub recall: (f64, f64),
    pub f1_score: (f64, f64),
}

/// Statistical significance tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalTests {
    pub normality_test: NormalityTest,
    pub variance_test: VarianceTest,
    pub paired_t_test: Option<TTestResult>,
    pub wilcoxon_test: Option<WilcoxonResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalityTest {
    pub test_name: String,
    pub statistic: f64,
    pub p_value: f64,
    pub is_normal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarianceTest {
    pub test_name: String,
    pub statistic: f64,
    pub p_value: f64,
    pub equal_variance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTestResult {
    pub statistic: f64,
    pub p_value: f64,
    pub degrees_of_freedom: f64,
    pub significant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WilcoxonResult {
    pub statistic: f64,
    pub p_value: f64,
    pub significant: bool,
}

/// Validation recommendation based on results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRecommendation {
    pub overall_assessment: AssessmentLevel,
    pub recommendations: Vec<String>,
    pub concerns: Vec<String>,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssessmentLevel {
    Excellent,
    Good,
    Acceptable,
    Poor,
    Unacceptable,
}

/// Main cross-validation system
pub struct CrossValidator {
    config: CrossValidationConfig,
    rng: Option<rand::rngs::StdRng>,
}

impl CrossValidator {
    /// Create a new cross-validator with configuration
    pub fn new(config: CrossValidationConfig) -> Self {
        use rand::SeedableRng;

        let rng = config
            .random_state
            .map(|seed| rand::rngs::StdRng::seed_from_u64(seed));

        Self { config, rng }
    }

    /// Create a default cross-validator
    pub fn default() -> Self {
        Self::new(CrossValidationConfig::default())
    }

    /// Perform cross-validation on a classifier
    pub async fn validate<T>(
        &mut self,
        classifier_factory: &dyn ClassifierFactory<T>,
        dataset: &TrainingDataset,
    ) -> Result<CrossValidationResults>
    where
        T: Classifier + Clone,
    {
        let start_time = Instant::now();
        info!(
            "Starting {}-fold cross-validation with {} strategy",
            self.config.k_folds,
            format!("{:?}", self.config.strategy)
        );

        // Create data splits
        let splits = self.create_splits(dataset)?;
        let mut fold_results = Vec::new();

        for (fold_id, (train_indices, test_indices)) in splits.into_iter().enumerate() {
            info!("Processing fold {} of {}", fold_id + 1, self.config.k_folds);

            let fold_result = self
                .evaluate_fold(
                    classifier_factory,
                    dataset,
                    fold_id,
                    &train_indices,
                    &test_indices,
                )
                .await?;

            fold_results.push(fold_result);
        }

        let total_duration = start_time.elapsed();
        let results = self.aggregate_results(fold_results, total_duration)?;

        info!(
            "Cross-validation completed. Mean accuracy: {:.3} ± {:.3}",
            results.aggregated_metrics.mean_metrics.accuracy,
            results.aggregated_metrics.std_metrics.accuracy
        );

        Ok(results)
    }

    /// Create train/test splits based on validation strategy
    fn create_splits(
        &mut self,
        dataset: &TrainingDataset,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
        let data_size = dataset.len();
        let indices: Vec<usize> = (0..data_size).collect();

        match &self.config.strategy {
            ValidationStrategy::KFold => self.create_kfold_splits(&indices),
            ValidationStrategy::StratifiedKFold => self.create_stratified_splits(dataset, &indices),
            ValidationStrategy::TimeSeriesSplit { gap } => {
                self.create_timeseries_splits(&indices, *gap)
            }
            ValidationStrategy::LeaveOneOut => self.create_loo_splits(&indices),
            ValidationStrategy::Custom { splits } => Ok(splits.clone()),
        }
    }

    /// Standard K-fold splits
    fn create_kfold_splits(&mut self, indices: &[usize]) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
        let mut indices = indices.to_vec();

        if self.config.shuffle {
            if let Some(ref mut rng) = self.rng {
                indices.shuffle(rng);
            } else {
                indices.shuffle(&mut thread_rng());
            }
        }

        let fold_size = indices.len() / self.config.k_folds;
        let mut splits = Vec::new();

        for i in 0..self.config.k_folds {
            let test_start = i * fold_size;
            let test_end = if i == self.config.k_folds - 1 {
                indices.len() // Include remaining items in last fold
            } else {
                (i + 1) * fold_size
            };

            let test_indices = indices[test_start..test_end].to_vec();
            let train_indices = indices[..test_start]
                .iter()
                .chain(indices[test_end..].iter())
                .cloned()
                .collect();

            splits.push((train_indices, test_indices));
        }

        Ok(splits)
    }

    /// Stratified K-fold splits (maintains class distribution)
    fn create_stratified_splits(
        &mut self,
        dataset: &TrainingDataset,
        indices: &[usize],
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
        // Group indices by class label
        let mut class_indices: HashMap<bool, Vec<usize>> = HashMap::new();

        for &idx in indices {
            let sample = dataset.get_sample(idx)?;
            let label = sample.1 > 0.5; // Convert to boolean
            class_indices.entry(label).or_default().push(idx);
        }

        // Shuffle each class separately
        for (_, indices) in class_indices.iter_mut() {
            if self.config.shuffle {
                if let Some(ref mut rng) = self.rng {
                    indices.shuffle(rng);
                } else {
                    indices.shuffle(&mut thread_rng());
                }
            }
        }

        let mut splits = Vec::new();

        for fold_id in 0..self.config.k_folds {
            let mut train_indices = Vec::new();
            let mut test_indices = Vec::new();

            for (_, class_idx) in &class_indices {
                let fold_size = class_idx.len() / self.config.k_folds;
                let test_start = fold_id * fold_size;
                let test_end = if fold_id == self.config.k_folds - 1 {
                    class_idx.len()
                } else {
                    (fold_id + 1) * fold_size
                };

                test_indices.extend(&class_idx[test_start..test_end]);
                train_indices.extend(&class_idx[..test_start]);
                train_indices.extend(&class_idx[test_end..]);
            }

            splits.push((train_indices, test_indices));
        }

        Ok(splits)
    }

    /// Time-series splits with gap to prevent data leakage
    fn create_timeseries_splits(
        &self,
        indices: &[usize],
        gap: usize,
    ) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
        let mut splits = Vec::new();
        let total_size = indices.len();
        let test_size = total_size / (self.config.k_folds + 1);

        for i in 0..self.config.k_folds {
            let test_start = (i + 1) * test_size;
            let test_end = std::cmp::min(test_start + test_size, total_size);
            let train_end = test_start.saturating_sub(gap);

            if train_end < test_size {
                continue; // Skip if not enough training data
            }

            let train_indices = indices[0..train_end].to_vec();
            let test_indices = indices[test_start..test_end].to_vec();

            splits.push((train_indices, test_indices));
        }

        if splits.is_empty() {
            return Err(anyhow!(
                "Not enough data for time-series validation with gap {}",
                gap
            ));
        }

        Ok(splits)
    }

    /// Leave-one-out splits
    fn create_loo_splits(&self, indices: &[usize]) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
        let mut splits = Vec::new();

        for i in 0..indices.len() {
            let test_indices = vec![indices[i]];
            let train_indices = indices[..i]
                .iter()
                .chain(indices[i + 1..].iter())
                .cloned()
                .collect();

            splits.push((train_indices, test_indices));
        }

        Ok(splits)
    }

    /// Evaluate a single fold
    async fn evaluate_fold<T>(
        &self,
        classifier_factory: &dyn ClassifierFactory<T>,
        dataset: &TrainingDataset,
        fold_id: usize,
        train_indices: &[usize],
        test_indices: &[usize],
    ) -> Result<FoldResult>
    where
        T: Classifier + Clone + Send + Sync,
    {
        let train_start = Instant::now();

        // Create and train classifier
        let mut classifier = classifier_factory.create()?;
        let train_data = dataset.get_samples_by_indices(train_indices)?;
        classifier.train(&train_data).await?;

        let training_time = train_start.elapsed();
        let inference_start = Instant::now();

        // Evaluate on test set
        let test_data = dataset.get_samples_by_indices(test_indices)?;
        let mut confusion_matrix = ConfusionMatrix::new();
        let mut predictions = Vec::new();
        let mut actual_labels = Vec::new();

        for (features, label) in &test_data {
            let prediction = classifier.predict(features).await?;
            let predicted_label = prediction > 0.5;
            let actual_label = *label > 0.5;

            confusion_matrix.add_prediction(actual_label, predicted_label);
            predictions.push(prediction);
            actual_labels.push(*label);
        }

        let inference_time = inference_start.elapsed();
        let metrics = self.calculate_metrics(&confusion_matrix, &predictions, &actual_labels)?;

        Ok(FoldResult {
            fold_id,
            train_size: train_indices.len(),
            test_size: test_indices.len(),
            metrics,
            training_time,
            inference_time,
            confusion_matrix,
        })
    }

    /// Calculate comprehensive metrics from predictions
    fn calculate_metrics(
        &self,
        confusion_matrix: &ConfusionMatrix,
        predictions: &[f32],
        actual_labels: &[f32],
    ) -> Result<ValidationMetrics> {
        let tp = confusion_matrix.true_positives as f64;
        let fp = confusion_matrix.false_positives as f64;
        let tn = confusion_matrix.true_negatives as f64;
        let fn_count = confusion_matrix.false_negatives as f64;
        let total = tp + fp + tn + fn_count;

        if total == 0.0 {
            return Err(anyhow!("No test samples available"));
        }

        let accuracy = (tp + tn) / total;
        let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 0.0 };
        let recall = if tp + fn_count > 0.0 {
            tp / (tp + fn_count)
        } else {
            0.0
        };
        let specificity = if tn + fp > 0.0 { tn / (tn + fp) } else { 0.0 };
        let sensitivity = recall;

        let f1_score = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        // Calculate Matthews correlation coefficient
        let denominator = ((tp + fp) * (tp + fn_count) * (tn + fp) * (tn + fn_count)).sqrt();
        let matthews_correlation = if denominator > 0.0 {
            (tp * tn - fp * fn_count) / denominator
        } else {
            0.0
        };

        // Calculate AUC-ROC and AUC-PR (simplified implementations)
        let auc_roc = self.calculate_auc_roc(predictions, actual_labels)?;
        let auc_pr = self.calculate_auc_pr(predictions, actual_labels)?;
        let log_loss = self.calculate_log_loss(predictions, actual_labels)?;

        Ok(ValidationMetrics {
            accuracy,
            precision,
            recall,
            f1_score,
            specificity,
            sensitivity,
            auc_roc,
            auc_pr,
            log_loss,
            matthews_correlation,
        })
    }

    /// Calculate AUC-ROC using trapezoidal rule
    fn calculate_auc_roc(&self, predictions: &[f32], actual_labels: &[f32]) -> Result<f64> {
        if predictions.len() != actual_labels.len() {
            return Err(anyhow!("Predictions and labels length mismatch"));
        }

        let mut data: Vec<(f32, f32)> = predictions
            .iter()
            .zip(actual_labels.iter())
            .map(|(&p, &l)| (p, l))
            .collect();
        data.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut tpr_values = Vec::new();
        let mut fpr_values = Vec::new();

        let positive_count = actual_labels.iter().filter(|&&x| x > 0.5).count() as f64;
        let negative_count = actual_labels.len() as f64 - positive_count;

        if positive_count == 0.0 || negative_count == 0.0 {
            return Ok(0.5); // Random classifier performance
        }

        let mut tp = 0.0;
        let mut fp = 0.0;

        for (_, label) in &data {
            if *label > 0.5 {
                tp += 1.0;
            } else {
                fp += 1.0;
            }

            tpr_values.push(tp / positive_count);
            fpr_values.push(fp / negative_count);
        }

        // Calculate AUC using trapezoidal rule
        let mut auc = 0.0;
        for i in 1..fpr_values.len() {
            let width = fpr_values[i] - fpr_values[i - 1];
            let height = (tpr_values[i] + tpr_values[i - 1]) / 2.0;
            auc += width * height;
        }

        Ok(auc.clamp(0.0, 1.0))
    }

    /// Calculate AUC-PR (Precision-Recall curve)
    fn calculate_auc_pr(&self, predictions: &[f32], actual_labels: &[f32]) -> Result<f64> {
        let mut data: Vec<(f32, f32)> = predictions
            .iter()
            .zip(actual_labels.iter())
            .map(|(&p, &l)| (p, l))
            .collect();
        data.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let positive_count = actual_labels.iter().filter(|&&x| x > 0.5).count() as f64;
        if positive_count == 0.0 {
            return Ok(0.0);
        }

        let mut precision_values = Vec::new();
        let mut recall_values = Vec::new();
        let mut tp = 0.0;
        let mut fp = 0.0;

        for (_, label) in &data {
            if *label > 0.5 {
                tp += 1.0;
            } else {
                fp += 1.0;
            }

            let precision = tp / (tp + fp);
            let recall = tp / positive_count;

            precision_values.push(precision);
            recall_values.push(recall);
        }

        // Calculate AUC-PR using trapezoidal rule
        let mut auc_pr = 0.0;
        for i in 1..recall_values.len() {
            let width = recall_values[i] - recall_values[i - 1];
            let height = (precision_values[i] + precision_values[i - 1]) / 2.0;
            auc_pr += width * height;
        }

        Ok(auc_pr.clamp(0.0, 1.0))
    }

    /// Calculate log loss
    fn calculate_log_loss(&self, predictions: &[f32], actual_labels: &[f32]) -> Result<f64> {
        if predictions.is_empty() {
            return Ok(0.0);
        }

        let mut log_loss = 0.0;
        let epsilon = 1e-15; // Prevent log(0)

        for (&pred, &actual) in predictions.iter().zip(actual_labels.iter()) {
            let pred_clamped = (pred as f64).clamp(epsilon, 1.0 - epsilon);
            let actual_binary = if actual > 0.5 { 1.0 } else { 0.0 };

            log_loss += actual_binary * pred_clamped.ln()
                + (1.0 - actual_binary) * (1.0 - pred_clamped).ln();
        }

        Ok(-log_loss / predictions.len() as f64)
    }

    /// Aggregate results from all folds
    fn aggregate_results(
        &self,
        fold_results: Vec<FoldResult>,
        total_duration: Duration,
    ) -> Result<CrossValidationResults> {
        if fold_results.is_empty() {
            return Err(anyhow!("No fold results to aggregate"));
        }

        let aggregated_metrics = self.calculate_aggregated_metrics(&fold_results)?;
        let statistical_tests = self.perform_statistical_tests(&fold_results)?;
        let recommendation = self.generate_recommendation(&aggregated_metrics, &statistical_tests);

        Ok(CrossValidationResults {
            config: self.config.clone(),
            fold_results,
            aggregated_metrics,
            statistical_tests,
            total_duration,
            recommendation,
        })
    }

    /// Calculate aggregated metrics across folds
    fn calculate_aggregated_metrics(
        &self,
        fold_results: &[FoldResult],
    ) -> Result<AggregatedMetrics> {
        let n_folds = fold_results.len() as f64;

        // Extract metrics from each fold
        let accuracies: Vec<f64> = fold_results.iter().map(|f| f.metrics.accuracy).collect();
        let precisions: Vec<f64> = fold_results.iter().map(|f| f.metrics.precision).collect();
        let recalls: Vec<f64> = fold_results.iter().map(|f| f.metrics.recall).collect();
        let f1_scores: Vec<f64> = fold_results.iter().map(|f| f.metrics.f1_score).collect();

        // Calculate means
        let mean_metrics = ValidationMetrics {
            accuracy: accuracies.iter().sum::<f64>() / n_folds,
            precision: precisions.iter().sum::<f64>() / n_folds,
            recall: recalls.iter().sum::<f64>() / n_folds,
            f1_score: f1_scores.iter().sum::<f64>() / n_folds,
            specificity: fold_results
                .iter()
                .map(|f| f.metrics.specificity)
                .sum::<f64>()
                / n_folds,
            sensitivity: fold_results
                .iter()
                .map(|f| f.metrics.sensitivity)
                .sum::<f64>()
                / n_folds,
            auc_roc: fold_results.iter().map(|f| f.metrics.auc_roc).sum::<f64>() / n_folds,
            auc_pr: fold_results.iter().map(|f| f.metrics.auc_pr).sum::<f64>() / n_folds,
            log_loss: fold_results.iter().map(|f| f.metrics.log_loss).sum::<f64>() / n_folds,
            matthews_correlation: fold_results
                .iter()
                .map(|f| f.metrics.matthews_correlation)
                .sum::<f64>()
                / n_folds,
        };

        // Calculate standard deviations
        let std_metrics = ValidationMetrics {
            accuracy: self.calculate_std(&accuracies, mean_metrics.accuracy),
            precision: self.calculate_std(&precisions, mean_metrics.precision),
            recall: self.calculate_std(&recalls, mean_metrics.recall),
            f1_score: self.calculate_std(&f1_scores, mean_metrics.f1_score),
            specificity: self.calculate_std(
                &fold_results
                    .iter()
                    .map(|f| f.metrics.specificity)
                    .collect::<Vec<_>>(),
                mean_metrics.specificity,
            ),
            sensitivity: self.calculate_std(
                &fold_results
                    .iter()
                    .map(|f| f.metrics.sensitivity)
                    .collect::<Vec<_>>(),
                mean_metrics.sensitivity,
            ),
            auc_roc: self.calculate_std(
                &fold_results
                    .iter()
                    .map(|f| f.metrics.auc_roc)
                    .collect::<Vec<_>>(),
                mean_metrics.auc_roc,
            ),
            auc_pr: self.calculate_std(
                &fold_results
                    .iter()
                    .map(|f| f.metrics.auc_pr)
                    .collect::<Vec<_>>(),
                mean_metrics.auc_pr,
            ),
            log_loss: self.calculate_std(
                &fold_results
                    .iter()
                    .map(|f| f.metrics.log_loss)
                    .collect::<Vec<_>>(),
                mean_metrics.log_loss,
            ),
            matthews_correlation: self.calculate_std(
                &fold_results
                    .iter()
                    .map(|f| f.metrics.matthews_correlation)
                    .collect::<Vec<_>>(),
                mean_metrics.matthews_correlation,
            ),
        };

        // Calculate 95% confidence intervals
        let confidence_intervals = ConfidenceIntervals {
            accuracy: self.calculate_confidence_interval(&accuracies, 0.95),
            precision: self.calculate_confidence_interval(&precisions, 0.95),
            recall: self.calculate_confidence_interval(&recalls, 0.95),
            f1_score: self.calculate_confidence_interval(&f1_scores, 0.95),
        };

        // Find best and worst folds
        let best_fold = fold_results
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.metrics.f1_score.partial_cmp(&b.metrics.f1_score)?)
            .map(|(i, _)| i)
            .unwrap_or(0);

        let worst_fold = fold_results
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.metrics.f1_score.partial_cmp(&b.metrics.f1_score)?)
            .map(|(i, _)| i)
            .unwrap_or(0);

        Ok(AggregatedMetrics {
            mean_metrics,
            std_metrics,
            confidence_intervals,
            best_fold,
            worst_fold,
        })
    }

    /// Calculate standard deviation
    fn calculate_std(&self, values: &[f64], mean: f64) -> f64 {
        if values.len() <= 1 {
            return 0.0;
        }

        let variance =
            values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;

        variance.sqrt()
    }

    /// Calculate confidence interval using t-distribution
    fn calculate_confidence_interval(&self, values: &[f64], confidence: f64) -> (f64, f64) {
        if values.len() <= 1 {
            let val = values.first().copied().unwrap_or(0.0);
            return (val, val);
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let std = self.calculate_std(values, mean);
        let n = values.len() as f64;

        // Simplified t-value for 95% confidence (approximation)
        let t_value = match values.len() {
            2 => 12.706,
            3 => 4.303,
            4 => 3.182,
            5 => 2.776,
            _ => 2.045, // Approximation for larger samples
        };

        let margin = t_value * std / n.sqrt();
        (mean - margin, mean + margin)
    }

    /// Perform statistical tests
    fn perform_statistical_tests(&self, fold_results: &[FoldResult]) -> Result<StatisticalTests> {
        let accuracies: Vec<f64> = fold_results.iter().map(|f| f.metrics.accuracy).collect();

        // Simplified normality test (Shapiro-Wilk approximation)
        let normality_test = self.perform_normality_test(&accuracies);

        // Variance test (simplified)
        let variance_test = self.perform_variance_test(&accuracies);

        // Skip t-test and Wilcoxon for now (would need baseline to compare against)
        Ok(StatisticalTests {
            normality_test,
            variance_test,
            paired_t_test: None,
            wilcoxon_test: None,
        })
    }

    /// Simplified normality test
    fn perform_normality_test(&self, values: &[f64]) -> NormalityTest {
        if values.len() < 3 {
            return NormalityTest {
                test_name: "Shapiro-Wilk".to_string(),
                statistic: 0.0,
                p_value: 1.0,
                is_normal: true,
            };
        }

        // Simplified implementation - in practice, use a proper statistical library
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;

        // Mock statistic - replace with actual Shapiro-Wilk implementation
        let statistic = 1.0 - variance / (0.25 * 0.25); // Normalized variance
        let p_value = if statistic > 0.95 { 0.1 } else { 0.01 };

        NormalityTest {
            test_name: "Shapiro-Wilk".to_string(),
            statistic,
            p_value,
            is_normal: p_value > self.config.significance_threshold,
        }
    }

    /// Simplified variance test
    fn perform_variance_test(&self, values: &[f64]) -> VarianceTest {
        if values.len() < 2 {
            return VarianceTest {
                test_name: "Levene".to_string(),
                statistic: 0.0,
                p_value: 1.0,
                equal_variance: true,
            };
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;

        // Mock test - replace with actual Levene's test
        let statistic = variance * 10.0; // Simplified
        let p_value = if variance < 0.01 { 0.1 } else { 0.01 };

        VarianceTest {
            test_name: "Levene".to_string(),
            statistic,
            p_value,
            equal_variance: p_value > self.config.significance_threshold,
        }
    }

    /// Generate validation recommendation
    fn generate_recommendation(
        &self,
        metrics: &AggregatedMetrics,
        tests: &StatisticalTests,
    ) -> ValidationRecommendation {
        let mut recommendations = Vec::new();
        let mut concerns = Vec::new();

        let mean_accuracy = metrics.mean_metrics.accuracy;
        let std_accuracy = metrics.std_metrics.accuracy;

        // Assess overall performance
        let assessment = if mean_accuracy >= 0.9 && std_accuracy < 0.05 {
            recommendations.push("Excellent model performance with low variance".to_string());
            AssessmentLevel::Excellent
        } else if mean_accuracy >= 0.8 && std_accuracy < 0.1 {
            recommendations
                .push("Good model performance, consider minor optimizations".to_string());
            AssessmentLevel::Good
        } else if mean_accuracy >= 0.7 && std_accuracy < 0.15 {
            recommendations.push("Acceptable performance, but room for improvement".to_string());
            AssessmentLevel::Acceptable
        } else if mean_accuracy >= 0.6 {
            concerns.push("Model performance is below expectations".to_string());
            recommendations.push(
                "Consider feature engineering, hyperparameter tuning, or more training data"
                    .to_string(),
            );
            AssessmentLevel::Poor
        } else {
            concerns.push("Model performance is unacceptable for production use".to_string());
            recommendations
                .push("Significant model improvements needed before deployment".to_string());
            AssessmentLevel::Unacceptable
        };

        // Check variance concerns
        if std_accuracy > 0.1 {
            concerns.push("High variance across folds indicates potential overfitting".to_string());
            recommendations
                .push("Consider regularization techniques or more robust validation".to_string());
        }

        // Statistical test recommendations
        if !tests.normality_test.is_normal {
            recommendations.push(
                "Consider non-parametric statistical tests due to non-normal distribution"
                    .to_string(),
            );
        }

        let confidence_level = if mean_accuracy > 0.8 && std_accuracy < 0.1 {
            0.9
        } else {
            0.7
        };

        ValidationRecommendation {
            overall_assessment: assessment,
            recommendations,
            concerns,
            confidence_level,
        }
    }
}

/// Trait for classifiers that can be cross-validated
#[async_trait::async_trait(?Send)]
pub trait Classifier {
    async fn train(&mut self, data: &[(Vec<f32>, f32)]) -> Result<()>;
    async fn predict(&self, features: &[f32]) -> Result<f32>;
}

/// Factory for creating classifiers
pub trait ClassifierFactory<T: Classifier> {
    fn create(&self) -> Result<T>;
}

/// Extension trait for TrainingDataset to support cross-validation
impl TrainingDataset {
    /// Get the number of samples in the dataset
    pub fn len(&self) -> usize {
        self.examples.len()
    }

    /// Check if the dataset is empty
    pub fn is_empty(&self) -> bool {
        self.examples.is_empty()
    }

    /// Get a sample by index
    pub fn get_sample(&self, index: usize) -> Result<(Vec<f32>, f32)> {
        if index >= self.examples.len() {
            return Err(anyhow!(
                "Index {} out of bounds for dataset with {} samples",
                index,
                self.examples.len()
            ));
        }

        let example = &self.examples[index];
        Ok((
            example.features.clone(),
            if example.is_true_positive { 1.0 } else { 0.0 },
        ))
    }

    /// Get multiple samples by indices
    pub fn get_samples_by_indices(&self, indices: &[usize]) -> Result<Vec<(Vec<f32>, f32)>> {
        let mut samples = Vec::with_capacity(indices.len());
        for &index in indices {
            samples.push(self.get_sample(index)?);
        }
        Ok(samples)
    }
}

/// Display implementation for CrossValidationResults
impl std::fmt::Display for CrossValidationResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📊 Cross-Validation Results")?;
        writeln!(
            f,
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        )?;
        writeln!(
            f,
            "Configuration: {}-fold {:?}",
            self.config.k_folds, self.config.strategy
        )?;
        writeln!(
            f,
            "Total Duration: {:.2}s",
            self.total_duration.as_secs_f64()
        )?;
        writeln!(f)?;

        let metrics = &self.aggregated_metrics.mean_metrics;
        let std_metrics = &self.aggregated_metrics.std_metrics;

        writeln!(f, "📈 Aggregated Metrics (Mean ± Std):")?;
        writeln!(
            f,
            "  Accuracy:    {:.3} ± {:.3}",
            metrics.accuracy, std_metrics.accuracy
        )?;
        writeln!(
            f,
            "  Precision:   {:.3} ± {:.3}",
            metrics.precision, std_metrics.precision
        )?;
        writeln!(
            f,
            "  Recall:      {:.3} ± {:.3}",
            metrics.recall, std_metrics.recall
        )?;
        writeln!(
            f,
            "  F1-Score:    {:.3} ± {:.3}",
            metrics.f1_score, std_metrics.f1_score
        )?;
        writeln!(
            f,
            "  AUC-ROC:     {:.3} ± {:.3}",
            metrics.auc_roc, std_metrics.auc_roc
        )?;
        writeln!(
            f,
            "  AUC-PR:      {:.3} ± {:.3}",
            metrics.auc_pr, std_metrics.auc_pr
        )?;
        writeln!(
            f,
            "  Matthews CC: {:.3} ± {:.3}",
            metrics.matthews_correlation, std_metrics.matthews_correlation
        )?;
        writeln!(f)?;

        writeln!(f, "🎯 Confidence Intervals (95%):")?;
        let ci = &self.aggregated_metrics.confidence_intervals;
        writeln!(
            f,
            "  Accuracy:  [{:.3}, {:.3}]",
            ci.accuracy.0, ci.accuracy.1
        )?;
        writeln!(
            f,
            "  Precision: [{:.3}, {:.3}]",
            ci.precision.0, ci.precision.1
        )?;
        writeln!(f, "  Recall:    [{:.3}, {:.3}]", ci.recall.0, ci.recall.1)?;
        writeln!(
            f,
            "  F1-Score:  [{:.3}, {:.3}]",
            ci.f1_score.0, ci.f1_score.1
        )?;
        writeln!(f)?;

        writeln!(f, "📋 Per-Fold Results:")?;
        for (i, fold) in self.fold_results.iter().enumerate() {
            let marker = if i == self.aggregated_metrics.best_fold {
                "🏆"
            } else if i == self.aggregated_metrics.worst_fold {
                "⚠️"
            } else {
                "  "
            };
            writeln!(
                f,
                "  {} Fold {}: Acc={:.3}, F1={:.3}, Train={}ms, Test={}ms",
                marker,
                i + 1,
                fold.metrics.accuracy,
                fold.metrics.f1_score,
                fold.training_time.as_millis(),
                fold.inference_time.as_millis()
            )?;
        }
        writeln!(f)?;

        writeln!(f, "🔬 Statistical Tests:")?;
        let tests = &self.statistical_tests;
        writeln!(
            f,
            "  Normality ({}): p={:.3} ({})",
            tests.normality_test.test_name,
            tests.normality_test.p_value,
            if tests.normality_test.is_normal {
                "Normal"
            } else {
                "Non-normal"
            }
        )?;
        writeln!(
            f,
            "  Variance ({}): p={:.3} ({})",
            tests.variance_test.test_name,
            tests.variance_test.p_value,
            if tests.variance_test.equal_variance {
                "Equal"
            } else {
                "Unequal"
            }
        )?;
        writeln!(f)?;

        writeln!(
            f,
            "💡 Assessment: {:?} (Confidence: {:.1}%)",
            self.recommendation.overall_assessment,
            self.recommendation.confidence_level * 100.0
        )?;

        if !self.recommendation.concerns.is_empty() {
            writeln!(f, "⚠️  Concerns:")?;
            for concern in &self.recommendation.concerns {
                writeln!(f, "   • {}", concern)?;
            }
        }

        if !self.recommendation.recommendations.is_empty() {
            writeln!(f, "✨ Recommendations:")?;
            for rec in &self.recommendation.recommendations {
                writeln!(f, "   • {}", rec)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::fann_classifier::{FannClassifier, NetworkConfig};

    // Mock classifier for testing
    #[derive(Clone)]
    struct MockClassifier {
        trained: bool,
        accuracy: f64,
    }

    impl MockClassifier {
        fn new(accuracy: f64) -> Self {
            Self {
                trained: false,
                accuracy,
            }
        }
    }

    impl Classifier for MockClassifier {
        async fn train(&mut self, _data: &[(Vec<f32>, f32)]) -> Result<()> {
            self.trained = true;
            Ok(())
        }

        async fn predict(&self, _features: &[f32]) -> Result<f32> {
            if !self.trained {
                return Err(anyhow!("Model not trained"));
            }
            // Return prediction based on accuracy (simplified)
            Ok(if rand::random::<f64>() < self.accuracy {
                1.0
            } else {
                0.0
            })
        }
    }

    struct MockClassifierFactory {
        accuracy: f64,
    }

    impl MockClassifierFactory {
        fn new(accuracy: f64) -> Self {
            Self { accuracy }
        }
    }

    impl ClassifierFactory<MockClassifier> for MockClassifierFactory {
        fn create(&self) -> Result<MockClassifier> {
            Ok(MockClassifier::new(self.accuracy))
        }
    }

    fn create_test_dataset(size: usize) -> TrainingDataset {
        let mut dataset = TrainingDataset::new();

        for i in 0..size {
            let features = vec![i as f32 / size as f32, (i % 2) as f32, (i % 3) as f32];
            let is_positive = i % 2 == 0;
            dataset.add_synthetic_example(&format!("test_{}", i), features, is_positive);
        }

        dataset
    }

    #[tokio::test]
    async fn test_kfold_cross_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = CrossValidationConfig::default();
        config.k_folds = 3;
        config.strategy = ValidationStrategy::KFold;

        let mut validator = CrossValidator::new(config);
        let dataset = create_test_dataset(30);
        let factory = MockClassifierFactory::new(0.8);

        let results = validator.validate(&factory, &dataset).await?;

        assert_eq!(results.fold_results.len(), 3);
        assert!(results.aggregated_metrics.mean_metrics.accuracy > 0.0);
        println!("{}", results);
    }

    #[tokio::test]
    async fn test_stratified_cross_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = CrossValidationConfig::default();
        config.k_folds = 5;
        config.strategy = ValidationStrategy::StratifiedKFold;

        let mut validator = CrossValidator::new(config);
        let dataset = create_test_dataset(50);
        let factory = MockClassifierFactory::new(0.85);

        let results = validator.validate(&factory, &dataset).await?;

        assert_eq!(results.fold_results.len(), 5);
        assert!(results.aggregated_metrics.mean_metrics.accuracy > 0.0);

        // Check that we have both positive and negative samples in each fold
        for fold in &results.fold_results {
            assert!(fold.train_size > 0);
            assert!(fold.test_size > 0);
        }
    }

    #[tokio::test]
    async fn test_timeseries_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = CrossValidationConfig::default();
        config.k_folds = 3;
        config.strategy = ValidationStrategy::TimeSeriesSplit { gap: 2 };

        let mut validator = CrossValidator::new(config);
        let dataset = create_test_dataset(100);
        let factory = MockClassifierFactory::new(0.75);

        let results = validator.validate(&factory, &dataset).await?;

        assert!(!results.fold_results.is_empty());

        // Time series validation should have chronological splits
        for fold in &results.fold_results {
            assert!(fold.train_size > 0);
            assert!(fold.test_size > 0);
        }
    }

    #[test]
    fn test_confusion_matrix() -> Result<(), Box<dyn std::error::Error>> {
        let mut cm = ConfusionMatrix::new();

        cm.add_prediction(true, true); // TP
        cm.add_prediction(false, false); // TN
        cm.add_prediction(true, false); // FN
        cm.add_prediction(false, true); // FP

        assert_eq!(cm.true_positives, 1);
        assert_eq!(cm.true_negatives, 1);
        assert_eq!(cm.false_negatives, 1);
        assert_eq!(cm.false_positives, 1);
        assert_eq!(cm.total(), 4);
    }

    #[test]
    fn test_validation_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = CrossValidationConfig::default();

        assert_eq!(config.k_folds, 5);
        assert!(config.stratified);
        assert_eq!(config.random_state, Some(42));
        assert_eq!(config.strategy, ValidationStrategy::StratifiedKFold);
    }

    #[test]
    fn test_assessment_levels() -> Result<(), Box<dyn std::error::Error>> {
        let config = CrossValidationConfig::default();
        let validator = CrossValidator::new(config);

        // Test excellent performance
        let excellent_metrics = AggregatedMetrics {
            mean_metrics: ValidationMetrics {
                accuracy: 0.95,
                precision: 0.94,
                recall: 0.96,
                f1_score: 0.95,
                specificity: 0.94,
                sensitivity: 0.96,
                auc_roc: 0.98,
                auc_pr: 0.97,
                log_loss: 0.05,
                matthews_correlation: 0.90,
            },
            std_metrics: ValidationMetrics {
                accuracy: 0.02,
                precision: 0.03,
                recall: 0.02,
                f1_score: 0.02,
                specificity: 0.03,
                sensitivity: 0.02,
                auc_roc: 0.01,
                auc_pr: 0.02,
                log_loss: 0.01,
                matthews_correlation: 0.03,
            },
            confidence_intervals: ConfidenceIntervals {
                accuracy: (0.93, 0.97),
                precision: (0.91, 0.97),
                recall: (0.94, 0.98),
                f1_score: (0.93, 0.97),
            },
            best_fold: 0,
            worst_fold: 1,
        };

        let tests = StatisticalTests {
            normality_test: NormalityTest {
                test_name: "Shapiro-Wilk".to_string(),
                statistic: 0.98,
                p_value: 0.1,
                is_normal: true,
            },
            variance_test: VarianceTest {
                test_name: "Levene".to_string(),
                statistic: 0.5,
                p_value: 0.1,
                equal_variance: true,
            },
            paired_t_test: None,
            wilcoxon_test: None,
        };

        let recommendation = validator.generate_recommendation(&excellent_metrics, &tests);
        assert_eq!(
            recommendation.overall_assessment,
            AssessmentLevel::Excellent
        );
        assert!(recommendation.confidence_level > 0.8);
    }
}
