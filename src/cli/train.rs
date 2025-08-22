use crate::cli::TrainArgs;
use crate::config::Config;
use crate::core::GuardianEngine;
use crate::ml::fann_classifier::{FannClassifier, NetworkConfig};
use crate::ml::training_data::{TrainingDataCollector, TrainingDataset};
use crate::types::Finding;
use crate::utils::progress::ProgressReporter;
use anyhow::Result;
use is_terminal::IsTerminal;
use std::path::Path;
use std::time::Instant;

pub async fn run(args: TrainArgs) -> Result<()> {
    let start_time = Instant::now();

    // Load configuration
    let config = Config::load(Path::new("codeguardian.toml")).unwrap_or_else(|_| {
        eprintln!("Warning: No configuration file found, using defaults");
        Config::default()
    });

    // Initialize progress reporter (TTY-aware)
    let progress = ProgressReporter::new(!args.quiet && std::io::stdout().is_terminal());

    progress.update("Initializing ML training pipeline...");

    // Create or load training dataset
    let mut dataset = if let Some(dataset_path) = &args.dataset {
        if Path::new(dataset_path).exists() {
            progress.update(&format!("Loading existing dataset: {}", dataset_path));
            TrainingDataset::load_from_file(dataset_path)?
        } else {
            progress.update("Creating new training dataset");
            TrainingDataset::new()
        }
    } else {
        progress.update("Creating new training dataset");
        TrainingDataset::new()
    };

    // If bootstrap mode, generate training data from existing codebase
    if args.bootstrap {
        progress.update("Bootstrapping training data from codebase analysis...");
        let bootstrap_dataset = generate_bootstrap_data(&config, &progress, &args.paths).await?;

        // Merge bootstrap data into main dataset
        for example in bootstrap_dataset.examples {
            dataset.add_example(example);
        }

        progress.update(&format!(
            "Generated {} bootstrap examples",
            dataset.examples.len()
        ));
    }

    // Generate synthetic data if requested
    if args.synthetic_samples > 0 {
        progress.update(&format!(
            "Generating {} synthetic training samples...",
            args.synthetic_samples
        ));
        for _ in 0..args.synthetic_samples {
            dataset.generate_synthetic_data()?;
        }
    }

    // Validate dataset size
    if dataset.examples.len() < 10 {
        eprintln!("Warning: Dataset has only {} examples. Consider using --bootstrap or --synthetic-samples", dataset.examples.len());
    }

    // Create or load neural network
    let mut classifier = if let Some(model_path) = &args.model_path {
        if Path::new(model_path).exists() {
            progress.update(&format!("Loading existing model: {}", model_path));
            FannClassifier::load(model_path)?
        } else {
            progress.update("Creating new neural network");
            create_default_network()?
        }
    } else {
        progress.update("Creating new neural network");
        create_default_network()?
    };

    // Train the model
    progress.update("Training neural network...");
    let training_pairs = if args.balanced {
        dataset.get_balanced_training_pairs()
    } else {
        dataset.get_training_pairs()
    };

    let training_start = std::time::Instant::now();
    let final_error = classifier.train_batch(&training_pairs, args.epochs)?;
    let _training_duration = training_start.elapsed();

    progress.update(&format!(
        "Training completed. Final error: {:.6}",
        final_error
    ));

    // Save the trained model
    let model_output = args
        .model_path
        .as_deref()
        .unwrap_or("codeguardian-model.fann");
    classifier.save(model_output)?;

    if !args.quiet {
        println!("✅ Model saved to: {}", model_output);
    }

    // Save the dataset if path provided
    if let Some(dataset_path) = &args.dataset {
        dataset.save_to_file(dataset_path)?;
        if !args.quiet {
            println!("📊 Dataset saved to: {}", dataset_path);
        }
    }

    // Print training summary
    if !args.quiet {
        let training_stats = classifier.get_training_stats();
        print_training_summary(
            &dataset,
            &classifier,
            final_error,
            start_time.elapsed().as_millis() as u64,
            &training_stats,
        );
    }

    Ok(())
}

