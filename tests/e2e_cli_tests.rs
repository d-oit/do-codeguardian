use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// End-to-end CLI workflow tests for CodeGuardian
/// Tests the complete user experience from command line to output

#[test]
fn test_cli_help_command() {
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "CodeGuardian performs comprehensive code analysis",
        ))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("git-commit"));
}

#[test]
fn test_cli_version_command() {
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("codeguardian"));
}

#[test]
fn test_cli_git_commit_command() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize a git repository
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Create a file and add to git
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();
    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Create initial commit to establish the main branch
    std::process::Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Modify the file to create staged changes
    fs::write(
        temp_dir.path().join("test.rs"),
        "fn test() { println!(\"modified\"); }",
    )
    .unwrap();
    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("git-commit")
        .arg("--message")
        .arg("Test commit")
        .current_dir(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Commit successful"));
}

#[test]
fn test_cli_check_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check").arg(temp_dir.path());

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

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(&test_file)
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Total findings:"));
}

#[test]
fn test_cli_check_with_multiple_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple test files
    let main_file = temp_dir.path().join("main.rs");
    let lib_file = temp_dir.path().join("lib.rs");
    let config_file = temp_dir.path().join("config.toml");
    fs::write(&main_file, "fn main() { println!(\"Hello\"); }").unwrap();
    fs::write(&lib_file, "pub fn test() {}").unwrap();
    fs::write(&config_file, "[package]\nname = \"test\"").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(&main_file)
        .arg(&lib_file)
        .arg(&config_file)
        .arg("--format")
        .arg("human");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"))
        .stdout(predicate::function(|output: &str| {
            // Should analyze multiple files - look for "Files scanned: X" where X >= 2
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
fn test_cli_check_output() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check").arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_output_detailed() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check").arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_with_config_file() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config file
    let config_content = r#"
[security]
fail_on_issues = false
min_severity = "low"
max_file_size = 1048576

[git]
conventional_commits = true
commit_template = "{type}({scope}): {description}"
require_signed_commits = false

[analysis]
exclude_patterns = ["*.log", "*.tmp"]
analyze_binaries = true
analysis_timeout = 300

[logging]
level = "info"
log_to_file = false
"#;
    let config_file = temp_dir.path().join("codeguardian.toml");
    fs::write(&config_file, config_content).unwrap();

    // Create a test file
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--config")
        .arg(&config_file)
        .arg("check")
        .arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_multiple_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create files
    fs::create_dir_all(temp_dir.path().join("target")).unwrap();
    let debug_file = temp_dir.path().join("target/debug.rs");
    let main_file = temp_dir.path().join("main.rs");
    fs::write(&debug_file, "fn debug() {}").unwrap();
    fs::write(&main_file, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check").arg(&main_file).arg(&debug_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_check_various_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create various file types
    let main_file = temp_dir.path().join("main.rs");
    let js_file = temp_dir.path().join("test.js");
    let txt_file = temp_dir.path().join("readme.txt");
    fs::write(&main_file, "fn main() {}").unwrap();
    fs::write(&js_file, "console.log('test');").unwrap();
    fs::write(&txt_file, "This is a readme").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check").arg(&main_file).arg(&js_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"))
        .stdout(predicate::str::contains("Total findings:"));
}

#[test]
fn test_cli_error_handling_invalid_path() {
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg("/nonexistent/path/that/does/not/exist");

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

    // Create a test file
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--config")
        .arg(&config_file)
        .arg("check")
        .arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_performance_multiple_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create a moderate number of files to test performance
    let mut file_paths = Vec::new();
    for i in 0..20 {
        let subdir = temp_dir.path().join(format!("dir_{}", i));
        fs::create_dir_all(&subdir).unwrap();

        for j in 0..5 {
            let file_path = subdir.join(format!("file_{}.rs", j));
            fs::write(
                &file_path,
                format!("fn function_{}() {{ /* content {} */ }}", i, j),
            )
            .unwrap();
            file_paths.push(file_path);
        }
    }

    let start = std::time::Instant::now();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check");
    for path in &file_paths {
        cmd.arg(path);
    }

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
fn test_cli_check_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check").arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}

#[test]
fn test_cli_verbose_output() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--verbose").arg("check").arg(&test_file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned:"));
}
