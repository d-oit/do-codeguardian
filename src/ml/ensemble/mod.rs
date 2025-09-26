//! # Ensemble Methods for CodeGuardian ML
//!
//! Advanced ensemble learning techniques including voting, stacking, and boosting
//! for improved model performance and robustness.

pub mod classifier;
pub mod config;
pub mod display;
pub mod meta_learners;

// Re-export main types
pub use classifier::*;
pub use config::*;
pub use display::*;
pub use meta_learners::*;

// Re-export for tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::cross_validation::{Classifier, ClassifierFactory};
    use crate::ml::training_data::TrainingDataset;
    use anyhow::Result;
    use std::collections::HashMap;

    // Mock classifier for testing
    #[derive(Clone)]
    struct MockClassifier {
        accuracy: f64,
        trained: bool,
    }

    impl MockClassifier {
        fn new(accuracy: f64) -> Self {
            Self {
                accuracy,
                trained: false,
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

            // Simulate prediction based on accuracy
            let correct_prediction = rand::random::<f64>() < self.accuracy;
            Ok(if correct_prediction { 0.8 } else { 0.2 })
        }
    }

    struct MockClassifierFactory {
        accuracies: Vec<f64>,
        index: std::cell::RefCell<usize>,
    }

    impl MockClassifierFactory {
        fn new(accuracies: Vec<f64>) -> Self {
            Self {
                accuracies,
                index: std::cell::RefCell::new(0),
            }
        }
    }

    impl ClassifierFactory<MockClassifier> for MockClassifierFactory {
        fn create(&self) -> Result<MockClassifier> {
            let mut idx = self.index.borrow_mut();
            let accuracy = self.accuracies.get(*idx).copied().unwrap_or(0.8);
            *idx = (*idx + 1) % self.accuracies.len();
            Ok(MockClassifier::new(accuracy))
        }
    }

    fn create_test_dataset(size: usize) -> TrainingDataset {
        let mut dataset = TrainingDataset::new();

        for i in 0..size {
            let features = vec![
                (i % 10) as f32 / 10.0,
                (i % 3) as f32 / 3.0,
                (i % 5) as f32 / 5.0,
                rand::random::<f32>(),
            ];
            let is_positive = i % 2 == 0;
            dataset.add_synthetic_example(&format!("test_{}", i), features, is_positive);
        }

        dataset
    }

    #[tokio::test]
    async fn test_majority_voting_ensemble() -> Result<(), Box<dyn std::error::Error>> {
        let config = EnsembleConfig {
            strategy: EnsembleStrategy::MajorityVoting,
            n_models: 3,
            ..Default::default()
        };

        let mut ensemble = EnsembleClassifier::new(config);
        let factory = MockClassifierFactory::new(vec![0.8, 0.7, 0.9]);
        let dataset = create_test_dataset(50);

        let result = ensemble.train(&factory, &dataset).await;
        assert!(result.is_ok());
        assert_eq!(ensemble.get_num_models(), 3);

        // Test prediction
        let features = vec![0.5, 0.3, 0.7, 0.2];
        let prediction = ensemble.predict(&features).await;
        assert!(prediction.is_ok());

        let pred_result = prediction?;
        assert_eq!(pred_result.individual_predictions.len(), 3);
        println!("{}", pred_result);
    }

    #[tokio::test]
    async fn test_weighted_voting_ensemble() -> Result<(), Box<dyn std::error::Error>> {
        let config = EnsembleConfig {
            strategy: EnsembleStrategy::WeightedVoting {
                weights: vec![0.5, 0.3, 0.2],
            },
            n_models: 3,
            ..Default::default()
        };

        let mut ensemble = EnsembleClassifier::new(config);
        let factory = MockClassifierFactory::new(vec![0.9, 0.8, 0.7]);
        let dataset = create_test_dataset(30);

        let result = ensemble.train(&factory, &dataset).await;
        assert!(result.is_ok());

        let weights = ensemble.get_model_weights();
        assert_eq!(weights.len(), 3);
        println!("Model weights: {:?}", weights);
    }

    #[tokio::test]
    async fn test_stacking_ensemble() -> Result<(), Box<dyn std::error::Error>> {
        let config = EnsembleConfig {
            strategy: EnsembleStrategy::Stacking {
                meta_learner_config: MetaLearnerConfig {
                    learner_type: MetaLearnerType::LogisticRegression,
                    parameters: HashMap::new(),
                    cv_folds: 3,
                },
            },
            n_models: 3,
            ..Default::default()
        };

        let mut ensemble = EnsembleClassifier::new(config);
        let factory = MockClassifierFactory::new(vec![0.8, 0.75, 0.85]);
        let dataset = create_test_dataset(60);

        let result = ensemble.train(&factory, &dataset).await;
        assert!(result.is_ok());

        // Test stacking prediction
        let features = vec![0.6, 0.4, 0.8, 0.1];
        let prediction = ensemble.predict(&features).await;
        assert!(prediction.is_ok());

        let pred_result = prediction?;
        println!("Stacking prediction: {}", pred_result);
    }

    #[tokio::test]
    async fn test_simple_linear_regression() -> Result<(), Box<dyn std::error::Error>> {
        let mut model = SimpleLinearRegression::new();

        let training_data = vec![
            (vec![1.0, 2.0], 0.8),
            (vec![2.0, 3.0], 0.9),
            (vec![0.5, 1.0], 0.3),
            (vec![1.5, 2.5], 0.7),
        ];

        let result = model.train(&training_data).await;
        assert!(result.is_ok());

        let prediction = model.predict(&[1.0, 2.0]).await;
        assert!(prediction.is_ok());

        let pred_value = prediction?;
        assert!(pred_value >= 0.0 && pred_value <= 1.0);
    }

    #[tokio::test]
    async fn test_simple_logistic_regression() -> Result<(), Box<dyn std::error::Error>> {
        let mut model = SimpleLogisticRegression::new();

        let training_data = vec![
            (vec![0.1, 0.2], 0.0),
            (vec![0.8, 0.9], 1.0),
            (vec![0.2, 0.1], 0.0),
            (vec![0.9, 0.8], 1.0),
        ];

        let result = model.train(&training_data).await;
        assert!(result.is_ok());

        let prediction = model.predict(&[0.7, 0.8]).await;
        assert!(prediction.is_ok());

        let pred_value = prediction?;
        assert!(pred_value >= 0.0 && pred_value <= 1.0);
    }

    #[tokio::test]
    async fn test_ensemble_diversity_metrics() -> Result<(), Box<dyn std::error::Error>> {
        let config = EnsembleConfig {
            strategy: EnsembleStrategy::MajorityVoting,
            n_models: 4,
            ..Default::default()
        };

        let mut ensemble = EnsembleClassifier::new(config);
        let factory = MockClassifierFactory::new(vec![0.9, 0.8, 0.85, 0.75]);
        let dataset = create_test_dataset(40);

        let result = ensemble.train(&factory, &dataset).await;
        assert!(result.is_ok());

        let history = ensemble.get_training_history();
        assert!(!history.diversity_metrics.is_empty());

        let diversity = &history.diversity_metrics[0];
        assert!(diversity.pairwise_diversity >= 0.0);
        assert!(diversity.disagreement >= 0.0);

        println!("Diversity metrics: {:?}", diversity);
        println!("{}", history);
    }
}
