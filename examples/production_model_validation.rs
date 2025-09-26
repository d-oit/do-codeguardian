//! # Production Model Validation Example
//!
//! This example demonstrates the comprehensive production model validation framework
//! for ensuring ML models meet quality and performance standards before deployment.

use anyhow::Result;
use do_codeguardian::ml::fann_classifier::FannClassifier;
use do_codeguardian::ml::production_validation::{
    ExpectedPerformance, ProductionValidationFramework, TestCase, TestCategory, TestCriticality,
    TestSuite, TestSuiteMetadata, ValidationConfig,
};
use do_codeguardian::types::{Finding, Severity};
use std::path::PathBuf;
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔬 Production Model Validation Framework Demo");
    println!("============================================");

    // Step 1: Create validation configuration
    let validation_config = create_validation_config();
    println!("✅ Created validation configuration");

    // Step 2: Initialize validation framework
    let mut framework = ProductionValidationFramework::new(validation_config);
    println!("✅ Initialized validation framework");

    // Step 3: Create test suites
    let test_suites = create_comprehensive_test_suites();
    println!("✅ Created {} test suites", test_suites.len());

    // Step 4: Save test suites for reuse
    save_test_suites(&test_suites).await?;
    println!("✅ Saved test suites to disk");

    // Step 5: Load test suites into framework
    framework
        .load_test_suites(&PathBuf::from("examples/test_suites"))
        .await?;
    println!("✅ Loaded test suites into framework");

    // Step 6: Demonstrate validation workflow
    demonstrate_validation_workflow(&mut framework).await?;

    // Step 7: Show deployment decision process
    demonstrate_deployment_decision().await?;

    println!("\n🎉 Production model validation demonstration completed!");
    Ok(())
}

/// Create comprehensive validation configuration
fn create_validation_config() -> ValidationConfig {
    ValidationConfig {
        required_accuracy: 0.85,
        required_precision: 0.80,
        required_recall: 0.80,
        required_f1_score: 0.80,
        max_false_positive_rate: 0.15,
        max_inference_time_ms: 100.0,
        min_test_coverage: 0.90,
        enable_performance_regression_check: true,
        enable_bias_detection: true,
        enable_robustness_testing: true,
    }
}

/// Create comprehensive test suites for different validation aspects
fn create_comprehensive_test_suites() -> Vec<TestSuite> {
    vec![
        create_security_test_suite(),
        create_performance_test_suite(),
        create_robustness_test_suite(),
        create_bias_fairness_test_suite(),
        create_regression_test_suite(),
    ]
}

/// Create security-focused test suite
fn create_security_test_suite() -> TestSuite {
    let test_cases = vec![
        // Critical security vulnerabilities
        TestCase {
            id: "sql_injection_critical".to_string(),
            finding: create_test_finding(
                "security",
                "sql_injection",
                Severity::Critical,
                "src/database.rs",
                45,
                "Potential SQL injection vulnerability detected",
            ),
            expected_classification: true,
            confidence_threshold: 0.8,
            category: TestCategory::SecurityVulnerability,
            criticality: TestCriticality::Critical,
        },
        TestCase {
            id: "hardcoded_secret_high".to_string(),
            finding: create_test_finding(
                "security",
                "hardcoded_secret",
                Severity::High,
                "src/config.rs",
                23,
                "Hardcoded API key detected",
            ),
            expected_classification: true,
            confidence_threshold: 0.7,
            category: TestCategory::SecurityVulnerability,
            criticality: TestCriticality::Critical,
        },
        // False positive examples
        TestCase {
            id: "test_file_todo".to_string(),
            finding: create_test_finding(
                "style",
                "todo_comment",
                Severity::Low,
                "tests/unit_test.rs",
                12,
                "TODO: Add more test cases",
            ),
            expected_classification: false,
            confidence_threshold: 0.5,
            category: TestCategory::FalsePositiveReduction,
            criticality: TestCriticality::High,
        },
    ];

    TestSuite {
        name: "Security Validation Suite".to_string(),
        description: "Comprehensive security vulnerability detection tests".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.90,
            min_precision: 0.85,
            min_recall: 0.90,
            min_f1_score: 0.87,
            max_false_positive_rate: 0.10,
            max_inference_time_ms: 50.0,
        },
        metadata: create_test_suite_metadata("security"),
    }
}

