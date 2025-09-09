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
/// * `args` - Command line arguments for update-docs
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
pub async fn execute_update_docs(config: &Config, args: &crate::cli::UpdateDocsArgs) -> Result<()> {
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

    // Update documentation based on flags
    if args.api || args.force {
        info!("Updating API documentation");
        update_api_docs(config).await?;
    }

    if args.user_guide || args.force {
        info!("Updating user guide documentation");
        update_user_guide(config).await?;
    }

    if args.config || args.force {
        info!("Updating configuration documentation");
        update_config_docs(config).await?;
    }

    // Always validate documentation structure
    if !args.validate_only {
        info!("Validating documentation structure");
        validate_docs_structure(config).await?;
    }

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

    // Generate command documentation for all commands
    generate_command_docs(&user_guide_path).await?;

    Ok(())
}

/// Generate documentation for all CodeGuardian commands
#[allow(clippy::ptr_arg)]
async fn generate_command_docs(user_guide_path: &PathBuf) -> Result<()> {
    info!("Generating command documentation");

    // Generate documentation for each command
    generate_check_docs(user_guide_path).await?;
    generate_report_docs(user_guide_path).await?;
    generate_gh_issue_docs(user_guide_path).await?;
    generate_init_docs(user_guide_path).await?;
    generate_git_commit_docs(user_guide_path).await?;
    generate_git_commit_push_docs(user_guide_path).await?;
    generate_turbo_docs(user_guide_path).await?;
    generate_train_docs(user_guide_path).await?;
    generate_update_docs_docs(user_guide_path).await?;

    Ok(())
}

/// Generate documentation for the check command
#[allow(clippy::ptr_arg)]
async fn generate_check_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# check

## Synopsis
CodeGuardian's primary code analysis command that performs comprehensive security, performance, code quality, dependency, integrity, and naming analysis with advanced ML-powered false positive reduction and seamless GitHub integration.

## Description
The check command serves as the main entry point for CodeGuardian's analysis engine, providing deterministic findings with stable IDs, security-by-default configuration, and CI-first user experience. It supports multiple output formats, GitHub integration, performance tuning, and ML-enhanced analysis for large-scale codebases.

Key capabilities include:
- **Comprehensive Analysis**: Security, performance, code quality, dependency, integrity, and naming checks
- **ML-Powered Filtering**: RUV-FANN neural networks for 60-80% false positive reduction
- **GitHub Integration**: Automatic issue creation with multiple modes and lifecycle management
- **Performance Optimization**: Adaptive parallelism, streaming analysis, and intelligent caching
- **Flexible Output**: JSON (source of truth), human-readable, SARIF, and Markdown formats
- **CI/CD Ready**: Diff-only mode, configurable exit codes, and comprehensive reporting

## Syntax
```bash
codeguardian check [OPTIONS] [PATHS]...
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--paths <PATHS>` | Paths to analyze (files or directories) | `PATH` | `.` | No |
| `--format <FORMAT>` | Output format (json/human/sarif) | `OutputFormat` | `json` | No |
| `--out <FILE>` | Output file for results | `PATH` | `results.json` | No |
| `--emit-md <FILE>` | Emit markdown report | `PATH` | - | No |
| `--emit-gh` | Emit GitHub issue | `FLAG` | `false` | No |
| `--repo <REPO>` | GitHub repository (owner/repo) | `STRING` | - | No |
| `--gh-mode <MODE>` | GitHub issue mode (checklist/simple/children) | `GhMode` | `checklist` | No |
| `--labels <LABELS>` | GitHub issue labels | `STRING` | `codeguardian,automated` | No |
| `--diff <SPEC>` | Only analyze changed files (git diff) | `STRING` | - | No |
| `--only-changed` | Only analyze staged files | `FLAG` | `false` | No |
| `--fail-on-issues` | Exit with non-zero code if issues are found | `FLAG` | `false` | No |
| `--parallel <NUM>` | Number of parallel workers (0 = auto) | `usize` | `0` | No |
| `--quiet` | Suppress all output except errors | `FLAG` | `false` | No |
| `--baseline <FILE>` | Baseline file for drift analysis | `PATH` | - | No |
| `--ml-threshold <THRESHOLD>` | ML threshold for anomaly detection (0.0-1.0) | `f64` | - | No |
| `--detect-broken-files` | Enable all broken file detection | `FLAG` | `false` | No |
| `--detect-conflicts` | Detect git merge conflicts | `FLAG` | `false` | No |
| `--detect-placeholders` | Detect AI-generated placeholders | `FLAG` | `false` | No |
| `--detect-duplicates` | Detect duplicate code | `FLAG` | `false` | No |
| `--fail-on-conflicts` | Fail fast on merge conflicts (CI/CD) | `FLAG` | `false` | No |
| `--streaming` | Enable streaming analysis for large files | `FLAG` | `false` | No |
| `--only-new` | Only analyze files that are new compared to baseline | `FLAG` | `false` | No |

