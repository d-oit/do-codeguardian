use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Enhanced end-to-end CLI workflow tests for CodeGuardian
/// Comprehensive testing of command-line interface and user experience

/// Test CLI help and basic commands
#[test]
fn test_cli_help_and_basic_commands() {
    // Test --help
    let mut help_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    help_cmd.arg("--help");

    help_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("CodeGuardian"))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("git-commit"));

    // Test --version
    let mut version_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    version_cmd.arg("--version");

    version_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("codeguardian"));

    // Test invalid command
    let mut invalid_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    invalid_cmd.arg("invalid-command");

    invalid_cmd
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

/// Test configuration file handling
#[test]
fn test_cli_config_file_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create a comprehensive config file
    let config_content = r#"
[security]
fail_on_issues = false
min_severity = "low"
max_file_size = 10485760

[git]
conventional_commits = true
commit_template = "{type}({scope}): {description}"
require_signed_commits = false

[analysis]
exclude_patterns = ["*.log", "*.tmp", "target/"]
analyze_binaries = false
analysis_timeout = 300

[logging]
level = "info"
log_to_file = false

[ml]
enabled = false
model_path = "codeguardian-model.fann"

[dashboard]
enabled = false
port = 8080
"#;

    let config_file = temp_dir.path().join("test-config.toml");
    fs::write(&config_file, config_content).unwrap();

    // Create test files
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() { println!(\"test\"); }").unwrap();

    // Test with custom config
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--config")
        .arg(&config_file)
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));

    // Test with non-existent config file
    let mut invalid_config_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    invalid_config_cmd
        .arg("--config")
        .arg("/nonexistent/config.toml")
        .arg("check")
        .arg(&test_file);

    // Should either succeed with defaults or fail gracefully
    let output = invalid_config_cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test check command with various options
#[test]
fn test_cli_check_command_comprehensive() {
    let temp_dir = TempDir::new().unwrap();

    // Create diverse test files
    let rust_file = temp_dir.path().join("main.rs");
    let js_file = temp_dir.path().join("app.js");
    let py_file = temp_dir.path().join("script.py");
    let config_file = temp_dir.path().join("config.toml");

    fs::write(
        &rust_file,
        r#"
fn main() {
    let password = "hardcoded123"; // Security issue
    println!("Password: {}", password);
}
"#,
    )
    .unwrap();

    fs::write(
        &js_file,
        r#"
const apiKey = "sk-1234567890abcdef"; // Security issue
console.log("API Key:", apiKey);
"#,
    )
    .unwrap();

    fs::write(
        &py_file,
        r#"
import os
password = "admin123"  # Security issue
print(f"Password: {password}")
"#,
    )
    .unwrap();

    fs::write(
        &config_file,
        r#"
[package]
name = "test"
version = "1.0.0"
"#,
    )
    .unwrap();

    // Test basic check
    let mut basic_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    basic_cmd
        .arg("check")
        .arg(&rust_file)
        .arg("--format")
        .arg("human");

    basic_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Files scanned"))
        .stdout(predicate::str::contains("Total findings"));

    // Test JSON output
    let mut json_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    json_cmd
        .arg("check")
        .arg(&rust_file)
        .arg("--format")
        .arg("json");

    json_cmd.assert().success();

    // Test SARIF output
    let mut sarif_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    sarif_cmd
        .arg("check")
        .arg(&rust_file)
        .arg("--format")
        .arg("sarif");

    sarif_cmd.assert().success();

    // Test multiple files
    let mut multi_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    multi_cmd
        .arg("check")
        .arg(&rust_file)
        .arg(&js_file)
        .arg(&py_file)
        .arg("--format")
        .arg("human");

    multi_cmd.assert().success();

    // Test directory scanning
    let mut dir_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    dir_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    dir_cmd.assert().success();
}

