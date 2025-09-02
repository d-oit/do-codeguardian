# 🔧 API Reference

This section provides detailed documentation for CodeGuardian's programmatic interfaces, including command-line options, configuration APIs, and integration points.

## Command Line Interface

### Global Options

All commands support these global options:

```bash
codeguardian [OPTIONS] <COMMAND>
```

| Option | Description | Example |
|--------|-------------|---------|
| `-h, --help` | Print help information | `codeguardian --help` |
| `-V, --version` | Print version information | `codeguardian --version` |
| `--config <FILE>` | Use specific config file | `codeguardian --config custom.toml check .` |
| `--verbose` | Enable verbose output | `codeguardian --verbose check .` |
| `--quiet` | Suppress non-error output | `codeguardian --quiet check .` |
| `--no-color` | Disable colored output | `codeguardian --no-color check .` |

### Core Commands

#### `check` - Primary Analysis Command

```bash
codeguardian check [OPTIONS] [PATH]
```

**Description**: Run comprehensive security, performance, and code quality analysis on the specified path.

**Arguments**:
- `PATH`: Directory or file to analyze (default: current directory)

**Input Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--diff <COMMIT>` | Analyze only changed files since commit | - |
| `--only-changed` | Analyze only staged files | `false` |
| `--include <PATTERN>` | Include only matching files | - |
| `--exclude <PATTERN>` | Exclude matching files | - |
| `--custom-rules <FILE>` | Load custom security rules | - |

**Output Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--format <FORMAT>` | Output format (json, human, sarif) | `human` |
| `--out <FILE>` | Output file path | - |
| `--emit-md <FILE>` | Generate Markdown report | - |
| `--emit-html <FILE>` | Generate HTML report | - |
| `--emit-gh` | Create GitHub issues | `false` |

**Performance Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--max-parallel <NUM>` | Maximum parallel workers | CPU cores |
| `--memory-limit <MB>` | Memory limit in MB | `1024` |
| `--streaming-threshold <MB>` | Enable streaming for large files | `5` |
| `--cache-enabled` | Enable analysis caching | `true` |
| `--timeout <SECONDS>` | Analysis timeout per file | `30` |

**ML Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--ml-model <PATH>` | Use ML model for false positive reduction | - |
| `--ml-threshold <FLOAT>` | ML confidence threshold (0.0-1.0) | `0.8` |
| `--online-learning` | Enable online learning | `false` |
| `--no-ml` | Disable ML features | `false` |

**GitHub Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--repo <OWNER/REPO>` | GitHub repository for issue creation | - |
| `--mode <MODE>` | GitHub issue format (checklist, simple, children) | `checklist` |
| `--title-prefix <PREFIX>` | Custom issue title prefix | `"Security Alert: "` |
| `--labels <LABELS>` | Comma-separated issue labels | `"security,automated"` |
| `--dry-run` | Preview GitHub issues without creating | `false` |

**Analysis Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--fail-on-issues` | Exit with error if issues found | `false` |
| `--min-severity <LEVEL>` | Minimum severity level (low, medium, high, critical) | `low` |
| `--max-severity <LEVEL>` | Maximum severity level | `critical` |
| `--analyzer <NAME>` | Run only specific analyzer | - |
| `--disable-analyzer <NAME>` | Disable specific analyzer | - |

**Examples**:
```bash
# Basic analysis
codeguardian check .

# JSON output with ML filtering
codeguardian check . --format json --out results.json --ml-model model.fann

# PR analysis with GitHub integration
codeguardian check . --diff origin/main..HEAD --emit-gh --repo myorg/myrepo

# High-performance analysis
codeguardian check . --max-parallel 16 --memory-limit 2048 --streaming-threshold 10
```

#### `report` - Generate Reports

```bash
codeguardian report [OPTIONS] --from <INPUT>
```

**Description**: Convert analysis results to different output formats.

**Required Options**:
| Option | Description |
|--------|-------------|
| `--from <FILE>` | Input results file (JSON) |

