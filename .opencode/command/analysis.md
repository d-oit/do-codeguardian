---
description: Run comprehensive code analysis with security, performance, and quality checks
---

# check

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
| `--ml-model <PATH>` | Path to ML model for false positive reduction | `STRING` | - | No |
| `--config <FILE>` | Configuration file path | `PATH` | - | No |
| `--exclude <PATTERN>` | Patterns to exclude from analysis | `STRING` | - | No |
| `--include <PATTERN>` | Patterns to include in analysis | `STRING` | - | No |
| `--output <FILE>` | Output file for results (alias for --out) | `PATH` | - | No |
| `--verbose` | Verbose output | `FLAG` | `false` | No |

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
  --ml-model enhanced-model.fann \
  --format json \
  --out ml-results.json
```

## Error Handling

### Common Errors
- **Configuration Error**: Missing or invalid configuration file
  ```bash
  codeguardian check . --config nonexistent.toml
  # Error: Failed to read config file: No such file or directory
  ```

- **GitHub Integration Error**: Missing repository specification or invalid format
  ```bash
  codeguardian check . --emit-gh --repo invalid-format
  # Error: Invalid repository format. Expected: owner/repo
  ```

- **File System Error**: Permission denied or path not found
  ```bash
  codeguardian check /root/private/
  # Error: Permission denied (os error 13)
  ```

- **Resource Error**: Memory exhaustion or timeout exceeded
  ```bash
  codeguardian check large-file.bin
  # Error: File size 500MB exceeds maximum allowed size 100MB
  ```

### Recovery Procedures
1. **Configuration Issues**: Check configuration file exists and validate syntax
   ```bash
   ls -la codeguardian.toml
   codeguardian check . --config codeguardian.toml --verbose
   ```

2. **GitHub Token Issues**: Verify token is set and has proper permissions
   ```bash
   echo $GITHUB_TOKEN | head -c 10
   curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user
   ```

3. **Performance Issues**: Reduce parallelism or exclude problematic directories
   ```bash
   codeguardian check . --parallel 2 --exclude "node_modules/**"
   ```

4. **Network Issues**: Check connectivity and repository access
   ```bash
   ping github.com
   curl https://api.github.com/repos/myorg/myrepo
   ```

## Security Considerations
- **Input Validation**: All input paths are validated and sanitized to prevent directory traversal attacks
- **File Size Limits**: Maximum file size limits (default 10MB) prevent resource exhaustion attacks
- **Pattern Validation**: Include/exclude patterns are validated to prevent regex injection
- **Sensitive Data Redaction**: Automatic detection and redaction of secrets, tokens, and credentials
- **HTTPS Only**: All GitHub API communications use HTTPS with certificate validation
- **Token Security**: GitHub tokens are handled securely and never logged in plain text
- **Sandboxing**: Analysis runs in isolated environment to prevent code execution
- **Resource Limits**: Memory and CPU limits prevent resource exhaustion
- **Timeout Protection**: Analysis timeouts prevent infinite loops and resource consumption

## Best Practices

### Security Considerations
- **Regular Token Rotation**: Rotate GitHub tokens regularly and use fine-grained permissions
- **Environment Isolation**: Run analysis in isolated environments to prevent credential leakage
- **Configuration Auditing**: Regularly audit configuration files for sensitive data exposure
- **Network Security**: Use VPN or secure networks when analyzing sensitive codebases

### Performance Optimization Tips
- **Use Parallel Processing**: Leverage `--parallel` option for multi-core systems (default auto-detection)
- **Enable Caching**: Use baseline files for incremental analysis in CI/CD pipelines
- **Diff-Only Analysis**: Use `--diff` or `--only-changed` for pull request analysis
- **Selective Analysis**: Use `--include`/`--exclude` patterns to focus on relevant code
- **ML Model Usage**: Train and use ML models for significant false positive reduction (60-80%)

### Common Pitfalls to Avoid
- **Missing Repository Specification**: Always specify `--repo owner/repo` when using `--emit-gh`
- **Incorrect Diff Format**: Use proper git diff format like `origin/main..HEAD` for PR analysis
- **Large File Analysis**: Avoid analyzing very large binary files that may cause timeouts
- **Token Permission Issues**: Ensure GitHub tokens have `repo` and `issues` permissions
- **Configuration Conflicts**: Don't mix multiple configuration sources without validation

### Integration Recommendations
- **CI/CD Integration**: Use JSON output format for programmatic processing in pipelines
- **GitHub Actions**: Combine with `--emit-gh` and `--diff` for automated PR analysis
- **Pre-commit Hooks**: Integrate with pre-commit for developer workflow
- **Scheduled Scans**: Set up regular scans for security drift detection
- **Multi-Repository**: Use consistent configuration across related repositories

### Maintenance Guidelines
- **Regular Updates**: Keep CodeGuardian updated for latest security rules and performance improvements
- **Model Retraining**: Retrain ML models quarterly or when false positive rates increase
- **Configuration Review**: Review and update configuration files with new analysis requirements
- **Result Archival**: Archive analysis results for trend analysis and compliance reporting
- **Team Training**: Ensure team members understand analysis results and remediation steps

## See Also
- [`codeguardian report`](report.md) - Convert analysis results to different formats
- [`codeguardian gh-issue`](gh-issue.md) - Create or update GitHub issues
- [`codeguardian turbo`](turbo.md) - High-performance analysis for large codebases
- [`codeguardian train`](train.md) - Train ML model for false positive reduction
- [`codeguardian init`](init.md) - Initialize configuration with presets
- [`codeguardian metrics`](metrics.md) - View ML model performance metrics
- [Configuration Guide](../configuration.md) - Configuration options and presets
- [CI/CD Setup Guide](../user-guide/ci-cd-setup.md) - CI/CD integration examples
- [GitHub Integration Guide](../user-guide/github-integration.md) - GitHub workflow integration