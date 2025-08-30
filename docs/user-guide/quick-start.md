# 🚀 Quick Start Guide

Get CodeGuardian up and running in your project in just a few minutes. This guide covers the essential steps to start analyzing your codebase for security issues, performance bottlenecks, and code quality problems.

## Prerequisites

Before you begin, ensure you have:

- **Rust 1.70+** installed ([installation guide](https://rustup.rs/))
- **Git** for repository operations
- **GitHub token** (optional, for GitHub integration)

## Installation

### Option 1: Install from Crates.io (Recommended)

```bash
cargo install codeguardian
```

### Option 2: Install from Source

```bash
git clone https://github.com/d-oit/codeguardian
cd codeguardian
cargo build --release
cargo install --path .
```

### Option 3: Using Docker

```bash
docker pull codeguardian/codeguardian:latest
```

## Basic Usage

### 1. Initialize Configuration

Create a basic configuration file with security-focused defaults:

```bash
codeguardian init --template security
```

This creates a `codeguardian.toml` file with recommended settings for security analysis.

### 2. Run Your First Analysis

Analyze your current directory:

```bash
codeguardian check .
```

For more detailed output with JSON results:

```bash
codeguardian check . --format json --out results.json
```

### 3. Generate a Report

Create a human-readable Markdown report:

```bash
codeguardian report --from results.json --md security-report.md
```

### 4. View Results

Open `security-report.md` to see your analysis results, including:
- Security vulnerabilities found
- Performance issues detected
- Code quality improvements suggested
- Dependency analysis results

## Common Next Steps

### Enable ML-Powered Analysis

Train and use ML models for better accuracy:

```bash
# Train a model (one-time setup)
codeguardian train --model-path enhanced-model.fann --epochs 2000

# Use ML-enhanced analysis
codeguardian check . --ml-model enhanced-model.fann --ml-threshold 0.8
```

### Set Up GitHub Integration

Automatically create GitHub issues for findings:

```bash
codeguardian check . --emit-gh --repo your-org/your-repo
```

### Analyze Pull Requests

Focus analysis on changed files in a PR:

```bash
codeguardian check . --diff origin/main..HEAD --format json --out pr-results.json
```

## Understanding Results

### Severity Levels

- **Critical**: Security vulnerabilities requiring immediate attention
- **High**: Significant security or performance issues
- **Medium**: Code quality and maintainability concerns
- **Low**: Minor issues and best practice violations

### Finding Types

- **Security**: Hardcoded secrets, SQL injection, XSS vulnerabilities
- **Performance**: Memory leaks, inefficient algorithms, blocking I/O
- **Code Quality**: Naming violations, complexity issues, duplication
- **Dependencies**: Outdated packages, security vulnerabilities in dependencies

## Configuration Quick Reference

### Essential Settings

```toml
[general]
max_file_size = 10485760  # 10MB limit
parallel_workers = 4       # CPU cores
memory_limit_mb = 1024     # Memory limit

[analyzers.security]
enabled = true
check_secrets = true
check_vulnerabilities = true
entropy_threshold = 4.5
```

### Performance Optimization

```toml
[performance]
cache_enabled = true
parallel_processing = true
streaming_threshold_mb = 5
```

## Troubleshooting

### Common Issues

**"Command not found"**
- Ensure CodeGuardian is installed: `cargo install codeguardian`
- Check your PATH includes Cargo's bin directory

**"Permission denied"**
- Run with appropriate permissions or use Docker
- Check file permissions in your project directory

**"No findings reported"**
- Verify your files are supported (Rust, JavaScript, Python, etc.)
- Check configuration excludes important files
- Try running with `--verbose` for more information

### Getting Help

```bash
# View all available commands
codeguardian --help

# Get help for specific commands
codeguardian check --help
codeguardian init --help
```

## What's Next?

Now that you have CodeGuardian running, explore these advanced features:

- **[Advanced Features Guide](advanced-features.md)** - ML integration, turbo mode, custom rules
- **[CI/CD Setup Guide](ci-cd-setup.md)** - Comprehensive CI/CD workflows and automation
- **[Configuration Guide](configuration.md)** - Fine-tune analysis for your needs
- **[GitHub Integration](github-integration.md)** - Set up automated issue management

---

<div align="center">

**[⬅️ Back to User Guide](../README.md)** | **[📖 Installation Guide](installation.md)** | **[⚙️ Configuration Guide](configuration.md)**

</div>