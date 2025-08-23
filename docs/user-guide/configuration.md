# ⚙️ Configuration Guide

CodeGuardian is designed to work out of the box with sensible defaults, but offers extensive configuration options to customize analysis behavior for your specific needs. This guide covers all configuration options and best practices.

## Configuration Files

### Primary Configuration File

CodeGuardian uses `codeguardian.toml` as its primary configuration file. You can create this file manually or use the `init` command:

```bash
# Create basic configuration
codeguardian init

# Create with specific template
codeguardian init --template security
codeguardian init --template performance
codeguardian init --template ci
```

### Configuration File Locations

CodeGuardian looks for configuration files in this order:
1. Path specified with `--config` flag
2. `./codeguardian.toml` (current directory)
3. `~/.config/codeguardian/codeguardian.toml` (user config)
4. `/etc/codeguardian.toml` (system-wide)

### Environment Variables

Configuration can also be overridden using environment variables:

```bash
# Override configuration file location
CODEGUARDIAN_CONFIG=/path/to/custom.toml

# Override specific settings
CODEGUARDIAN_MAX_FILE_SIZE=20971520
CODEGUARDIAN_MEMORY_LIMIT_MB=2048
```

## Configuration Structure

### General Settings

```toml
[general]
# Maximum file size to analyze (bytes)
max_file_size = 10485760  # 10MB

# Number of parallel workers (0 = auto-detect CPU cores)
parallel_workers = 4

# Memory limit in MB
memory_limit_mb = 1024

# Enable streaming analysis for large files (MB)
streaming_threshold_mb = 5

# Cache directory path
cache_dir = "~/.cache/codeguardian"

# Temporary directory for analysis
temp_dir = "/tmp/codeguardian"

# Log level (error, warn, info, debug, trace)
log_level = "info"

# Enable TTY-aware output
tty_output = true

# Progress reporting interval (seconds)
progress_interval = 2
```

### Analyzer Configuration

#### Security Analyzer

```toml
[analyzers.security]
enabled = true

# Secret detection
check_secrets = true
check_hardcoded_secrets = true
entropy_threshold = 4.5  # Minimum entropy for secret detection

# Vulnerability checks
check_vulnerabilities = true
check_sql_injection = true
check_xss = true
check_command_injection = true
check_path_traversal = true
check_weak_crypto = true

# Authentication & authorization
check_auth_bypass = true
check_insecure_random = true

# Information disclosure
check_info_disclosure = true
redact_secrets = true

# Custom patterns file
custom_patterns_file = "custom-security-patterns.json"
```

#### Performance Analyzer

```toml
[analyzers.performance]
enabled = true

# Memory and allocation checks
check_allocations = true
check_memory_leaks = true
check_large_objects = true

# Algorithm efficiency
check_nested_loops = true
check_string_operations = true
check_blocking_io = true

# Resource usage
check_resource_exhaustion = true
max_function_length = 100
max_complexity = 10
max_nesting_depth = 4

# Performance thresholds
memory_threshold_mb = 100
cpu_threshold_percent = 80
```

#### Code Quality Analyzer

```toml
[analyzers.code_quality]
enabled = true

# Naming conventions
check_naming = true
naming_convention = "snake_case"  # snake_case, camelCase, PascalCase

# Code structure
check_complexity = true
check_duplication = true
check_unused_code = true

# Documentation
check_documentation = true
require_doc_comments = false

# Code style
max_line_length = 120
max_function_length = 50
max_file_length = 1000

# Language-specific rules
[analyzers.code_quality.rust]
check_unsafe_usage = true
check_clippy_lints = true

[analyzers.code_quality.javascript]
check_eslint_rules = true
check_typescript_strict = true
```

#### Dependency Analyzer

```toml
[analyzers.dependency]
enabled = true

# Vulnerability scanning
check_vulnerabilities = true
check_outdated = true
check_licenses = true

# Dependency management
check_unused_dependencies = true
check_circular_dependencies = true
max_dependency_depth = 10

# Security policies
allowed_licenses = ["MIT", "Apache-2.0", "BSD-3-Clause"]
blocked_licenses = ["GPL-3.0"]
```

### ML Configuration

```toml
[ml]
enabled = true

# Model configuration
model_path = "enhanced-model.fann"
model_type = "ruf_fann"  # ruf_fann, bert, custom

# Training parameters
online_learning = true
learning_rate = 0.01
epochs = 2000
bootstrap_training = true

# Feature extraction
feature_extraction = "enhanced"
include_context = true
context_window = 5

# False positive reduction
confidence_threshold = 0.8
auto_threshold_adjustment = true
feedback_loop = true
```

### Performance Configuration

```toml
[performance]
# Caching
cache_enabled = true
cache_max_age_days = 30
cache_compression = true
cache_max_size_mb = 500

# Parallel processing
parallel_processing = true
max_parallel_tasks = 16
semaphore_limit = 8

# Memory management
memory_optimization = true
streaming_enabled = true
streaming_threshold_mb = 5
garbage_collection_interval = 100

# I/O optimization
buffered_io = true
async_io = true
io_buffer_size_kb = 64
```

### GitHub Integration

```toml
[github]
# Authentication
token = "${GITHUB_TOKEN}"
app_id = "your_app_id"
private_key_path = "/path/to/private-key.pem"

# Issue management
default_labels = ["security", "automated"]
title_prefix = "Security Alert: "
body_template = "github-issue-template.md"
max_body_size = 60000

# Rate limiting
rate_limit_buffer = 100
retry_attempts = 3
retry_delay_ms = 1000

# Issue modes
default_mode = "checklist"  # simple, checklist, children
create_issues = true
update_existing = true
close_resolved = true
```

