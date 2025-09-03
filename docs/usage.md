# Usage Guide

> **Note**: Command examples in this guide are based on actual CI/CD workflows from `.github/workflows/`. For local development, you may need to adjust paths (e.g., use `do-do-codeguardian` instead of `./target/release/do-do-codeguardian` if installed via `cargo install`).

## Quick Start

### Basic Usage

```bash
# 1. Install CodeGuardian
cargo install do-do-codeguardian

# 2. Initialize configuration with security template
do-do-codeguardian init --template security

# 3. Run analysis with ML filtering
./target/release/do-do-codeguardian check . --format json --out results.json --ml-model enhanced-model.fann

# 4. Generate comprehensive report
./target/release/do-do-codeguardian report --from results.json --md report.md --html report.html

# 5. Create GitHub issues with checklist format
./target/release/do-do-codeguardian gh-issue --from results.json --repo owner/repo --mode checklist
```

### One-Liner Analysis

```bash
# Quick security scan with GitHub integration (from CI workflows)
./target/release/do-do-codeguardian check . --emit-gh --repo ${{ github.repository }}

# Fast PR analysis with ML filtering (from turbo-pr-analysis.yml)
./target/release/do-do-codeguardian check . --diff origin/main..HEAD --ml-model enhanced-model.fann --emit-gh --repo ${{ github.repository }}

# High-performance analysis (from turbo-security-analysis.yml)
./target/release/do-do-codeguardian turbo . --max-parallel 16 --metrics --format json --output results.json

# Security audit with comprehensive reporting (from do-codeguardian-ci.yml)
./target/release/do-do-codeguardian check . --config security-config.toml --format json --out audit.json --emit-md audit-report.md --emit-gh --repo ${{ github.repository }} --fail-on-issues
```

## Commands

### Core Commands

| Command | Description |
|---------|-------------|
| `check` | Primary analysis command with comprehensive options |
| `report` | Convert analysis results to different formats |
| `gh-issue` | Create/update GitHub issues from analysis results |
| `init` | Initialize configuration with presets |
| `turbo` | High-performance analysis for large codebases |
| `train` | Train ML model for better accuracy |
| `metrics` | View ML model performance metrics |

### Advanced Commands

| Command | Description |
|---------|-------------|
| `config` | Manage configuration files and presets |
| `cache` | Manage analysis cache and performance data |
| `stream` | Streaming analysis for very large files |
| `analyze` | Run specific analyzers individually |
| `validate` | Validate configuration and environment |
| `doctor` | Diagnose system and configuration issues |
| `optimize` | Performance optimization and tuning |
| `benchmark` | Run performance benchmarks |
| `export` | Export analysis data and models |
| `import` | Import external analysis data |

## Common Usage Patterns

### PR Analysis (Fast, Focused)

```bash
# From do-codeguardian-ci.yml (PR diff-only mode)
./target/release/do-do-codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out pr-results.json \
  --emit-md pr-report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist \
  --labels "do-codeguardian,automated,pr-${{ github.event.number }}"
```

### Full Repository Scan

```bash
# From do-codeguardian-ci.yml (full scan for main branch)
./target/release/do-do-codeguardian check . \
  --format json \
  --out full-results.json \
  --emit-md full-report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist \
  --labels "do-codeguardian,automated,full-scan" \
  --fail-on-issues
```

### Security Audit

```bash
# From do-codeguardian-ci.yml (full scan with security focus)
./target/release/do-do-codeguardian check . \
  --config security-config.toml \
  --format json \
  --out security-audit.json \
  --emit-md security-report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist \
  --labels "do-codeguardian,automated,full-scan" \
  --fail-on-issues
```

### Performance Analysis

```bash
# From turbo-security-analysis.yml (comprehensive mode)
./target/release/do-do-codeguardian turbo . \
  --aggressive \
  --max-parallel 8 \
  --memory-limit 1024 \
  --format json \
  --output performance-results.json \
  --metrics
```

### High-Performance Analysis

```bash
# From turbo-security-analysis.yml (comprehensive mode with aggressive)
./target/release/do-do-codeguardian turbo . \
  --aggressive \
  --max-parallel 8 \
  --memory-limit 1024 \
  --format json \
  --output turbo-results.json \
  --metrics
```

### ML-Enhanced Analysis

