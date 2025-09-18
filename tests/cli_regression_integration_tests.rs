//! Integration tests for CLI regression prevention
//!
//! These tests ensure that the CLI behavior improvements are preserved
//! and validate end-to-end functionality with real file scenarios.

use assert_cmd::Command;
use do_codeguardian::analyzers::git_conflict_analyzer::GitConflictAnalyzer;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_cli_handles_test_files_correctly() {
    let temp_dir = TempDir::new().unwrap();

    // Create a test file with conflict markers that should be ignored
    let test_file = temp_dir.path().join("conflict_test.rs");
    let test_content = r#"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_conflict_detection() {
        let sample_conflict = r#"
some code here
<<<<<<< HEAD
original version
=======
modified version
>>>>>>> feature-branch
more code
"#;

        let analyzer = GitConflictAnalyzer::new();
        let findings = analyzer.analyze(Path::new("sample.rs"), sample_conflict.as_bytes()).unwrap();
        assert!(findings.len() > 0);
    }

    #[test]
    fn test_incomplete_implementation_detection() {
        let incomplete_code = "TODO: implement this function";
        assert!(incomplete_code.contains("TODO"));
    }
}

fn main() {
    println!("This is a test file");
}
"#;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Parse JSON output
    if let Ok(json) = serde_json::from_str::<Value>(&stdout) {
        let findings = json["findings"].as_array().unwrap();

        // Should not detect git conflicts in test code
        let git_conflict_findings: Vec<_> = findings.iter()
            .filter(|f| f["analyzer"] == "git_conflict")
            .collect();

        assert_eq!(git_conflict_findings.len(), 0,
                  "Should not detect git conflicts in test file content");

        // Should not detect incomplete implementations in test strings
        let ai_findings: Vec<_> = findings.iter()
            .filter(|f| f["analyzer"] == "ai_content" && f["rule"] == "incomplete_implementation")
            .collect();

        // If there are AI findings, they should not be from test string literals
        for finding in ai_findings {
            let line = finding["line"].as_u64().unwrap() as usize;
            let lines: Vec<&str> = test_content.lines().collect();
            if line > 0 && line <= lines.len() {
                let line_content = lines[line - 1];
                assert!(!line_content.contains("let sample_conflict = r#"),
                       "Should not flag test string literals");
                assert!(!line_content.contains("let incomplete_code = \"TODO:"),
                       "Should not flag TODO in test string assignments");
            }
        }
    }
}

#[test]
fn test_cli_config_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("simple.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--config")
       .arg("nonexistent-config.toml")
       .arg("check")
       .arg(&test_file)
       .arg("--quiet")
       .assert()
       .success(); // Should not fail, should use defaults
}

#[test]
fn test_cli_parallel_processing_stability() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple test files
    for i in 0..10 {
        let test_file = temp_dir.path().join(format!("test_{}.rs", i));
        let content = format!(r#"
fn function_{}() {{
    println!("Function {}", {});
}}

#[cfg(test)]
mod tests {{
    #[test]
    fn test_{}() {{
        let conflict_marker = "<<<<<<< HEAD";
        let separator = "=======";
        let end_marker = ">>>>>>> branch";
        assert!(conflict_marker.len() == 7);
    }}
}}
", i, i, i, i);
        fs::write(&test_file, content).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
       .arg(temp_dir.path())
       .arg("--parallel")
       .arg("4")
       .arg("--format")
       .arg("json")
       .assert()
       .success();
}

#[test]
fn test_cli_output_format_consistency() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("consistency_test.rs");

    let test_content = r#"
fn main() {
    // This might generate a finding
    eprintln!("Debug output");
}
"#;
    fs::write(&test_file, test_content).unwrap();

    // Test JSON format
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    let json_output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    assert!(json_output.status.success());
    let json_stdout = String::from_utf8(json_output.stdout).unwrap();

    // Should be valid JSON
    let json_result: Result<Value, _> = serde_json::from_str(&json_stdout);
    assert!(json_result.is_ok(), "JSON output should be valid");

    // Test SARIF format
    let sarif_file = temp_dir.path().join("output.sarif");
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
       .arg(&test_file)
       .arg("--format")
       .arg("sarif")
       .arg("--out")
       .arg(&sarif_file)
       .assert()
       .success();

    assert!(sarif_file.exists(), "SARIF file should be created");

    // Test human format
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
       .arg(&test_file)
       .arg("--format")
       .arg("human")
       .assert()
       .success();
}

