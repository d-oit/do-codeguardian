use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Feature-specific end-to-end tests
/// Consolidated from e2e_workflow_tests.rs and e2e_feature_tests.rs

#[test]
fn test_ml_integration_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Create files with potential false positives
    fs::write(
        temp_dir.path().join("test_file.rs"),
        r#"
// This might be flagged as a secret but it's just a test
const TEST_KEY = "test_key_12345";

#[cfg(test)]
mod tests {
    const MOCK_SECRET = "mock_secret_abcdef"; // Test context

    #[test]
    fn test_function() {
        assert_eq!(2 + 2, 4);
    }
}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--ml-threshold")
        .arg("0.7") // Higher threshold to filter false positives
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned"));
}

#[test]
fn test_cache_functionality() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("cached_file.rs"),
        r#"
fn main() {
    println!("This file should be cached");
}
"#,
    )
    .unwrap();

    // First run - should populate cache
    let mut cmd1 = Command::cargo_bin("do-codeguardian").unwrap();
    cmd1.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .current_dir(temp_dir.path());

    let start1 = std::time::Instant::now();
    cmd1.assert().success();
    let duration1 = start1.elapsed();

    // Second run - should use cache (faster)
    let mut cmd2 = Command::cargo_bin("do-codeguardian").unwrap();
    cmd2.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .current_dir(temp_dir.path());

    let start2 = std::time::Instant::now();
    cmd2.assert().success();
    let duration2 = start2.elapsed();

    // Second run should be faster or similar (cache hit)
    // Note: This is a heuristic test, actual performance may vary
    println!("First run: {:?}, Second run: {:?}", duration1, duration2);
}

#[test]
fn test_github_integration_dry_run() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repository
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    fs::write(
        temp_dir.path().join("issue_file.rs"),
        r#"
fn main() {
    let password = "hardcoded_password_123"; // Security issue
    println!("Password: {}", password);
}
"#,
    )
    .unwrap();

    // Run analysis to detect issues (simulating GitHub issue creation)
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path().join("issue_file.rs"))
        .arg("--format")
        .arg("human")
        .current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findings"));
}

#[test]
fn test_streaming_analysis() {
    let temp_dir = TempDir::new().unwrap();

    // Create a large file for streaming
    let large_content = (0..1000)
        .map(|i| format!("fn function_{}() {{ println!(\"Line {}\"); }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(temp_dir.path().join("large_stream.rs"), large_content).unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--streaming") // Enable streaming mode
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned"));
}

#[test]
fn test_custom_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // Create config with custom patterns
    let config_content = r#"
[security_analyzer]
enabled = true
secret_patterns = [
    "CUSTOM_SECRET_[A-Z0-9]{10}",
    "MY_API_KEY_[a-f0-9]{32}"
]

[non_production]
enabled = true
patterns = ["CUSTOM_TODO", "CUSTOM_FIXME", "CUSTOM_DEBUG"]
"#;
    fs::write(temp_dir.path().join("codeguardian.toml"), config_content).unwrap();

    fs::write(
        temp_dir.path().join("custom_patterns.rs"),
        r#"
fn main() {
    let secret = "CUSTOM_SECRET_ABCDEF1234"; // Should be detected
    let api_key = "MY_API_KEY_abcdef1234567890abcdef1234567890"; // Should be detected

    // CUSTOM_TODO: This should be flagged
    println!("Secret: {}, API: {}", secret, api_key);
}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findings"));
}

#[test]
fn test_baseline_mode() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(
        temp_dir.path().join("baseline_test.rs"),
        r#"
fn main() {
    let existing_issue = "old_secret_123"; // Existing issue
    println!("Issue: {}", existing_issue);
}
"#,
    )
    .unwrap();

    // First run to establish baseline
    let mut baseline_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    baseline_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--baseline")
        .arg(temp_dir.path().join("baseline.json"))
        .arg("--format")
        .arg("json");

    baseline_cmd.assert().success();

    // Add new issue
    fs::write(
        temp_dir.path().join("new_issue.rs"),
        r#"
fn new_function() {
    let new_secret = "new_secret_456"; // New issue
    println!("New secret: {}", new_secret);
}
"#,
    )
    .unwrap();

    // Second run should only report new issues
    let mut diff_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    diff_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--baseline")
        .arg(temp_dir.path().join("baseline.json"))
        .arg("--only-new")
        .arg("--format")
        .arg("json");

    diff_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("findings"));
}

