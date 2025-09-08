# Configuration Guide

CodeGuardian works out of the box with sensible defaults, but can be customized via `do-codeguardian.toml`:

## Basic Configuration

```toml
[general]
max_file_size = 10485760  # 10MB
parallel_workers = 4       # Auto-detected CPU cores
memory_limit_mb = 1024     # Memory limit
streaming_threshold_mb = 5 # Enable streaming for large files

[analyzers.security]
enabled = true
check_secrets = true
check_vulnerabilities = true
check_hardcoded_secrets = true
check_sql_injection = true
check_xss = true
check_command_injection = true
entropy_threshold = 4.5

[analyzers.performance]
enabled = true
check_allocations = true
check_nested_loops = true
check_string_operations = true
check_blocking_io = true
max_complexity = 10
max_function_length = 100

[analyzers.code_quality]
enabled = true
check_naming = true
check_complexity = true
check_duplication = true
max_nesting_depth = 4
max_line_length = 120

[ml]
enabled = true
model_path = "enhanced-model.fann"
online_learning = true
feature_extraction = "enhanced"

[performance]
cache_enabled = true
cache_max_age_days = 30
parallel_processing = true
memory_optimization = true
compression_enabled = true

[github]
default_labels = ["security", "automated"]
title_prefix = "Security Alert: "
max_body_size = 60000
rate_limit_buffer = 100
```

## Preset Configurations

CodeGuardian includes several preset configurations optimized for different use cases:

### Minimal
Essential security checks only - fastest execution for basic security scanning.

### Security (Recommended)
Comprehensive security analysis with all analyzers - balanced performance and coverage.

### CI
Optimized for continuous integration with fast execution and minimal resource usage.

### Performance
Focus on performance bottlenecks and optimization opportunities.

### Enterprise
Full analysis suite with maximum security coverage and detailed reporting.

## Using Presets

```bash
# Use preset configuration
do-codeguardian init --template security

# List available templates
do-codeguardian init --list

# Create custom configuration
do-codeguardian init --interactive
```

## Configuration Sections

### General Settings

```toml
[general]
max_file_size = 10485760        # Maximum file size to analyze (10MB)
parallel_workers = 4            # Number of parallel analysis workers
memory_limit_mb = 1024          # Memory limit in MB
streaming_threshold_mb = 5      # Enable streaming for files larger than this
timeout_seconds = 300           # Analysis timeout per file
verbose = false                 # Enable verbose logging
quiet = false                   # Suppress progress output
```

### Security Analyzer

```toml
[analyzers.security]
enabled = true
check_secrets = true
check_vulnerabilities = true
check_hardcoded_secrets = true
check_sql_injection = true
check_xss = true
check_command_injection = true
check_path_traversal = true
check_weak_crypto = true
check_info_disclosure = true
check_insecure_random = true
entropy_threshold = 4.5
max_secret_length = 100
```

### Performance Analyzer

```toml
[analyzers.performance]
enabled = true
check_allocations = true
check_nested_loops = true
check_string_operations = true
check_blocking_io = true
check_memory_leaks = true
max_complexity = 10
max_function_length = 100
max_nesting_depth = 5
```

### Code Quality Analyzer

```toml
[analyzers.code_quality]
enabled = true
check_naming = true
check_complexity = true
check_duplication = true
check_style = true
max_nesting_depth = 4
max_line_length = 120
max_function_length = 50
naming_convention = "snake_case"
```

### Dependency Analyzer

```toml
[analyzers.dependency]
enabled = true
check_vulnerabilities = true
check_licenses = true
check_outdated = true
check_unused = true
allowed_licenses = ["MIT", "Apache-2.0", "BSD-3-Clause"]
```

### Integrity Analyzer

```toml
[analyzers.integrity]
enabled = true
check_file_integrity = true
check_git_history = true
check_commit_signatures = true
check_author_verification = true
```

### Naming Analyzer

```toml
[analyzers.naming]
enabled = true
check_variables = true
check_functions = true
check_classes = true
check_constants = true
variable_pattern = "^[a-z_][a-z0-9_]*$"
function_pattern = "^[a-z_][a-z0-9_]*$"
class_pattern = "^[A-Z][a-zA-Z0-9]*$"
constant_pattern = "^[A-Z_][A-Z0-9_]*$"
```

### Non-Production Analyzer

```toml
[analyzers.non_production]
enabled = true
check_debug_code = true
check_test_code = true
check_todo_comments = true
check_placeholder_values = true
allowed_debug_functions = ["println!", "dbg!"]
```

### Optimized Analyzer

```toml
[analyzers.optimized]
enabled = true
check_inefficient_algorithms = true
check_memory_usage = true
check_cpu_usage = true
check_io_operations = true
max_iterations = 1000000
max_memory_mb = 1024
```

## ML Configuration

```toml
[ml]
enabled = true
model_path = "enhanced-model.fann"
online_learning = true
feature_extraction = "enhanced"
confidence_threshold = 0.8
training_epochs = 2000
bootstrap_sampling = true
cross_validation = true
```

## Performance Configuration

```toml
[performance]
cache_enabled = true
cache_max_age_days = 30
cache_max_size_mb = 512
parallel_processing = true
memory_optimization = true
compression_enabled = true
streaming_enabled = true
adaptive_parallelism = true
resource_monitoring = true
```

## GitHub Integration

CodeGuardian supports seamless integration with GitHub for automated issue creation and pull request comments.

### Basic Configuration

