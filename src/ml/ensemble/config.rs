use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ensemble strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleConfig {
    /// Ensemble strategy to use
    pub strategy: EnsembleStrategy,
    /// Number of base models
    pub n_models: usize,
    /// Model diversity settings
    pub diversity: DiversityConfig,
    /// Performance optimization settings
    pub optimization: OptimizationConfig,
}

/// Different ensemble strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnsembleStrategy {
    /// Simple majority voting
    MajorityVoting,
    /// Weighted voting based on model performance
    WeightedVoting { weights: Vec<f64> },
    /// Stacking with meta-learner
    Stacking {
        meta_learner_config: MetaLearnerConfig,
    },
    /// Adaptive boosting
    AdaBoost {
        learning_rate: f64,
        max_estimators: usize,
    },
    /// Gradient boosting
    GradientBoosting {
        learning_rate: f64,
        max_depth: usize,
    },
    /// Random forest-like approach
    RandomForest {
        max_features: Option<usize>,
        bootstrap: bool,
    },
}

/// Model diversity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityConfig {
    /// Use different random seeds for each model
    pub random_seeds: bool,
    /// Use different feature subsets
    pub feature_bagging: bool,
    /// Use different training data samples
    pub data_bagging: bool,
    /// Bagging fraction (0.0 to 1.0)
    pub bagging_fraction: f64,
    /// Feature subset fraction (0.0 to 1.0)
    pub feature_fraction: f64,
}

/// Performance optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable parallel training
    pub parallel_training: bool,
    /// Maximum number of parallel threads
    pub max_threads: usize,
    /// Enable early stopping
    pub early_stopping: bool,
    /// Early stopping patience
    pub patience: usize,
    /// Validation fraction for early stopping
    pub validation_fraction: f64,
}

/// Meta-learner configuration for stacking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearnerConfig {
    /// Type of meta-learner
    pub learner_type: MetaLearnerType,
    /// Meta-learner specific parameters
    pub parameters: HashMap<String, f64>,
    /// Cross-validation folds for meta-features
    pub cv_folds: usize,
}

/// Types of meta-learners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaLearnerType {
    /// Simple linear regression
    LinearRegression,
    /// Logistic regression
    LogisticRegression,
    /// Decision tree
    DecisionTree,
    /// Neural network
    NeuralNetwork,
}

impl Default for EnsembleConfig {
    fn default() -> Self {
        Self {
            strategy: EnsembleStrategy::WeightedVoting {
                weights: vec![1.0; 5], // Equal weights for 5 models
            },
            n_models: 5,
            diversity: DiversityConfig::default(),
            optimization: OptimizationConfig::default(),
        }
    }
}

impl Default for DiversityConfig {
    fn default() -> Self {
        Self {
            random_seeds: true,
            feature_bagging: true,
            data_bagging: true,
            bagging_fraction: 0.8,
            feature_fraction: 0.8,
        }
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            parallel_training: true,
            max_threads: num_cpus::get().min(8),
            early_stopping: true,
            patience: 10,
            validation_fraction: 0.2,
        }
    }
}
