# Usage Guide

## Quick Start

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

# Security audit with comprehensive reporting
codeguardian check . --config security-config.toml --format json --out audit.json --emit-md audit-report.md --emit-gh --repo owner/repo --fail-on-issues
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
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out pr-results.json \
  --emit-md pr-report.md \
  --emit-gh \
  --repo ${{ github.repository }} \
  --ml-model enhanced-model.fann \
  --max-parallel 4
```

### Full Repository Scan

```bash
codeguardian check . \
  --format json \
  --out full-results.json \
  --emit-md full-report.md \
  --fail-on-issues \
  --cache-enabled \
  --memory-limit 2048
```

### Security Audit

```bash
codeguardian check . \
  --config security-config.toml \
  --format json \
  --out security-audit.json \
  --emit-md security-report.md \
  --emit-gh \
  --repo owner/repo \
  --fail-on-issues
```

### Performance Analysis

```bash
codeguardian turbo . \
  --metrics \
  --format json \
  --out performance-results.json \
  --max-parallel 16 \
  --memory-limit 4096 \
  --streaming-threshold 5
```

### High-Performance Analysis

```bash
codeguardian turbo . \
  --max-parallel 16 \
  --memory-limit 2048 \
  --metrics \
  --format json \
  --output turbo-results.json \
  --streaming-threshold 5 \
  --aggressive
```

### ML-Enhanced Analysis

```bash
codeguardian check . \
  --ml-model enhanced-model.fann \
  --ml-threshold 0.8 \
  --online-learning \
  --format json \
  --out ml-results.json
```

### Enterprise-Scale Analysis

```bash
codeguardian turbo . \
  --max-parallel 32 \
  --memory-limit 8192 \
  --streaming-threshold 10 \
  --cache-enabled \
  --compression \
  --metrics \
  --format json \
  --output enterprise-results.json
```

### Custom Security Rules

```bash
codeguardian check . \
  --config security-config.toml \
  --custom-rules custom-rules.json \
  --format json \
  --out custom-results.json
```

## Check Command Options

### Input Options
- `--diff COMMIT` - Analyze only changed files since commit
- `--only-changed` - Analyze only staged files
- `--include PATTERN` - Include only files matching pattern
- `--exclude PATTERN` - Exclude files matching pattern
- `--config PATH` - Use specific configuration file
- `--custom-rules PATH` - Load custom security rules

### Output Options
- `--format json|human|sarif` - Output format (JSON, human-readable, SARIF)
- `--out FILE` - Output file path
- `--emit-md FILE` - Generate Markdown report
- `--emit-html FILE` - Generate HTML report
- `--emit-gh` - Create GitHub issues
- `--quiet` - Suppress progress output
- `--verbose` - Enable verbose logging

### Performance Options
- `--max-parallel NUM` - Maximum parallel workers (default: CPU cores)
- `--memory-limit MB` - Memory limit in MB (default: 1024)
- `--streaming-threshold MB` - Enable streaming for files larger than threshold
- `--cache-enabled` - Enable analysis caching
- `--timeout SECONDS` - Analysis timeout per file

### ML Options
- `--ml-model PATH` - Use ML model for false positive reduction
- `--ml-threshold FLOAT` - ML confidence threshold (0.0-1.0)
- `--online-learning` - Enable online learning from user feedback
- `--no-ml` - Disable ML features

### GitHub Options
- `--repo OWNER/REPO` - GitHub repository for issue creation
- `--mode checklist|simple|children` - GitHub issue format
- `--title-prefix PREFIX` - Custom issue title prefix
- `--labels LABELS` - Comma-separated issue labels
- `--dry-run` - Preview GitHub issues without creating

### Analysis Options
- `--fail-on-issues` - Exit with error if issues found
- `--min-severity LEVEL` - Minimum severity level to report
- `--max-severity LEVEL` - Maximum severity level to report
- `--analyzer NAME` - Run only specific analyzer
- `--disable-analyzer NAME` - Disable specific analyzer

## Advanced Workflows

### Training ML Model

```bash
# Train ML model with bootstrap sampling
codeguardian train --model-path enhanced-model.fann --epochs 2000 --bootstrap
```

### CI/CD Integration

#### GitHub Actions

```yaml
- name: Run CodeGuardian
  uses: d-oit/codeguardian-action@v1
  with:
    args: |
      check . \
        --diff origin/main..HEAD \
        --format json \
        --out results.json \
        --emit-gh \
        --repo ${{ github.repository }} \
        --ml-model enhanced-model.fann \
        --max-parallel 4 \
        --memory-limit 1024
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

#### GitLab CI

```yaml
codeguardian:
  stage: security
  image: codeguardian/codeguardian:latest
  script:
    - |
      if [ -n "$CI_MERGE_REQUEST_TARGET_BRANCH_NAME" ]; then
        codeguardian check . \
          --diff origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME..HEAD \
          --format json \
          --out results.json \
          --emit-md report.md
      else
        codeguardian check . \
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
pipeline {
    agent {
        docker {
            image 'codeguardian/codeguardian:latest'
            args '-v $WORKSPACE:/workspace -w /workspace'
        }
    }
    stages {
        stage('CodeGuardian Analysis') {
            steps {
                sh '''
                    codeguardian check . \
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