```toml
[integrations.github]
enabled = true
repository = "owner/repo"
token = "${CODEGUARDIAN_GITHUB_TOKEN}"
create_issues = true
issue_labels = ["security", "codeguardian"]
comment_prs = true
min_severity = "high"
```

### Environment Variables

Set the GitHub token securely using environment variables:

```bash
export CODEGUARDIAN_GITHUB_TOKEN="your_github_token_here"
```

### Configuration Options

- `enabled`: Enable/disable GitHub integration (default: true)
- `repository`: GitHub repository in "owner/repo" format
- `token`: GitHub personal access token (supports environment variables)
- `create_issues`: Automatically create GitHub issues for findings
- `issue_labels`: Labels to apply to created issues
- `comment_prs`: Comment on pull requests with findings
- `min_severity`: Minimum severity level to create issues/PR comments

### Security Best Practices

1. **Use Environment Variables**: Never hardcode tokens in configuration files
2. **Token Permissions**: Use tokens with minimal required permissions
3. **Repository Access**: Limit token access to specific repositories
4. **Token Rotation**: Regularly rotate GitHub tokens

### CI/CD Integration

For CI/CD pipelines, set the environment variable in your pipeline configuration:

```yaml
# GitHub Actions
env:
  CODEGUARDIAN_GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

# GitLab CI
variables:
  CODEGUARDIAN_GITHUB_TOKEN: $GITHUB_TOKEN
```

## Output Configuration

```toml
[output]
format = "json"                    # json, human, sarif
output_file = "results.json"
emit_markdown = true
emit_html = true
emit_github = false
markdown_file = "report.md"
html_file = "report.html"
sarif_file = "results.sarif"
```

## Logging Configuration

```toml
[logging]
level = "info"                     # trace, debug, info, warn, error
file = "do-codeguardian.log"
console = true
structured = false
max_file_size_mb = 10
max_files = 5
```

## Advanced Configuration

### Custom Rules

```toml
[custom_rules]
enabled = true
rules_file = "custom-rules.json"
override_defaults = false
```

### Environment Variables

CodeGuardian supports environment variable substitution in configuration files using the `${VARIABLE_NAME}` syntax.

#### Supported Environment Variables

- `CODEGUARDIAN_CONFIG` - Path to configuration file
- `CODEGUARDIAN_CACHE_DIR` - Cache directory path
- `CODEGUARDIAN_LOG_LEVEL` - Logging level
- `CODEGUARDIAN_GITHUB_TOKEN` - GitHub API token for integration
- `CODEGUARDIAN_GITLAB_TOKEN` - GitLab API token for integration
- `CODEGUARDIAN_ML_MODEL` - ML model path
- `CODEGUARDIAN_MEMORY_LIMIT` - Memory limit in MB
- `CODEGUARDIAN_TIMEOUT` - Analysis timeout in seconds

#### Usage Examples

```toml
[integrations.github]
enabled = true
repository = "myorg/myrepo"
token = "${CODEGUARDIAN_GITHUB_TOKEN}"

[integrations.gitlab]
enabled = true
project = "mygroup/myproject"
token = "${CODEGUARDIAN_GITLAB_TOKEN}"
```

If an environment variable is not set, the placeholder remains unchanged, maintaining backward compatibility.

### Configuration Validation

Validate your configuration:

```bash
# Validate configuration file
do-codeguardian validate --config do-codeguardian.toml

# Check environment compatibility
do-codeguardian doctor

# Show current configuration
do-codeguardian config --show
```

### Configuration Examples

#### Security-Focused Configuration

```toml
[general]
parallel_workers = 8
memory_limit_mb = 2048

[analyzers.security]
enabled = true
check_secrets = true
check_vulnerabilities = true
check_hardcoded_secrets = true
entropy_threshold = 4.0

[analyzers.performance]
enabled = false

[ml]
enabled = true
model_path = "security-model.fann"
confidence_threshold = 0.9

[github]
enabled = true
default_labels = ["security", "critical"]
```

#### Performance-Focused Configuration

```toml
[general]
parallel_workers = 16
memory_limit_mb = 4096
streaming_threshold_mb = 10

[analyzers.performance]
enabled = true
check_allocations = true
check_nested_loops = true
max_complexity = 15

[analyzers.security]
enabled = false

[performance]
cache_enabled = true
parallel_processing = true
adaptive_parallelism = true
```

#### CI/CD Configuration

```toml
[general]
parallel_workers = 4
memory_limit_mb = 1024
timeout_seconds = 600

[analyzers.security]
enabled = true
check_secrets = true

[analyzers.code_quality]
enabled = true
check_naming = true

[output]
format = "json"
emit_markdown = true
emit_github = true

[github]
enabled = true
issue_template = "simple"
dry_run = false
```

## Troubleshooting

### Common Configuration Issues

1. **Configuration file not found**
   - Ensure `do-codeguardian.toml` exists in the current directory or specify path with `--config`

2. **Invalid configuration values**
   - Use `do-codeguardian validate --config do-codeguardian.toml` to check for errors

3. **Performance issues**
   - Reduce `parallel_workers` if memory usage is too high
   - Increase `memory_limit_mb` for large codebases
   - Enable `streaming_enabled` for files larger than `streaming_threshold_mb`

4. **ML model issues**
   - Ensure `model_path` points to a valid FANN model file
   - Check `confidence_threshold` is between 0.0 and 1.0
   - Disable ML with `enabled = false` if experiencing issues

5. **GitHub integration issues**
   - Verify `GITHUB_TOKEN` environment variable is set
   - Check `repo` format is `owner/repo`
   - Use `dry_run = true` to test without creating issues