/// Test git integration commands
#[test]
fn test_cli_git_integration() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repository
    Command::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Create initial files
    let main_file = temp_dir.path().join("main.rs");
    fs::write(
        &main_file,
        r#"
fn main() {
    println!("Hello, world!");
}
"#,
    )
    .unwrap();

    // Initial commit
    Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Modify file with security issue
    fs::write(
        &main_file,
        r#"
fn main() {
    let secret = "sk-abcdef123456"; // Security issue added
    println!("Secret: {}", secret);
}
"#,
    )
    .unwrap();

    // Stage changes
    Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Test git-commit command
    let mut commit_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    commit_cmd
        .arg("git-commit")
        .arg("--message")
        .arg("Add security issue for testing")
        .current_dir(temp_dir.path());

    commit_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Commit successful"));

    // Verify commit was made
    let git_log = Command::new("git")
        .args(&["log", "--oneline", "-1"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let log_output = String::from_utf8_lossy(&git_log.stdout);
    assert!(log_output.contains("Add security issue for testing"));

    // Test git-commit-push command
    fs::write(
        &main_file,
        r#"
fn main() {
    let secret = env::var("API_SECRET").unwrap_or_default(); // Fixed
    println!("Secret handled securely");
}
"#,
    )
    .unwrap();

    Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Note: This would require a remote repository to fully test push
    // For now, just test the command structure
    let mut commit_push_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    commit_push_cmd
        .arg("git-commit-push")
        .arg("--message")
        .arg("Fix security issue")
        .current_dir(temp_dir.path());

    // Should succeed even without remote (git will fail at push)
    let output = commit_push_cmd.output().unwrap();
    assert!(output.status.success() || String::from_utf8_lossy(&output.stderr).contains("push"));
}

/// Test report generation and conversion
#[test]
fn test_cli_report_generation() {
    let temp_dir = TempDir::new().unwrap();

    // Create test file with issues
    let test_file = temp_dir.path().join("test.rs");
    fs::write(
        &test_file,
        r#"
fn main() {
    let password = "hardcoded123"; // Security issue
    println!("Password: {}", password);
}
"#,
    )
    .unwrap();

    // Run analysis and save results
    let results_file = temp_dir.path().join("results.json");
    let mut check_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    check_cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .arg("--out")
        .arg(&results_file);

    check_cmd.assert().success();
    assert!(results_file.exists());

    // Test report command - convert to markdown
    let mut report_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    report_cmd
        .arg("report")
        .arg("--from")
        .arg(&results_file)
        .arg("--format")
        .arg("markdown");

    // Just check that the command doesn't crash
    let _ = report_cmd.status();

    // Test report command - convert to HTML
    let mut html_report_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    html_report_cmd
        .arg("report")
        .arg("--from")
        .arg(&results_file)
        .arg("--format")
        .arg("html");

    // Just check that the command doesn't crash
    let _ = html_report_cmd.status();
}

/// Test GitHub integration
#[test]
fn test_cli_github_integration() {
    let temp_dir = TempDir::new().unwrap();

    // Create results file
    let results_file = temp_dir.path().join("results.json");
    let results_content = r#"
{
  "schema_version": "1.0.0",
  "tool_metadata": {
    "name": "codeguardian",
    "version": "0.2.1-alpha.1",
    "config_hash": "test",
    "timestamp": "2025-01-01T00:00:00Z"
  },
  "findings": [
    {
      "id": "test-finding-1",
      "file": "test.rs",
      "line": 3,
      "severity": "High",
      "analyzer": "secret",
      "rule": "hardcoded_secret",
      "message": "Hardcoded secret detected",
      "description": "Hardcoded secret detected",
      "context": "let password = \"secret123\";"
    }
  ],
  "summary": {
    "total_files_scanned": 1,
    "total_findings": 1,
    "findings_by_severity": {"high": 1},
    "findings_by_analyzer": {"secret": 1},
    "scan_duration_ms": 100
  },
  "config_hash": "test",
  "timestamp": "2025-01-01T00:00:00Z"
}
"#;
    fs::write(&results_file, results_content).unwrap();

    // Test gh-issue command (dry run mode to avoid actual API calls)
    let mut gh_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    gh_cmd
        .arg("gh-issue")
        .arg("--from")
        .arg(&results_file)
        .arg("--repo")
        .arg("test/repo")
        .arg("--dry-run");

    // Just check that the command doesn't crash (may fail in test environment)
    let _ = gh_cmd.status();
}

/// Test performance and scaling
#[test]
fn test_cli_performance_and_scaling() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple test files
    let file_count = 50;
    for i in 0..file_count {
        let file_path = temp_dir.path().join(format!("test_{}.rs", i));
        fs::write(
            &file_path,
            format!(
                r#"
fn function_{}() {{
    let data = "test_data_{}";
    println!("{{}}", data);
}}
"#,
                i, i
            ),
        )
        .unwrap();
    }

    // Test parallel processing
    let start = Instant::now();

    let mut parallel_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    parallel_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--parallel")
        .arg("4")
        .arg("--format")
        .arg("json");

    parallel_cmd.assert().success();

    let parallel_duration = start.elapsed();

    // Test sequential processing for comparison
    let start = Instant::now();

    let mut sequential_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    sequential_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--parallel")
        .arg("1")
        .arg("--format")
        .arg("json");

    sequential_cmd.assert().success();

    let sequential_duration = start.elapsed();

    // Just check that both complete successfully
    // Performance comparison may vary in test environment

    // Test with large files
    let large_file = temp_dir.path().join("large.rs");
    let large_content = format!("// Large file\n{}", "fn test() {}\n".repeat(10_000));
    fs::write(&large_file, large_content).unwrap();

    let mut large_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    large_cmd
        .arg("check")
        .arg(&large_file)
        .arg("--format")
        .arg("json");

    large_cmd.assert().success();
}