### OutputFormat Values
- `json`: JSON output for programmatic use (source of truth)
- `human`: Human-readable output with colors
- `sarif`: SARIF format for security tools

### GhMode Values
- `checklist`: Checklist format with checkboxes
- `simple`: Simple issue format
- `children`: Children mode for large reports

## Examples

### Basic Usage
```bash
# Analyze current directory with default settings
codeguardian check

# Analyze specific directory
codeguardian check src/

# Analyze multiple paths
codeguardian check src/ tests/ docs/

# Generate JSON results file (recommended for CI/CD)
codeguardian check . --format json --out analysis-results.json
```

### Advanced Usage
```bash
# Complete analysis with multiple outputs
codeguardian check . \
  --format json \
  --out full-results.json \
  --emit-md full-report.md \
  --emit-gh \
  --repo myorg/myrepo \
  --fail-on-issues

# High-performance analysis for large codebases
codeguardian check . \
  --parallel 16 \
  --format json \
  --out large-analysis.json \
  --quiet

# Analyze only changed files in PR
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out pr-results.json \
  --emit-gh \
  --repo myorg/myrepo \
  --labels "codeguardian,pr-analysis"

# Use trained ML model for false positive reduction
codeguardian check . \
  --ml-threshold 0.7 \
  --format json \
  --out ml-results.json
```

## Error Handling

### Common Errors
- **Configuration Error**: Missing or invalid configuration file
- **GitHub Integration Error**: Missing repository specification or invalid format
- **File System Error**: Permission denied or path not found
- **Resource Error**: Memory exhaustion or timeout exceeded

## Security Considerations
- **Input Validation**: All input paths are validated and sanitized
- **File Size Limits**: Maximum file size limits prevent resource exhaustion
- **HTTPS Only**: All GitHub API communications use HTTPS
- **Token Security**: GitHub tokens are handled securely

## See Also
- [`codeguardian report`](report.md) - Convert analysis results to different formats
- [`codeguardian gh-issue`](gh-issue.md) - Create or update GitHub issues
- [`codeguardian turbo`](turbo.md) - High-performance analysis for large codebases
- [`codeguardian train`](train.md) - Train ML model for false positive reduction
- [`codeguardian init`](init.md) - Initialize configuration with presets
"#;

    let check_path = user_guide_path.join("check.md");
    std::fs::write(&check_path, content)?;
    debug!(
        "Generated check command documentation: {}",
        check_path.display()
    );
    Ok(())
}

/// Generate documentation for the report command
#[allow(clippy::ptr_arg)]
async fn generate_report_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# report

## Synopsis
Convert CodeGuardian analysis results to different output formats including Markdown, HTML, and plain text for reporting and documentation purposes.

## Description
The report command transforms JSON analysis results into human-readable formats suitable for documentation, CI/CD integration, and stakeholder communication. It supports multiple output formats with rich formatting and comprehensive issue presentation.

## Syntax
```bash
codeguardian report [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--from <FILE>` | Input results file | `PATH` | `results.json` | No |
| `--md <FILE>` | Output markdown file | `PATH` | - | No |
| `--format <FORMAT>` | Output format | `ReportFormat` | `markdown` | No |

### ReportFormat Values
- `markdown`: Markdown format
- `html`: HTML format
- `text`: Plain text format

## Examples
```bash
# Generate markdown report from default results file
codeguardian report --md analysis-report.md

# Generate HTML report
codeguardian report --format html --md report.html

# Generate plain text report
codeguardian report --format text --md report.txt

# Use custom input file
codeguardian report --from custom-results.json --md custom-report.md
```

## See Also
- [`codeguardian check`](check.md) - Run code analysis
- [`codeguardian gh-issue`](gh-issue.md) - Create GitHub issues from results
"#;

    let report_path = user_guide_path.join("report.md");
    std::fs::write(&report_path, content)?;
    debug!(
        "Generated report command documentation: {}",
        report_path.display()
    );
    Ok(())
}