/// Create performance-focused test suite
fn create_performance_test_suite() -> TestSuite {
    let mut test_cases = Vec::new();

    // Generate performance test cases
    for i in 0..100 {
        test_cases.push(TestCase {
            id: format!("perf_test_{}", i),
            finding: create_test_finding(
                "performance",
                "inefficient_loop",
                if i % 4 == 0 {
                    Severity::High
                } else {
                    Severity::Medium
                },
                &format!("src/module_{}.rs", i % 10),
                (i * 13) % 500 + 1,
                &format!("Performance issue detected in loop {}", i),
            ),
            expected_classification: i % 4 == 0, // 25% should be flagged
            confidence_threshold: 0.5,
            category: TestCategory::Performance,
            criticality: TestCriticality::Medium,
        });
    }

    TestSuite {
        name: "Performance Validation Suite".to_string(),
        description: "Tests model inference speed and throughput".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.75,
            min_precision: 0.70,
            min_recall: 0.70,
            min_f1_score: 0.70,
            max_false_positive_rate: 0.25,
            max_inference_time_ms: 10.0, // Strict performance requirement
        },
        metadata: create_test_suite_metadata("performance"),
    }
}

/// Create robustness test suite
fn create_robustness_test_suite() -> TestSuite {
    let test_cases = vec![
        // Edge cases
        TestCase {
            id: "empty_file".to_string(),
            finding: create_test_finding(
                "style",
                "empty_file",
                Severity::Info,
                "src/empty.rs",
                1,
                ""
            ),
            expected_classification: false,
            confidence_threshold: 0.5,
            category: TestCategory::EdgeCase,
            criticality: TestCriticality::Low,
         },

        TestCase {
            id: "very_long_message".to_string(),
            finding: create_test_finding(
                "security",
                "complex_vulnerability",
                Severity::Medium,
                "src/complex.rs",
                999,
                &"This is a very long vulnerability description that contains many technical details about the security issue and potential attack vectors and mitigation strategies and references to security standards".repeat(5)
            ),
            expected_classification: true,
            confidence_threshold: 0.5,
            category: TestCategory::EdgeCase,
            criticality: TestCriticality::Medium,
        },
    ];

    TestSuite {
        name: "Robustness Validation Suite".to_string(),
        description: "Tests model stability with edge cases and unusual inputs".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.70,
            min_precision: 0.65,
            min_recall: 0.60,
            min_f1_score: 0.62,
            max_false_positive_rate: 0.30,
            max_inference_time_ms: 100.0,
        },
        metadata: create_test_suite_metadata("robustness"),
    }
}

/// Create bias and fairness test suite
fn create_bias_fairness_test_suite() -> TestSuite {
    let test_cases = vec![
        // Test different file types for bias
        TestCase {
            id: "rust_file_security".to_string(),
            finding: create_test_finding(
                "security",
                "buffer_overflow",
                Severity::High,
                "src/main.rs",
                100,
                "Buffer overflow vulnerability",
            ),
            expected_classification: true,
            confidence_threshold: 0.7,
            category: TestCategory::SecurityVulnerability,
            criticality: TestCriticality::High,
        },
        TestCase {
            id: "javascript_file_security".to_string(),
            finding: create_test_finding(
                "security",
                "xss_vulnerability",
                Severity::High,
                "web/app.js",
                150,
                "XSS vulnerability detected",
            ),
            expected_classification: true,
            confidence_threshold: 0.7,
            category: TestCategory::SecurityVulnerability,
            criticality: TestCriticality::High,
        },
        TestCase {
            id: "python_file_security".to_string(),
            finding: create_test_finding(
                "security",
                "injection_vulnerability",
                Severity::High,
                "scripts/deploy.py",
                75,
                "Command injection vulnerability",
            ),
            expected_classification: true,
            confidence_threshold: 0.7,
            category: TestCategory::SecurityVulnerability,
            criticality: TestCriticality::High,
        },
    ];

    TestSuite {
        name: "Bias and Fairness Validation Suite".to_string(),
        description: "Tests for unfair bias across different file types and contexts".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.80,
            min_precision: 0.75,
            min_recall: 0.80,
            min_f1_score: 0.77,
            max_false_positive_rate: 0.20,
            max_inference_time_ms: 75.0,
        },
        metadata: create_test_suite_metadata("bias_fairness"),
    }
}

/// Create regression test suite
fn create_regression_test_suite() -> TestSuite {
    let test_cases = vec![
        // Known good cases that should remain stable
        TestCase {
            id: "baseline_security_positive".to_string(),
            finding: create_test_finding(
                "security",
                "known_vulnerability",
                Severity::Critical,
                "src/auth.rs",
                42,
                "Authentication bypass vulnerability",
            ),
            expected_classification: true,
            confidence_threshold: 0.8,
            category: TestCategory::Regression,
            criticality: TestCriticality::Critical,
        },
        TestCase {
            id: "baseline_false_positive".to_string(),
            finding: create_test_finding(
                "style",
                "debug_print",
                Severity::Info,
                "examples/demo.rs",
                10,
                "Debug print statement",
            ),
            expected_classification: false,
            confidence_threshold: 0.5,
            category: TestCategory::Regression,
            criticality: TestCriticality::High,
        },
    ];

    TestSuite {
        name: "Regression Validation Suite".to_string(),
        description: "Ensures no performance regression from baseline model".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.85,
            min_precision: 0.80,
            min_recall: 0.80,
            min_f1_score: 0.80,
            max_false_positive_rate: 0.15,
            max_inference_time_ms: 50.0,
        },
        metadata: create_test_suite_metadata("regression"),
    }
}

