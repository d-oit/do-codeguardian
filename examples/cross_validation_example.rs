//! Example demonstrating the K-fold cross-validation system in CodeGuardian
//!
//! This example shows how to:
//! 1. Create training data
//! 2. Configure cross-validation
//! 3. Run validation with different strategies
//! 4. Interpret results

use anyhow::Result;
use do_codeguardian::ml::cross_validation::{
    AssessmentLevel, CrossValidationConfig, CrossValidator, ValidationStrategy,
};
use do_codeguardian::ml::fann_classifier::NetworkConfig;
use do_codeguardian::ml::fann_cross_validation_integration::validate_fann_model;
use do_codeguardian::ml::training_data::TrainingDataset;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 CodeGuardian Cross-Validation Example");
    println!("=========================================\n");

    // Create sample training data
    let dataset = create_sample_dataset(200);
    println!("📊 Created dataset with {} samples", dataset.len());

    let stats = dataset.get_stats();
    println!("   Balance ratio: {:.2}", stats.balance_ratio);
    println!("   True positives: {}", stats.true_positives);
    println!("   False positives: {}\n", stats.false_positives);

    // Run different validation strategies
    run_basic_kfold_validation(&dataset).await?;
    run_stratified_validation(&dataset).await?;
    run_timeseries_validation(&dataset).await?;

    // Compare network architectures
    compare_network_architectures(&dataset).await?;

    println!("✅ Cross-validation examples completed!");
    Ok(())
}

/// Create a realistic dataset for security finding classification
fn create_sample_dataset(size: usize) -> TrainingDataset {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut dataset = TrainingDataset::new();

    for i in 0..size {
        // Simulate realistic features for security findings
        let severity_score = rng.gen_range(0.0..1.0);
        let line_number_context = rng.gen_range(0.0..1.0);
        let file_type_score = rng.gen_range(0.0..1.0);
        let pattern_match_strength = rng.gen_range(0.0..1.0);
        let complexity_score = rng.gen_range(0.0..1.0);
        let historical_accuracy = rng.gen_range(0.4..0.9);

        let features = vec![
            severity_score,
            line_number_context,
            file_type_score,
            pattern_match_strength,
            complexity_score,
            historical_accuracy,
            rng.gen_range(0.0..1.0), // Additional feature 1
            rng.gen_range(0.0..1.0), // Additional feature 2
        ];

        // Create realistic true/false positive labels
        // Higher severity and pattern match strength increase true positive probability
        let true_positive_prob =
            (severity_score + pattern_match_strength + historical_accuracy) / 3.0;
        let is_true_positive = rng.gen::<f64>() < true_positive_prob;

        dataset.add_synthetic_example(&format!("sample_{}", i), features, is_true_positive);
    }

    dataset
}