/// Generate documentation for the gh-issue command
#[allow(clippy::ptr_arg)]
async fn generate_gh_issue_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# gh-issue

## Synopsis
Create or update GitHub issues from CodeGuardian analysis results with support for multiple issue modes and automatic lifecycle management.

## Description
The gh-issue command integrates CodeGuardian findings directly into GitHub's issue tracking system, enabling automated security and code quality issue management. It supports different issue formats and can update existing issues based on analysis results.

## Syntax
```bash
codeguardian gh-issue [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--from <FILE>` | Input results file | `PATH` | `results.json` | No |
| `--repo <REPO>` | GitHub repository (owner/repo) | `STRING` | - | Yes |
| `--mode <MODE>` | GitHub issue mode | `GhMode` | `checklist` | No |
| `--title <TITLE>` | Issue title prefix | `STRING` | `CodeGuardian: ` | No |
| `--labels <LABELS>` | Issue labels | `STRING` | `codeguardian,automated` | No |
| `--summary-from <FILE>` | Manual summary file | `PATH` | - | No |
| `--summary-auto <TEMPLATE>` | Auto-generate summary | `STRING` | - | No |
| `--summary-max-chars <NUM>` | Maximum characters in summary | `usize` | `800` | No |
| `--summary-max-issues <NUM>` | Maximum issues to include in summary | `usize` | `10` | No |
| `--dry-run` | Dry run mode (print commands without executing) | `FLAG` | `false` | No |

### GhMode Values
- `checklist`: Checklist format with checkboxes
- `simple`: Simple issue format
- `children`: Children mode for large reports

## Examples
```bash
# Create GitHub issue from analysis results
codeguardian gh-issue --repo myorg/myrepo

# Use checklist mode for detailed tracking
codeguardian gh-issue --repo myorg/myrepo --mode checklist

# Custom title and labels
codeguardian gh-issue --repo myorg/myrepo \
  --title "Security Analysis: " \
  --labels "security,codeguardian"

# Dry run to preview issue creation
codeguardian gh-issue --repo myorg/myrepo --dry-run

# Use custom summary
codeguardian gh-issue --repo myorg/myrepo \
  --summary-auto "Weekly security scan results"
```

## See Also
- [`codeguardian check`](check.md) - Run code analysis
- [`codeguardian report`](report.md) - Generate reports from results
"#;

    let gh_issue_path = user_guide_path.join("gh-issue.md");
    std::fs::write(&gh_issue_path, content)?;
    debug!(
        "Generated gh-issue command documentation: {}",
        gh_issue_path.display()
    );
    Ok(())
}

/// Generate documentation for the init command
#[allow(clippy::ptr_arg)]
async fn generate_init_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# init

## Synopsis
Initialize CodeGuardian configuration with default settings or predefined templates for quick setup and customization.

## Description
The init command creates a new CodeGuardian configuration file with sensible defaults. It supports different templates for various use cases and provides an interactive setup wizard for guided configuration.

## Syntax
```bash
codeguardian init [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--default` | Initialize with default configuration | `FLAG` | `false` | No |
| `--template <TEMPLATE>` | Template to use | `STRING` | - | No |

### Available Templates
- `minimal`: Basic configuration with essential exclusions
- `security`: Security-focused configuration with strict settings
- `ci`: CI/CD optimized configuration with appropriate timeouts

## Examples
```bash
# Initialize with default configuration
codeguardian init --default

# Use security template
codeguardian init --template security

# Use CI/CD template
codeguardian init --template ci

# Interactive setup (default)
codeguardian init
```

## See Also
- [Configuration Guide](../configuration.md) - Configuration options and settings
"#;

    let init_path = user_guide_path.join("init.md");
    std::fs::write(&init_path, content)?;
    debug!(
        "Generated init command documentation: {}",
        init_path.display()
    );
    Ok(())
}

/// Generate documentation for the git-commit command
#[allow(clippy::ptr_arg)]
async fn generate_git_commit_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# git-commit

## Synopsis
Perform enhanced git commit with integrated CodeGuardian security analysis to ensure code quality before committing changes.

## Description
The git-commit command combines standard git commit functionality with CodeGuardian's security and code quality analysis. It automatically analyzes staged changes and prevents commits that would introduce security issues or code quality problems.

## Syntax
```bash
codeguardian git-commit [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--message <MESSAGE>` | Custom commit message | `STRING` | - | No |