```bash
# From turbo-pr-analysis.yml (with ML model)
./target/release/do-do-codeguardian check . \
  --diff origin/main..HEAD \
  --ml-model enhanced-model.fann \
  --format json \
  --out ml-results.json \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist
```

### Enterprise-Scale Analysis

```bash
# From turbo-security-analysis.yml (comprehensive mode scaled up)
./target/release/do-do-codeguardian turbo . \
  --aggressive \
  --max-parallel 8 \
  --memory-limit 1024 \
  --format json \
  --output enterprise-results.json \
  --metrics
```

### Custom Security Rules

```bash
# From do-codeguardian-ci.yml (with config)
./target/release/do-do-codeguardian check . \
  --config security-config.toml \
  --format json \
  --out custom-results.json \
  --emit-md custom-report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
   --gh-mode checklist
```

### Comprehensive Examples

#### Basic Usage with Different Output Formats
```bash
# JSON output for machine processing
./target/release/do-do-codeguardian check . --format json --out results.json

# Human-readable output for manual review
./target/release/do-do-codeguardian check . --format human

# SARIF output for security tools integration
./target/release/do-do-codeguardian check . --format sarif --out security.sarif
```

#### CI/CD Integration with GitHub
```bash
# PR analysis with GitHub issue creation
./target/release/do-do-codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out pr-results.json \
  --emit-gh \
  --repo ${{ github.repository }} \
  --gh-mode checklist \
  --labels "codeguardian,pr-analysis"

# Scheduled security scan
./target/release/do-do-codeguardian check . \
  --config security-config.toml \
  --format json \
  --out nightly-scan.json \
  --emit-md nightly-report.md \
  --fail-on-issues
```

#### Performance-Optimized Analysis for Large Codebases
```bash
# High-parallel analysis with streaming
./target/release/do-do-codeguardian check . \
  --parallel 8 \
  --streaming \
  --memory-limit 2048 \
  --format json \
  --out large-repo-results.json

# Turbo mode for enterprise scale
./target/release/do-do-codeguardian turbo . \
  --max-parallel 16 \
  --aggressive \
  --format json \
  --output enterprise-results.json
```

#### Security-Focused Analysis with Broken Files Detection
```bash
# Comprehensive security audit
./target/release/do-do-codeguardian check . \
  --detect-broken-files \
  --config security-config.toml \
  --format json \
  --out security-audit.json \
  --emit-md security-report.md \
  --fail-on-issues

# Focused conflict detection
./target/release/do-do-codeguardian check . \
  --detect-conflicts \
  --fail-on-conflicts \
  --format human
```

#### Baseline Comparison and Drift Analysis
```bash
# Establish baseline
./target/release/do-do-codeguardian check . \
  --format json \
  --out baseline.json

# Compare against baseline
./target/release/do-do-codeguardian check . \
  --baseline baseline.json \
  --only-new \
  --format json \
  --out drift-results.json
```

#### ML-Enhanced Analysis with Threshold Tuning
```bash
# Balanced ML filtering
./target/release/do-do-codeguardian check . \
  --ml-model enhanced-model.fann \
  --ml-threshold 0.5 \
  --format json \
  --out ml-results.json

# High-precision mode (fewer false positives)
./target/release/do-do-codeguardian check . \
  --ml-model enhanced-model.fann \
  --ml-threshold 0.8 \
  --format json \
  --out precise-results.json
```

## Check Command Options

### Input Selection
- `--diff COMMIT` - Analyze only changed files since specified commit (optional)
- `--only-changed` - Analyze only staged files in git (optional)
- `--only-new` - Only analyze files new compared to baseline (optional)
- `--baseline PATH` - Baseline file for drift analysis (optional, short: -b)

### Output Control
- `--format json|human|sarif` - Output format (default: json)
- `--out FILE` - Output file path (default: results.json)
- `--emit-md FILE` - Generate Markdown report (optional)
- `--emit-gh` - Create GitHub issues (optional)
- `--quiet` - Suppress progress output (optional)

### Performance & Analysis
- `--parallel NUM` - Number of parallel workers (default: 0/auto, short: -p)
- `--streaming` - Enable streaming analysis for large files (optional)
- `--fail-on-issues` - Exit with error if issues found (optional)
- `--ml-threshold FLOAT` - ML threshold for anomaly detection (0.0-1.0, optional)

### GitHub Integration
- `--repo OWNER/REPO` - GitHub repository for issue creation (optional)
- `--gh-mode checklist|simple|children` - GitHub issue format (default: checklist)
- `--labels LABELS` - Comma-separated issue labels (default: "codeguardian,automated")

