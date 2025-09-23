#![cfg(feature = "ml")]

//! Example demonstrating the Ensemble Methods system in CodeGuardian
//!
//! This example shows how to:
//! 1. Create diverse base models
//! 2. Configure different ensemble strategies
//! 3. Train ensemble classifiers
//! 4. Compare ensemble performance

use anyhow::Result;
use do_codeguardian::ml::cross_validation::{Classifier, ClassifierFactory};
use do_codeguardian::ml::ensemble::{
    DiversityConfig, EnsembleClassifier, EnsembleConfig, EnsembleStrategy, MetaLearnerConfig,
    MetaLearnerType,
};
use do_codeguardian::ml::fann_classifier::{FannClassifier, NetworkConfig};
use do_codeguardian::ml::training_data::TrainingDataset;
use std::collections::HashMap;
use std::time::Instant;

/// FANN classifier wrapper for ensemble
#[derive(Clone)]
struct FannEnsembleWrapper {
    classifier: FannClassifier,
    config: NetworkConfig,
}

impl FannEnsembleWrapper {
    fn new(config: NetworkConfig) -> Result<Self> {
        let classifier = FannClassifier::new(config.clone())?;
        Ok(Self { classifier, config })
    }
}

impl Classifier for FannEnsembleWrapper {
    async fn train(&mut self, data: &[(Vec<f32>, f32)]) -> Result<()> {
        self.classifier.train_batch(data, 200)?; // More epochs for better training
        Ok(())
    }

    async fn predict(&self, features: &[f32]) -> Result<f32> {
        self.classifier.predict(features)
    }
}

/// Factory for creating diverse FANN classifiers
struct DiverseFannFactory {
    configs: Vec<NetworkConfig>,
    index: std::cell::RefCell<usize>,
}

impl DiverseFannFactory {
    fn new() -> Self {
        let configs = vec![
            NetworkConfig::basic(),
            NetworkConfig {
                input_size: 8,
                hidden_layers: vec![16, 8],
                output_size: 1,
                activation_function: "sigmoid".to_string(),
            },
            NetworkConfig {
                input_size: 8,
                hidden_layers: vec![10, 6, 4],
                output_size: 1,
                activation_function: "sigmoid".to_string(),
            },
            NetworkConfig {
                input_size: 8,
                hidden_layers: vec![20, 10],
                output_size: 1,
                activation_function: "sigmoid".to_string(),
            },
            NetworkConfig {
                input_size: 8,
                hidden_layers: vec![12],
                output_size: 1,
                activation_function: "sigmoid".to_string(),
            },
        ];

        Self {
            configs,
            index: std::cell::RefCell::new(0),
        }
    }
}

