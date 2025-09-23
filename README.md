# d.o. CodeGuardian 🛡️

[![CI Status](https://github.com/d-oit/do-codeguardian/workflows/CodeGuardian%20CI/badge.svg)](https://github.com/d-oit/do-codeguardian/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub stars](https://img.shields.io/github/stars/d-oit/do-codeguardian.svg)](https://github.com/d-oit/do-codeguardian/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/d-oit/do-codeguardian.svg)](https://github.com/d-oit/do-codeguardian/issues)

> **Secure Your Codebase with AI-Powered Analysis**

**Version: 0.2.1-alpha.2** | [📋 Changelog](CHANGELOG.md) | [📚 API Documentation](docs/api/index.md)

CodeGuardian is a cutting-edge, security-first code analysis tool that combines machine learning, advanced static analysis, and seamless GitHub integration to help developers identify vulnerabilities, optimize performance, and maintain high code quality standards. Built with Rust for speed and reliability, it empowers teams to ship secure software faster.

## ✨ Key Features

- 🔒 **Advanced Security Analysis**: Multi-layered vulnerability detection (SQL injection, XSS, command injection, secrets, hardcoded credentials)
- 🤖 **AI-Powered ML Classification**: RUV-FANN neural networks with optional AST-enhanced features for 60-80% false positive reduction
- 🚀 **Turbo Mode**: High-performance parallel analysis with adaptive resource management for large codebases
- 🔗 **Comprehensive GitHub Integration**: Automated issue creation with duplicate prevention, PR analysis, and multiple workflow modes
- 📊 **Intelligent Reporting**: JSON source-of-truth with human-readable, SARIF, and Markdown formats
- 📋 **Organized Validation Reports**: Version-specific validation and UAT reports with historical data access
- ⚡ **CI/CD Optimization**: Diff-only analysis, baseline comparisons, and seamless pipeline integration
- 🛠️ **Broken Files Detection**: Git merge conflicts, AI-generated placeholders, and duplicate code detection
- 🔄 **Duplicate Prevention System**: Multi-strategy duplicate detection with cross-workflow coordination
- 🏗️ **Extensible Architecture**: Plugin-based analyzer system with custom integrations (Azure DevOps, GitLab, Jira)
- 📈 **Performance Intelligence**: Memory pooling, streaming analysis, and intelligent caching
- 🗂️ **Retention Policy Engine**: Automatic cleanup with data integrity verification and repair mechanisms
- 🎛️ **Advanced Configuration**: Preset configurations with environment variable support and validation
- 🌐 **Web Dashboard**: Optional real-time monitoring and analysis visualization (feature-gated)

### 🆕 New in v0.2.1-alpha.2

- 🔧 **External System Integrations**: Native support for Jira, Confluence, Jenkins, GitLab, Bitbucket, and Azure DevOps
- 📱 **Interactive Web Dashboard**: Real-time monitoring with customizable views and stakeholder-specific dashboards
- 🔄 **Automated Remediation Workflows**: AI-powered remediation suggestions with approval workflows and automated execution
- 🕸️ **Advanced Relationship Management**: Graph-based artifact relationship tracking with impact analysis and visualization
- 📊 **Enhanced Metrics & Monitoring**: Comprehensive metrics collection with alerting and trend analysis
- 🔍 **Semantic Search & Discovery**: ML-enhanced duplicate detection across external systems
- 🎯 **Risk-Based Approval System**: Intelligent risk assessment for remediation actions with configurable approval thresholds
- 📈 **Enterprise-Grade Reporting**: Unified reporting across all integrated systems with compliance support

## 🏗️ Architecture & Components

CodeGuardian implements a modular, security-first architecture designed for high performance and extensibility:

### Core Analysis Engine
- **Security Analyzer**: Multi-layered security analysis with specialized detectors
- **Performance Analyzer**: Code optimization and efficiency analysis
- **Code Quality Analyzer**: Maintainability and best practices validation
- **Dependency Analyzer**: License compliance and vulnerability scanning
- **Integrity Analyzer**: File integrity and git history validation
- **Naming Analyzer**: Consistent naming convention enforcement

### Advanced Detection Systems
- **Broken Files Detection**: Git conflicts, AI placeholders, duplicate code analysis
- **ML Classification Engine**: RUV-FANN neural networks with optional AST enhancement
- **Duplicate Prevention System**: Multi-strategy duplicate detection and management
- **Streaming Analysis**: Memory-efficient processing of large files

### Integration Layer
- **GitHub API Client**: Rate-limited, retry-enabled GitHub integration
- **External Integrations**: Azure DevOps, GitLab, Jira, Confluence, Jenkins, Bitbucket
- **CI/CD Pipeline Support**: Native integration with major CI/CD platforms
- **Webhook System**: Real-time notifications and external system integration
- **Integration Manager**: Unified interface for managing multiple external systems
- **Bulk Operations**: Efficient batch processing for large-scale operations

### Performance & Optimization
- **Parallel Processing**: Adaptive parallelism with resource management
- **Intelligent Caching**: Multi-level caching with performance monitoring
- **Memory Pooling**: Efficient memory management for large-scale analysis
- **Turbo Mode**: High-performance analysis for enterprise codebases

### Configuration & Management
- **Preset Configurations**: Optimized configurations for different use cases
- **Environment Integration**: Comprehensive environment variable support
- **Configuration Validation**: Real-time validation and best practice enforcement
- **Web Dashboard**: Optional real-time monitoring and visualization
- **Integration Configuration**: Centralized management of external system connections
- **Remediation Policies**: Configurable approval workflows and risk thresholds
- **Relationship Discovery**: Automated relationship discovery and management settings

## 🚀 Quick Start

### Prerequisites
- Rust 1.70 or later
- Git

### Installation

#### From Source (Recommended)
```bash
git clone https://github.com/d-oit/do-codeguardian.git
cd do-codeguardian
cargo build --release
```

#### Using Cargo
```bash
cargo install do-codeguardian
```

#### Docker
```bash
docker run -v $(pwd):/workspace d-oit/codeguardian:latest analyze .
```

## 📖 Usage Examples

### Comprehensive Security Analysis
```bash
# Full security analysis with ML-powered false positive reduction
codeguardian check . --format json --out security-audit.json --ml-model enhanced-model.fann

# Analyze with all detection systems enabled (results auto-placed in build/analysis-results/check/{date}/comprehensive-analysis.json)
codeguardian check . \
  --detect-broken-files \
  --detect-conflicts \
  --detect-placeholders \
  --detect-duplicates \
  --format json \
  --out comprehensive-analysis.json

# High-performance analysis for large codebases (results auto-placed in build/analysis-results/turbo/{date}/large-scale-results.json)
codeguardian turbo . \
  --max-parallel 16 \
  --memory-limit 4096 \
  --output large-scale-results.json \
  --metrics
```

### Retention Policy Management
```bash
# Check data integrity and generate report
codeguardian retention --check-integrity --report-integrity

# Run automatic cleanup based on configured policies
codeguardian retention --cleanup

# Show retention status and statistics
codeguardian retention

# Dry run cleanup to see what would be cleaned
codeguardian retention --cleanup --dry-run
```

### Advanced GitHub Integration
```bash
# Analyze pull request with duplicate prevention
codeguardian check . \
  --diff origin/main..HEAD \
  --emit-gh \
  --repo myorg/myrepo \
  --gh-mode checklist \
  --labels "security,automated,pr-analysis"

# Create GitHub issues with comprehensive reporting
codeguardian gh-issue \
  --from results.json \
  --repo myorg/myrepo \
  --mode children \
  --title "Security Analysis Results" \
  --summary-auto "Weekly security scan results"

# Update existing issues instead of creating duplicates
codeguardian gh-issue \
  --from results.json \
  --repo myorg/myrepo \
  --mode checklist \
  --update-existing
```

### ML Model Training & Optimization
```bash
# Train ML model for false positive reduction
codeguardian train \
  --model-path enhanced-model.fann \
  --epochs 2000 \
  --bootstrap \
  --enhanced

# Analyze with trained model and feature importance analysis (results auto-placed in build/analysis-results/check/{date}/ml-optimized-results.json)
codeguardian check . \
  --ml-model enhanced-model.fann \
  --ml-threshold 0.8 \
  --format json \
  --out ml-optimized-results.json

# View ML model performance metrics
codeguardian metrics --model enhanced-model.fann --show
```

### Performance & Large-Scale Analysis
```bash
# Memory-efficient streaming analysis for large files
codeguardian check large-file.bin \
  --streaming \
  --parallel 8 \
  --memory-limit 2048

# Baseline comparison for drift detection (results auto-placed in build/analysis-results/check/{date}/drift-analysis.json)
codeguardian check . \
  --baseline baseline.json \
  --only-new \
  --format json \
  --out drift-analysis.json

# Performance benchmarking and optimization (results auto-placed in build/analysis-results/turbo/{date}/performance-report.json)
codeguardian turbo . \
  --aggressive \
  --metrics \
  --output performance-report.json
```

### Configuration & Preset Usage
```bash
# Use enterprise-grade security preset
codeguardian init --template enterprise

# Custom configuration with environment variables
export CODEGUARDIAN_GITHUB_TOKEN="your_token_here"
export CODEGUARDIAN_MEMORY_LIMIT="4096"
codeguardian check . --config enterprise-security.toml

# Validate configuration before use
codeguardian validate --config codeguardian.toml
```

## 📚 Documentation

Explore CodeGuardian's comprehensive documentation:

### 🚀 Getting Started
- **[Quick Start Guide](docs/user-guide/quick-start.md)** - Get up and running in minutes
- **[Installation Guide](docs/user-guide/installation.md)** - Platform-specific installation instructions
- **[Basic Usage](docs/user-guide/check.md)** - Essential commands and workflows

### 🔧 Core Features
- **[Security Analysis](docs/user-guide/security-analysis.md)** - Advanced vulnerability detection
- **[ML Integration](docs/user-guide/ml-integration.md)** - AI-powered false positive reduction
- **[GitHub Integration](docs/user-guide/github-integration.md)** - Automated issue management
- **[Duplicate Prevention](docs/duplicate-prevention.md)** - Intelligent duplicate detection
- **[Turbo Mode](docs/user-guide/turbo-mode.md)** - High-performance analysis
- **[Broken Files Detection](docs/user-guide/broken-files-detection.md)** - Conflict and placeholder detection

### 🏗️ Technical Documentation
- **[Architecture Overview](docs/architecture/overview.md)** - System design and components
- **[API Reference](docs/api/index.md)** - Programmatic integration and customization
- **[Configuration Guide](docs/configuration.md)** - Advanced setup and presets
- **[Performance Tuning](docs/architecture/performance.md)** - Optimization and benchmarking
- **[Validation Reports](docs/validation-reports/index.md)** - Version-specific testing and validation data

### 🚀 Advanced Features
- **[CI/CD Integration](docs/user-guide/ci-cd-integration.md)** - Pipeline integration guides
- **[External Integrations](docs/integrations/index.md)** - Azure DevOps, GitLab, Jira integration
- **[Dashboard Setup](docs/dashboard/setup.md)** - Web dashboard configuration
- **[Remediation Workflows](docs/remediation/index.md)** - Automated remediation
- **[Relationship Management](docs/relationships/index.md)** - Advanced artifact relationships

### 📚 API Documentation (v0.2.1-alpha.2)
- **[Core API](docs/api/core.md)** - Core functionality and library interfaces
- **[CLI API](docs/api/cli.md)** - Command-line interface documentation
- **[Configuration API](docs/api/config.md)** - Configuration management
- **[Analysis API](docs/api/analysis.md)** - Security analysis interfaces
- **[Integrations API](docs/api/integrations.md)** - External system integrations
- **[Dashboard API](docs/api/dashboard.md)** - Web dashboard interfaces
- **[Remediation API](docs/api/remediation.md)** - Automated remediation workflows
- **[Relationships API](docs/api/relationships.md)** - Relationship management

### 🔍 Troubleshooting & Support
- **[Troubleshooting Guide](docs/troubleshooting/index.md)** - Common issues and solutions
- **[FAQ](docs/troubleshooting/faq.md)** - Frequently asked questions
- **[Performance Issues](docs/troubleshooting/performance.md)** - Optimization tips

## 🤝 Contributing

We welcome contributions from the community! Here's how you can get involved:

### Development Setup
```bash
git clone https://github.com/d-oit/do-codeguardian.git
cd do-codeguardian
cargo build
cargo test
```

### Ways to Contribute
- 🐛 **Bug Reports**: [Open an issue](https://github.com/d-oit/do-codeguardian/issues/new?labels=bug)
- 💡 **Feature Requests**: [Start a discussion](https://github.com/d-oit/do-codeguardian/discussions/categories/ideas)
- 📝 **Documentation**: Improve docs in the `docs/` directory
- 🔧 **Code**: Submit pull requests with fixes or enhancements
- 🧪 **Testing**: Add tests or improve test coverage

See our **[Contributing Guide](CONTRIBUTING.md)** for detailed guidelines.

## 🌟 Why CodeGuardian?

### 🔐 Security-First Architecture
- **Multi-Layered Analysis**: Specialized analyzers for different vulnerability types
- **ML-Powered Accuracy**: 60-80% false positive reduction with neural networks
- **Deterministic Results**: Stable finding IDs and reproducible analysis
- **Secure by Default**: Conservative configuration with explicit opt-in for advanced features

### ⚡ Enterprise-Grade Performance
- **Adaptive Parallelism**: Automatic CPU core detection and resource management
- **Streaming Analysis**: Memory-efficient processing of large files and codebases
- **Intelligent Caching**: Multi-level caching with performance monitoring
- **Turbo Mode**: High-performance analysis for million-line codebases

### 🤖 AI & Machine Learning
- **RUV-FANN Integration**: Rust-optimized neural networks for pattern recognition
- **AST-Enhanced Features**: Optional abstract syntax tree analysis for deeper insights
- **Online Learning**: Continuous model improvement with user feedback
- **Feature Importance Analysis**: Explainable AI for better understanding

### 🔄 DevSecOps Integration
- **GitHub Native**: Seamless integration with GitHub Issues, PRs, and Actions
- **CI/CD Optimized**: Diff-only analysis, baseline comparisons, and fast feedback
- **Duplicate Prevention**: Intelligent duplicate detection across workflows
- **Webhook Support**: Real-time notifications and external system integration

### 📊 Comprehensive Intelligence
- **Multiple Output Formats**: JSON (source of truth), human-readable, SARIF, Markdown
- **Advanced Reporting**: Executive summaries, trend analysis, and actionable insights
- **Performance Metrics**: Built-in benchmarking and optimization recommendations
- **Broken Files Detection**: Git conflicts, AI placeholders, and duplicate code analysis

### 🌍 Enterprise Ready
- **Extensible Architecture**: Plugin-based system for custom analyzers and integrations
- **Multi-Platform Support**: Native binaries for Linux, macOS, Windows
- **Configuration Management**: Environment-based configuration with validation
- **Commercial Support**: Enterprise support and custom development options

## 🏢 Use Cases & Applications

### 🔒 Enterprise Security Teams
- **Comprehensive Vulnerability Assessment**: Multi-layered security analysis with ML-powered accuracy
- **Compliance Automation**: Automated compliance checking and reporting for standards (SOC2, ISO27001)
- **Risk Management**: Continuous security monitoring with trend analysis and risk scoring
- **Incident Response**: Rapid vulnerability detection and prioritization
- **Unified Security Operations**: Cross-system vulnerability correlation and remediation
- **Automated Remediation**: AI-powered remediation workflows with approval management

### 🚀 DevSecOps Teams
- **CI/CD Pipeline Integration**: Seamless integration with GitHub Actions, Jenkins, GitLab CI
- **Shift-Left Security**: Early vulnerability detection in development workflows
- **Automated Remediation**: Intelligent remediation suggestions and workflow automation
- **Security Gates**: Configurable quality gates for deployment pipelines
- **Multi-System Integration**: Unified workflows across GitHub, Jira, Jenkins, and other tools
- **Real-time Monitoring**: Web dashboard for continuous security monitoring and alerting

### 👥 Development Teams
- **Code Review Enhancement**: Automated analysis for pull requests with duplicate prevention
- **Quality Standards**: Consistent code quality enforcement across large teams
- **Performance Optimization**: Automated detection of performance bottlenecks and anti-patterns
- **Technical Debt Management**: Continuous monitoring and reporting of code quality metrics

### 🏗️ Platform Teams
- **Infrastructure Security**: Container, Kubernetes, and cloud infrastructure analysis
- **Dependency Management**: Automated license compliance and vulnerability scanning
- **Large-Scale Analysis**: High-performance analysis for million-line codebases
- **Custom Integration**: Extensible architecture for organization-specific requirements

### 📊 Security Operations Centers (SOC)
- **Continuous Monitoring**: Real-time security analysis with webhook notifications
- **Threat Intelligence**: Pattern recognition and anomaly detection with ML
- **Incident Correlation**: Cross-system vulnerability correlation and prioritization
- **Reporting & Compliance**: Automated report generation for regulatory requirements

## 📞 Community & Support

- **🐛 Issues**: [Report bugs](https://github.com/d-oit/do-codeguardian/issues)
- **💬 Discussions**: [Join the conversation](https://github.com/d-oit/do-codeguardian/discussions)
- **📧 Security**: [Report vulnerabilities](SECURITY.md)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Powered by advanced machine learning algorithms
- Inspired by the need for better security in software development

---

<div align="center">

**Made with ❤️ by the CodeGuardian Community**

[⭐ Star us on GitHub](https://github.com/d-oit/do-codeguardian) | [🐛 Report Issues](https://github.com/d-oit/do-codeguardian/issues) | [💬 Join Discussions](https://github.com/d-oit/do-codeguardian/discussions)

</div># Test comment for workflow verification
# Test change to trigger license workflow
# CI/CD Pipeline Optimization - Tue Sep 23 05:53:06 PM UTC 2025