/// Create test finding helper
fn create_test_finding(
    analyzer: &str,
    rule: &str,
    severity: Severity,
    file_path: &str,
    line: u32,
    message: &str,
) -> Finding {
    Finding::new(
        analyzer,
        rule,
        severity,
        PathBuf::from(file_path),
        line,
        message.to_string(),
    )
}

/// Create test suite metadata helper
fn create_test_suite_metadata(category: &str) -> TestSuiteMetadata {
    TestSuiteMetadata {
        version: "1.0.0".to_string(),
        created_by: "ValidationDemo".to_string(),
        created_at: SystemTime::now(),
        last_updated: SystemTime::now(),
        tags: vec![category.to_string(), "validation".to_string()],
        source_datasets: vec!["synthetic".to_string()],
    }
}

/// Save test suites to disk
async fn save_test_suites(test_suites: &[TestSuite]) -> Result<()> {
    tokio::fs::create_dir_all("examples/test_suites").await?;

    for test_suite in test_suites {
        let filename = test_suite.name.to_lowercase().replace(" ", "_") + ".json";
        let path = PathBuf::from("examples/test_suites").join(filename);
        let json = serde_json::to_string_pretty(test_suite)?;
        tokio::fs::write(&path, json).await?;
    }

    Ok(())
}

/// Demonstrate validation workflow
async fn demonstrate_validation_workflow(
    framework: &mut ProductionValidationFramework,
) -> Result<()> {
    println!("\n🔄 Demonstrating Validation Workflow");
    println!("===================================");

    // Simulate model validation (would use real model in practice)
    println!("⏳ Running comprehensive validation...");

    // In a real scenario, you would:
    // let validation_result = framework.validate_for_production(&model_path).await?;

    // For demo, show what the validation process covers
    println!("✅ Accuracy Validation: Testing prediction accuracy");
    println!("✅ Performance Validation: Measuring inference speed");
    println!("✅ Robustness Validation: Testing edge case stability");
    println!("✅ Bias Validation: Checking fairness across groups");
    println!("✅ Regression Validation: Comparing with baseline");

    println!("\n📊 Sample Validation Results:");
    println!("  • Overall Accuracy: 87.3%");
    println!("  • Security Test Suite: 92.1% accuracy");
    println!("  • Performance Test Suite: 89.7% accuracy");
    println!("  • Avg Inference Time: 12.3ms");
    println!("  • Bias Score: 0.91 (excellent fairness)");

    Ok(())
}

/// Demonstrate deployment decision process
async fn demonstrate_deployment_decision() -> Result<()> {
    println!("\n🎯 Deployment Decision Matrix");
    println!("============================");

    println!("✅ PASSED - All thresholds met:");
    println!("  • Accuracy ≥ 85%: ✅ 87.3%");
    println!("  • Precision ≥ 80%: ✅ 84.2%");
    println!("  • Recall ≥ 80%: ✅ 81.7%");
    println!("  • F1-Score ≥ 80%: ✅ 82.9%");
    println!("  • False Positive Rate ≤ 15%: ✅ 12.1%");
    println!("  • Inference Time ≤ 100ms: ✅ 12.3ms");

    println!("\n🚀 DEPLOYMENT APPROVED");
    println!("Model is ready for production deployment!");

    println!("\n📋 Next Steps:");
    println!("  1. Deploy to staging environment");
    println!("  2. Run integration tests");
    println!("  3. Monitor performance metrics");
    println!("  4. Gradual rollout to production");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_config_creation() {
        let config = create_validation_config();
        assert_eq!(config.required_accuracy, 0.85);
        assert_eq!(config.max_inference_time_ms, 100.0);
        assert!(config.enable_bias_detection);
    }

    #[test]
    fn test_test_suite_creation() {
        let test_suites = create_comprehensive_test_suites();
        assert_eq!(test_suites.len(), 5);

        let security_suite = &test_suites[0];
        assert_eq!(security_suite.name, "Security Validation Suite");
        assert!(!security_suite.test_cases.is_empty());
    }

    #[test]
    fn test_finding_creation() {
        let finding = create_test_finding(
            "security",
            "test_rule",
            Severity::High,
            "test.rs",
            42,
            "Test message",
        );

        assert_eq!(finding.analyzer, "security");
        assert_eq!(finding.rule, "test_rule");
        assert_eq!(finding.severity, Severity::High);
        assert_eq!(finding.line, 42);
    }
}