impl ClassifierFactory<FannEnsembleWrapper> for DiverseFannFactory {
    fn create(&self) -> Result<FannEnsembleWrapper> {
        let mut idx = self.index.borrow_mut();
        let config = self.configs[*idx % self.configs.len()].clone();
        *idx += 1;
        FannEnsembleWrapper::new(config)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 CodeGuardian Ensemble Methods Example");
    println!("=========================================\n");

    // Create a realistic dataset for security finding classification
    let dataset = create_realistic_dataset(300);
    println!("📊 Created dataset with {} samples", dataset.len());

    let stats = dataset.get_stats();
    println!("   Balance ratio: {:.2}", stats.balance_ratio);
    println!("   True positives: {}", stats.true_positives);
    println!("   False positives: {}\n", stats.false_positives);

    // Test different ensemble strategies
    test_majority_voting(&dataset).await?;
    test_weighted_voting(&dataset).await?;
    test_stacking_ensemble(&dataset).await?;
    test_adaboost_ensemble(&dataset).await?;

    // Compare ensemble performance
    compare_ensemble_strategies(&dataset).await?;

    println!("✅ Ensemble methods examples completed!");
    Ok(())
}

/// Create a realistic dataset for security finding classification
fn create_realistic_dataset(size: usize) -> TrainingDataset {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut dataset = TrainingDataset::new();

    for i in 0..size {
        // Simulate realistic security finding features
        let severity_score = rng.gen_range(0.0..1.0);
        let confidence_score = rng.gen_range(0.4..1.0);
        let pattern_strength = rng.gen_range(0.0..1.0);
        let context_relevance = rng.gen_range(0.0..1.0);
        let historical_accuracy = rng.gen_range(0.5..0.95);
        let file_type_risk = rng.gen_range(0.0..1.0);
        let line_complexity = rng.gen_range(0.0..1.0);
        let codebase_maturity = rng.gen_range(0.3..1.0);

        let features = vec![
            severity_score,
            confidence_score,
            pattern_strength,
            context_relevance,
            historical_accuracy,
            file_type_risk,
            line_complexity,
            codebase_maturity,
        ];

        // Create realistic classification based on multiple factors
        let true_positive_probability = (severity_score * 0.3
            + confidence_score * 0.25
            + pattern_strength * 0.2
            + historical_accuracy * 0.15
            + context_relevance * 0.1);

        let is_true_positive = rng.gen::<f64>() < true_positive_probability;
        dataset.add_synthetic_example(&format!("finding_{}", i), features, is_true_positive);
    }

    dataset
}

/// Test majority voting ensemble
async fn test_majority_voting(dataset: &TrainingDataset) -> Result<()> {
    println!("🗳️  Testing Majority Voting Ensemble");
    println!("-------------------------------------");

    let start_time = Instant::now();

    let config = EnsembleConfig {
        strategy: EnsembleStrategy::MajorityVoting,
        n_models: 5,
        diversity: DiversityConfig {
            data_bagging: true,
            feature_bagging: false,
            bagging_fraction: 0.8,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut ensemble = EnsembleClassifier::new(config);
    let factory = DiverseFannFactory::new();

    ensemble.train(&factory, dataset).await?;

    println!(
        "⏱️  Training completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );

    // Test prediction
    let test_features = vec![0.8, 0.7, 0.9, 0.6, 0.85, 0.4, 0.3, 0.7];
    let prediction = ensemble.predict(&test_features).await?;

    println!("{}", prediction);
    println!("{}", ensemble.get_training_history());

    Ok(())
}

/// Test weighted voting ensemble
async fn test_weighted_voting(dataset: &TrainingDataset) -> Result<()> {
    println!("\n⚖️  Testing Weighted Voting Ensemble");
    println!("-------------------------------------");

    let start_time = Instant::now();

    let config = EnsembleConfig {
        strategy: EnsembleStrategy::WeightedVoting {
            weights: vec![], // Will be calculated based on performance
        },
        n_models: 5,
        diversity: DiversityConfig {
            data_bagging: true,
            feature_bagging: true,
            bagging_fraction: 0.75,
            feature_fraction: 0.9,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut ensemble = EnsembleClassifier::new(config);
    let factory = DiverseFannFactory::new();

    ensemble.train(&factory, dataset).await?;

    println!(
        "⏱️  Training completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );
    println!("📊 Model weights: {:?}", ensemble.get_model_weights());

    // Test prediction
    let test_features = vec![0.6, 0.8, 0.5, 0.7, 0.9, 0.3, 0.4, 0.8];
    let prediction = ensemble.predict(&test_features).await?;

    println!("{}", prediction);

    Ok(())
}

/// Test stacking ensemble
async fn test_stacking_ensemble(dataset: &TrainingDataset) -> Result<()> {
    println!("\n🏗️  Testing Stacking Ensemble");
    println!("-----------------------------");

    let start_time = Instant::now();

    let config = EnsembleConfig {
        strategy: EnsembleStrategy::Stacking {
            meta_learner_config: MetaLearnerConfig {
                learner_type: MetaLearnerType::LogisticRegression,
                parameters: HashMap::new(),
                cv_folds: 3,
            },
        },
        n_models: 4,
        diversity: DiversityConfig {
            data_bagging: true,
            random_seeds: true,
            bagging_fraction: 0.8,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut ensemble = EnsembleClassifier::new(config);
    let factory = DiverseFannFactory::new();

    ensemble.train(&factory, dataset).await?;

    println!(
        "⏱️  Training completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );

    // Test prediction
    let test_features = vec![0.9, 0.6, 0.8, 0.7, 0.75, 0.5, 0.6, 0.9];
    let prediction = ensemble.predict(&test_features).await?;

    println!("{}", prediction);

    Ok(())
}

/// Test AdaBoost ensemble
async fn test_adaboost_ensemble(dataset: &TrainingDataset) -> Result<()> {
    println!("\n🚀 Testing AdaBoost Ensemble");
    println!("-----------------------------");

    let start_time = Instant::now();

    let config = EnsembleConfig {
        strategy: EnsembleStrategy::AdaBoost {
            learning_rate: 1.0,
            max_estimators: 5,
        },
        n_models: 5,
        diversity: DiversityConfig {
            data_bagging: false, // AdaBoost handles sampling
            random_seeds: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut ensemble = EnsembleClassifier::new(config);
    let factory = DiverseFannFactory::new();

    ensemble.train(&factory, dataset).await?;

    println!(
        "⏱️  Training completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );
    println!("📊 AdaBoost weights: {:?}", ensemble.get_model_weights());

    // Test prediction
    let test_features = vec![0.7, 0.9, 0.4, 0.8, 0.6, 0.7, 0.5, 0.8];
    let prediction = ensemble.predict(&test_features).await?;

    println!("{}", prediction);

    Ok(())
}

/// Compare different ensemble strategies
async fn compare_ensemble_strategies(dataset: &TrainingDataset) -> Result<()> {
    println!("\n📊 Comparing Ensemble Strategies");
    println!("=================================");

    let strategies = vec![
        ("Majority Voting", EnsembleStrategy::MajorityVoting),
        (
            "Weighted Voting",
            EnsembleStrategy::WeightedVoting { weights: vec![] },
        ),
        (
            "Stacking",
            EnsembleStrategy::Stacking {
                meta_learner_config: MetaLearnerConfig {
                    learner_type: MetaLearnerType::LinearRegression,
                    parameters: HashMap::new(),
                    cv_folds: 3,
                },
            },
        ),
        (
            "AdaBoost",
            EnsembleStrategy::AdaBoost {
                learning_rate: 0.8,
                max_estimators: 4,
            },
        ),
    ];

    let mut results = Vec::new();

    for (name, strategy) in strategies {
        println!("\nTesting {name}...");
        let start_time = Instant::now();

        let config = EnsembleConfig {
            strategy,
            n_models: 4, // Smaller for faster comparison
            diversity: DiversityConfig {
                data_bagging: true,
                bagging_fraction: 0.8,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut ensemble = EnsembleClassifier::new(config);
        let factory = DiverseFannFactory::new();

        if let Err(e) = ensemble.train(&factory, dataset).await {
            println!("  ❌ Training failed: {}", e);
            continue;
        }

        let training_time = start_time.elapsed();

        // Test prediction accuracy on a few samples
        let mut correct_predictions = 0;
        let test_samples = 20.min(dataset.len());

        for i in 0..test_samples {
            let (features, actual_label) = dataset.get_sample(i)?;
            if let Ok(prediction) = ensemble.predict(&features).await {
                let predicted_label = prediction.prediction > 0.5;
                let actual_bool = actual_label > 0.5;
                if predicted_label == actual_bool {
                    correct_predictions += 1;
                }
            }
        }

        let accuracy = correct_predictions as f64 / test_samples as f64;
        let history = ensemble.get_training_history();
        let diversity = history
            .diversity_metrics
            .first()
            .map(|d| d.pairwise_diversity)
            .unwrap_or(0.0);

        results.push((name, accuracy, training_time, diversity));

        println!("  ✅ Accuracy: {:.3}", accuracy);
        println!("  ⏱️  Time: {:.2}s", training_time.as_secs_f64());
        println!("  🎯 Diversity: {:.3}", diversity);
    }

    // Display comparison summary
    println!("\n🏆 Strategy Comparison Summary:");
    println!("┌─────────────────┬─────────┬──────────┬──────────┐");
    println!("│ Strategy        │ Accuracy│ Time (s) │ Diversity│");
    println!("├─────────────────┼─────────┼──────────┼──────────┤");

    for (name, accuracy, time, diversity) in &results {
        println!(
            "│ {:<15} │ {:.3}   │ {:.2}     │ {:.3}    │",
            name,
            accuracy,
            time.as_secs_f64(),
            diversity
        );
    }
    println!("└─────────────────┴─────────┴──────────┴──────────┘");

    // Find best strategy
    if let Some((best_name, best_accuracy, _, _)) =
        results.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    {
        println!(
            "\n🏆 Best performing strategy: {} ({:.3} accuracy)",
            best_name, best_accuracy
        );
    }

    Ok(())
}