**Output Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--md <FILE>` | Generate Markdown report | - |
| `--html <FILE>` | Generate HTML report | - |
| `--sarif <FILE>` | Generate SARIF report | - |
| `--pdf <FILE>` | Generate PDF report | - |
| `--template <FILE>` | Use custom template | - |

**Examples**:
```bash
# Generate multiple report formats
codeguardian report --from results.json --md report.md --html report.html --sarif report.sarif

# Use custom template
codeguardian report --from results.json --template custom-template.md --out custom-report.md
```

#### `init` - Initialize Configuration

```bash
codeguardian init [OPTIONS]
```

**Description**: Initialize CodeGuardian configuration with presets.

**Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--template <TEMPLATE>` | Configuration template (minimal, security, ci, performance, enterprise) | `security` |
| `--interactive` | Interactive configuration setup | `false` |
| `--force` | Overwrite existing configuration | `false` |
| `--list` | List available templates | `false` |

**Examples**:
```bash
# Create security-focused configuration
codeguardian init --template security

# Interactive setup
codeguardian init --interactive

# List available templates
codeguardian init --list
```

#### `gh-issue` - GitHub Issue Management

```bash
codeguardian gh-issue [OPTIONS] --from <INPUT> --repo <REPOSITORY>
```

**Description**: Create and manage GitHub issues from analysis results.

**Required Options**:
| Option | Description |
|--------|-------------|
| `--from <FILE>` | Input results file (JSON) |
| `--repo <OWNER/REPO>` | GitHub repository |

**Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--mode <MODE>` | Issue format (simple, checklist, children) | `checklist` |
| `--title-prefix <PREFIX>` | Issue title prefix | `"Security Alert: "` |
| `--labels <LABELS>` | Comma-separated labels | `"security,automated"` |
| `--assignees <USERS>` | Comma-separated assignees | - |
| `--milestone <NAME>` | Milestone name | - |
| `--dry-run` | Preview without creating issues | `false` |
| `--update-existing` | Update existing issues | `true` |
| `--close-resolved` | Close resolved issues | `true` |

**Examples**:
```bash
# Create checklist-style issues
codeguardian gh-issue --from results.json --repo myorg/myrepo --mode checklist

# Assign issues to specific users
codeguardian gh-issue --from results.json --repo myorg/myrepo --assignees alice,bob
```

#### `train` - ML Model Training

```bash
codeguardian train [OPTIONS]
```

**Description**: Train ML models for improved false positive detection.

**Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--model-path <PATH>` | Output model path | `enhanced-model.fann` |
| `--epochs <NUM>` | Training epochs | `2000` |
| `--learning-rate <FLOAT>` | Learning rate | `0.01` |
| `--bootstrap` | Use bootstrap sampling | `false` |
| `--validation-split <FLOAT>` | Validation data split (0.0-1.0) | `0.2` |
| `--early-stopping` | Enable early stopping | `true` |
| `--patience <NUM>` | Early stopping patience | `50` |

**Examples**:
```bash
# Train with default settings
codeguardian train

# Custom training configuration
codeguardian train --model-path custom-model.fann --epochs 5000 --learning-rate 0.001
```

#### `turbo` - High-Performance Analysis

```bash
codeguardian turbo [OPTIONS] [PATH]
```

**Description**: Run high-performance analysis optimized for large codebases.

**Arguments**:
- `PATH`: Directory to analyze (default: current directory)

**Options**:
| Option | Description | Default |
|--------|-------------|---------|
| `--max-parallel <NUM>` | Maximum parallel workers | `16` |
| `--memory-limit <MB>` | Memory limit in MB | `2048` |
| `--streaming-threshold <MB>` | Streaming threshold | `5` |
| `--metrics` | Enable performance metrics | `false` |
| `--aggressive` | Enable aggressive optimizations | `false` |
| `--format <FORMAT>` | Output format | `json` |
| `--output <FILE>` | Output file | `turbo-results.json` |

