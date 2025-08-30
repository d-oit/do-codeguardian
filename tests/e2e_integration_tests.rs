use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Integration tests for CLI workflows with real file scenarios

#[test]
fn test_real_world_rust_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create a realistic Rust project structure
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();
    fs::create_dir_all(temp_dir.path().join("tests")).unwrap();

    // Cargo.toml
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
"#,
    )
    .unwrap();

    // Main source file with potential issues
    fs::write(
        temp_dir.path().join("src/main.rs"),
        r#"
use std::env;

fn main() {
    let api_key = "sk-test123456789abcdef"; // Hardcoded secret
    println!("Starting application with key: {}", api_key);
    
    // TODO: Remove this debug code
    println!("Debug mode enabled");
    
    // Complex nested loops (performance issue)
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..10 {
                println!("{}-{}-{}", i, j, k);
            }
        }
    }
}
"#,
    )
    .unwrap();

    // Library file
    fs::write(
        temp_dir.path().join("src/lib.rs"),
        r#"
pub fn calculate(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate() {
        assert_eq!(calculate(2, 2), 4);
    }
}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path().join("src/main.rs"))
        .arg(temp_dir.path().join("src/lib.rs"))
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files scanned"))
        .stdout(predicate::str::contains("findings"));
}

#[test]
fn test_javascript_project_analysis() {
    let temp_dir = TempDir::new().unwrap();

    // Create JavaScript project with security issues
    fs::write(
        temp_dir.path().join("package.json"),
        r#"
{
  "name": "test-app",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0"
  }
}
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("app.js"),
        r#"
const express = require('express');
const app = express();

// Security issues
const dbPassword = "admin123"; // Hardcoded password
const apiKey = process.env.API_KEY || "default-key-123";

app.get('/user/:id', (req, res) => {
    // SQL injection vulnerability
    const query = "SELECT * FROM users WHERE id = " + req.params.id;
    
    // XSS vulnerability
    res.send("<h1>User: " + req.query.name + "</h1>");
});

app.listen(3000);
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path().join("app.js"))
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findings"));
}