### Analysis Control
- `--fail-on-conflicts` - Fail fast on merge conflicts (CI/CD, optional)

### Broken Files Detection
- `--detect-broken-files` - Enable all broken file detection (optional)
- `--detect-conflicts` - Detect git merge conflicts (optional)
- `--detect-placeholders` - Detect AI-generated placeholders (optional)
- `--detect-duplicates` - Detect duplicate code (optional)

### Implementation Notes
- **Option Precedence**: Configuration file settings are overridden by command-line options
- **Parallel Processing**: Default parallel workers (0) uses CPU core count; set explicitly for control
- **Streaming Analysis**: Automatically enabled for files >10MB; use --streaming to force for smaller files
- **ML Threshold**: Values closer to 1.0 reduce false positives but may miss issues; 0.5 is balanced
- **GitHub Integration**: Requires GITHUB_TOKEN environment variable when using --emit-gh
- **Baseline Analysis**: Used for drift detection; --only-new compares against this baseline
- **Broken Files Detection**: Individual detectors can be enabled separately or all with --detect-broken-files
- **Conflict Detection**: --fail-on-conflicts exits immediately on conflicts, useful for CI pipelines

## Performance Guidance

### Parallel Processing Optimization
- **CPU Core Detection**: Default `--parallel 0` automatically detects available CPU cores
- **Recommended Values**: Use `--parallel 4-8` for most systems; scale up to 16 for high-end servers
- **Memory Considerations**: Each worker uses ~50-100MB; adjust based on available RAM
- **CI/CD Tuning**: Use `--parallel 2-4` in CI environments to avoid resource contention

### Memory Usage Optimization
- **Streaming Mode**: Enable `--streaming` for files >10MB to reduce memory footprint
- **Memory Limits**: Set `--memory-limit` (in MB) to prevent excessive usage
- **Large Codebase Strategies**: Combine `--streaming` with `--parallel` for optimal performance
- **Cache Utilization**: Enable caching to reduce repeated analysis overhead

### Large Codebase Analysis
- **Incremental Analysis**: Use `--diff` for PR-focused scans to analyze only changes
- **Baseline Comparison**: Use `--baseline` for drift detection in large repos
- **Turbo Mode**: Switch to `turbo` command for enterprise-scale analysis
- **Batch Processing**: Process large repos in chunks using directory filtering

### CI/CD Performance Tuning
- **Timeout Handling**: Set reasonable timeouts to prevent hanging builds
- **Resource Allocation**: Reserve adequate CPU/memory for analysis jobs
- **Parallel Jobs**: Run analysis in parallel with other CI tasks where possible
- **Caching**: Use build caches to speed up repeated analyses

## Security Considerations

### Input Validation
- **Path Traversal Prevention**: All file paths are canonicalized and validated
- **File Size Limits**: Default 10MB limit prevents resource exhaustion
- **Content Validation**: Binary files and oversized content are safely handled
- **Git Reference Validation**: Commit SHAs and refs are verified before processing

### File Size and Resource Limits
- **Automatic Streaming**: Files >10MB automatically use streaming analysis
- **Configurable Limits**: Override defaults via `--max-file-size` and `--memory-limit`
- **Bypass Prevention**: Hard limits prevent malicious large file attacks
- **Resource Monitoring**: Analysis tracks and reports resource usage

### GitHub Token Security
- **Environment Variables**: Use `GITHUB_TOKEN` securely in CI environments
- **Scope Limitation**: Tokens should have minimal required permissions
- **Token Rotation**: Regularly rotate tokens for security
- **Audit Logging**: All GitHub API calls are logged for compliance

### Environment Isolation
- **Container Security**: Run in isolated containers with minimal privileges
- **Network Isolation**: Limit network access to required endpoints only
- **Configuration Security**: Validate and audit configuration files
- **Secure Defaults**: All security options default to safe values

## Advanced Workflows

### Training ML Model

```bash
# From examples/ml-training-example.rs (training command)
./target/release/do-do-codeguardian train --model-path enhanced-model.fann --epochs 2000 --bootstrap
```

### CI/CD Integration

#### GitHub Actions

