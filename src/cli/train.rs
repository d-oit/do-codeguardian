use crate::cli::TrainArgs;
use crate::ml::fann_classifier::{FannClassifier, NetworkConfig};
use crate::ml::training_data::TrainingDataset;
use crate::Config;
use anyhow::Result;

// TrainArgs is defined in cli.rs

#[cfg(feature = "ml")]
pub async fn run(args: TrainArgs, _config: &Config) -> Result<()> {
    use tracing::{info, warn};

    info!("Starting ML model training...");

    // Load or create training dataset
    let mut dataset = if let Some(data_path) = &args.training_data {
        info!("Loading training data from: {}", data_path.display());
        TrainingDataset::load_from_file_async(&data_path.to_string_lossy()).await?
    } else {
        info!("Creating new training dataset");
        TrainingDataset::new()
    };

    // Generate synthetic data if bootstrap is enabled
    if args.bootstrap {
        info!("Generating synthetic training data for cold start...");
        dataset.generate_synthetic_data()?;
    }

    // Check if we have enough training data
    let stats = dataset.get_stats();
    info!("{}", stats);

    if stats.total_examples < 10 {
        warn!("Very few training examples ({}). Consider using --bootstrap or providing more training data.", stats.total_examples);
    }

    // Create or load neural network
    let mut classifier = if args.continue_training && args.model_path.exists() {
        info!("Loading existing model from: {}", args.model_path.display());
        FannClassifier::load(&args.model_path)?
    } else {
        info!("Creating new neural network...");
        let config = if args.enhanced && cfg!(feature = "ast") {
            info!("Using AST-enhanced network configuration...");
            NetworkConfig::enhanced()
        } else if args.enhanced && !cfg!(feature = "ast") {
            warn!(
                "Enhanced mode requested but AST feature not available. Using basic configuration."
            );
            NetworkConfig::basic()
        } else {
            NetworkConfig::default()
        };
        FannClassifier::new(config)?
    };

    // Get training data in the format expected by FANN
    let training_pairs = if stats.balance_ratio > 0.2 && stats.balance_ratio < 5.0 {
        // Dataset is reasonably balanced, use all data
        dataset.get_training_pairs()
    } else {
        // Dataset is imbalanced, use balanced subset
        info!(
            "Dataset is imbalanced (ratio: {:.2}), using balanced subset",
            stats.balance_ratio
        );
        dataset.get_balanced_training_pairs()
    };

    if training_pairs.is_empty() {
        return Err(anyhow::anyhow!(
            "No training data available. Use --bootstrap to generate synthetic data."
        ));
    }

    info!(
        "Training with {} examples for {} epochs...",
        training_pairs.len(),
        args.epochs
    );

    // Train the model
    let final_error = classifier.train_batch(&training_pairs, args.epochs)?;

    info!("Training completed. Final error: {:.6}", final_error);

    // Display network statistics
    let network_stats = classifier.get_stats();
    info!("{}", network_stats);

    // Save the trained model
    info!("Saving model to: {}", args.model_path.display());
    classifier.save(&args.model_path)?;

    // Validate model performance if requested
    if args.validate {
        info!("Validating model performance...");
        validate_model(&classifier, &training_pairs)?;

        // Run cross-validation if enabled
        if args.cross_validate.unwrap_or(false) {
            info!("Running cross-validation...");
            run_cross_validation(&dataset, &args).await?;
        }
    }

    info!("Model training completed successfully!");
    info!(
        "Use the model with: codeguardian check . --ml-model {}",
        args.model_path.display()
    );

    Ok(())
}

#[cfg(not(feature = "ml"))]
pub async fn run(_args: TrainArgs, _config: &Config) -> Result<()> {
    Err(anyhow::anyhow!(
        "ML training is not available. Build with --features ml to enable machine learning functionality."
    ))
}