/// Demonstrate basic K-fold cross-validation
async fn run_basic_kfold_validation(dataset: &TrainingDataset) -> Result<()> {
    println!("🔄 Running Basic K-Fold Cross-Validation");
    println!("----------------------------------------");

    let start_time = Instant::now();

    let mut config = CrossValidationConfig::default();
    config.k_folds = 5;
    config.strategy = ValidationStrategy::KFold;
    config.shuffle = true;
    config.random_state = Some(42);

    let network_config = NetworkConfig::basic();
    let results = validate_fann_model(network_config, config, dataset).await?;

    println!(
        "⏱️  Validation completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );
    print_summary(&results);

    Ok(())
}

/// Demonstrate stratified K-fold cross-validation
async fn run_stratified_validation(dataset: &TrainingDataset) -> Result<()> {
    println!("\n🎯 Running Stratified K-Fold Cross-Validation");
    println!("----------------------------------------------");

    let start_time = Instant::now();

    let mut config = CrossValidationConfig::default();
    config.k_folds = 5;
    config.strategy = ValidationStrategy::StratifiedKFold;
    config.shuffle = true;
    config.random_state = Some(42);

    let network_config = NetworkConfig::basic();
    let results = validate_fann_model(network_config, config, dataset).await?;

    println!(
        "⏱️  Validation completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );
    print_summary(&results);

    Ok(())
}

/// Demonstrate time-series cross-validation
async fn run_timeseries_validation(dataset: &TrainingDataset) -> Result<()> {
    println!("\n📈 Running Time-Series Cross-Validation");
    println!("----------------------------------------");

    let start_time = Instant::now();

    let mut config = CrossValidationConfig::default();
    config.k_folds = 4;
    config.strategy = ValidationStrategy::TimeSeriesSplit { gap: 5 };
    config.shuffle = false; // Don't shuffle for time-series

    let network_config = NetworkConfig::basic();
    let results = validate_fann_model(network_config, config, dataset).await?;

    println!(
        "⏱️  Validation completed in {:.2}s",
        start_time.elapsed().as_secs_f64()
    );
    print_summary(&results);

    Ok(())
}

/// Compare different network architectures
async fn compare_network_architectures(dataset: &TrainingDataset) -> Result<()> {
    println!("\n🏗️  Comparing Network Architectures");
    println!("------------------------------------");

    let mut config = CrossValidationConfig::default();
    config.k_folds = 3; // Smaller for faster comparison
    config.strategy = ValidationStrategy::StratifiedKFold;

    // Test basic network
    println!("Testing Basic Network (8 inputs → [12,8] → 1)...");
    let basic_config = NetworkConfig::basic();
    let basic_results = validate_fann_model(basic_config, config.clone(), dataset).await?;

    // Test enhanced network (if AST features are available)
    #[cfg(feature = "ast")]
    {
        println!("Testing Enhanced Network (24 inputs → [48,24,12] → 1)...");
        let enhanced_config = NetworkConfig::enhanced();
        let enhanced_results =
            validate_fann_model(enhanced_config, config.clone(), dataset).await?;

        println!("\n📊 Architecture Comparison:");
        println!("  Basic Network:");
        println!(
            "    Accuracy: {:.3} ± {:.3}",
            basic_results.aggregated_metrics.mean_metrics.accuracy,
            basic_results.aggregated_metrics.std_metrics.accuracy
        );
        println!(
            "    F1-Score: {:.3} ± {:.3}",
            basic_results.aggregated_metrics.mean_metrics.f1_score,
            basic_results.aggregated_metrics.std_metrics.f1_score
        );

        println!("  Enhanced Network:");
        println!(
            "    Accuracy: {:.3} ± {:.3}",
            enhanced_results.aggregated_metrics.mean_metrics.accuracy,
            enhanced_results.aggregated_metrics.std_metrics.accuracy
        );
        println!(
            "    F1-Score: {:.3} ± {:.3}",
            enhanced_results.aggregated_metrics.mean_metrics.f1_score,
            enhanced_results.aggregated_metrics.std_metrics.f1_score
        );

        // Determine which is better
        let basic_f1 = basic_results.aggregated_metrics.mean_metrics.f1_score;
        let enhanced_f1 = enhanced_results.aggregated_metrics.mean_metrics.f1_score;

        if enhanced_f1 > basic_f1 + 0.02 {
            println!("🏆 Enhanced network shows significant improvement!");
        } else if basic_f1 > enhanced_f1 + 0.02 {
            println!("🤔 Basic network performs better - may indicate overfitting");
        } else {
            println!("⚖️  Both networks show similar performance");
        }
    }

    #[cfg(not(feature = "ast"))]
    {
        println!("Basic Network Results:");
        print_summary(&basic_results);
        println!(
            "ℹ️  Enhanced network comparison requires AST features (build with --features ast)"
        );
    }

    Ok(())
}

/// Print a concise summary of validation results
fn print_summary(results: &do_codeguardian::ml::cross_validation::CrossValidationResults) {
    let metrics = &results.aggregated_metrics.mean_metrics;
    let std_metrics = &results.aggregated_metrics.std_metrics;

    println!("📈 Results Summary:");
    println!(
        "   Accuracy:  {:.3} ± {:.3}",
        metrics.accuracy, std_metrics.accuracy
    );
    println!(
        "   Precision: {:.3} ± {:.3}",
        metrics.precision, std_metrics.precision
    );
    println!(
        "   Recall:    {:.3} ± {:.3}",
        metrics.recall, std_metrics.recall
    );
    println!(
        "   F1-Score:  {:.3} ± {:.3}",
        metrics.f1_score, std_metrics.f1_score
    );
    println!(
        "   AUC-ROC:   {:.3} ± {:.3}",
        metrics.auc_roc, std_metrics.auc_roc
    );

    let assessment_emoji = match results.recommendation.overall_assessment {
        AssessmentLevel::Excellent => "🎉",
        AssessmentLevel::Good => "👍",
        AssessmentLevel::Acceptable => "✅",
        AssessmentLevel::Poor => "⚠️",
        AssessmentLevel::Unacceptable => "❌",
    };

    println!(
        "   Assessment: {} {:?} ({:.0}% confidence)",
        assessment_emoji,
        results.recommendation.overall_assessment,
        results.recommendation.confidence_level * 100.0
    );

    if !results.recommendation.concerns.is_empty() {
        println!("   ⚠️  Concerns: {}", results.recommendation.concerns.len());
    }

    if !results.recommendation.recommendations.is_empty() {
        println!(
            "   💡 Recommendations: {}",
            results.recommendation.recommendations.len()
        );
    }
}