```yaml
# From do-codeguardian-ci.yml (PR analysis job)
- name: Run CodeGuardian (PR diff-only)
  run: |
    ./target/release/do-do-codeguardian check . \
      --diff origin/main..HEAD \
      --format json \
      --out results.json \
      --emit-md report.md \
      --emit-gh \
      --repo ${{ github.repository }} \
      --gh-mode checklist \
      --labels "do-codeguardian,automated,pr-${{ github.event.number }}"
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    GITHUB_REPOSITORY: ${{ github.repository }}
```

#### GitLab CI

```yaml
# From README.md (GitLab CI example)
do-codeguardian:
  stage: security
  image: do-codeguardian/do-codeguardian:latest
  script:
    - |
      if [ -n "$CI_MERGE_REQUEST_TARGET_BRANCH_NAME" ]; then
        ./target/release/do-do-codeguardian check . \
          --diff origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME..HEAD \
          --format json \
          --out results.json \
          --emit-md report.md
      else
        ./target/release/do-do-codeguardian check . \
          --format json \
          --out results.json \
          --emit-md report.md
      fi
  artifacts:
    paths:
      - results.json
      - report.md
    reports:
      junit: results.json
```

#### Jenkins Pipeline

```groovy
# From README.md (Jenkins example)
pipeline {
    agent {
        docker {
            image 'do-codeguardian/do-codeguardian:latest'
            args '-v $WORKSPACE:/workspace -w /workspace'
        }
    }
    stages {
        stage('CodeGuardian Analysis') {
            steps {
                sh '''
                    ./target/release/do-do-codeguardian check . \
                      --format json \
                      --out results.json \
                      --emit-md report.md \
                      --max-parallel 4
                '''
            }
        }
    }
}
```

### Complex Diff Analysis with Multiple Branches
```bash
# Compare feature branch against multiple bases
./target/release/do-do-codeguardian check . \
  --diff origin/develop..feature-branch \
  --baseline main-baseline.json \
  --format json \
  --out multi-branch-analysis.json

# Analyze merge conflicts before merging
./target/release/do-do-codeguardian check . \
  --detect-conflicts \
  --fail-on-conflicts \
  --diff origin/main..HEAD \
  --format human
```

### Custom Baseline Workflows
```bash
# Create baseline from specific commit
./target/release/do-do-codeguardian check . \
  --diff v1.0.0..HEAD \
  --format json \
  --out release-baseline.json

# Drift analysis with custom thresholds
./target/release/do-do-codeguardian check . \
  --baseline release-baseline.json \
  --only-new \
  --ml-threshold 0.7 \
  --format json \
  --out drift-analysis.json
```

### Integration with External Tools
```bash
# SARIF output for security scanners
./target/release/do-do-codeguardian check . \
  --format sarif \
  --out codeguardian-results.sarif

# JSON output for custom dashboards
./target/release/do-do-codeguardian check . \
  --format json \
  --out analysis-data.json \
  --emit-md dashboard-report.md
```

### Automated Remediation Workflows
```bash
# Generate fix scripts from analysis
./target/release/do-do-codeguardian check . \
  --format json \
  --out issues.json

# Apply automated fixes (future feature)
# ./target/release/do-do-codeguardian fix \
#   --from issues.json \
#   --auto-apply
```

## Best Practices

### Pre-commit Hook Integration
```bash
# Install pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
./target/release/do-do-codeguardian check . --diff HEAD --fail-on-issues --quiet
EOF
chmod +x .git/hooks/pre-commit

# For staged files only
./target/release/do-do-codeguardian check . --only-changed --fail-on-issues
```

### Scheduled Security Scans
```bash
# Cron job for nightly scans
0 2 * * * /path/to/do-do-codeguardian check . \
  --config security-config.toml \
  --format json \
  --out /var/log/codeguardian/nightly-$(date +\%Y\%m\%d).json \
  --emit-md /var/log/codeguardian/nightly-report.md \
  --fail-on-issues

# Weekly comprehensive audit
0 3 * * 0 /path/to/do-do-codeguardian check . \
  --detect-broken-files \
  --format json \
  --out /var/log/codeguardian/weekly-audit.json
```

### Multi-Repository Consistency
```bash
# Standardize across repositories
for repo in repo1 repo2 repo3; do
  cd $repo
  ../do-do-codeguardian check . \
    --config ../shared-config.toml \
    --format json \
    --out ../results/$repo.json \
    --baseline ../baselines/$repo.json
  cd ..
done

# Centralized reporting
./do-do-codeguardian report \
  --from results/*.json \
  --consolidated-report multi-repo-report.md
```

