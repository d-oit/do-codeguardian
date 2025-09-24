//! Component Interaction Tests
//!
//! End-to-end workflows and error handling tests for CodeGuardian components.

use do_codeguardian::cache::UnifiedCache;
use do_codeguardian::core::GuardianEngine;
use do_codeguardian::ml::feature_extractor::FeatureExtractor;
use do_codeguardian::output::formatter::OutputFormatter;
use do_codeguardian::performance::monitoring::PerformanceMonitor;
use do_codeguardian::utils::progress::ProgressReporter;
use do_codeguardian::Config;
use std::path::PathBuf;
use tempfile::TempDir;

/// Integration tests for component interactions
#[cfg(test)]
mod component_interaction_tests {
    use super::*;

    #[tokio::test]
    async fn test_end_to_end_analysis_pipeline() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");

        // Create a test file with various types of issues
        let test_content = r#"
use std::process::Command;

fn main() {
    // This should trigger security warnings
    let user_input = "test";
    Command::new("echo").arg(user_input).output().unwrap();

    // This should trigger performance warnings
    let mut result = String::new();
    for i in 0..1000 {
        result = result + &format!("item {}", i); // Inefficient string concatenation
    }

    println!("{}", result);
}
"#;
        std::fs::write(&test_file, test_content).unwrap();

        // Create configuration
        let config = Config::default();
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config.clone(), progress).await.unwrap();

        // Run analysis
        let results = engine.analyze_files(&[test_file], 1).await.unwrap();

        // Verify results contain findings from multiple analyzers
        assert!(
            !results.findings.is_empty(),
            "Should find security and performance issues"
        );

        // Verify summary is populated
        assert!(results.summary.total_files_scanned > 0);
        assert!(results.summary.total_findings > 0);

        // Verify different types of findings are present
        let rule_types: std::collections::HashSet<String> =
            results.findings.iter().map(|f| f.rule.clone()).collect();

        assert!(
            rule_types.len() > 1,
            "Should have findings from multiple analyzers"
        );
    }

    #[tokio::test]
    async fn test_error_propagation() {
        let config = Config::default();
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await.unwrap();

        // Test with non-existent file
        let non_existent = PathBuf::from("non_existent_file.rs");
        let result = engine.analyze_files(&[non_existent], 1).await;

        // Should handle error gracefully
        match result {
            Ok(results) => {
                // Engine should handle missing files gracefully
                assert_eq!(results.summary.total_files_scanned, 0);
            }
            Err(e) => {
                // Or return a descriptive error
                assert!(!format!("{}", e).is_empty());
            }
        }
    }
}