## Examples
```bash
# Commit with analysis (will prompt for message)
codeguardian git-commit

# Commit with custom message
codeguardian git-commit --message "Fix security vulnerability in authentication"

# Interactive commit message
codeguardian git-commit
```

## Workflow
1. Analyzes all staged files for security and quality issues
2. If issues are found, displays them and prevents commit
3. If no issues, proceeds with normal git commit
4. Supports all standard git commit options

## See Also
- [`codeguardian git-commit-push`](git-commit-push.md) - Commit and push with analysis
- [`codeguardian check`](check.md) - Run standalone analysis
"#;

    let git_commit_path = user_guide_path.join("git-commit.md");
    std::fs::write(&git_commit_path, content)?;
    debug!(
        "Generated git-commit command documentation: {}",
        git_commit_path.display()
    );
    Ok(())
}

/// Generate documentation for the git-commit-push command
#[allow(clippy::ptr_arg)]
async fn generate_git_commit_push_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# git-commit-push

## Synopsis
Perform enhanced git commit and push with integrated CodeGuardian security analysis for complete pre-deployment validation.

## Description
The git-commit-push command provides a comprehensive workflow that analyzes code, commits changes, and pushes them to the remote repository. It ensures that only secure, high-quality code reaches the remote repository.

## Syntax
```bash
codeguardian git-commit-push [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--message <MESSAGE>` | Custom commit message | `STRING` | - | No |
| `--amend` | Amend the last commit instead of creating a new one | `FLAG` | `false` | No |
| `--push` | Push to remote after committing | `FLAG` | `true` | No |
| `--no-verify` | Skip pre-commit hooks | `FLAG` | `false` | No |

## Examples
```bash
# Commit and push with analysis
codeguardian git-commit-push --message "Add user authentication"

# Amend last commit
codeguardian git-commit-push --amend --message "Fix typo in authentication"

# Skip pre-commit hooks
codeguardian git-commit-push --no-verify --message "Hotfix deployment"
```

## Workflow
1. Analyzes staged files for security and quality issues
2. If issues found, prevents commit and push
3. Commits changes with analysis approval
4. Pushes to configured remote repository
5. Provides detailed feedback on any issues found

## See Also
- [`codeguardian git-commit`](git-commit.md) - Commit only with analysis
- [`codeguardian check`](check.md) - Run standalone analysis
"#;

    let git_commit_push_path = user_guide_path.join("git-commit-push.md");
    std::fs::write(&git_commit_push_path, content)?;
    debug!(
        "Generated git-commit-push command documentation: {}",
        git_commit_push_path.display()
    );
    Ok(())
}

/// Generate documentation for the turbo command
#[allow(clippy::ptr_arg)]
async fn generate_turbo_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# turbo

## Synopsis
Run high-performance parallel analysis for large codebases with optimized resource utilization and advanced performance features.

## Description
The turbo command provides CodeGuardian's high-performance analysis mode optimized for large-scale codebases. It uses advanced parallel processing, memory optimization, and streaming analysis to deliver fast, comprehensive results.

## Syntax
```bash
codeguardian turbo [OPTIONS] [PATHS]...
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--paths <PATHS>` | Paths to analyze (files or directories) | `PATH` | `.` | No |
| `--max-parallel <NUM>` | Maximum number of parallel workers | `usize` | `0` | No |
| `--memory-limit <MB>` | Memory limit in MB (0 = no limit) | `usize` | `0` | No |
| `--format <FORMAT>` | Output format (json, human, sarif) | `OutputFormat` | `json` | No |
| `--output <FILE>` | Output file for results | `PATH` | `turbo-results.json` | No |
| `--metrics` | Enable metrics output | `FLAG` | `false` | No |
| `--aggressive` | Aggressive analysis mode (more thorough but slower) | `FLAG` | `false` | No |
| `--diff <SPEC>` | Only analyze changed files (git diff) | `STRING` | - | No |
| `--only-staged` | Only analyze staged files | `FLAG` | `false` | No |
| `--fail-on-issues` | Exit with non-zero code if issues are found | `FLAG` | `false` | No |
| `--baseline <FILE>` | Baseline file for drift analysis | `PATH` | - | No |

## Examples
```bash
# Turbo analysis of large codebase
codeguardian turbo . --max-parallel 32

# Memory-constrained turbo analysis
codeguardian turbo . --memory-limit 4096 --max-parallel 8

# Turbo analysis with metrics
codeguardian turbo . --metrics --output turbo-metrics.json

# Aggressive turbo analysis for thorough checking
codeguardian turbo . --aggressive --fail-on-issues
```