#[cfg(feature = "ml")]
fn validate_model(classifier: &FannClassifier, training_data: &[(Vec<f32>, f32)]) -> Result<()> {
    use tracing::info;

    let mut correct_predictions = 0;
    let mut total_predictions = 0;
    let mut true_positives = 0;
    let mut false_positives = 0;
    let mut true_negatives = 0;
    let mut false_negatives = 0;

    for (features, expected) in training_data {
        let prediction = classifier.predict(features)?;
        let predicted_class = if prediction > 0.5 { 1.0 } else { 0.0 };

        total_predictions += 1;
        if (predicted_class - expected).abs() < 0.1 {
            correct_predictions += 1;
        }

        // Calculate confusion matrix
        match (expected > &0.5, predicted_class > 0.5) {
            (true, true) => true_positives += 1,
            (false, true) => false_positives += 1,
            (false, false) => true_negatives += 1,
            (true, false) => false_negatives += 1,
        }
    }

    let accuracy = correct_predictions as f32 / total_predictions as f32;
    let precision = if true_positives + false_positives > 0 {
        true_positives as f32 / (true_positives + false_positives) as f32
    } else {
        0.0
    };
    let recall = if true_positives + false_negatives > 0 {
        true_positives as f32 / (true_positives + false_negatives) as f32
    } else {
        0.0
    };
    let f1_score = if precision + recall > 0.0 {
        2.0 * (precision * recall) / (precision + recall)
    } else {
        0.0
    };

    info!("Model Validation Results:");
    info!(
        "  Accuracy: {:.3} ({}/{})",
        accuracy, correct_predictions, total_predictions
    );
    info!("  Precision: {:.3}", precision);
    info!("  Recall: {:.3}", recall);
    info!("  F1 Score: {:.3}", f1_score);
    info!("  Confusion Matrix:");
    info!("    True Positives: {}", true_positives);
    info!("    False Positives: {}", false_positives);
    info!("    True Negatives: {}", true_negatives);
    info!("    False Negatives: {}", false_negatives);

    if accuracy < 0.7 {
        tracing::warn!("Model accuracy is low ({:.3}). Consider more training data or different hyperparameters.", accuracy);
    }

    Ok(())
}

#[cfg(feature = "ml")]
async fn run_cross_validation(dataset: &TrainingDataset, args: &TrainArgs) -> Result<()> {
    use crate::ml::cross_validation::{CrossValidationConfig, CrossValidator, ValidationStrategy};
    use crate::ml::fann_classifier::NetworkConfig;
    use crate::ml::fann_cross_validation_integration::{
        validate_fann_model, FannClassifierFactory,
    };
    use tracing::info;

    info!("Starting cross-validation analysis...");

    // Configure cross-validation
    let mut cv_config = CrossValidationConfig::default();
    cv_config.k_folds = args.cv_folds.unwrap_or(5);
    cv_config.strategy = if args.stratified.unwrap_or(true) {
        ValidationStrategy::StratifiedKFold
    } else {
        ValidationStrategy::KFold
    };
    cv_config.random_state = Some(42); // Reproducible results

    // Configure network based on training args
    let network_config = if args.enhanced && cfg!(feature = "ast") {
        NetworkConfig::enhanced()
    } else {
        NetworkConfig::basic()
    };

    // Run cross-validation
    match validate_fann_model(network_config, cv_config, dataset).await {
        Ok(results) => {
            info!("Cross-validation completed successfully!");
            println!("\n{}", results);

            // Save results if requested
            if let Some(output_path) = &args.cv_output {
                save_cv_results(&results, output_path)?;
                info!(
                    "Cross-validation results saved to: {}",
                    output_path.display()
                );
            }
        }
        Err(e) => {
            tracing::error!("Cross-validation failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[cfg(feature = "ml")]
fn save_cv_results(
    results: &crate::ml::cross_validation::CrossValidationResults,
    output_path: &std::path::Path,
) -> Result<()> {
    use std::fs;

    let json = serde_json::to_string_pretty(results)
        .map_err(|e| anyhow::anyhow!("Failed to serialize results: {}", e))?;

    fs::write(output_path, json).map_err(|e| anyhow::anyhow!("Failed to write results: {}", e))?;

    Ok(())
}
