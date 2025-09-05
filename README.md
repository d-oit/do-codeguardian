# d.o. CodeGuardian 🛡️

[![CI Status](https://github.com/d-oit/do-codeguardian/workflows/CodeGuardian%20CI/badge.svg)](https://github.com/d-oit/do-codeguardian/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub stars](https://img.shields.io/github/stars/d-oit/do-codeguardian.svg)](https://github.com/d-oit/do-codeguardian/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/d-oit/do-codeguardian.svg)](https://github.com/d-oit/do-codeguardian/issues)

> **Secure Your Codebase with AI-Powered Analysis**

CodeGuardian is a cutting-edge, security-first code analysis tool that combines machine learning, advanced static analysis, and seamless GitHub integration to help developers identify vulnerabilities, optimize performance, and maintain high code quality standards. Built with Rust for speed and reliability, it empowers teams to ship secure software faster.

## ✨ Key Features

- 🔒 **Security-First Analysis**: Advanced vulnerability detection with ML-powered insights
- 🤖 **Machine Learning Integration**: Intelligent code quality assessment and pattern recognition
- 🚀 **Turbo Mode**: High-performance scanning for large codebases
- 🔗 **GitHub Integration**: Automated issue creation, PR analysis, and workflow optimization
- 📊 **Comprehensive Reporting**: Detailed analysis reports with actionable recommendations
- ⚡ **CI/CD Ready**: Seamless integration with popular CI/CD pipelines
- 📋 **License Compliance**: Automated dependency license checking and reporting
- 🐳 **Container Support**: Docker and cloud-native deployment options
- 📈 **Performance Monitoring**: Built-in benchmarking and optimization tools

## 🤖 Agent System

CodeGuardian features a sophisticated agent system for specialized development and management tasks. The system includes consolidated and enhanced agents for optimal performance:

### Recent Consolidations
- **Enhanced GitHub Workflow Manager**: Combines workflow management and optimization
- **Enhanced Documentation Specialist**: Unified documentation creation and maintenance
- **Enhanced Orchestrator**: Integrated task coordination and swarm management

### New Specialized Agents
- **Analyzer Orchestrator**: Coordinates comprehensive code analysis modules
- **ML Pipeline Manager**: Manages machine learning workflows and model optimization
- **Cache Intelligence Agent**: Optimizes caching performance and intelligence
- **Streaming Processor**: Enhances streaming analysis performance
- **Configuration Validator**: Ensures configuration integrity and best practices

For detailed agent information, see [AGENTS.md](AGENTS.md) and the `.opencode/agent/` directory.

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

### Basic Code Analysis
```bash
# Analyze current directory
codeguardian check

# Analyze specific files
codeguardian check src/main.rs src/lib.rs

# Generate detailed report
codeguardian report --output analysis.json
```

### Security Scanning
```bash
# Run security audit
codeguardian security --deep-scan

# Check for vulnerabilities
codeguardian vuln --severity high
```

### GitHub Integration
```bash
# Initialize GitHub integration
codeguardian init --github

# Analyze pull request
codeguardian pr analyze 123

# Create issues for findings
codeguardian issue create --auto
```

### Performance Analysis
```bash
# Run performance benchmarks
codeguardian bench

# Optimize code patterns
codeguardian optimize --target performance
```

## 📚 Documentation

Dive deeper into CodeGuardian's capabilities:

- **[📖 User Guide](docs/user-guide/)** - Complete usage tutorials and workflows
- **[🔧 API Reference](docs/api/)** - Programmatic integration and customization
- **[🏗️ Architecture](docs/architecture/)** - Technical design and components
- **[⚙️ Configuration](docs/configuration.md)** - Advanced setup and customization
- **[🚀 CI/CD Integration](docs/user-guide/ci-cd-setup.md)** - Pipeline integration guides
- **[🔍 Troubleshooting](docs/troubleshooting/)** - Common issues and solutions

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

- **🔐 Security by Design**: Built with security best practices from the ground up
- **⚡ Performance Optimized**: Fast analysis even for large, complex codebases
- **🤖 AI-Powered**: Machine learning enhances detection accuracy over time
- **🔄 Continuous Integration**: Seamless integration with your development workflow
- **📊 Data-Driven**: Comprehensive metrics and reporting for informed decisions
- **🌍 Open Source**: Transparent, community-driven development

## 🏢 Use Cases

- **Enterprise Security**: Large-scale vulnerability assessment and compliance
- **DevSecOps**: Integrate security scanning into CI/CD pipelines
- **Code Review Automation**: Automated analysis for pull requests and commits
- **Performance Optimization**: Identify and fix performance bottlenecks
- **Quality Assurance**: Maintain consistent code standards across teams

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
