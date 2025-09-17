# Core API Documentation

This document provides comprehensive API documentation for CodeGuardian's core functionality and interfaces.

## Table of Contents

- [Core Library API](#core-library-api)
- [Error Handling](#error-handling)
- [Configuration](#configuration)
- [Security Analysis](#security-analysis)
- [Git Operations](#git-operations)

## Core Library API

### Main Library Functions

#### `analyze_files`

```rust
pub async fn analyze_files(
    files: &[std::path::PathBuf],
    config: &Config,
) -> Result<security::AnalysisResults>
```

Performs comprehensive security analysis on the provided files.

**Parameters:**
- `files`: A slice of file paths to analyze
- `config`: Configuration settings for analysis

**Returns:**
- `Result<security::AnalysisResults>`: Analysis results or error

**Errors:**
- `CodeGuardianError::IoError`: File read failures
- `CodeGuardianError::AnalysisError`: Security analysis failures
- `CodeGuardianError::ConfigError`: Invalid configuration

**Example:**
```rust
use do_codeguardian::{analyze_files, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let results = analyze_files(&["src/main.rs".into()], &config).await?;
    println!("Found {} issues", results.issues.len());
    Ok(())
}
```

#### `git_commit`

```rust
pub async fn git_commit(message: Option<&str>, config: &Config) -> Result<()>
```

Performs a git commit with security analysis.

**Parameters:**
- `message`: Optional custom commit message
- `config`: Configuration for the commit operation

**Returns:**
- `Result<()>`: Success or error

**Errors:**
- `CodeGuardianError::GitError`: Git repository issues
- `CodeGuardianError::AnalysisError`: Security analysis failures

**Example:**
```rust
use do_codeguardian::{git_commit, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    git_commit(Some("Fix security vulnerability"), &config).await?;
    println!("Commit successful");
    Ok(())
}
```

## Error Handling

### CodeGuardianError

```rust
#[derive(Debug, thiserror::Error)]
pub enum CodeGuardianError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Integration error: {0}")]
    IntegrationError(String),

    #[error("Remediation error: {0}")]
    RemediationError(String),
}
```

### Result Type

```rust
pub type Result<T> = std::result::Result<T, CodeGuardianError>;
```

## Configuration

### Config Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub security: SecurityConfig,
    pub git: GitConfig,
    pub output: OutputConfig,
    pub integrations: IntegrationsConfig,
    pub dashboard: DashboardConfig,
    pub remediation: RemediationConfig,
    pub relationships: RelationshipConfig,
}
```

**Fields:**
- `security`: Security analysis configuration
- `git`: Git operations configuration
- `output`: Output formatting configuration
- `integrations`: External system integrations (new in v0.2.0-alpha.5)
- `dashboard`: Dashboard configuration (new in v0.2.0-alpha.5)
- `remediation`: Remediation workflows (new in v0.2.0-alpha.5)
- `relationships`: Relationship management (new in v0.2.0-alpha.5)

### Default Configuration

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            security: SecurityConfig::default(),
            git: GitConfig::default(),
            output: OutputConfig::default(),
            integrations: IntegrationsConfig::default(),
            dashboard: DashboardConfig::default(),
            remediation: RemediationConfig::default(),
            relationships: RelationshipConfig::default(),
        }
    }
}
```

## Security Analysis

### AnalysisResults

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub issues: Vec<SecurityIssue>,
    pub summary: AnalysisSummary,
    pub metadata: AnalysisMetadata,
}
```

**Fields:**
- `issues`: List of detected security issues
- `summary`: Summary statistics
- `metadata`: Analysis metadata

### SecurityIssue

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub id: String,
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub file: PathBuf,
    pub line: u32,
    pub column: Option<u32>,
    pub code: Option<String>,
    pub confidence: f64,
    pub cwe_id: Option<String>,
    pub references: Vec<String>,
}
```

## Git Operations

### GitConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub enable_security_checks: bool,
    pub require_clean_tree: bool,
    pub max_commit_size_kb: u32,
    pub allowed_file_types: Vec<String>,
    pub blocked_file_patterns: Vec<String>,
}
```

### Git Operations

```rust
pub mod git {
    pub async fn commit(message: Option<&str>, config: &Config) -> Result<()>;
    pub async fn analyze_staged_changes(config: &Config) -> Result<AnalysisResults>;
    pub fn generate_commit_message(results: &AnalysisResults) -> String;
}
```

## Usage Examples

### Basic Security Analysis

```rust
use do_codeguardian::{analyze_files, Config};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let files = vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/lib.rs"),
    ];

    let results = analyze_files(&files, &config).await?;

    println!("Analysis completed:");
    println!("- Total issues: {}", results.issues.len());
    println!("- High severity: {}", results.summary.high_severity_count);
    println!("- Medium severity: {}", results.summary.medium_severity_count);
    println!("- Low severity: {}", results.summary.low_severity_count);

    Ok(())
}
```

### Git Commit with Security Checks

```rust
use do_codeguardian::{git_commit, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::default();
    config.git.enable_security_checks = true;
    config.git.require_clean_tree = true;

    git_commit(Some("Implement new feature with security fixes"), &config).await?;
    println!("Secure commit completed");
    Ok(())
}
```

## Error Handling Examples

```rust
use do_codeguardian::{analyze_files, CodeGuardianError, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let files = vec!["nonexistent_file.rs".into()];

    match analyze_files(&files, &config).await {
        Ok(results) => {
            println!("Analysis successful: {} issues found", results.issues.len());
        }
        Err(CodeGuardianError::IoError(e)) => {
            eprintln!("File access error: {}", e);
        }
        Err(CodeGuardianError::AnalysisError(e)) => {
            eprintln!("Analysis failed: {}", e);
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }

    Ok(())
}
```