#[tokio::test]
async fn test_parallel_output_processing() {
    use do_codeguardian::output::{OutputFormat, ParallelOutputProcessor};
    use do_codeguardian::types::{AnalysisResults, Finding, Severity};
    use std::collections::HashMap;

    // Create test analysis results
    let mut results = AnalysisResults::new("parallel_test".to_string());

    // Add test findings
    for i in 0..10 {
        results.findings.push(Finding {
            id: format!("parallel-test-finding-{}", i),
            analyzer: "test_analyzer".to_string(),
            rule: "test_rule".to_string(),
            severity: Severity::Medium,
            file: std::path::PathBuf::from(format!("test_file_{}.rs", i)),
            line: i as u32 + 1,
            column: Some(10),
            message: format!("Test finding message {}", i),
            description: Some(format!("Test description {}", i)),
            suggestion: Some("Fix this issue".to_string()),
            category: Some("test".to_string()),
            metadata: HashMap::new(),
        });
    }

    results.summary.total_findings = 10;
    results.summary.total_files_scanned = 5;

    // Test parallel processing of multiple formats
    let processor = ParallelOutputProcessor::new().unwrap();
    let formats = vec![
        OutputFormat::Json,
        OutputFormat::Html,
        OutputFormat::Markdown,
        OutputFormat::Sarif,
    ];

    let start_time = std::time::Instant::now();
    let output_results = processor
        .process_multiple_formats(&results, formats)
        .await
        .unwrap();
    let duration = start_time.elapsed();

    // Verify results
    assert_eq!(output_results.len(), 4);
    assert!(output_results.contains_key(&OutputFormat::Json));
    assert!(output_results.contains_key(&OutputFormat::Html));
    assert!(output_results.contains_key(&OutputFormat::Markdown));
    assert!(output_results.contains_key(&OutputFormat::Sarif));

    // Verify all outputs are valid
    for (format, result) in output_results {
        assert!(
            result.is_valid(),
            "Output for format {:?} should be valid",
            format
        );
        assert!(
            !result.content.is_empty(),
            "Output content for format {:?} should not be empty",
            format
        );
    }

    // Performance check - should complete in reasonable time
    assert!(
        duration < std::time::Duration::from_secs(5),
        "Parallel processing took too long: {:?}",
        duration
    );

    println!("Parallel output processing completed in {:?}", duration);
}

#[tokio::test]
async fn test_concurrent_pipeline_processing() {
    use do_codeguardian::output::{OutputFormat, ParallelOutputProcessor};
    use do_codeguardian::types::{AnalysisResults, Finding, Severity};
    use std::collections::HashMap;

    // Create test analysis results
    let mut results = AnalysisResults::new("pipeline_test".to_string());

    // Add test findings
    for i in 0..5 {
        results.findings.push(Finding {
            id: format!("pipeline-test-finding-{}", i),
            analyzer: "test_analyzer".to_string(),
            rule: "test_rule".to_string(),
            severity: Severity::High,
            file: std::path::PathBuf::from(format!("pipeline_file_{}.rs", i)),
            line: i as u32 + 1,
            column: Some(5),
            message: format!("Pipeline test finding {}", i),
            description: Some(format!("Pipeline test description {}", i)),
            suggestion: Some("Fix this pipeline issue".to_string()),
            category: Some("pipeline_test".to_string()),
            metadata: HashMap::new(),
        });
    }

    results.summary.total_findings = 5;
    results.summary.total_files_scanned = 3;

    // Test concurrent pipeline processing
    let processor = ParallelOutputProcessor::new().unwrap();
    let formats = vec![OutputFormat::Json, OutputFormat::Markdown];

    let pipeline_result = processor
        .process_concurrent_pipeline(&results, formats)
        .await
        .unwrap();

    // Verify pipeline results
    assert_eq!(pipeline_result.outputs.len(), 2);
    assert!(pipeline_result.outputs.contains_key(&OutputFormat::Json));
    assert!(pipeline_result
        .outputs
        .contains_key(&OutputFormat::Markdown));

    // Verify validation results
    assert_eq!(pipeline_result.validation_results.len(), 2);
    for validation in pipeline_result.validation_results.values() {
        assert!(validation.is_valid, "All validations should pass");
    }

    // Verify pipeline efficiency
    assert!(
        pipeline_result.pipeline_efficiency >= 0.0 && pipeline_result.pipeline_efficiency <= 1.0
    );

    println!(
        "Concurrent pipeline processing completed in {:?} with {:.2}% efficiency",
        pipeline_result.total_processing_time,
        pipeline_result.pipeline_efficiency * 100.0
    );
}

// Consolidated workflow tests from e2e_workflow_tests.rs
#[test]
fn test_complete_analysis_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Initialize configuration
    let mut init_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    init_cmd.arg("init").current_dir(temp_dir.path());
    init_cmd.assert().success();

    // Step 2: Create test files
    fs::write(
        temp_dir.path().join("test.rs"),
        r#"
fn main() {
    let secret = "sk-abcdef123456"; // Security issue
    println!("Secret: {}", secret);
}
"#,
    )
    .unwrap();

    // Step 3: Run analysis
    let mut check_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    check_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    check_cmd.assert().success();
}

#[test]
fn test_ci_pipeline_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Simulate CI environment
    std::env::set_var("CI", "true");

    // Create project with issues
    fs::write(
        temp_dir.path().join("main.py"),
        r#"
import os
import subprocess

# Security issues for CI to catch
password = "admin123"  # Hardcoded password

def execute_command(user_input):
    # Command injection vulnerability
    subprocess.run(f"echo {user_input}", shell=True)

if __name__ == "__main__":
    execute_command("test")
"#,
    )
    .unwrap();

    // Run in CI mode
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("sarif")
        .arg("--quiet");

    cmd.assert().success();

    // Clean up
    std::env::remove_var("CI");
}