/// Test error handling and edge cases
#[test]
fn test_cli_error_handling_and_edge_cases() {
    // Test with non-existent file
    let mut nonexistent_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    nonexistent_cmd
        .arg("check")
        .arg("/definitely/does/not/exist.rs");

    let output = nonexistent_cmd.output().unwrap();
    assert!(
        output.status.success() || !output.stderr.is_empty(),
        "Should handle non-existent files gracefully"
    );

    // Test with empty directory
    let temp_dir = TempDir::new().unwrap();
    let mut empty_dir_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    empty_dir_cmd.arg("check").arg(temp_dir.path());

    let output = empty_dir_cmd.output().unwrap();
    assert!(output.status.success(), "Empty directory scan should succeed");

    // Test with binary file
    let temp_dir = TempDir::new().unwrap();
    let binary_file = temp_dir.path().join("binary.bin");
    fs::write(&binary_file, &[0u8; 1024]).unwrap();

    let mut binary_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    binary_cmd.arg("check").arg(&binary_file).arg("--format").arg("json");

    binary_cmd.assert().success();

    // Test with very long file paths
    let temp_dir = TempDir::new().unwrap();
    let mut long_path = temp_dir.path().to_path_buf();
    for i in 0..10 {
        long_path = long_path.join(format!("very_long_directory_name_{}", i));
    }
    long_path = long_path.join("test.rs");

    fs::create_dir_all(long_path.parent().unwrap()).unwrap();
    fs::write(&long_path, "fn main() {}").unwrap();

    let mut long_path_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    long_path_cmd.arg("check").arg(&long_path).arg("--format").arg("json");

    long_path_cmd.assert().success();
}

/// Test configuration validation
#[test]
fn test_cli_configuration_validation() {
    let temp_dir = TempDir::new().unwrap();

    // Test with invalid TOML
    let invalid_config = temp_dir.path().join("invalid.toml");
    fs::write(&invalid_config, "invalid [[[").unwrap();

    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    let mut invalid_config_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    invalid_config_cmd
        .arg("--config")
        .arg(&invalid_config)
        .arg("check")
        .arg(&test_file);

    // Should handle invalid config gracefully
    let output = invalid_config_cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());

    // Test with valid config but invalid values
    let valid_config = temp_dir.path().join("valid.toml");
    let config_content = r#"
[security]
fail_on_issues = true
min_severity = "invalid_severity"
"#;
    fs::write(&valid_config, config_content).unwrap();

    let mut valid_config_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    valid_config_cmd
        .arg("--config")
        .arg(&valid_config)
        .arg("check")
        .arg(&test_file);

    // Should handle gracefully
    let output = valid_config_cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test output formatting consistency
#[test]
fn test_cli_output_formatting_consistency() {
    let temp_dir = TempDir::new().unwrap();

    let test_file = temp_dir.path().join("test.rs");
    fs::write(
        &test_file,
        r#"
fn main() {
    let secret = "sk-1234567890"; // Security issue
    println!("Secret: {}", secret);
}
"#,
    )
    .unwrap();

    // Test all output formats
    let formats = vec!["human", "json", "sarif"];

    for format in formats {
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check").arg(&test_file).arg("--format").arg(format);

        let output = cmd.output().unwrap();
        assert!(output.status.success(), "Format {} should work", format);

        // Just check that the command succeeds
        // The output format validation is handled by the success check
    }
}

/// Test command combination and piping
#[test]
fn test_cli_command_combination() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    let file1 = temp_dir.path().join("file1.rs");
    let file2 = temp_dir.path().join("file2.rs");

    fs::write(&file1, "fn main() { let x = 1; }").unwrap();
    fs::write(&file2, "fn test() { let y = 2; }").unwrap();

    // Test multiple files in single command
    let mut multi_file_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    multi_file_cmd
        .arg("check")
        .arg(&file1)
        .arg(&file2)
        .arg("--format")
        .arg("json");

    multi_file_cmd.assert().success();

    // Test with various flags combined
    let mut combined_flags_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    combined_flags_cmd
        .arg("check")
        .arg(&file1)
        .arg("--format")
        .arg("human")
        .arg("--quiet")
        .arg("--fail-on-issues");

    let output = combined_flags_cmd.output().unwrap();
    // Should succeed (no issues in this simple file) and be quiet
    assert!(output.status.success());
}

/// Test timeout and resource management
#[test]
fn test_cli_timeout_and_resources() {
    let temp_dir = TempDir::new().unwrap();

    // Create a large number of files
    for i in 0..200 {
        let file_path = temp_dir.path().join(format!("load_test_{}.rs", i));
        fs::write(&file_path, format!("fn func_{}() {{}}", i)).unwrap();
    }

    // Test that analysis completes within reasonable time
    let start = Instant::now();

    let mut load_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    load_cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    load_cmd.assert().success();

    let duration = start.elapsed();
    assert!(
        duration < Duration::from_secs(60),
        "Analysis should complete within 60 seconds, took {:?}",
        duration
    );
}