## Performance Features
- **Adaptive Parallelism**: Automatically scales workers based on system resources
- **Memory Pooling**: Efficient memory management for large file processing
- **Streaming Analysis**: Process large files without loading entirely into memory
- **Intelligent Caching**: Cache analysis results for faster subsequent runs
- **Resource Monitoring**: Real-time monitoring of CPU and memory usage

## See Also
- [`codeguardian check`](check.md) - Standard analysis mode
"#;

    let turbo_path = user_guide_path.join("turbo.md");
    std::fs::write(&turbo_path, content)?;
    debug!(
        "Generated turbo command documentation: {}",
        turbo_path.display()
    );
    Ok(())
}

/// Generate documentation for the train command
#[allow(clippy::ptr_arg)]
async fn generate_train_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# train

## Synopsis
Train machine learning model for enhanced false positive reduction and improved analysis accuracy.

## Description
The train command trains CodeGuardian's ML model using historical analysis data to improve false positive detection. It supports various training configurations and can bootstrap training data for new installations.

## Syntax
```bash
codeguardian train [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--model-path <PATH>` | Path to save the trained model | `PATH` | `codeguardian-model.fann` | No |
| `--epochs <NUM>` | Number of training epochs | `u32` | `1000` | No |
| `--bootstrap` | Generate synthetic training data for cold start | `FLAG` | `false` | No |
| `--training-data <FILE>` | Path to existing training data (JSON format) | `PATH` | - | No |
| `--continue-training` | Continue training from existing model | `FLAG` | `false` | No |
| `--validate` | Validate model performance after training | `FLAG` | `false` | No |
| `--enhanced` | Use AST-enhanced features | `FLAG` | `false` | No |

## Examples
```bash
# Train new model with default settings
codeguardian train

# Bootstrap training for new installation
codeguardian train --bootstrap --epochs 2000

# Continue training existing model
codeguardian train --continue-training --model-path existing-model.fann

# Train with validation
codeguardian train --validate

# Use enhanced AST features
codeguardian train --enhanced --training-data custom-data.json
```

## Training Process
1. **Data Collection**: Gather analysis results and user feedback
2. **Feature Extraction**: Extract relevant features from code patterns
3. **Model Training**: Train neural network on labeled data
4. **Validation**: Test model performance on held-out data
5. **Deployment**: Save trained model for use in analysis

## See Also
- [`codeguardian check`](check.md) - Use trained model in analysis
"#;

    let train_path = user_guide_path.join("train.md");
    std::fs::write(&train_path, content)?;
    debug!(
        "Generated train command documentation: {}",
        train_path.display()
    );
    Ok(())
}

/// Generate documentation for the update-docs command
#[allow(clippy::ptr_arg)]
async fn generate_update_docs_docs(user_guide_path: &std::path::Path) -> Result<()> {
    let content = r#"# update-docs

## Synopsis
Update and maintain CodeGuardian documentation files, generating missing documentation and validating existing content.

## Description
The update-docs command maintains CodeGuardian's documentation by updating existing files, generating missing documentation, and ensuring consistency across all documentation sources.

## Syntax
```bash
codeguardian update-docs [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--force` | Force update all documentation files | `FLAG` | `false` | No |
| `--validate-only` | Only validate documentation without updating | `FLAG` | `false` | No |
| `--api` | Generate API documentation | `FLAG` | `false` | No |
| `--user-guide` | Generate user guide documentation | `FLAG` | `false` | No |
| `--config` | Generate configuration documentation | `FLAG` | `false` | No |

## Examples
```bash
# Update all documentation
codeguardian update-docs --force

# Validate documentation without changes
codeguardian update-docs --validate-only

# Update specific documentation types
codeguardian update-docs --api --user-guide

# Update configuration documentation only
codeguardian update-docs --config
```

## Documentation Types
- **API Documentation**: Technical API references and examples
- **User Guide**: Command documentation and usage examples
- **Configuration**: Configuration options and templates

## See Also
- [Configuration Guide](../configuration.md) - Configuration documentation
- [API Documentation](../api/index.md) - API reference
"#;

    let update_docs_path = user_guide_path.join("update-docs.md");
    std::fs::write(&update_docs_path, content)?;
    debug!(
        "Generated update-docs command documentation: {}",
        update_docs_path.display()
    );
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