async fn generate_bootstrap_data(
    config: &Config,
    progress: &ProgressReporter,
    paths: &[std::path::PathBuf],
) -> Result<TrainingDataset> {
    // Initialize the Guardian engine for analysis
    let mut engine = GuardianEngine::new(config.clone(), ProgressReporter::new(false)).await?;

    // Get files to analyze
    let files_to_scan = engine.get_all_files(paths).await?;

    progress.update(&format!(
        "Analyzing {} files for bootstrap data...",
        files_to_scan.len()
    ));

    // Run analysis to get findings
    let results = engine.analyze_files(&files_to_scan, 0).await?;

    // Create training data collector
    let mut collector = TrainingDataCollector::new();

    // Apply heuristics to classify findings
    collector.apply_heuristics(&results.findings)?;

    // Apply heuristics to create labeled training data
    let labeled_findings: Vec<(Finding, bool)> = results
        .findings
        .into_iter()
        .map(|finding| {
            // Use heuristic classification to determine if it's a true positive
            let is_true_positive = collector.heuristic_classification(&finding);
            (finding, is_true_positive)
        })
        .collect();

    // Collect findings into training examples
    collector.collect_from_findings(&labeled_findings)?;

    Ok(collector.get_dataset())
}

fn create_default_network() -> Result<FannClassifier> {
    let config = NetworkConfig {
        input_size: 12,                 // Enhanced 12 features from FeatureExtractor
        hidden_layers: vec![16, 12, 8], // Three hidden layers for better learning
        output_size: 1,                 // Single relevance score
        learning_rate: 0.05,            // More conservative learning rate
        activation_function: "sigmoid".to_string(),
    };

    FannClassifier::new(config)
}

fn print_training_summary(
    dataset: &TrainingDataset,
    classifier: &FannClassifier,
    final_error: f32,
    duration_ms: u64,
    training_stats: &crate::ml::fann_classifier::TrainingStats,
) {
    let stats = dataset.get_stats();
    let network_stats = classifier.get_stats();

    println!("\n🧠 Enhanced ML Training Summary");
    println!("===============================");
    println!("Training duration: {}ms", duration_ms);
    println!("Final training error: {:.6}", final_error);
    println!("Best error achieved: {:.6}", training_stats.best_error);
    println!("Epochs completed: {}", training_stats.epochs_trained);
    if training_stats.early_stopped {
        println!("✋ Early stopping triggered (no improvement detected)");
    }
    println!();
    println!("📊 Dataset Statistics:");
    println!("  Total examples: {}", stats.total_examples);
    println!("  True positives: {}", stats.true_positives);
    println!("  False positives: {}", stats.false_positives);
    println!("  Balance ratio: {:.2}", stats.balance_ratio);
    println!();
    println!("🔗 Enhanced Network Architecture:");
    println!("  {}", network_stats);
    println!(
        "  Final learning rate: {:.6}",
        training_stats.current_learning_rate
    );
    println!("  Feature vector size: 12 (enhanced from 8)");
    println!();
    println!("📈 Training Progress:");
    if training_stats.error_history.len() > 1 {
        let improvement = training_stats.error_history[0] - training_stats.best_error;
        let improvement_pct = (improvement / training_stats.error_history[0]) * 100.0;
        println!(
            "  Error reduction: {:.2}% ({:.6} → {:.6})",
            improvement_pct, training_stats.error_history[0], training_stats.best_error
        );

        // Show convergence trend
        if training_stats.error_history.len() >= 5 {
            let recent_trend =
                &training_stats.error_history[training_stats.error_history.len() - 5..];
            let is_converging = recent_trend.windows(2).all(|w| w[1] <= w[0]);
            println!(
                "  Convergence: {}",
                if is_converging {
                    "✅ Stable"
                } else {
                    "⚠️  Oscillating"
                }
            );
        }
    }
    println!();
    println!("🚀 New Features:");
    println!("  ✅ Adaptive learning rate adjustment");
    println!("  ✅ Early stopping for optimal convergence");
    println!("  ✅ Enhanced 12-feature extraction");
    println!("  ✅ Training data shuffling");
    println!("  ✅ Real-time progress monitoring");
    println!();
    println!("💡 Next Steps:");
    println!("  1. Test the model: codeguardian check --ml-model codeguardian-model.fann");
    println!("  2. Monitor performance: codeguardian metrics");
    println!("  3. Provide feedback to improve accuracy");
    println!("  4. Retrain periodically with new data");
}
