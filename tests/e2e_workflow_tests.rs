use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// End-to-end workflow tests covering complete user scenarios

#[test]
fn test_complete_analysis_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Step 1: Initialize configuration
    let mut init_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    init_cmd.arg("init").current_dir(temp_dir.path());
    init_cmd.assert().success();

    // Verify config file exists
    assert!(temp_dir.path().join("codeguardian.toml").exists());

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
        .arg("json")
        .arg("--output")
        .arg(temp_dir.path().join("results.json"));

    check_cmd.assert().success();

    // Step 4: Verify results file
    let results_file = temp_dir.path().join("results.json");
    assert!(results_file.exists());

    let results_content = fs::read_to_string(&results_file).unwrap();
    assert!(results_content.contains("hardcoded_secret"));

    // Step 5: Convert to markdown report
    let mut report_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    report_cmd
        .arg("report")
        .arg("--from")
        .arg(&results_file)
        .arg("--format")
        .arg("markdown")
        .arg("--output")
        .arg(temp_dir.path().join("report.md"));

    report_cmd.assert().success();

    // Step 6: Verify markdown report
    let report_file = temp_dir.path().join("report.md");
    assert!(report_file.exists());

    let report_content = fs::read_to_string(&report_file).unwrap();
    assert!(report_content.contains("# CodeGuardian Analysis Report"));
    assert!(report_content.contains("hardcoded_secret"));
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
api_key = os.getenv("API_KEY", "fallback-key-456")

def execute_command(user_input):
    # Command injection vulnerability
    subprocess.run(f"echo {user_input}", shell=True)

if __name__ == "__main__":
    execute_command("test")
"#,
    )
    .unwrap();

    // Run in CI mode (should be faster, less verbose)
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("sarif") // CI-friendly format
        .arg("--quiet"); // No progress output

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"$schema\""))
        .stdout(predicate::str::contains("command_injection"))
        .stderr(predicate::str::is_empty());

    // Clean up
    std::env::remove_var("CI");
}
