//! Model Validation CLI Command
//!
//! Provides comprehensive production model validation capabilities
//! including accuracy testing, performance benchmarking, and deployment readiness assessment.

use crate::cli::ModelValidationArgs;
use crate::ml::production_validation::{
    ExpectedPerformance, ProductionValidationFramework, TestCase, TestCategory, TestCriticality,
    TestSuite, TestSuiteMetadata, ValidationConfig, ValidationStatus,
};
use crate::types::{AnalysisResults, Finding, Severity};
use crate::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs;

/// Run model validation command
pub async fn run(args: ModelValidationArgs, _config: &Config) -> Result<()> {
    println!("🔬 Production Model Validation Framework");
    println!("=======================================");

    // Load validation configuration
    let validation_config = if let Some(config_path) = &args.config_file {
        load_validation_config(config_path).await?
    } else {
        ValidationConfig::default()
    };

    // Initialize validation framework
    let mut framework = ProductionValidationFramework::new(validation_config);

    // Load test suites
    if let Some(test_suites_dir) = &args.test_suites_dir {
        framework.load_test_suites(test_suites_dir).await?;
        println!("✅ Loaded test suites from: {}", test_suites_dir.display());
    } else if args.generate_test_suites {
        // Generate test suites from findings
        generate_test_suites(&args).await?;
        return Ok(());
    }

    // Set baseline model if provided
    if let Some(baseline_path) = &args.baseline_model {
        framework.set_baseline_model(baseline_path).await?;
        println!("✅ Set baseline model: {}", baseline_path.display());
    }

    // Validate the target model
    println!(
        "🔄 Starting validation for model: {}",
        args.model_path.display()
    );
    let validation_result = framework.validate_for_production(&args.model_path).await?;

    // Display results
    display_validation_results(&validation_result);

    // Save detailed report if requested
    if let Some(output_dir) = &args.output_dir {
        save_validation_outputs(&validation_result, output_dir, &args).await?;
    }

    // Handle deployment decision
    handle_deployment_decision(&validation_result, &args).await?;

    Ok(())
}

/// Display validation results summary
fn display_validation_results(result: &crate::ml::production_validation::ValidationResult) {
    println!("\n📊 Validation Results Summary");
    println!("============================");

    // Overall status
    let status_icon = match result.validation_status {
        ValidationStatus::Passed => "✅",
        ValidationStatus::PassedWithWarnings => "⚠️",
        ValidationStatus::Failed => "❌",
        ValidationStatus::Inconclusive => "❓",
    };

    println!(
        "{} Overall Status: {:?}",
        status_icon, result.validation_status
    );

    // Key metrics
    println!("\n📈 Performance Metrics:");
    println!(
        "  • Accuracy:    {:.3} ({:.1}%)",
        result.overall_metrics.accuracy,
        result.overall_metrics.accuracy * 100.0
    );
    println!(
        "  • Precision:   {:.3} ({:.1}%)",
        result.overall_metrics.precision,
        result.overall_metrics.precision * 100.0
    );
    println!(
        "  • Recall:      {:.3} ({:.1}%)",
        result.overall_metrics.recall,
        result.overall_metrics.recall * 100.0
    );
    println!(
        "  • F1-Score:    {:.3} ({:.1}%)",
        result.overall_metrics.f1_score,
        result.overall_metrics.f1_score * 100.0
    );
    println!(
        "  • FP Rate:     {:.3} ({:.1}%)",
        result.overall_metrics.false_positive_rate,
        result.overall_metrics.false_positive_rate * 100.0
    );

    println!("\n⚡ Performance:");
    println!(
        "  • Avg Inference: {:.2}ms",
        result.overall_metrics.avg_inference_time_ms
    );
    println!(
        "  • Max Inference: {:.2}ms",
        result.overall_metrics.max_inference_time_ms
    );
    println!(
        "  • Min Inference: {:.2}ms",
        result.overall_metrics.min_inference_time_ms
    );

    // Validator breakdown
    println!("\n🔍 Validator Results:");
    for (validator_name, metrics) in &result.validator_results {
        let status = if metrics.accuracy >= 0.8 {
            "✅"
        } else {
            "❌"
        };
        println!(
            "  {} {}: Accuracy {:.3}, F1 {:.3}",
            status, validator_name, metrics.accuracy, metrics.f1_score
        );
    }

    // Test suite results
    if !result.test_suite_results.is_empty() {
        println!("\n📋 Test Suite Results:");
        for (suite_name, suite_result) in &result.test_suite_results {
            let passed_rate = suite_result.passed_tests as f32
                / (suite_result.passed_tests + suite_result.failed_tests) as f32;
            let status = if passed_rate >= 0.9 {
                "✅"
            } else if passed_rate >= 0.7 {
                "⚠️"
            } else {
                "❌"
            };

            println!(
                "  {} {}: {}/{} passed ({:.1}%)",
                status,
                suite_name,
                suite_result.passed_tests,
                suite_result.passed_tests + suite_result.failed_tests,
                passed_rate * 100.0
            );

            if suite_result.critical_failures.len() > 0 {
                println!(
                    "    ❗ Critical failures: {}",
                    suite_result.critical_failures.len()
                );
            }
        }
    }

    // Performance comparison
    if let Some(baseline_comparison) = &result.performance_comparison.vs_baseline {
        println!("\n📊 vs Baseline:");
        print_performance_delta("Accuracy", baseline_comparison.accuracy_delta);
        print_performance_delta("Precision", baseline_comparison.precision_delta);
        print_performance_delta("Recall", baseline_comparison.recall_delta);
        print_performance_delta("F1-Score", baseline_comparison.f1_score_delta);
        print_performance_delta("Inference Time", baseline_comparison.inference_time_delta);
    }

    // Deployment readiness
    println!("\n🚀 Deployment Readiness:");
    println!(
        "  • Ready for Production: {}",
        if result.deployment_readiness.ready_for_production {
            "✅ Yes"
        } else {
            "❌ No"
        }
    );
    println!(
        "  • Readiness Score: {:.1}/1.0",
        result.deployment_readiness.readiness_score
    );

    if !result.deployment_readiness.blocking_issues.is_empty() {
        println!("  • Blocking Issues:");
        for issue in &result.deployment_readiness.blocking_issues {
            println!("    ❌ {}", issue);
        }
    }

    if !result.deployment_readiness.warnings.is_empty() {
        println!("  • Warnings:");
        for warning in &result.deployment_readiness.warnings {
            println!("    ⚠️  {}", warning);
        }
    }

    // Recommendations
    if !result.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for (i, recommendation) in result.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, recommendation);
        }
    }
}

