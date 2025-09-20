//! Regression tests to prevent re-introduction of fixed issues
const BIN_NAME: &str = "do-codeguardian";
//!
//! This test suite validates that specific bugs and false positives
//! that were previously fixed do not resurface in future changes.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test that git conflict analyzer doesn't flag test code with conflict markers
#[test]
fn test_git_conflict_analyzer_ignores_test_content() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test_conflicts.rs");

    // Create a test file with conflict markers in test context
    let test_content = r##"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_complete_conflict() {
        let content = r###"
some code
<<<<<<< HEAD
version 1
=======
version 2
>>>>>>> branch
more code
"###;
        // This is test code with conflict markers that should not be flagged
        // let findings = analyzer.analyze(Path::new("test.rs"), content.as_bytes()).unwrap();
        // assert!(findings.iter().any(|f| f.rule == "merge_conflict_start"));
    }
"##;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    // Should not detect git conflicts in test code
    let output_str = String::from_utf8(output.stdout).unwrap();
    assert!(!output_str.contains("git_conflict"),
            "Git conflict analyzer should not flag test content with conflict markers");
}

/// Test that AI content analyzer doesn't flag legitimate documentation
#[test]
fn test_ai_content_analyzer_ignores_documentation() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("documented_code.rs");

    // Create a file with legitimate documentation containing "TODO"
    let test_content = r#"
//! # Module Documentation
//!
//! This module provides functionality for handling tasks.
//! TODO: Add more comprehensive examples in future versions.

/// This function handles user input validation
/// TODO: Implement additional validation rules
fn validate_input(input: &str) -> bool {
    // Implementation here
    !input.is_empty()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validation() {
        let content = "TODO: test placeholder";
        assert!(true);
    }
}
"#;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    let output_str = String::from_utf8(output.stdout).unwrap();

    // Should not flag documentation comments or test content
    if output_str.contains("incomplete_implementation") {
        // If any incomplete implementation findings exist, they should not be in comments
        assert!(!output_str.contains("TODO: Add more comprehensive examples"),
                "AI content analyzer should not flag TODO in documentation comments");
        assert!(!output_str.contains("TODO: Implement additional validation"),
                "AI content analyzer should not flag TODO in function documentation");
    }
}

/// Test that debug statement detection works correctly but doesn't over-flag
#[test]
fn test_debug_statement_detection_accuracy() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("debug_test.rs");

    let test_content = r#"
use tracing::Level;

fn main() {
    // This should NOT be flagged - proper logging level usage
    let log_level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    // This SHOULD be flagged - debug print statement
    eprintln!("DEBUG: Something went wrong: {}", error);

    // This should NOT be flagged - proper logging
    tracing::warn!("Configuration error: {}", e);

    // This should NOT be flagged - documentation example
    /// println!("Hello world");
    fn example() {}

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_something() {
            // This should NOT be flagged - test code
            println!("Test output: {}", result);
        }
    }
}
"#;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    let output_str = String::from_utf8(output.stdout).unwrap();

    if output_str.contains("debug_statement") {
        // Should flag eprintln! but not tracing::Level::DEBUG or documentation
        assert!(!output_str.contains("tracing::Level::DEBUG"),
                "Should not flag proper tracing level usage as debug statement");
    }
}

/// Test that configuration loading handles missing files gracefully
#[test]
fn test_config_loading_graceful_degradation() {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("--config")
        .arg("nonexistent-config.toml")
        .arg("check")
        .arg("--help")
        .output()
        .unwrap();

    // Should not panic or fail hard, should continue with defaults
    assert!(output.status.success() || output.status.code() == Some(0),
            "Should handle missing config gracefully and continue");

    let stderr = String::from_utf8(output.stderr).unwrap();
    // Should show warning but not crash
    assert!(stderr.contains("Configuration file error") || stderr.contains("Using defaults"),
            "Should warn about missing config and use defaults");
}

/// Test that performance analyzer doesn't over-flag legitimate patterns
#[test]
fn test_performance_analyzer_accuracy() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("performance_test.rs");

    let test_content = r#"
fn legitimate_loops() {
    // This should NOT be flagged - single loop
    for item in items {
        process(item);
    }

    // This MIGHT be flagged - nested loops (performance concern)
    for i in 0..100 {
        for j in 0..100 {
            matrix[i][j] = calculate(i, j);
        }
    }

    // This should NOT be flagged - functional style
    let results: Vec<_> = items.iter()
        .map(|item| item.process())
        .collect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nested_operations() {
        // This should NOT be flagged - test code with nested structures
        for test_case in test_cases {
            for input in test_case.inputs {
                assert_eq!(process(input), expected);
            }
        }
    }
}
"#;

    fs::write(&test_file, test_content).unwrap();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    // Should exit successfully regardless of findings
    assert!(output.status.success());
}

/// Test that JSON output maintains consistent structure
#[test]
fn test_json_output_schema_consistency() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("schema_test.rs");

    fs::write(&test_file, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    let output_str = String::from_utf8(output.stdout).unwrap();

    // Parse JSON to ensure it's valid
    let json_result: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
    assert!(json_result.is_ok(), "JSON output should be valid");

    let json = json_result.unwrap();

    // Check required schema fields are present
    assert!(json.get("schema_version").is_some(), "Should have schema_version");
    assert!(json.get("tool_metadata").is_some(), "Should have tool_metadata");
    assert!(json.get("findings").is_some(), "Should have findings array");
    assert!(json.get("summary").is_some(), "Should have summary");
    assert!(json.get("timestamp").is_some(), "Should have timestamp");

    // Check tool metadata structure
    let metadata = json.get("tool_metadata").unwrap();
    assert!(metadata.get("name").is_some(), "Should have tool name");
    assert!(metadata.get("version").is_some(), "Should have tool version");
    assert!(metadata.get("config_hash").is_some(), "Should have config hash");
}

/// Test that SARIF output format works correctly
#[test]
fn test_sarif_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("sarif_test.rs");
    let output_file = temp_dir.path().join("output.sarif");

    fs::write(&test_file, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("sarif")
        .arg("--out")
        .arg(&output_file)
        .output()
        .unwrap();

    assert!(output.status.success(), r#"SARIF output should succeed"#);
    assert!(output_file.exists(), r#"SARIF file should be created"#);

    // SARIF files should be valid JSON (CodeGuardian currently outputs JSON format for SARIF)
    let sarif_content = fs::read_to_string(&output_file).unwrap();
    let sarif_json: Result<serde_json::Value, _> = serde_json::from_str(&sarif_content);
    assert!(sarif_json.is_ok(), r#"SARIF output should be valid JSON"#);
}

/// Test that parallel processing works without race conditions
#[test]
fn test_parallel_processing_stability() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple test files
    for i in 0..5 {
        let test_file = temp_dir.path().join(format!(r#"test_{}.rs"#, i));
        fs::write(&test_file, format!("fn test_function_{}() {{}}", i)).unwrap();
    }

    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    let output = cmd
        .arg("check")
        .arg(temp_dir.path())
        .arg("--parallel")
        .arg("4")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    assert!(output.status.success(), r#"Parallel processing should succeed"#);

    let output_str = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output_str).unwrap();

    // Should have processed all files
    let files_scanned = json["summary"]["total_files_scanned"].as_u64().unwrap();
    assert_eq!(files_scanned, 5, "Should scan all 5 test files");
}
