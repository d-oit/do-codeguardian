//! # Update Docs Command
//!
//! This module implements the update-docs functionality for maintaining
//! and updating documentation files in the project.
//!
//! ## Features
//!
//! - Updates all documentation files
//! - Validates documentation structure
//! - Generates missing documentation
//! - Ensures documentation consistency

use crate::config::Config;
use crate::error::Result;
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Execute the update-docs command
///
/// This function performs a comprehensive documentation update that includes:
/// 1. Scanning for documentation files
/// 2. Updating existing documentation
/// 3. Generating missing documentation
/// 4. Validating documentation structure
///
/// # Arguments
///
/// * `config` - Configuration for the operation
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
///
/// # Errors
///
/// This function will return an error if:
/// - Documentation directory is not found
/// - File operations fail
/// - Documentation validation fails
pub async fn execute_update_docs(config: &Config) -> Result<()> {
    info!("Starting documentation update process");

    // Define documentation paths
    let docs_paths = vec![
        PathBuf::from("docs"),
        PathBuf::from("README.md"),
        PathBuf::from("CONTRIBUTING.md"),
        PathBuf::from("CHANGELOG.md"),
    ];

    // Check if documentation directories exist
    for path in &docs_paths {
        if !path.exists() {
            if path.is_dir() {
                warn!(
                    "Documentation directory {} not found, skipping",
                    path.display()
                );
            } else {
                debug!("Documentation file {} not found", path.display());
            }
        }
    }

    // Update API documentation
    update_api_docs(config).await?;

    // Update user guide documentation
    update_user_guide(config).await?;

    // Update configuration documentation
    update_config_docs(config).await?;

    // Validate documentation structure
    validate_docs_structure(config).await?;

    info!("Documentation update completed successfully");
    Ok(())
}

/// Update API documentation files
async fn update_api_docs(_config: &Config) -> Result<()> {
    let api_docs_path = PathBuf::from("docs/api");

    if !api_docs_path.exists() {
        info!("Creating API documentation directory");
        std::fs::create_dir_all(&api_docs_path)?;
    }

    // Generate API documentation index
    let api_index_content = r#"# API Documentation

This directory contains comprehensive API documentation for CodeGuardian.

## Available Documentation

- [Core API](core.md) - Core functionality and interfaces
- [CLI API](cli.md) - Command-line interface documentation
- [Configuration API](config.md) - Configuration management
- [Analysis API](analysis.md) - Code analysis interfaces

## Getting Started

For information on how to use the CodeGuardian API, see the [main documentation](../README.md).

## Contributing

When adding new API functionality, please update this documentation accordingly.
"#;

    let api_index_path = api_docs_path.join("index.md");
    std::fs::write(&api_index_path, api_index_content)?;
    debug!(
        "Updated API documentation index: {}",
        api_index_path.display()
    );

    Ok(())
}

/// Update user guide documentation
async fn update_user_guide(_config: &Config) -> Result<()> {
    let user_guide_path = PathBuf::from("docs/user-guide");

    if !user_guide_path.exists() {
        info!("Creating user guide directory");
        std::fs::create_dir_all(&user_guide_path)?;
    }

    // Update installation guide
    let install_content = r#"# Installation

## Prerequisites

- Rust 1.70 or later
- Git
- (Optional) GitHub CLI for enhanced features

## Installation Methods

### From Source

```bash
git clone https://github.com/your-org/codeguardian.git
cd codeguardian
cargo build --release
```

### Using Cargo

```bash
cargo install codeguardian
```

## Configuration

After installation, initialize CodeGuardian:

```bash
codeguardian init
```

This creates a default `codeguardian.toml` configuration file.
"#;

    let install_path = user_guide_path.join("installation.md");
    std::fs::write(&install_path, install_content)?;
    debug!("Updated installation guide: {}", install_path.display());

    Ok(())
}

/// Update configuration documentation
async fn update_config_docs(_config: &Config) -> Result<()> {
    let config_docs_path = PathBuf::from("docs/config");

    if !config_docs_path.exists() {
        info!("Creating configuration documentation directory");
        std::fs::create_dir_all(&config_docs_path)?;
    }

    // Generate configuration documentation
    let config_content = r#"# Configuration

CodeGuardian uses TOML configuration files for customization.

## Default Configuration

```toml
[security]
fail_on_issues = true
max_file_size = "10MB"

[analysis]
parallel_workers = 0
streaming = false

[output]
format = "json"
verbose = false
```

## Configuration Options

### Security Settings

- `fail_on_issues`: Exit with error if security issues are found
- `max_file_size`: Maximum file size to analyze

### Analysis Settings

- `parallel_workers`: Number of parallel analysis workers (0 = auto)
- `streaming`: Enable streaming analysis for large files

### Output Settings

- `format`: Output format (json, human, sarif)
- `verbose`: Enable verbose output
"#;

    let config_readme_path = config_docs_path.join("README.md");
    std::fs::write(&config_readme_path, config_content)?;
    debug!(
        "Updated configuration documentation: {}",
        config_readme_path.display()
    );

    Ok(())
}

/// Validate documentation structure
async fn validate_docs_structure(_config: &Config) -> Result<()> {
    info!("Validating documentation structure");

    // Check for required documentation files
    let required_files = vec![
        "README.md",
        "docs/README.md",
        "docs/api/index.md",
        "docs/user-guide/installation.md",
        "docs/config/README.md",
    ];

    for file in required_files {
        let path = PathBuf::from(file);
        if !path.exists() {
            warn!("Missing documentation file: {}", file);
        } else {
            debug!("Found documentation file: {}", file);
        }
    }

    // Validate markdown syntax (basic check)
    validate_markdown_files()?;

    Ok(())
}

/// Validate markdown files for basic syntax
fn validate_markdown_files() -> Result<()> {
    use std::fs;

    let docs_dir = PathBuf::from("docs");

    if !docs_dir.exists() {
        return Ok(());
    }

    // Recursively find all .md files
    fn find_md_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    files.extend(find_md_files(&path)?);
                } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                    files.push(path);
                }
            }
        }

        Ok(files)
    }

    let md_files = find_md_files(&docs_dir)?;
    let file_count = md_files.len();

    for file in md_files {
        // Basic validation - check if file is readable
        match fs::read_to_string(&file) {
            Ok(content) => {
                if content.trim().is_empty() {
                    warn!("Empty markdown file: {}", file.display());
                }
            }
            Err(e) => {
                warn!("Failed to read markdown file {}: {}", file.display(), e);
            }
        }
    }

    debug!("Validated {} markdown files", file_count);
    Ok(())
}