#[test]
fn test_cli_debug_statement_detection_improvement() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("debug_improved.rs");

    let test_content = r#"
use tracing::Level;

fn main() {
    // This should NOT be flagged after our fix
    let log_level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    // This SHOULD still be flagged
    eprintln!("DEBUG: Error occurred");

    // This should NOT be flagged
    tracing::debug!("Proper debug logging");
}
"#;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    if let Ok(json) = serde_json::from_str::<Value>(&stdout) {
        let findings = json["findings"].as_array().unwrap();
        let debug_findings: Vec<_> = findings.iter()
            .filter(|f| f["rule"] == "debug_statement")
            .collect();

        // Should still flag eprintln! but not tracing::Level::DEBUG
        for finding in debug_findings {
            let line = finding["line"].as_u64().unwrap() as usize;
            let lines: Vec<&str> = test_content.lines().collect();
            if line > 0 && line <= lines.len() {
                let line_content = lines[line - 1];
                assert!(!line_content.contains("tracing::Level::DEBUG"),
                       "Should not flag tracing::Level::DEBUG as debug statement");
                assert!(!line_content.contains("tracing::debug!"),
                       "Should not flag proper tracing::debug! calls");
            }
        }
    }
}

#[test]
fn test_cli_mixed_content_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("mixed_content.rs");

    // File with mix of legitimate and problematic content
    let test_content = r#"
//! Module documentation
//! TODO: This should not be flagged - it's documentation

use std::collections::HashMap;

/// Function documentation
/// TODO: This should also not be flagged
fn process_data() {
    // This TODO in a comment might be flagged depending on analyzer settings
    // TODO: implement caching logic

    let data = HashMap::new();

    // This should definitely be flagged
    eprintln!("DEBUG: Processing started");

    // Process the data
    for item in data.values() {
        process_item(item);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_conflict_handling() {
        let conflict_content = r#"
<<<<<<< HEAD
test version
=======
other version
>>>>>>> branch
"#;
        // This should NOT be flagged as a real conflict
        assert!(conflict_content.contains("======="));
    }

    #[test]
    fn test_incomplete_detection() {
        // This should NOT be flagged
        let incomplete_msg = "TODO: write more tests";
        assert!(incomplete_msg.contains("TODO"));
    }
}

fn process_item(item: &str) {
    // Real implementation
    println!("Processing: {}", item);
}
"#;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    if let Ok(json) = serde_json::from_str::<Value>(&stdout) {
        let findings = json["findings"].as_array().unwrap();

        // Check git conflict findings
        let git_conflict_findings: Vec<_> = findings.iter()
            .filter(|f| f["analyzer"] == "git_conflict")
            .collect();

        assert_eq!(git_conflict_findings.len(), 0,
                  "Should not detect git conflicts in test string literals");

        // Check AI content findings
        let ai_findings: Vec<_> = findings.iter()
            .filter(|f| f["analyzer"] == "ai_content" && f["rule"] == "incomplete_implementation")
            .collect();

        // Should not flag documentation TODOs or test string literals
        for finding in ai_findings {
            let line = finding["line"].as_u64().unwrap() as usize;
            let lines: Vec<&str> = test_content.lines().collect();
            if line > 0 && line <= lines.len() {
                let line_content = lines[line - 1];
                assert!(!line_content.contains("//! TODO: This should not be flagged"),
                       "Should not flag module documentation TODOs");
                assert!(!line_content.contains("/// TODO: This should also not be flagged"),
                       "Should not flag function documentation TODOs");
                assert!(!line_content.contains("let incomplete_msg = \"TODO:"),
                       "Should not flag TODOs in test string literals");
            }
        }
    }
}

#[test]
fn test_cli_error_exit_codes() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("exit_code_test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    // Test normal operation (should exit 0)
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
       .arg(&test_file)
       .assert()
       .success();

    // Test with --fail-on-issues flag (behavior depends on findings)
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--fail-on-issues")
        .output()
        .unwrap();

    // Should exit successfully if no issues found, or with error code if issues found
    // Either way, it should not crash or panic
    assert!(output.status.code().is_some(), "Should have a proper exit code");
}

#[test]
fn test_cli_version_and_help() {
    // Test --version flag
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--version")
       .assert()
       .success()
       .stdout(predicate::str::contains("codeguardian"));

    // Test --help flag
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--help")
       .assert()
       .success()
       .stdout(predicate::str::contains("Usage:"));

    // Test subcommand help
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
       .arg("--help")
       .assert()
       .success()
       .stdout(predicate::str::contains("Run code analysis"));
}