/// Print performance delta with appropriate formatting
fn print_performance_delta(metric_name: &str, delta: f32) {
    let (icon, sign) = if delta > 0.0 {
        ("📈", "+")
    } else if delta < 0.0 {
        ("📉", "")
    } else {
        ("➖", "")
    };

    println!(
        "  {} {}: {}{:.3} ({:.1}%)",
        icon,
        metric_name,
        sign,
        delta,
        delta * 100.0
    );
}

/// Save validation outputs to directory
async fn save_validation_outputs(
    result: &crate::ml::production_validation::ValidationResult,
    output_dir: &PathBuf,
    args: &ModelValidationArgs,
) -> Result<()> {
    fs::create_dir_all(output_dir).await?;

    // Save detailed JSON report
    let report_path = output_dir.join("validation_report.json");
    let report_json = serde_json::to_string_pretty(result)?;
    fs::write(&report_path, report_json).await?;
    println!("📄 Detailed report saved: {}", report_path.display());

    // Save human-readable summary
    let summary_path = output_dir.join("validation_summary.md");
    let summary_md = generate_markdown_summary(result, args);
    fs::write(&summary_path, summary_md).await?;
    println!("📝 Summary report saved: {}", summary_path.display());

    // Save CSV metrics for analysis
    if args.export_metrics {
        let metrics_path = output_dir.join("validation_metrics.csv");
        let metrics_csv = generate_metrics_csv(result)?;
        fs::write(&metrics_path, metrics_csv).await?;
        println!("📊 Metrics CSV saved: {}", metrics_path.display());
    }

    Ok(())
}

