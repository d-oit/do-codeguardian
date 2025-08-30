# Installation Guide

## System Requirements

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

## Installation Methods

### From Crates.io (Recommended)

```bash
cargo install codeguardian
```

### From Source

```bash
git clone https://github.com/d-oit/do-codeguardian.git
cd codeguardian
cargo build --release
cargo install --path .
```

### Using Docker

```bash
# Pull official image
docker pull codeguardian/codeguardian:latest

# Run analysis
docker run --rm -v $(pwd):/workspace codeguardian/codeguardian check /workspace

# With custom configuration
docker run --rm -v $(pwd):/workspace -v $(pwd)/codeguardian.toml:/etc/codeguardian.toml codeguardian/codeguardian check /workspace
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/d-oit/do-codeguardian/releases) for:
- Linux (x64, ARM64)
- macOS (x64, ARM64/M1)
- Windows (x64)

### Package Managers

#### Homebrew (macOS/Linux)
```bash
brew install codeguardian
```

#### APT (Ubuntu/Debian)
```bash
curl -fsSL https://apt.codeguardian.dev/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/codeguardian-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/codeguardian-archive-keyring.gpg] https://apt.codeguardian.dev stable main" | sudo tee /etc/apt/sources.list.d/codeguardian.list
sudo apt update && sudo apt install codeguardian
```

#### Chocolatey (Windows)
```bash
choco install codeguardian
```

## CI/CD Integration

CodeGuardian includes several GitHub Actions workflows for automated CI/CD processes:

- **codeguardian-ci.yml**: Main CI pipeline for building, testing, and linting the codebase
- **turbo-nightly.yml**: Nightly builds and performance monitoring
- **turbo-performance-monitor.yml**: Continuous performance benchmarking
- **turbo-pr-analysis.yml**: Automated analysis of pull requests
- **turbo-release.yml**: Automated release process and publishing
- **turbo-security-analysis.yml**: Security scanning and vulnerability assessment
- **issue-triage.yml**: Automated issue triage and labeling

These workflows are located in `.github/workflows/` and can be customized for your specific needs. For more details, see the [CI/CD Guide](TURBO_CI_GUIDE.md).

## Quick Start

After installation, initialize with a security template:

```bash
# Initialize configuration with security template
codeguardian init --template security

# Run analysis with ML filtering
codeguardian check . --format json --out results.json --ml-model enhanced-model.fann

# Generate comprehensive report
codeguardian report --from results.json --md report.md --html report.html

# Create GitHub issues with checklist format
codeguardian gh-issue --from results.json --repo owner/repo --mode checklist
```

## Docker Usage

```bash
# Pull and run with default settings
docker run --rm -v $(pwd):/workspace codeguardian/codeguardian check /workspace

# Run with custom configuration
docker run --rm -v $(pwd):/workspace -v $(pwd)/codeguardian.toml:/etc/codeguardian.toml codeguardian/codeguardian check /workspace

# Run high-performance analysis
docker run --rm -v $(pwd):/workspace codeguardian/codeguardian turbo /workspace --max-parallel 8 --memory-limit 2048
```

## Verification

After installation, verify everything works:

```bash
# Check version
codeguardian --version

# Initialize configuration
codeguardian init --template security

# Run a quick test
codeguardian check . --format human
```