# 🚀 CodeGuardian

> **Security-first code analysis CLI with GitHub integration, built with best-practice defaults for CI/CD workflows.**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI Status](https://github.com/d-oit/do-codeguardian/workflows/CodeGuardian%20CI/badge.svg)](https://github.com/d-oit/do-codeguardian/actions)
[![codecov](https://codecov.io/gh/d-oit/do-codeguardian/branch/main/graph/badge.svg)](https://codecov.io/gh/d-oit/do-codeguardian)
[![Downloads](https://img.shields.io/github/downloads/d-oit/do-codeguardian/total.svg)](https://github.com/d-oit/do-codeguardian/releases)
[![Contributors](https://img.shields.io/github/contributors/d-oit/do-codeguardian.svg)](https://github.com/d-oit/do-codeguardian/graphs/contributors)
[![Last Commit](https://img.shields.io/github/last-commit/d-oit/do-codeguardian.svg)](https://github.com/d-oit/do-codeguardian/commits/main)

**CodeGuardian** is a comprehensive security and code quality analysis tool designed specifically for modern development workflows. It combines deterministic analysis, advanced ML-powered false positive reduction with RUV-FANN neural networks, turbo-mode performance optimization, and seamless CI/CD integration to help teams maintain high code quality and security standards.

Built with Rust for memory safety and performance, CodeGuardian provides code analysis with advanced features like streaming analysis for large files, intelligent caching, comprehensive security checks, ML model training, and detailed performance metrics.

## ✨ Key Features

- 🔒 **Security-by-Default**: Deterministic findings with stable IDs, automatic secret redaction, sandboxed execution
- 🚀 **CI-First UX**: JSON-first output, diff-only mode for PRs, TTY-aware progress bars
- 📊 **Comprehensive Analysis**: Security, performance, code quality, dependency, integrity, and naming analysis
- 🧠 **Advanced ML Capabilities**: RUV-FANN neural networks for 60-80% false positive reduction, custom model training, and detailed metrics
- ⚡ **Turbo Mode**: High-performance analysis for large codebases with 18x speedup, streaming support, and adaptive parallelism
- 🔗 **GitHub Integration**: Idempotent issue creation with automatic updates, multiple issue modes, and comprehensive CI/CD workflows
- 📈 **Performance Monitoring**: Real-time metrics, optimization presets, and resource usage tracking
- 🔧 **Extensible Architecture**: Pluggable analyzer system with custom security checks and patterns

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** (for building from source)
- **Git** for repository operations
- **GitHub token** (optional, for GitHub integration)
- **Docker** (optional, for containerized usage)
- **Python 3.7+** (optional, for advanced scripting)

### System Requirements

- **Memory**: Minimum 512MB, Recommended 2GB+ for large codebases
- **CPU**: Multi-core recommended for parallel analysis
- **Storage**: 100MB+ for cache and temporary files
- **Network**: Required for GitHub integration and dependency analysis

### Basic Usage

```bash
# 1. Install CodeGuardian
cargo install codeguardian

# 2. Initialize configuration with security template
codeguardian init --template security

# 3. Run analysis with ML filtering
codeguardian check . --format json --out results.json --ml-model enhanced-model.fann

# 4. Generate comprehensive report
codeguardian report --from results.json --md report.md --html report.html

# 5. Create GitHub issues with checklist format
codeguardian gh-issue --from results.json --repo owner/repo --mode checklist
```

### One-Liner Analysis

```bash
# Quick security scan with GitHub integration
codeguardian check . --emit-gh --repo owner/repo

# Fast PR analysis with ML filtering
codeguardian check . --diff origin/main..HEAD --ml-model enhanced-model.fann --emit-gh --repo owner/repo

# High-performance analysis
codeguardian turbo . --max-parallel 16 --metrics --format json --output results.json

# Train ML model for false positive reduction
codeguardian train --data training-data.json --model custom-model.fann --epochs 1000

# Show detailed ML model metrics and performance
codeguardian metrics --model enhanced-model.fann --format json

# Security audit with comprehensive reporting
codeguardian check . --config security-config.toml --format json --out audit.json --emit-md audit-report.md --emit-gh --repo owner/repo --fail-on-issues
```

## 📚 Documentation

- 📦 **[Installation Guide](docs/installation.md)** - Installation instructions and system requirements
- 🚀 **[Quick Start Guide](docs/user-guide/quick-start.md)** - Get up and running quickly
- 🔧 **[Usage Guide](docs/usage.md)** - Commands, patterns, and examples
- ⚙️ **[Configuration Guide](docs/configuration.md)** - Configuration options and presets
- 🔄 **[CI/CD Setup Guide](docs/user-guide/ci-cd-setup.md)** - Comprehensive CI/CD workflows and automation
- 📊 **[API Reference](docs/api.md)** - Output formats, programmatic integration, and webhooks
- 🤝 **[Contributing Guide](docs/contributing.md)** - Development setup and contribution process
- 📄 **[License Information](docs/license.md)** - License details and third-party dependencies

## 🔗 Integrations

### GitHub Actions

CodeGuardian includes comprehensive GitHub Actions workflows for automated CI/CD integration. All workflows build CodeGuardian from source and run analysis directly, ensuring you always use the latest version.

#### Available Workflows

- **CodeGuardian CI** (`.github/workflows/codeguardian-ci.yml`): Standard CI pipeline with diff-only PR analysis and full scans on main branch
- **Turbo Security Analysis** (`.github/workflows/turbo-security-analysis.yml`): High-performance security analysis with multiple modes and performance metrics
- **Turbo PR Analysis** (`.github/workflows/turbo-pr-analysis.yml`): Fast PR-focused analysis that only scans changed files
- **Turbo Performance Monitor** (`.github/workflows/turbo-performance-monitor.yml`): Continuous performance tracking and benchmarking
- **Turbo Nightly** (`.github/workflows/turbo-nightly.yml`): Automated nightly builds and comprehensive analysis
- **Turbo Release** (`.github/workflows/turbo-release.yml`): Pre-release validation and quality gates

For detailed workflow documentation, configuration options, and best practices, see the **[CI/CD Setup Guide](docs/user-guide/ci-cd-setup.md)**.

#### Quick Setup

```yaml
name: 🔍 CodeGuardian Analysis

on:
  pull_request:
    branches: [main, develop]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0

    - name: Setup Rust
      uses: actions-rust-lang/setup-rust-toolchain@v1
      with:
        toolchain: stable
        cache: true

    - name: Build CodeGuardian
      run: cargo build --release

    - name: Run Analysis
      run: |
        ./target/release/codeguardian check . \
          --diff origin/main..HEAD \
          --format json \
          --out results.json \
          --emit-gh \
          --repo ${{ github.repository }}
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Other CI/CD Platforms

CodeGuardian works with all major CI/CD platforms. For detailed integration guides, see the **[CI/CD Setup Guide](docs/user-guide/ci-cd-setup.md)**.

#### Build Instructions

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Build in release mode for optimal performance
cargo build --release

# Run analysis
./target/release/codeguardian check . --format json --out results.json
```

## 📈 Performance Benchmarks

CodeGuardian's turbo mode delivers significant performance improvements for large codebases:

- **18x Speedup**: Turbo mode achieves up to 18x faster analysis compared to standard mode
- **Adaptive Parallelism**: Automatically scales parallelism based on available CPU cores and memory
- **Streaming Analysis**: Processes large files without loading entire content into memory
- **Intelligent Caching**: Reduces redundant analysis through smart caching mechanisms

### Benchmark Results

```bash
# Run performance benchmarks
./performance_benchmark.sh

# Compare standard vs turbo mode
codeguardian check large-codebase/ --benchmark
codeguardian turbo large-codebase/ --benchmark --metrics
```

For detailed performance comparisons and optimization tips, see [Performance Benchmarks](examples/performance-comparison.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/contributing/) for details.

### Development Setup

```bash
git clone https://github.com/d-oit/do-codeguardian
cd do-codeguardian
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Areas for Contribution

- 🔍 **New Analyzers**: Security, performance, code quality, or dependency checks
- 🌐 **Platform Support**: Additional CI/CD platform integrations and cloud providers
- 📊 **Output Formats**: New report formats, integrations, and visualization
- 🧠 **ML Improvements**: Enhanced false positive detection and model training
- 📚 **Documentation**: Improve guides, examples, and API documentation
- 🐛 **Bug Fixes**: Help improve stability, performance, and reliability
- 🛡️ **Security**: Vulnerability research and security enhancement
- ⚡ **Performance**: Optimization of analysis speed and memory usage
- 🔧 **DevOps**: CI/CD improvements and deployment automation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

### Getting Help

- 📖 **[Documentation](docs/)** - Comprehensive user guides and API reference
- ❓ **[FAQ](docs/troubleshooting/faq.md)** - Frequently asked questions
- 🐛 **[Issue Tracker](https://github.com/d-oit/do-codeguardian/issues)** - Bug reports and feature requests
- 💬 **[Discussions](https://github.com/d-oit/do-codeguardian/discussions)** - Community discussions and support


### Community Resources

- 🌟 **GitHub Stars** - Show your support by starring the repository
- 🔄 **Contributing** - Help improve CodeGuardian by contributing code or documentation

---

<div align="center">

**[⭐ Star us on GitHub](https://github.com/d-oit/do-codeguardian)** | **[📖 Documentation](docs/)** | **[🐛 Report Issues](https://github.com/d-oit/do-codeguardian/issues)** | **[🚀 Releases](https://github.com/d-oit/do-codeguardian/releases)** | **[💬 Discussions](https://github.com/d-oit/do-codeguardian/discussions)**

Made with ❤️ by the CodeGuardian team

**CodeGuardian is a security-first code analysis tool that helps teams maintain high code quality and security standards through comprehensive analysis, ML-powered false positive reduction, and seamless CI/CD integration.**

</div>