/// Generate markdown summary report
fn generate_markdown_summary(
    result: &crate::ml::production_validation::ValidationResult,
    args: &ModelValidationArgs,
) -> String {
    let mut md = String::new();

    md.push_str("# Model Validation Report\n\n");
    md.push_str(&format!("**Model:** {}\n", args.model_path.display()));
    md.push_str(&format!(
        "**Validation Date:** {}\n",
        DateTime::<Utc>::from(result.timestamp).format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!("**Status:** {:?}\n\n", result.validation_status));

    md.push_str("## Performance Metrics\n\n");
    md.push_str("| Metric | Value | Percentage |\n");
    md.push_str("|--------|-------|------------|\n");
    md.push_str(&format!(
        "| Accuracy | {:.3} | {:.1}% |\n",
        result.overall_metrics.accuracy,
        result.overall_metrics.accuracy * 100.0
    ));
    md.push_str(&format!(
        "| Precision | {:.3} | {:.1}% |\n",
        result.overall_metrics.precision,
        result.overall_metrics.precision * 100.0
    ));
    md.push_str(&format!(
        "| Recall | {:.3} | {:.1}% |\n",
        result.overall_metrics.recall,
        result.overall_metrics.recall * 100.0
    ));
    md.push_str(&format!(
        "| F1-Score | {:.3} | {:.1}% |\n",
        result.overall_metrics.f1_score,
        result.overall_metrics.f1_score * 100.0
    ));
    md.push_str(&format!(
        "| False Positive Rate | {:.3} | {:.1}% |\n",
        result.overall_metrics.false_positive_rate,
        result.overall_metrics.false_positive_rate * 100.0
    ));

    md.push_str("\n## Performance Benchmarks\n\n");
    md.push_str(&format!(
        "- **Average Inference Time:** {:.2}ms\n",
        result.overall_metrics.avg_inference_time_ms
    ));
    md.push_str(&format!(
        "- **Maximum Inference Time:** {:.2}ms\n",
        result.overall_metrics.max_inference_time_ms
    ));
    md.push_str(&format!(
        "- **Minimum Inference Time:** {:.2}ms\n",
        result.overall_metrics.min_inference_time_ms
    ));

    md.push_str("\n## Deployment Readiness\n\n");
    md.push_str(&format!(
        "- **Ready for Production:** {}\n",
        if result.deployment_readiness.ready_for_production {
            "✅ Yes"
        } else {
            "❌ No"
        }
    ));
    md.push_str(&format!(
        "- **Readiness Score:** {:.1}/1.0\n",
        result.deployment_readiness.readiness_score
    ));

    if !result.deployment_readiness.blocking_issues.is_empty() {
        md.push_str("\n### Blocking Issues\n");
        for issue in &result.deployment_readiness.blocking_issues {
            md.push_str(&format!("- ❌ {}\n", issue));
        }
    }

    if !result.recommendations.is_empty() {
        md.push_str("\n## Recommendations\n\n");
        for (i, recommendation) in result.recommendations.iter().enumerate() {
            md.push_str(&format!("{}. {}\n", i + 1, recommendation));
        }
    }

    md
}

/// Generate CSV metrics for analysis
fn generate_metrics_csv(
    result: &crate::ml::production_validation::ValidationResult,
) -> Result<String> {
    let mut csv = String::new();

    // Header
    csv.push_str(
        "validator,accuracy,precision,recall,f1_score,false_positive_rate,avg_inference_time_ms\n",
    );

    // Overall metrics
    csv.push_str(&format!(
        "Overall,{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}\n",
        result.overall_metrics.accuracy,
        result.overall_metrics.precision,
        result.overall_metrics.recall,
        result.overall_metrics.f1_score,
        result.overall_metrics.false_positive_rate,
        result.overall_metrics.avg_inference_time_ms
    ));

    // Individual validator metrics
    for (validator_name, metrics) in &result.validator_results {
        csv.push_str(&format!(
            "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}\n",
            validator_name,
            metrics.accuracy,
            metrics.precision,
            metrics.recall,
            metrics.f1_score,
            metrics.false_positive_rate,
            metrics.avg_inference_time_ms
        ));
    }

    Ok(csv)
}

/// Handle deployment decision based on validation results
async fn handle_deployment_decision(
    result: &crate::ml::production_validation::ValidationResult,
    args: &ModelValidationArgs,
) -> Result<()> {
    println!("\n🎯 Deployment Decision");
    println!("=====================");

    match result.validation_status {
        ValidationStatus::Passed => {
            println!("✅ Model APPROVED for production deployment");
            if args.auto_deploy {
                println!("🚀 Auto-deployment would proceed (simulation mode)");
                // In real implementation, would trigger deployment pipeline
            }
        }
        ValidationStatus::PassedWithWarnings => {
            println!("⚠️  Model CONDITIONALLY APPROVED with warnings");
            println!("   Consider addressing warnings before deployment");
            if args.auto_deploy {
                println!("⏸️  Auto-deployment paused due to warnings");
            }
        }
        ValidationStatus::Failed => {
            println!("❌ Model REJECTED for production deployment");
            println!("   Address blocking issues before revalidation");
            if args.auto_deploy {
                println!("🛑 Auto-deployment blocked");
            }
        }
        ValidationStatus::Inconclusive => {
            println!("❓ Validation INCONCLUSIVE");
            println!("   Insufficient data or test failures");
            if args.auto_deploy {
                println!("⏸️  Auto-deployment paused");
            }
        }
    }

    // Exit with appropriate code for CI/CD
    if args.fail_on_issues && !matches!(result.validation_status, ValidationStatus::Passed) {
        std::process::exit(1);
    }

    Ok(())
}

/// Generate test suites from findings
async fn generate_test_suites(args: &ModelValidationArgs) -> Result<()> {
    println!("🔧 Generating Test Suites");
    println!("=========================");

    // Load findings from input file
    let findings = if let Some(input_file) = &args.findings_file {
        load_findings_from_file(input_file).await?
    } else {
        return Err(anyhow::anyhow!(
            "--findings-file required for test suite generation"
        ));
    };

    if findings.is_empty() {
        println!("⚠️  No findings found in input file");
        return Ok(());
    }

    println!(
        "📊 Processing {} findings for test suite generation",
        findings.len()
    );

    // Generate different test suites
    let security_suite = generate_security_test_suite(&findings)?;
    let performance_suite = generate_performance_test_suite(&findings)?;
    let edge_case_suite = generate_edge_case_test_suite(&findings)?;
    let regression_suite = generate_regression_test_suite(&findings)?;

    // Save test suites
    let output_dir = args
        .output_dir
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--output-dir required for test suite generation"))?;

    fs::create_dir_all(output_dir).await?;

    save_test_suite(
        &security_suite,
        &output_dir.join("security_test_suite.json"),
    )
    .await?;
    save_test_suite(
        &performance_suite,
        &output_dir.join("performance_test_suite.json"),
    )
    .await?;
    save_test_suite(
        &edge_case_suite,
        &output_dir.join("edge_case_test_suite.json"),
    )
    .await?;
    save_test_suite(
        &regression_suite,
        &output_dir.join("regression_test_suite.json"),
    )
    .await?;

    println!("✅ Generated 4 test suites in: {}", output_dir.display());
    println!(
        "  • security_test_suite.json ({} tests)",
        security_suite.test_cases.len()
    );
    println!(
        "  • performance_test_suite.json ({} tests)",
        performance_suite.test_cases.len()
    );
    println!(
        "  • edge_case_test_suite.json ({} tests)",
        edge_case_suite.test_cases.len()
    );
    println!(
        "  • regression_test_suite.json ({} tests)",
        regression_suite.test_cases.len()
    );

    Ok(())
}

/// Generate security-focused test suite
fn generate_security_test_suite(findings: &[Finding]) -> Result<TestSuite> {
    let security_findings: Vec<_> = findings
        .iter()
        .filter(|f| {
            f.analyzer.contains("security")
                || matches!(f.severity, Severity::Critical | Severity::High)
        })
        .collect();

    let mut test_cases = Vec::new();

    for (i, finding) in security_findings.iter().enumerate() {
        let test_case = TestCase {
            id: format!("security_test_{}", i),
            finding: (*finding).clone(),
            expected_classification: true, // Security findings should be flagged
            confidence_threshold: 0.7,     // Higher threshold for security
            category: TestCategory::SecurityVulnerability,
            criticality: match finding.severity {
                Severity::Critical => TestCriticality::Critical,
                Severity::High => TestCriticality::High,
                _ => TestCriticality::Medium,
            },
        };
        test_cases.push(test_case);
    }

    Ok(TestSuite {
        name: "Security Vulnerability Test Suite".to_string(),
        description: "Tests model's ability to detect security vulnerabilities".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.90,
            min_precision: 0.85,
            min_recall: 0.90,
            min_f1_score: 0.87,
            max_false_positive_rate: 0.10,
            max_inference_time_ms: 50.0,
        },
        metadata: TestSuiteMetadata {
            version: "1.0.0".to_string(),
            created_by: "CodeGuardian".to_string(),
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
            tags: vec!["security".to_string(), "vulnerability".to_string()],
            source_datasets: vec!["production_findings".to_string()],
        },
    })
}