**Examples**:
```bash
# Turbo analysis with maximum performance
codeguardian turbo . --max-parallel 32 --memory-limit 8192 --aggressive

# Turbo analysis with metrics
codeguardian turbo . --metrics --format json --output performance-results.json
```

## Configuration API

### Configuration File Format

CodeGuardian uses TOML format for configuration files. Here's the complete schema:

```toml
[general]
max_file_size = 10485760
parallel_workers = 4
memory_limit_mb = 1024
streaming_threshold_mb = 5
cache_dir = "~/.cache/codeguardian"
temp_dir = "/tmp/codeguardian"
log_level = "info"
tty_output = true
progress_interval = 2

[analyzers.security]
enabled = true
check_secrets = true
check_hardcoded_secrets = true
entropy_threshold = 4.5
check_vulnerabilities = true
check_sql_injection = true
check_xss = true
check_command_injection = true
check_path_traversal = true
check_weak_crypto = true
check_auth_bypass = true
check_insecure_random = true
check_info_disclosure = true
redact_secrets = true
custom_patterns_file = "custom-security-patterns.json"

[analyzers.performance]
enabled = true
check_allocations = true
check_memory_leaks = true
check_large_objects = true
check_nested_loops = true
check_string_operations = true
check_blocking_io = true
check_resource_exhaustion = true
max_function_length = 100
max_complexity = 10
max_nesting_depth = 4
memory_threshold_mb = 100
cpu_threshold_percent = 80

[analyzers.code_quality]
enabled = true
check_naming = true
naming_convention = "snake_case"
check_complexity = true
check_duplication = true
check_unused_code = true
check_documentation = true
require_doc_comments = false
max_line_length = 120
max_function_length = 50
max_file_length = 1000

[analyzers.dependency]
enabled = true
check_vulnerabilities = true
check_outdated = true
check_licenses = true
check_unused_dependencies = true
check_circular_dependencies = true
max_dependency_depth = 10
allowed_licenses = ["MIT", "Apache-2.0", "BSD-3-Clause"]
blocked_licenses = ["GPL-3.0"]

[ml]
enabled = true
model_path = "enhanced-model.fann"
model_type = "ruf_fann"
online_learning = true
learning_rate = 0.01
epochs = 2000
bootstrap_training = true
feature_extraction = "enhanced"
include_context = true
context_window = 5
confidence_threshold = 0.8
auto_threshold_adjustment = true
feedback_loop = true

[performance]
cache_enabled = true
cache_max_age_days = 30
cache_compression = true
cache_max_size_mb = 500
parallel_processing = true
max_parallel_tasks = 16
semaphore_limit = 8
memory_optimization = true
streaming_enabled = true
streaming_threshold_mb = 5
garbage_collection_interval = 100
buffered_io = true
async_io = true
io_buffer_size_kb = 64

[github]
token = "${GITHUB_TOKEN}"
app_id = "your_app_id"
private_key_path = "/path/to/private-key.pem"
default_labels = ["security", "automated"]
title_prefix = "Security Alert: "
body_template = "github-issue-template.md"
max_body_size = 60000
rate_limit_buffer = 100
retry_attempts = 3
retry_delay_ms = 1000
default_mode = "checklist"
create_issues = true
update_existing = true
close_resolved = true

[output]
default_format = "human"
color_output = true
pretty_print = true
output_dir = "./codeguardian-results"
overwrite_existing = false
compress_output = false
generate_summary = true
include_statistics = true
include_recommendations = true
template_dir = "./templates"
custom_templates = ["security-report.md", "performance-report.html"]
```

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `CODEGUARDIAN_CONFIG` | Configuration file path | `/etc/codeguardian.toml` |
| `CODEGUARDIAN_MAX_FILE_SIZE` | Maximum file size | `20971520` |
| `CODEGUARDIAN_MEMORY_LIMIT_MB` | Memory limit | `2048` |
| `CODEGUARDIAN_LOG_LEVEL` | Log level | `debug` |
| `CODEGUARDIAN_ML_MODEL` | ML model path | `/path/to/model.fann` |
| `GITHUB_TOKEN` | GitHub API token | `ghp_xxx` |

