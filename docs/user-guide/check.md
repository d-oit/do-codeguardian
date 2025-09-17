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
