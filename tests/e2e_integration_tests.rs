use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Integration tests for CLI workflows with real file scenarios

#[test]
fn test_real_world_rust_project() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    // Create a realistic Rust project structure
    fs::create_dir_all(temp_dir.path().join("src")).expect("Failed to create src directory");
    fs::create_dir_all(temp_dir.path().join("tests")).expect("Failed to create tests directory");

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

    // Complex nested loops (performance issue)
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..5 {
                // Removed println for performance
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

    let mut cmd =
        Command::cargo_bin("do-codeguardian").expect("Failed to find do-codeguardian binary");
    cmd.arg("check")
        .arg(temp_dir.path().join("src/main.rs"))
        .arg(temp_dir.path().join("src/lib.rs"))
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"))
        .stdout(predicate::str::contains("findings"));
}

#[test]
fn test_javascript_project_analysis() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

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
const dbPassword = process.env.DB_PASSWORD || "test_password"; // Use environment variable
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
    .expect("Failed to write file");

    let mut cmd =
        Command::cargo_bin("do-codeguardian").expect("Failed to find do-codeguardian binary");
    cmd.arg("check")
        .arg(temp_dir.path().join("app.js"))
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("findings"));
}

#[test]
fn test_comprehensive_end_to_end_validation() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    // Create a comprehensive test project with multiple components
    fs::create_dir_all(temp_dir.path().join("src")).expect("Failed to write file");
    fs::create_dir_all(temp_dir.path().join("tests")).expect("Failed to write file");
    fs::create_dir_all(temp_dir.path().join("config")).expect("Failed to write file");
    fs::create_dir_all(temp_dir.path().join("scripts")).expect("Failed to write file");

    // Create Cargo.toml
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"
[package]
name = "comprehensive-test"
version = "1.0.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = "0.11"
"#,
    )
    .unwrap();

    // Create main.rs with various security issues
    fs::write(
        temp_dir.path().join("src/main.rs"),
        r#"
use std::env;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hardcoded credentials (security issue)
    let db_password = "super_secret_password_123";
    let api_key = "sk-1234567890abcdef1234567890abcdef";

    println!("Starting with password: {}", db_password);
    println!("API Key: {}", api_key);

    // Insecure HTTP client (no TLS verification)
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // SQL injection vulnerability
    let user_id = env::var("USER_ID").unwrap_or("1".to_string());
    let query = format!("SELECT * FROM users WHERE id = {}", user_id);

    // Command injection vulnerability
    let filename = env::var("FILENAME").unwrap_or("data.txt".to_string());
    let output = std::process::Command::new("cat")
        .arg(&filename)
        .output()?;

    println!("Query: {}", query);
    println!("File content: {:?}", String::from_utf8_lossy(&output.stdout));

    // Memory leak simulation
    let mut large_vec = Vec::new();
    for i in 0..1000000 {
        large_vec.push(format!("item_{}", i));
    }

    // Race condition potential
    let data = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..10 {
        let data_clone = std::sync::Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut data = data_clone.lock().expect("Failed to acquire mutex lock");
            data.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
"#,
    )
    .unwrap();

    // Create lib.rs with additional issues
    fs::write(
        temp_dir.path().join("src/lib.rs"),
        r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String, // Should be hashed
}

pub fn authenticate_user(username: &str, password: &str) -> Result<User, String> {
    // Insecure authentication - no password hashing
    if username == "admin" && password == "password123" {
        Ok(User {
            id: 1,
            name: "Admin".to_string(),
            password_hash: password.to_string(), // Storing plain password
        })
    } else {
        Err("Invalid credentials".to_string())
    }
}

pub fn process_payment(amount: f64, card_number: &str) -> Result<(), String> {
    // Logging sensitive payment data (security issue)
    println!("Processing payment of ${} with card: {}", amount, card_number);

    // No input validation
    if amount <= 0.0 {
        return Err("Invalid amount".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication() {
        let result = authenticate_user("admin", "password123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_payment_processing() {
        let result = process_payment(100.0, "4111111111111111");
        assert!(result.is_ok());
    }
}
"#,
    )
    .unwrap();

    // Create configuration file
    fs::write(
        temp_dir.path().join("config/settings.toml"),
        r#"
[database]
host = "localhost"
port = 5432
username = "admin"
password = "secret123"  # Hardcoded password in config

[api]
key = "api-key-123456789"
secret = "api-secret-abcdef123456"

[security]
enable_encryption = false
allow_insecure_connections = true
"#,
    )
    .unwrap();

    // Create shell script with security issues
    fs::write(
        temp_dir.path().join("scripts/deploy.sh"),
        r#"#!/bin/bash

# Insecure script with hardcoded credentials
DB_PASSWORD="prod_password_456"
API_KEY="prod-api-key-789"

echo "Deploying with DB password: $DB_PASSWORD"
echo "Using API key: $API_KEY"

# Command injection vulnerability
eval "rm -rf $1"

# Insecure file permissions
echo "secret data" > secret.txt
chmod 777 secret.txt

echo "Deployment complete"
"#,
    )
    .unwrap();

    // Make script executable
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(temp_dir.path().join("scripts/deploy.sh"))
        .expect("Failed to get file metadata")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(temp_dir.path().join("scripts/deploy.sh"), perms)
        .expect("Failed to set file permissions");

    // Create test file
    fs::write(
        temp_dir.path().join("tests/integration_test.rs"),
        r#"
#[test]
fn test_comprehensive_functionality() {
    // Test with various edge cases
    let result = 2 + 2;
    assert_eq!(result, 4);
}
"#,
    )
    .unwrap();

    // Run comprehensive analysis
    let mut cmd =
        Command::cargo_bin("do-codeguardian").expect("Failed to find do-codeguardian binary");
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .arg("--verbose");

    let output = cmd.output().expect("Failed to execute command");

    // Validate the analysis completed successfully
    assert!(
        output.status.success(),
        "Analysis failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Analysis output: {}", stdout);

    println!("✅ Comprehensive end-to-end validation passed");
}

#[test]
fn test_cross_component_workflow_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");

    // Create a multi-component project
    fs::create_dir_all(temp_dir.path().join("backend")).expect("Failed to write file");
    fs::create_dir_all(temp_dir.path().join("frontend")).expect("Failed to write file");
    fs::create_dir_all(temp_dir.path().join("infrastructure")).expect("Failed to write file");

    // Backend component (Rust)
    fs::write(
        temp_dir.path().join("backend/Cargo.toml"),
        r#"
[package]
name = "backend"
version = "1.0.0"

[dependencies]
axum = "0.6"
tokio = { version = "1.0", features = ["full"] }
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("backend/src/main.rs"),
        r#"
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello from backend!" }))
        .route("/api/users", get(|| async {
            // Security issue: exposing sensitive data
            "[{\"id\": 1, \"password\": \"user123\", \"ssn\": \"123-45-6789\"}]"
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
"#,
    )
    .unwrap();

    // Frontend component (JavaScript)
    fs::write(
        temp_dir.path().join("frontend/package.json"),
        r#"
{
  "name": "frontend",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "axios": "^1.0.0"
  }
}
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("frontend/app.js"),
        r#"
// Frontend with security issues
const API_BASE = "http://localhost:3000"; // Insecure HTTP
const API_KEY = "frontend-api-key-123"; // Hardcoded key

async function fetchUsers() {
    try {
        const response = await fetch(`${API_BASE}/api/users`, {
            headers: {
                'Authorization': `Bearer ${API_KEY}`,
                'X-API-Key': API_KEY
            }
        });

        const users = await response.json();

        // XSS vulnerability
        users.forEach(user => {
            document.body.innerHTML += `<div>User: ${user.password}</div>`;
        });

    } catch (error) {
        console.error('Error fetching users:', error);
    }
}

fetchUsers();
"#,
    )
    .unwrap();

    // Infrastructure component (Terraform-like)
    fs::write(
        temp_dir.path().join("infrastructure/main.tf"),
        r#"
// Infrastructure configuration with security issues
resource "aws_db_instance" "app_db" {
  username = "admin"
  password = "hardcoded_db_password_789"  # Security issue

  publicly_accessible = true  # Security issue
  backup_retention_period = 0  # Security issue
}

resource "aws_s3_bucket" "app_data" {
  bucket = "my-app-data-bucket"

  # Missing encryption
  server_side_encryption_configuration {
    rule {
      apply_server_side_encryption_by_default {
        sse_algorithm = "AES256"
      }
    }
  }
}

resource "aws_iam_user" "app_user" {
  name = "app-user"

  # Overly permissive policy
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = "*"
        Resource = "*"
      }
    ]
  })
}
"#,
    )
    .unwrap();

    // Run cross-component analysis
    let mut cmd =
        Command::cargo_bin("do-codeguardian").expect("Failed to find do-codeguardian binary");
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .arg("--recursive");

    let output = cmd.output().expect("Failed to execute command");

    // Validate cross-component analysis
    assert!(output.status.success(), "Cross-component analysis failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Cross-component analysis output: {}", stdout);

    // Basic validation that cross-component analysis completed

    println!("✅ Cross-component workflow integration test passed");
}
