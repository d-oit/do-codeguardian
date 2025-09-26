//! Integration of FANN classifier with cross-validation system

use super::cross_validation::{
    Classifier, ClassifierFactory, CrossValidationConfig, CrossValidator,
};
use super::fann_classifier::{FannClassifier, NetworkConfig};
use super::training_data::TrainingDataset;
use anyhow::Result;

/// FANN classifier wrapper for cross-validation
#[derive(Clone)]
pub struct FannCrossValidationWrapper {
    classifier: FannClassifier,
}

impl FannCrossValidationWrapper {
    pub fn new(config: NetworkConfig) -> Result<Self> {
        let classifier = FannClassifier::new(config)?;
        Ok(Self { classifier })
    }
}

impl Classifier for FannCrossValidationWrapper {
    async fn train(&mut self, data: &[(Vec<f32>, f32)]) -> Result<()> {
        self.classifier
            .train_batch(data, 100) // Default 100 epochs
            .map(|_| ()) // Ignore final error for this interface
    }

    async fn predict(&self, features: &[f32]) -> Result<f32> {
        self.classifier.predict(features)
    }
}

/// Factory for creating FANN classifiers
pub struct FannClassifierFactory {
    config: NetworkConfig,
}

impl FannClassifierFactory {
    pub fn new(config: NetworkConfig) -> Self {
        Self { config }
    }
}

impl ClassifierFactory<FannCrossValidationWrapper> for FannClassifierFactory {
    fn create(&self) -> Result<FannCrossValidationWrapper> {
        FannCrossValidationWrapper::new(self.config.clone())
    }
}

/// Convenience function for running cross-validation on FANN models
pub async fn validate_fann_model(
    network_config: NetworkConfig,
    cv_config: CrossValidationConfig,
    dataset: &TrainingDataset,
) -> Result<super::cross_validation::CrossValidationResults> {
    let factory = FannClassifierFactory::new(network_config);
    let mut validator = CrossValidator::new(cv_config);

    validator.validate(&factory, dataset).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::cross_validation::{CrossValidationConfig, ValidationStrategy};

    #[tokio::test]
    async fn test_fann_cross_validation_integration() -> Result<(), Box<dyn std::error::Error>> {
        // Create test dataset
        let mut dataset = TrainingDataset::new();
        for i in 0..50 {
            let features = vec![(i % 10) as f32 / 10.0, (i % 3) as f32, (i % 2) as f32];
            let is_positive = i % 2 == 0;
            dataset.add_synthetic_example(&format!("test_{}", i), features, is_positive);
        }

        // Configure cross-validation
        let mut cv_config = CrossValidationConfig::default();
        cv_config.k_folds = 3;
        cv_config.strategy = ValidationStrategy::StratifiedKFold;

        // Configure network
        let network_config = NetworkConfig::basic();

        // Run cross-validation
        let results = validate_fann_model(network_config, cv_config, &dataset).await;

        assert!(results.is_ok());
        let results = results?;
        assert_eq!(results.fold_results.len(), 3);

        println!("FANN Cross-Validation Results:");
        println!("{}", results);
    }
}
