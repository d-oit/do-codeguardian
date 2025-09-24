//! # Training Data Collection Example
//!
//! This example demonstrates how to use the comprehensive training data collection pipeline
//! to build high-quality datasets for machine learning models.

use anyhow::Result;
use do_codeguardian::cli::training_data::{CollectionConfig, TrainingDataCollectionPipeline};
use do_codeguardian::types::{Finding, Severity};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 CodeGuardian Training Data Collection Example");
    println!("================================================");

    // Step 1: Create sample findings (in practice, these would come from analysis)
    let sample_findings = create_sample_findings();
    println!("✅ Created {} sample findings", sample_findings.len());

    // Step 2: Configure collection pipeline
    let config = create_collection_config();
    println!("✅ Configured collection pipeline");

    // Step 3: Run collection pipeline
    let mut pipeline = TrainingDataCollectionPipeline::new();
    let output_dir = PathBuf::from("examples/output/training_data");

    println!("🔄 Running training data collection...");
    let stats = pipeline
        .collect_training_data(config, sample_findings, &output_dir)
        .await?;

    // Step 4: Display results
    println!("\n📊 Collection Results:");
    println!("  Total findings processed: {}", stats.total_findings);
    println!("  Successfully labeled: {}", stats.labeled_findings);
    println!("  True positives: {}", stats.true_positives);
    println!("  False positives: {}", stats.false_positives);
    println!("  Balance ratio: {:.2}", stats.balance_ratio);
    println!("  Quality score: {:.2}", stats.quality_score);

    println!("\n📁 Output files created:");
    println!("  • {}/training_data.json", output_dir.display());
    println!("  • {}/training_data.csv", output_dir.display());
    println!("  • {}/collection_report.json", output_dir.display());

    // Step 5: Demonstrate usage in training
    demonstrate_training_usage(&output_dir).await?;

    println!("\n🎉 Example completed successfully!");
    Ok(())
}

/// Create sample findings for demonstration
fn create_sample_findings() -> Vec<Finding> {
    vec![
        // High-confidence true positives
        Finding::new(
            "security",
            "sql_injection",
            Severity::Critical,
            PathBuf::from("src/database.rs"),
            42,
            "Potential SQL injection vulnerability detected".to_string(),
        )
        .with_description("User input is directly concatenated into SQL query".to_string())
        .with_suggestion("Use parameterized queries or prepared statements".to_string()),
        Finding::new(
            "integrity",
            "corrupted_binary",
            Severity::High,
            PathBuf::from("src/crypto.rs"),
            156,
            "Binary file corruption detected in cryptographic module".to_string(),
        )
        .with_description("File hash doesn't match expected signature".to_string()),
        Finding::new(
            "security",
            "hardcoded_secret",
            Severity::Critical,
            PathBuf::from("src/config.rs"),
            23,
            "Hardcoded API key detected".to_string(),
        )
        .with_suggestion("Move secrets to environment variables or secure vault".to_string()),
        // High-confidence false positives
        Finding::new(
            "style",
            "todo_comment",
            Severity::Low,
            PathBuf::from("tests/integration_test.rs"),
            89,
            "TODO comment found in code".to_string(),
        )
        .with_description("TODO: Add more test cases for edge conditions".to_string()),
        Finding::new(
            "duplicate",
            "similar_functions",
            Severity::Medium,
            PathBuf::from("tests/helper.rs"),
            45,
            "Similar function detected".to_string(),
        )
        .with_description("Test helper functions with similar patterns".to_string()),
        Finding::new(
            "style",
            "debug_print",
            Severity::Info,
            PathBuf::from("examples/demo.rs"),
            67,
            "Debug print statement found".to_string(),
        ),
        // Medium confidence cases
        Finding::new(
            "performance",
            "inefficient_loop",
            Severity::Medium,
            PathBuf::from("src/analyzer.rs"),
            234,
            "Potentially inefficient loop detected".to_string(),
        )
        .with_description("Nested loop with O(n²) complexity".to_string())
        .with_suggestion("Consider using a more efficient algorithm".to_string()),
        Finding::new(
            "lint_drift",
            "config_inconsistency",
            Severity::Medium,
            PathBuf::from("config/production.toml"),
            12,
            "Configuration drift detected".to_string(),
        ),
        // Edge cases
        Finding::new(
            "duplicate",
            "repeated_code",
            Severity::Low,
            PathBuf::from("src/utils.rs"),
            78,
            "Duplicate code block detected".to_string(),
        ),
        Finding::new(
            "non_production",
            "test_artifact",
            Severity::Low,
            PathBuf::from("src/main.rs"),
            5,
            "Test artifact in production code".to_string(),
        ),
    ]
}

/// Create collection configuration
fn create_collection_config() -> CollectionConfig {
    CollectionConfig {
        min_examples: 10,
        target_balance_ratio: 1.0,
        include_low_confidence: true, // Include for demonstration
        require_manual_review: false, // Skip for automation
        export_formats: vec![
            "json".to_string(),
            "csv".to_string(),
            "tfrecord".to_string(),
        ],
        labeling_strategies: vec![
            "heuristic".to_string(),
            "severity_based".to_string(),
            "file_type_based".to_string(),
            "analyzer_based".to_string(),
        ],
    }
}

/// Demonstrate how to use collected training data
async fn demonstrate_training_usage(output_dir: &PathBuf) -> Result<()> {
    println!("\n🧠 Training Usage Example:");
    println!("  # Train a new model with collected data:");
    println!("  codeguardian train \\");
    println!(
        "    --training-data {}/training_data.json \\",
        output_dir.display()
    );
    println!("    --model-path models/custom-model.fann \\");
    println!("    --epochs 1000 \\");
    println!("    --validate \\");
    println!("    --cross-validate");

    println!("\n  # Collect more data from real analysis:");
    println!("  codeguardian check . --format json --out findings.json");
    println!("  codeguardian training-data \\");
    println!("    --input-file findings.json \\");
    println!("    --output-dir data/training \\");
    println!("    --interactive \\");
    println!("    --min-examples 500");

    println!("\n  # Use trained model in analysis:");
    println!("  codeguardian check . \\");
    println!("    --ml-model models/custom-model.fann \\");
    println!("    --ml-threshold 0.7");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_findings_creation() {
        let findings = create_sample_findings();
        assert!(!findings.is_empty());

        // Verify we have different severity levels
        let has_critical = findings
            .iter()
            .any(|f| matches!(f.severity, Severity::Critical));
        let has_low = findings.iter().any(|f| matches!(f.severity, Severity::Low));
        assert!(has_critical);
        assert!(has_low);
    }

    #[test]
    fn test_collection_config() {
        let config = create_collection_config();
        assert_eq!(config.min_examples, 10);
        assert!(!config.export_formats.is_empty());
        assert!(!config.labeling_strategies.is_empty());
    }
}