### Result Archival and Trend Analysis
```bash
# Archive results with timestamps
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
./target/release/do-do-codeguardian check . \
  --format json \
  --out archive/results_$TIMESTAMP.json

# Trend analysis script
#!/bin/bash
# Compare last 7 days
find archive/ -name "results_*.json" -mtime -7 | \
  xargs ./target/release/do-do-codeguardian report \
    --trend-analysis \
    --output trend-report.md
```

### Team Training Recommendations
- **Onboarding**: Include CodeGuardian in developer onboarding checklists
- **Documentation**: Maintain team-specific configuration guides
- **Regular Reviews**: Schedule quarterly reviews of analysis results
- **Feedback Loop**: Encourage team feedback on false positives/negatives
- **Automation**: Integrate into code review processes

## Error Handling

### Configuration Validation Errors
- **Missing Files**: Ensure `codeguardian.toml` exists and is readable
- **Invalid Syntax**: Check for YAML/TOML syntax errors in config files
- **Permission Issues**: Verify read/write permissions for output directories
- **Recovery**: Use `do-do-codeguardian validate` to check configuration before analysis

### Network Connectivity Issues with GitHub
- **Token Validation**: Verify `GITHUB_TOKEN` is set and valid
- **API Rate Limits**: Handle GitHub API rate limiting with exponential backoff
- **Network Timeouts**: Configure appropriate timeout values for slow connections
- **Offline Mode**: Use `--no-emit-gh` for offline analysis

### Resource Exhaustion Handling
- **Memory Limits**: Monitor and handle out-of-memory conditions
- **Disk Space**: Check available space before writing large output files
- **CPU Overload**: Implement graceful degradation under high load
- **File Handle Limits**: Handle systems with low file descriptor limits

### ML Model Loading Failures
- **Model File Missing**: Verify model file path and permissions
- **Corrupted Models**: Detect and handle corrupted FANN model files
- **Version Compatibility**: Ensure model compatibility with current version
- **Fallback Mode**: Continue analysis without ML when model fails to load

### Specific Error Scenarios and Recovery
- **Git Repository Issues**: Handle detached HEAD, missing refs, corrupted repos
- **Large File Processing**: Graceful handling of files exceeding size limits
- **Concurrent Access**: Manage conflicts from multiple analysis processes
- **Partial Failures**: Continue analysis when individual files fail

## Output Formats

### JSON (Source of Truth)

```json
{
  "metadata": {
    "version": "0.1.0",
    "timestamp": "2024-01-01T12:00:00Z",
    "files_analyzed": 42
  },
  "findings": [...],
  "summary": {
    "total_findings": 15,
    "critical": 2,
    "high": 5,
    "medium": 6,
    "low": 2
  }
}
```

### Markdown Reports
Human-readable reports with organized sections, code snippets, and severity-based organization.

### HTML Reports
Interactive HTML reports with filtering, sorting, and detailed analysis views.

### SARIF Format
Security-focused SARIF output for integration with security scanning tools and platforms.

### GitHub Issues
Automatic issue creation with:
- Interactive checklists with progress tracking
- Stable finding IDs based on SHA-256 hashes
- Idempotent updates to prevent duplicates
- Customizable templates and labels
- Multiple issue modes (checklist, simple, children)
- Automatic issue lifecycle management

## See Also

### Related Commands
- **[Check Command](docs/user-guide/check.md)**: Comprehensive guide to the primary analysis command
- **[Turbo Command](docs/user-guide/turbo.md)**: High-performance analysis for large codebases
- **[Configuration Guide](docs/user-guide/configuration.md)**: Advanced configuration options and customization
- **[CI/CD Setup Guide](docs/user-guide/ci-cd-setup.md)**: Integration examples for various CI/CD platforms
- **[GitHub Integration Guide](docs/user-guide/github-integration.md)**: Detailed GitHub workflow and API integration
- **[Troubleshooting Guide](docs/troubleshooting/index.md)**: Common issues and resolution steps

### Additional Resources
- **[API Reference](docs/api/index.md)**: Complete API documentation for developers
- **[Performance Benchmarks](docs/coverage.md)**: Performance testing and optimization guides
- **[Security Best Practices](docs/security.md)**: Security-focused usage and configuration
- **[Contributing Guide](CONTRIBUTING.md)**: Development and contribution guidelines