/// Generate performance-focused test suite
fn generate_performance_test_suite(findings: &[Finding]) -> Result<TestSuite> {
    let sample_size = findings.len().min(100); // Limit for performance testing
    let test_cases: Vec<_> = findings
        .iter()
        .take(sample_size)
        .enumerate()
        .map(|(i, finding)| TestCase {
            id: format!("perf_test_{}", i),
            finding: finding.clone(),
            expected_classification: matches!(
                finding.severity,
                Severity::Critical | Severity::High
            ),
            confidence_threshold: 0.5,
            category: TestCategory::Performance,
            criticality: TestCriticality::Medium,
        })
        .collect();

    Ok(TestSuite {
        name: "Performance Test Suite".to_string(),
        description: "Tests model inference speed and resource efficiency".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.75,
            min_precision: 0.70,
            min_recall: 0.70,
            min_f1_score: 0.70,
            max_false_positive_rate: 0.25,
            max_inference_time_ms: 10.0, // Strict performance requirement
        },
        metadata: TestSuiteMetadata {
            version: "1.0.0".to_string(),
            created_by: "CodeGuardian".to_string(),
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
            tags: vec!["performance".to_string(), "speed".to_string()],
            source_datasets: vec!["production_findings".to_string()],
        },
    })
}

/// Generate edge case test suite
fn generate_edge_case_test_suite(findings: &[Finding]) -> Result<TestSuite> {
    let edge_cases: Vec<_> = findings
        .iter()
        .filter(|f| {
            // Identify edge cases
            f.message.len() < 10
                || f.message.len() > 200
                || f.line == 1
                || f.line > 1000
                || matches!(f.severity, Severity::Info)
                || f.file.to_string_lossy().contains("test")
        })
        .collect();

    let test_cases: Vec<_> = edge_cases
        .iter()
        .enumerate()
        .map(|(i, finding)| TestCase {
            id: format!("edge_test_{}", i),
            finding: (*finding).clone(),
            expected_classification: !finding.file.to_string_lossy().contains("test"), // Test files often false positives
            confidence_threshold: 0.5,
            category: TestCategory::EdgeCase,
            criticality: TestCriticality::Low,
        })
        .collect();

    Ok(TestSuite {
        name: "Edge Case Test Suite".to_string(),
        description: "Tests model robustness with unusual or edge case inputs".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.70,
            min_precision: 0.65,
            min_recall: 0.60,
            min_f1_score: 0.62,
            max_false_positive_rate: 0.30,
            max_inference_time_ms: 100.0,
        },
        metadata: TestSuiteMetadata {
            version: "1.0.0".to_string(),
            created_by: "CodeGuardian".to_string(),
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
            tags: vec!["edge_cases".to_string(), "robustness".to_string()],
            source_datasets: vec!["production_findings".to_string()],
        },
    })
}