## Output Formats

### JSON Format

Standard output format for programmatic use:

```json
{
  "metadata": {
    "version": "0.1.0",
    "timestamp": "2024-01-01T12:00:00Z",
    "files_analyzed": 42,
    "analysis_time_seconds": 15.3,
    "codeguardian_version": "1.2.0"
  },
  "findings": [
    {
      "id": "sha256_hash_of_finding",
      "file_path": "src/main.rs",
      "line_number": 25,
      "column": 12,
      "severity": "high",
      "category": "security",
      "analyzer": "security",
      "title": "Hardcoded secret detected",
      "description": "Potential hardcoded API key found",
      "confidence": 0.95,
      "code_snippet": "let api_key = \"sk-123456789\";",
      "recommendation": "Use environment variables or secure credential storage",
      "cwe_id": "CWE-798",
      "references": ["https://cwe.mitre.org/data/definitions/798.html"],
      "tags": ["secret", "hardcoded", "api_key"]
    }
  ],
  "summary": {
    "total_findings": 15,
    "critical": 2,
    "high": 5,
    "medium": 6,
    "low": 2,
    "by_category": {
      "security": 8,
      "performance": 4,
      "code_quality": 3
    },
    "by_analyzer": {
      "security": 8,
      "performance": 4,
      "code_quality": 3
    }
  },
  "statistics": {
    "files_processed": 42,
    "total_lines": 15420,
    "analysis_time_seconds": 15.3,
    "memory_peak_mb": 256,
    "cache_hits": 15,
    "cache_misses": 27
  }
}
```

### SARIF Format

Security-focused format for integration with security tools:

```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "CodeGuardian",
          "version": "1.2.0",
          "informationUri": "https://github.com/d-oit/codeguardian",
          "rules": [...]
        }
      },
      "results": [...]
    }
  ]
}
```

## Exit Codes

| Code | Description |
|------|-------------|
| `0` | Success - no issues found or analysis completed |
| `1` | General error |
| `2` | Configuration error |
| `3` | Analysis failed |
| `4` | Issues found (when using `--fail-on-issues`) |
| `5` | Authentication error |
| `6` | Network error |
| `7` | File system error |

## Integration Examples

### GitHub Actions

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

### Docker Integration

```bash
docker run --rm \
  -v $(pwd):/workspace \
  -v $(pwd)/codeguardian.toml:/etc/codeguardian.toml \
  codeguardian/codeguardian \
  check /workspace --format json --out /workspace/results.json
```

### Programmatic Usage

```bash
#!/bin/bash
# Run analysis and check results
codeguardian check . --format json --out results.json

# Check if issues were found
if [ $? -eq 4 ]; then
    echo "Issues found, check results.json"
    exit 1
fi

# Generate report
codeguardian report --from results.json --md report.md
```

## Error Handling

### Common Error Messages

| Error | Description | Solution |
|-------|-------------|----------|
| `Configuration file not found` | Config file missing or invalid path | Check file path and permissions |
| `Authentication failed` | Invalid GitHub token | Verify token has required permissions |
| `Analysis timeout` | File analysis took too long | Increase timeout or check file size |
| `Memory limit exceeded` | Analysis used too much memory | Increase memory limit or use streaming |
| `Network error` | Failed to connect to external services | Check network connectivity |

### Debugging

Enable verbose logging for troubleshooting:

```bash
# Enable debug logging
CODEGUARDIAN_LOG_LEVEL=debug codeguardian check .

# Save logs to file
codeguardian check . --verbose 2>&1 | tee analysis.log
```

## Next Steps

- **[User Guide](../user-guide/)** - Learn about using CodeGuardian
- **[Configuration Guide](../user-guide/configuration.md)** - Customize CodeGuardian
- **[Examples](../examples/)** - See CodeGuardian in action

---

<div align="center">

**[⬅️ Back to Documentation](../README.md)** | **[📖 User Guide](../user-guide/)** | **[💡 Examples](../examples/)**

</div>