### Output Configuration

```toml
[output]
# Format options
default_format = "human"  # json, human, sarif, markdown
color_output = true
pretty_print = true

# File output
output_dir = "./codeguardian-results"
overwrite_existing = false
compress_output = false

# Reporting
generate_summary = true
include_statistics = true
include_recommendations = true

# Custom templates
template_dir = "./templates"
custom_templates = ["security-report.md", "performance-report.html"]
```

## Configuration Templates

### Security Template

Optimized for security analysis with maximum vulnerability detection:

```bash
codeguardian init --template security
```

This creates a configuration focused on:
- Maximum security checks enabled
- Conservative thresholds for secret detection
- Comprehensive vulnerability scanning
- GitHub integration for issue tracking

### Performance Template

Optimized for performance analysis and optimization:

```bash
codeguardian init --template performance
```

Features:
- Enhanced performance monitoring
- Memory and CPU profiling
- Algorithm efficiency checks
- Resource usage optimization

### CI Template

Optimized for continuous integration environments:

```bash
codeguardian init --template ci
```

Configuration includes:
- Fast execution settings
- JSON output for automation
- Minimal resource usage
- CI-friendly error reporting

### Enterprise Template

Comprehensive analysis for enterprise environments:

```bash
codeguardian init --template enterprise
```

Includes:
- Maximum security and compliance checks
- Detailed reporting and audit trails
- Integration with enterprise tools
- Custom rule support

## Advanced Configuration

### Custom Security Rules

Create custom security patterns in JSON format:

```json
{
  "custom_rules": [
    {
      "id": "custom-sql-injection",
      "name": "Custom SQL Injection Pattern",
      "description": "Detects application-specific SQL injection patterns",
      "severity": "high",
      "pattern": "SELECT.*FROM.*WHERE.*\\$\\{[^}]+\\}",
      "language": "javascript",
      "file_pattern": "*.js",
      "confidence": 0.9
    }
  ]
}
```

Reference in configuration:
```toml
[analyzers.security]
custom_rules_file = "custom-security-rules.json"
```

### Environment-Specific Configuration

Use environment variables for dynamic configuration:

```bash
# Development
CODEGUARDIAN_LOG_LEVEL=debug codeguardian check .

# Production
CODEGUARDIAN_ML_ENABLED=false codeguardian check .

# CI Environment
CODEGUARDIAN_FORMAT=json CODEGUARDIAN_OUTPUT=results.json codeguardian check .
```

### Multi-Environment Setup

Create different configurations for different environments:

```bash
# Development
cp codeguardian.dev.toml codeguardian.toml

# Production
cp codeguardian.prod.toml codeguardian.toml

# CI
cp codeguardian.ci.toml codeguardian.toml
```

## Configuration Validation

### Validate Configuration

Check your configuration for errors:

```bash
codeguardian config validate
codeguardian config validate --config custom.toml
```

### List Available Options

View all configuration options:

```bash
codeguardian config list
```

### Generate Configuration

Generate a complete configuration file with all options:

```bash
codeguardian config generate > codeguardian.toml
```

## Best Practices

### 1. Start with Templates
Use built-in templates as a starting point:
```bash
codeguardian init --template security
```

### 2. Environment-Specific Configs
Create separate configurations for different environments:
```toml
# codeguardian.dev.toml
[general]
log_level = "debug"
[ml]
enabled = false

# codeguardian.prod.toml
[general]
log_level = "warn"
[ml]
enabled = true
```

### 3. Version Control
Keep your configuration in version control:
```bash
git add codeguardian.toml
git commit -m "Add CodeGuardian configuration"
```

### 4. Regular Updates
Review and update your configuration regularly:
```bash
# Check for configuration updates
codeguardian config check-updates
```

### 5. Performance Tuning
Monitor and adjust performance settings:
```bash
# Run with performance metrics
codeguardian check . --metrics

# Adjust based on results
codeguardian init --template performance
```

## Troubleshooting

### Common Configuration Issues

**"Configuration file not found"**
- Ensure `codeguardian.toml` exists in the current directory
- Check file permissions and path

**"Invalid configuration"**
```bash
# Validate configuration
codeguardian config validate --verbose
```

**"Settings not applied"**
- Check environment variable precedence
- Verify configuration file syntax
- Use `--config` flag to specify exact path

**"Performance issues"**
- Reduce parallel workers: `parallel_workers = 2`
- Increase memory limit: `memory_limit_mb = 2048`
- Enable streaming: `streaming_threshold_mb = 5`

### Getting Help
```bash
# View configuration help
codeguardian config --help

# List all available options
codeguardian config list --detailed

# Generate example configuration
codeguardian config generate --template security
```

## Next Steps

After configuring CodeGuardian:

- **[Basic Usage Guide](basic-usage.md)** - Learn fundamental commands
- **[Advanced Features](advanced-features.md)** - Explore ML integration and turbo mode
- **[CI/CD Integration](ci-cd-integration.md)** - Automate analysis in your pipeline

---

<div align="center">

**[⬅️ Back to User Guide](../README.md)** | **[📦 Installation Guide](installation.md)** | **[🔧 Basic Usage Guide](basic-usage.md)**

</div>