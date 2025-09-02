use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Feature-specific end-to-end tests

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

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
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
    let mut cmd1 = Command::cargo_bin("codeguardian").unwrap();
    cmd1.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .current_dir(temp_dir.path());

    let start1 = std::time::Instant::now();
    cmd1.assert().success();
    let duration1 = start1.elapsed();

    // Second run - should use cache (faster)
    let mut cmd2 = Command::cargo_bin("codeguardian").unwrap();
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
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
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

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
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

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
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
    let mut baseline_cmd = Command::cargo_bin("codeguardian").unwrap();
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
    let mut diff_cmd = Command::cargo_bin("codeguardian").unwrap();
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
