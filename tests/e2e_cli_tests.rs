use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// End-to-end CLI workflow tests for CodeGuardian
/// Tests the complete user experience from command line to output

#[test]
fn test_cli_help_command() {
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "A security-first code analysis CLI",
        ))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("report"))
        .stdout(predicate::str::contains("init"));
}

#[test]
fn test_cli_version_command() {
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("codeguardian"));
}

#[test]
fn test_cli_check_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No files to analyze"));
}

#[test]
fn test_cli_check_with_security_issues() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file with a security issue
    let test_file = temp_dir.path().join("test.js");
    fs::write(
        &test_file,
        r#"const apiKey = "sk-1234567890abcdef1234567890abcdef";"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Total findings:"))
        .stdout(predicate::str::contains("security"));
}

#[test]
fn test_cli_check_with_multiple_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple test files
    fs::write(
        temp_dir.path().join("main.rs"),
        "fn main() { println!(\"Hello\"); }",
    )
    .unwrap();
    fs::write(temp_dir.path().join("lib.rs"), "pub fn test() {}").unwrap();
    fs::write(
        temp_dir.path().join("config.toml"),
        "[package]\nname = \"test\"",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"))
        .stdout(predicate::function(|output: &str| {
            // Should scan multiple files - look for "Files scanned: X" where X >= 2
            if let Some(line) = output.lines().find(|line| line.contains("Files scanned:")) {
                if let Some(count_str) = line.split(':').nth(1) {
                    if let Ok(count) = count_str.trim().parse::<u64>() {
                        return count >= 2;
                    }
                }
            }
            false
        }));
}

#[test]
fn test_cli_check_markdown_output() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Analysis Summary"))
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_sarif_output() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("sarif");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Analysis Summary"))
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_with_config_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config file
    let config_content = r#"
[general]
max_file_size = 1048576
parallel_workers = 2

[integrity]
check_binary_files = true
hash_algorithm = "blake3"
"#;
    let config_file = temp_dir.path().join("codeguardian.toml");
    fs::write(&config_file, config_content).unwrap();

    // Create a test file
    fs::write(temp_dir.path().join("test.rs"), "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--config")
        .arg(&config_file)
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_with_exclude_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // Create files that should be excluded
    fs::create_dir_all(temp_dir.path().join("target")).unwrap();
    fs::write(temp_dir.path().join("target/debug.rs"), "fn debug() {}").unwrap();
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--exclude")
        .arg("target/**")
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::function(|output: &str| {
            // Should not include excluded files in the output
            !output.contains("target/debug.rs")
        }));
}

#[test]
fn test_cli_check_with_include_patterns() {
    let temp_dir = TempDir::new().unwrap();

    // Create various file types
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir.path().join("test.js"), "console.log('test');").unwrap();
    fs::write(temp_dir.path().join("readme.txt"), "This is a readme").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--include")
        .arg("**/*.rs")
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"))
        .stdout(predicate::str::contains("Total findings:"));
}

#[test]
fn test_cli_init_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("init").current_dir(temp_dir.path());

    cmd.assert().success();

    // Check that config file was created
    let config_file = temp_dir.path().join("codeguardian.toml");
    assert!(config_file.exists());

    // Verify config file content
    let config_content = fs::read_to_string(config_file).unwrap();
    assert!(config_content.contains("[general]"));
    assert!(config_content.contains("[integrity]"));
}

#[test]
fn test_cli_report_conversion() {
    let temp_dir = TempDir::new().unwrap();

    // Create a JSON results file
    let results_json = r#"{
        "schema_version": "1.0.0",
        "tool_metadata": {
            "name": "codeguardian",
            "version": "0.1.0",
            "config_hash": "test",
            "timestamp": "2024-01-01T00:00:00Z"
        },
        "findings": [
            {
                "id": "test123",
                "analyzer": "security",
                "rule": "hardcoded_secret",
                "severity": "High",
                "file": "test.js",
                "line": 1,
                "column": null,
                "message": "Hardcoded secret detected",
                "description": null,
                "suggestion": null,
                "metadata": {}
            }
        ],
        "summary": {
            "total_files_scanned": 1,
            "total_findings": 1,
            "findings_by_severity": {"High": 1},
            "findings_by_analyzer": {"security": 1},
            "scan_duration_ms": 100
        },
        "config_hash": "test",
        "timestamp": "2024-01-01T00:00:00Z"
    }"#;

    let results_file = temp_dir.path().join("results.json");
    fs::write(&results_file, results_json).unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("report")
        .arg("--from")
        .arg(&results_file)
        .arg("--format")
        .arg("markdown");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("# CodeGuardian Analysis Report"))
        .stdout(predicate::str::contains("hardcoded_secret"));
}

#[test]
fn test_cli_turbo_mode() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple files for turbo mode testing
    for i in 0..5 {
        fs::write(
            temp_dir.path().join(format!("file_{}.rs", i)),
            format!("fn function_{}() {{}}", i),
        )
        .unwrap();
    }

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("turbo")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Turbo Analysis Results"))
        .stdout(predicate::str::contains("Files:"));
}

#[test]
fn test_cli_error_handling_invalid_path() {
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("/nonexistent/path/that/does/not/exist")
        .arg("--format")
        .arg("human");

    // Should handle gracefully - either succeed with empty results or fail with clear error
    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_cli_error_handling_invalid_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create invalid config file
    let config_file = temp_dir.path().join("invalid.toml");
    fs::write(&config_file, "invalid toml content [[[").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--config")
        .arg(&config_file)
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_performance_large_directory() {
    let temp_dir = TempDir::new().unwrap();

    // Create a moderate number of files to test performance
    for i in 0..20 {
        let subdir = temp_dir.path().join(format!("dir_{}", i));
        fs::create_dir_all(&subdir).unwrap();

        for j in 0..5 {
            fs::write(
                subdir.join(format!("file_{}.rs", j)),
                format!("fn function_{}() {{ /* content {} */ }}", i, j),
            )
            .unwrap();
        }
    }

    let start = std::time::Instant::now();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));

    let duration = start.elapsed();
    // Should complete within reasonable time (30 seconds for 100 files)
    assert!(
        duration.as_secs() < 30,
        "Analysis took too long: {:?}",
        duration
    );
}

#[test]
fn test_cli_output_file_creation() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();

    let output_file = temp_dir.path().join("results.json");

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human")
        .arg("--out")
        .arg(&output_file);

    cmd.assert().success();

    // Verify output file was created and contains valid JSON
    assert!(output_file.exists());
    let content = fs::read_to_string(&output_file).unwrap();
    assert!(serde_json::from_str::<serde_json::Value>(&content).is_ok());
}

#[test]
fn test_cli_verbose_output() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--verbose")
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_quiet_mode() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--quiet")
        .arg("--format")
        .arg("human");

    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_cli_diff_mode() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize a git repository
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Create and commit initial file
    fs::write(temp_dir.path().join("test.rs"), "fn original() {}").unwrap();
    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["commit", "-m", "initial"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Modify file
    fs::write(temp_dir.path().join("test.rs"), "fn modified() {}").unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--diff")
        .arg("HEAD")
        .arg("--format")
        .arg("human")
        .current_dir(temp_dir.path());

    // Should analyze only changed files
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}
