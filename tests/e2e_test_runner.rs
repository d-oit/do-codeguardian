use std::fs;
/// Comprehensive end-to-end test runner and utilities
use std::process::Command;
use tempfile::TempDir;

/// Helper functions for E2E tests
pub fn create_sample_rust_project(temp_dir: &TempDir) {
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"
[package]
name = "sample-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("src/main.rs"),
        r#"
fn main() {
    println!("Hello, world!");
}
"#,
    )
    .unwrap();
}

pub fn create_sample_javascript_project(temp_dir: &TempDir) {
    fs::write(
        temp_dir.path().join("package.json"),
        r#"
{
  "name": "sample-app",
  "version": "1.0.0"
}
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("index.js"),
        r#"
console.log("Hello, world!");
"#,
    )
    .unwrap();
}

pub fn create_git_repository(temp_dir: &TempDir) {
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
}

#[cfg(test)]
mod integration_helpers {
    use super::*;

    #[test]
    fn test_helper_rust_project_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_sample_rust_project(&temp_dir);

        assert!(temp_dir.path().join("Cargo.toml").exists());
        assert!(temp_dir.path().join("src/main.rs").exists());
    }

    #[test]
    fn test_helper_javascript_project_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_sample_javascript_project(&temp_dir);

        assert!(temp_dir.path().join("package.json").exists());
        assert!(temp_dir.path().join("index.js").exists());
    }

    #[test]
    fn test_helper_git_repository_creation() {
        let temp_dir = TempDir::new().unwrap();
        create_git_repository(&temp_dir);

        assert!(temp_dir.path().join(".git").exists());
    }
}

/// Test suite runner for comprehensive E2E testing
#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    use assert_cmd::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn test_full_development_workflow() {
        let temp_dir = TempDir::new().unwrap();

        // 1. Create project
        create_sample_rust_project(&temp_dir);
        create_git_repository(&temp_dir);

        // 2. Initialize CodeGuardian
        let mut init_cmd = Command::cargo_bin("codeguardian").unwrap();
        init_cmd.arg("init").current_dir(temp_dir.path());
        init_cmd.assert().success();

        // 3. Add some code with issues
        fs::write(
            temp_dir.path().join("src/lib.rs"),
            r#"
pub fn authenticate(password: &str) -> bool {
    let hardcoded_pass = "admin123"; // Security issue
    password == hardcoded_pass
}

pub fn process_data() {
    // TODO: Implement this function
    println!("Processing...");
}
"#,
        )
        .unwrap();

        // 4. Commit initial version
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

        // 5. Run full analysis
        let mut check_cmd = Command::cargo_bin("codeguardian").unwrap();
        check_cmd
            .arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json")
            .current_dir(temp_dir.path());

        check_cmd
            .assert()
            .success()
            .stdout(predicate::str::contains("hardcoded_secret"))
            .stdout(predicate::str::contains("non_production"));

        // 6. Fix issues and test diff mode
        fs::write(
            temp_dir.path().join("src/lib.rs"),
            r#"
use std::env;

pub fn authenticate(password: &str) -> bool {
    let expected_pass = env::var("ADMIN_PASSWORD").unwrap_or_default();
    password == expected_pass
}

pub fn process_data() {
    println!("Processing data...");
}
"#,
        )
        .unwrap();

        // 7. Run diff analysis
        let mut diff_cmd = Command::cargo_bin("codeguardian").unwrap();
        diff_cmd
            .arg("check")
            .arg(temp_dir.path())
            .arg("--diff")
            .arg("HEAD")
            .arg("--format")
            .arg("json")
            .current_dir(temp_dir.path());

        diff_cmd
            .assert()
            .success()
            .stdout(predicate::function(|output: &str| {
                // Should have fewer issues after fixes
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(output) {
                    if let Some(findings) = json["findings"].as_array() {
                        return findings.len() < 2; // Should be fewer issues
                    }
                }
                false
            }));
    }
}