/// Generate regression test suite
fn generate_regression_test_suite(findings: &[Finding]) -> Result<TestSuite> {
    // Select representative sample across all categories
    let sample_size = findings.len().min(50);
    let step = if findings.len() > sample_size {
        findings.len() / sample_size
    } else {
        1
    };

    let test_cases: Vec<_> = findings
        .iter()
        .step_by(step)
        .take(sample_size)
        .enumerate()
        .map(|(i, finding)| TestCase {
            id: format!("regression_test_{}", i),
            finding: finding.clone(),
            expected_classification: !matches!(finding.severity, Severity::Info | Severity::Low),
            confidence_threshold: 0.5,
            category: TestCategory::Regression,
            criticality: TestCriticality::High, // Regression is important
        })
        .collect();

    Ok(TestSuite {
        name: "Regression Test Suite".to_string(),
        description: "Ensures no performance regression compared to baseline".to_string(),
        test_cases,
        expected_performance: ExpectedPerformance {
            min_accuracy: 0.80,
            min_precision: 0.75,
            min_recall: 0.75,
            min_f1_score: 0.75,
            max_false_positive_rate: 0.20,
            max_inference_time_ms: 50.0,
        },
        metadata: TestSuiteMetadata {
            version: "1.0.0".to_string(),
            created_by: "CodeGuardian".to_string(),
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
            tags: vec!["regression".to_string(), "baseline".to_string()],
            source_datasets: vec!["production_findings".to_string()],
        },
    })
}

/// Save test suite to JSON file
async fn save_test_suite(test_suite: &TestSuite, path: &PathBuf) -> Result<()> {
    let json = serde_json::to_string_pretty(test_suite)?;
    fs::write(path, json).await?;
    Ok(())
}

/// Load findings from analysis results file
async fn load_findings_from_file(path: &PathBuf) -> Result<Vec<Finding>> {
    let content = fs::read_to_string(path).await?;
    let results: AnalysisResults = serde_json::from_str(&content)?;
    Ok(results.findings)
}

/// Load validation configuration from file
async fn load_validation_config(path: &PathBuf) -> Result<ValidationConfig> {
    let content = fs::read_to_string(path).await?;
    let config: ValidationConfig = serde_json::from_str(&content)?;
    Ok(config)
}
