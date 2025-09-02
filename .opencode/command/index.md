---
title: "Command Reference"
description: "Complete reference for all CodeGuardian CLI commands"
category: "Documentation"
tags: ["commands", "reference", "cli", "documentation"]
---

# CodeGuardian Command Reference

This comprehensive command reference provides detailed documentation for all CodeGuardian CLI commands, organized by category for easy navigation and discovery.

## Table of Contents

- [Core Commands](#core-commands)
- [Git Integration Commands](#git-integration-commands)
- [Docker/Container Commands](#dockercontainer-commands)
- [CI/CD Platform Commands](#cicd-platform-commands)
- [Configuration Management Commands](#configuration-management-commands)
- [Troubleshooting and Diagnostics Commands](#troubleshooting-and-diagnostics-commands)
- [Performance and Monitoring Commands](#performance-and-monitoring-commands)
- [Security and Compliance Commands](#security-and-compliance-commands)

## Core Commands

### Primary Analysis Commands

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian check` | Run comprehensive code analysis (primary command) | [check.md](check.md) |
| `codeguardian report` | Convert analysis results to different formats | [report.md](report.md) |
| `codeguardian gh-issue` | Create or update GitHub issues from results | [gh-issue.md](gh-issue.md) |
| `codeguardian init` | Initialize CodeGuardian configuration | [init.md](init.md) |
| `codeguardian turbo` | High-performance analysis for large codebases | [turbo.md](turbo.md) |

### Machine Learning Commands

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian train` | Train ML models for false positive reduction | [train.md](train.md) |
| `codeguardian metrics` | Show detailed ML model metrics and reports | [metrics.md](metrics.md) |

## Git Integration Commands

### Git Hooks and Automation

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian git-hooks-setup` | Set up Git hooks for automatic analysis | [git/git-hooks-setup.md](git/git-hooks-setup.md) |
| `codeguardian git-workflow` | Analyze and optimize Git workflows | [git/git-workflow.md](git/git-workflow.md) |
| `codeguardian git-branch-protection` | Set up branch protection with analysis requirements | [git/git-branch-protection.md](git/git-branch-protection.md) |
| `codeguardian git-merge-validation` | Validate merges against security policies | [git/git-merge-validation.md](git/git-merge-validation.md) |

## Docker/Container Commands

### Containerization and Deployment

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian docker-deploy` | Deploy CodeGuardian in containerized environments | [docker/docker-deploy.md](docker/docker-deploy.md) |
| `codeguardian docker-compose` | Generate and manage Docker Compose configurations | [docker/docker-compose.md](docker/docker-compose.md) |
| `codeguardian docker-build-optimize` | Optimize Docker builds with multi-stage builds | [docker/docker-build-optimize.md](docker/docker-build-optimize.md) |

## CI/CD Platform Commands

### Platform-Specific Integrations

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian ci-cd github-actions` | Generate GitHub Actions workflows | [ci-cd/github-actions.md](ci-cd/github-actions.md) |
| `codeguardian ci-cd gitlab-ci` | Generate GitLab CI/CD pipelines | [ci-cd/gitlab-ci.md](ci-cd/gitlab-ci.md) |
| `codeguardian ci-cd jenkins` | Generate Jenkins pipeline configurations | [ci-cd/jenkins.md](ci-cd/jenkins.md) |
| `codeguardian ci-cd azure-devops` | Generate Azure DevOps YAML pipelines | [ci-cd/azure-devops.md](ci-cd/azure-devops.md) |

## Configuration Management Commands

### Configuration and Environment Management

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian config validate` | Validate configuration files | [config/config-validate.md](config/config-validate.md) |
| `codeguardian config migrate` | Migrate configuration between versions | [config/config-migrate.md](config/config-migrate.md) |
| `codeguardian config env` | Manage environment-specific configurations | [config/config-env.md](config/config-env.md) |
| `codeguardian config template` | Generate configuration templates | [config/config-template.md](config/config-template.md) |

## Troubleshooting and Diagnostics Commands

### Debugging and Diagnostics

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian debug mode` | Enable debug mode for detailed analysis | [troubleshooting/debug-mode.md](troubleshooting/debug-mode.md) |
| `codeguardian debug logs` | Analyze and troubleshoot log files | [troubleshooting/debug-logs.md](troubleshooting/debug-logs.md) |
| `codeguardian debug profile` | Profile analysis performance and bottlenecks | [troubleshooting/debug-profile.md](troubleshooting/debug-profile.md) |
| `codeguardian debug health` | Run health checks and diagnostics | [troubleshooting/debug-health.md](troubleshooting/debug-health.md) |

## Performance and Monitoring Commands

### Performance Analysis and Monitoring

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian performance benchmark` | Run performance benchmarks | [performance/performance-benchmark.md](performance/performance-benchmark.md) |
| `codeguardian performance monitor` | Monitor resource usage and performance | [performance/performance-monitor.md](performance/performance-monitor.md) |
| `codeguardian performance optimize` | Optimize performance settings | [performance/performance-optimize.md](performance/performance-optimize.md) |
| `codeguardian performance cache` | Manage analysis caching | [performance/performance-cache.md](performance/performance-cache.md) |

## Security and Compliance Commands

### Security Analysis and Compliance

| Command | Description | Documentation |
|---------|-------------|---------------|
| `codeguardian security audit` | Perform comprehensive security audits | [security/security-audit.md](security/security-audit.md) |
| `codeguardian security policy` | Manage and enforce security policies | [security/security-policy.md](security/security-policy.md) |
| `codeguardian security compliance` | Generate compliance reports | [security/security-compliance.md](security/security-compliance.md) |
| `codeguardian security scan` | Run security vulnerability scans | [security/security-scan.md](security/security-scan.md) |

## Command Categories Overview

### By Use Case

#### Development Workflow Integration
- **Git Integration**: Automate analysis in Git workflows
- **CI/CD Integration**: Integrate with popular CI/CD platforms
- **Docker Integration**: Containerize and deploy CodeGuardian

#### Configuration and Setup
- **Configuration Management**: Handle configuration files and environments
- **Initialization**: Set up CodeGuardian for new projects
- **Template Generation**: Generate configuration templates

#### Analysis and Reporting
- **Core Analysis**: Primary code analysis functionality
- **Reporting**: Generate reports in various formats
- **Issue Management**: Create and manage issues from analysis results

#### Performance and Optimization
- **Performance Monitoring**: Track and optimize performance
- **Caching**: Manage analysis caching for efficiency
- **Benchmarking**: Run performance benchmarks

#### Security and Compliance
- **Security Analysis**: Comprehensive security scanning
- **Policy Management**: Manage security policies
- **Compliance Reporting**: Generate compliance documentation

#### Troubleshooting and Maintenance
- **Debugging**: Debug analysis issues and problems
- **Health Checks**: Monitor system health
- **Log Analysis**: Analyze logs for insights

### By Target Audience

#### Developers
- `codeguardian check` - Primary analysis command
- `codeguardian init` - Project setup
- `codeguardian debug mode` - Development debugging
- Git integration commands for workflow automation

#### DevOps Engineers
- CI/CD platform commands for pipeline integration
- Docker commands for containerization
- Configuration management commands
- Performance monitoring commands

#### Security Teams
- Security audit and scanning commands
- Policy management commands
- Compliance reporting commands
- Git integration for security gates

#### Platform Administrators
- Performance optimization commands
- Health check and monitoring commands
- Configuration management commands
- Troubleshooting and diagnostics commands

## Quick Start Guides

### Getting Started with CodeGuardian

1. **Initialize Configuration**
   ```bash
   codeguardian init
   ```

2. **Run Basic Analysis**
   ```bash
   codeguardian check
   ```

3. **Set Up Git Integration**
   ```bash
   codeguardian git-hooks-setup
   ```

4. **Integrate with CI/CD**
   ```bash
   # Choose your platform
   codeguardian ci-cd github-actions generate
   # or
   codeguardian ci-cd gitlab-ci generate
   ```

### Common Workflows

#### Development Workflow
```bash
# Initialize project
codeguardian init

# Set up Git hooks
codeguardian git-hooks-setup

# Run analysis
codeguardian check

# Generate report
codeguardian report --format markdown
```

#### CI/CD Integration
```bash
# Generate CI/CD configuration
codeguardian ci-cd github-actions generate --workflow-type pr

# Validate configuration
codeguardian ci-cd github-actions validate

# Optimize performance
codeguardian ci-cd github-actions optimize
```

#### Security Audit
```bash
# Run security audit
codeguardian security audit

# Generate compliance report
codeguardian security compliance

# Check policies
codeguardian security policy validate
```

#### Performance Optimization
```bash
# Run performance benchmark
codeguardian performance benchmark

# Monitor resources
codeguardian performance monitor

# Optimize settings
codeguardian performance optimize
```

## Command Line Help

All commands support the following global options:

```bash
codeguardian [COMMAND] --help          # Show help for specific command
codeguardian [COMMAND] --verbose       # Enable verbose output
codeguardian [COMMAND] --quiet         # Suppress non-error output
codeguardian [COMMAND] --config FILE   # Specify configuration file
```

### Getting Help

```bash
# General help
codeguardian --help

# Command-specific help
codeguardian check --help
codeguardian git-hooks-setup --help

# Subcommand help
codeguardian ci-cd github-actions generate --help
```

## Configuration Files

CodeGuardian uses the following configuration files:

- `codeguardian.toml` - Main configuration file
- `.codeguardian/` - Configuration directory
- `.gitignore` - Git ignore patterns
- `Dockerfile` - Container build configuration
- CI/CD platform configuration files

## Environment Variables

Common environment variables:

- `CODEGUARDIAN_CONFIG` - Configuration file path
- `CODEGUARDIAN_VERBOSE` - Enable verbose logging
- `CODEGUARDIAN_CACHE_DIR` - Cache directory path
- `GITHUB_TOKEN` - GitHub API token
- `GITLAB_TOKEN` - GitLab API token

## Exit Codes

CodeGuardian uses the following exit codes:

- `0` - Success
- `1` - Analysis found issues
- `2` - Configuration error
- `3` - Runtime error
- `4` - Permission error
- `5` - Network error

## Support and Resources

### Documentation
- [User Guide](../../user-guide/) - Comprehensive user documentation
- [Configuration Guide](../../user-guide/configuration.md) - Configuration options
- [Troubleshooting](../../troubleshooting/) - Common issues and solutions

### Community Resources
- [GitHub Repository](https://github.com/your-org/codeguardian) - Source code and issues
- [Discussions](https://github.com/your-org/codeguardian/discussions) - Community discussions
- [Contributing Guide](../../contributing.md) - How to contribute

### Professional Support
- Enterprise support options
- Training and certification programs
- Consulting services

## Version Information

This documentation covers CodeGuardian version 1.0.0 and later. For version-specific information, see the [changelog](../../CHANGELOG.md) and [migration guides](../../user-guide/migration.md).

---

*Last updated: 2025-01-15 | CodeGuardian v1.0.0*
