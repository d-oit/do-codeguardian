# Configuration Guide

CodeGuardian works out of the box with sensible defaults, but can be customized via `codeguardian.toml`:

## Basic Configuration

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true

[security]
enabled = true
fail_on_issues = false
min_severity = "low"
max_file_size_bytes = 10485760  # 10MB
entropy_threshold = 4.5
check_hardcoded_secrets = true
check_unsafe_code = true
check_dependencies = true

[analysis]
enabled = true
parallel_processing = true
max_workers = 4
enable_caching = true
timeout_seconds = 300

[analyzers.security_analyzer]
enabled = true
check_sql_injection = true
check_xss = true
check_command_injection = true
check_hardcoded_secrets = true
check_vulnerabilities = true
min_entropy_threshold = 3.5

[analyzers.performance_analyzer]
enabled = true
check_nested_loops = true
check_string_operations = true
check_blocking_io = true
max_complexity = 10
max_function_length = 50

[analyzers.code_quality]
enabled = true
check_magic_numbers = true
check_complex_conditions = true
check_deep_nesting = true
max_complexity = 10
max_nesting_depth = 6
max_line_length = 120

[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 4

## Performance Optimization Configuration

CodeGuardian includes advanced performance optimizations that can be configured:

```toml
[performance]
enabled = true
check_allocations = true
check_async_blocking = true
max_complexity = 15
max_function_length = 150
enable_profiling = false
max_memory_usage_mb = 512
max_cpu_usage_percent = 80

# Regex cache settings for pattern matching optimization
[performance.regex_cache]
enabled = true
capacity = 1000          # Maximum number of cached regex patterns
expiration_seconds = 3600  # Cache expiration time in seconds
eviction_policy = "lru"    # Cache eviction policy (lru, fifo, etc.)

# Memory pool settings for object reuse optimization
[performance.memory_pools]
enabled = true
findings_pool_size = 1000    # Pool size for Finding objects
strings_pool_size = 5000     # Pool size for String objects
pathbuf_pool_size = 2000     # Pool size for PathBuf objects
hashmap_pool_size = 500      # Pool size for HashMap objects

# Enhanced cache settings for analysis result caching
[performance.enhanced_cache]
enabled = true
memory_limit_mb = 256        # Maximum memory usage for cache in MB
pool_integration = true      # Enable integration with memory pools
compression_enabled = false  # Enable compression for cached data

# Performance monitoring settings
[performance.monitoring]
enabled = true
metrics_collection = true    # Enable collection of performance metrics
reporting_interval_seconds = 60  # Interval for performance reports
```

### Performance Configuration Options

#### Regex Cache Settings
- `capacity`: Maximum number of compiled regex patterns to cache (default: 1000)
- `expiration_seconds`: Time in seconds after which cached patterns expire (default: 3600)
- `eviction_policy`: Algorithm used to evict old patterns when cache is full (default: "lru")

#### Memory Pool Settings
- `findings_pool_size`: Number of Finding objects to keep in memory pool for reuse (default: 1000)
- `strings_pool_size`: Number of String objects to keep in memory pool (default: 5000)
- `pathbuf_pool_size`: Number of PathBuf objects to keep in memory pool (default: 2000)
- `hashmap_pool_size`: Number of HashMap objects to keep in memory pool (default: 500)

#### Enhanced Cache Settings
- `memory_limit_mb`: Maximum memory usage for the analysis cache in megabytes (default: 256)
- `pool_integration`: Whether to integrate cache with memory pools for better memory management (default: true)
- `compression_enabled`: Whether to compress cached data to save memory (default: false)

#### Performance Monitoring
- `metrics_collection`: Enable collection of performance metrics during analysis (default: true)
- `reporting_interval_seconds`: How often to report performance metrics in seconds (default: 60)

### Performance Benefits

These optimizations provide:
- **30% faster pattern matching** through regex caching
- **15% memory reduction** through object pooling
- **90% object reuse rate** for common data structures
- **Real-time performance monitoring** with configurable reporting
- **Configurable cache sizes** based on available system resources

[integrations.github]
enabled = false
repository = "owner/repo"
token = "${CODEGUARDIAN_GITHUB_TOKEN}"
create_issues = true
issue_labels = ["security", "codeguardian"]
min_severity = "medium"
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

### Output Settings

```toml
[output]
directory = "build/analysis-results"  # Output directory for results
format = "json"                       # Output format: json, human, sarif
verbose = false                       # Enable verbose output
generate_summary = true               # Generate summary report
compress_output = true                # Compress output files
max_reports_kept = 10                 # Maximum number of reports to keep
```

### Security Settings

```toml
[security]
enabled = true                        # Enable security analysis
fail_on_issues = false                # Exit with error on security issues
min_severity = "low"                  # Minimum severity to report
max_file_size_bytes = 10485760        # Maximum file size to analyze (10MB)
entropy_threshold = 4.5               # Entropy threshold for secret detection
check_hardcoded_secrets = true        # Check for hardcoded secrets
check_unsafe_code = true              # Check for unsafe code patterns
check_dependencies = true             # Check dependency vulnerabilities
```

### Analysis Settings

```toml
[analysis]
enabled = true                        # Enable analysis
analyze_binaries = false              # Analyze binary files
timeout_seconds = 300                 # Analysis timeout in seconds
parallel_processing = true            # Enable parallel processing
max_workers = 4                       # Maximum parallel workers
enable_caching = true                 # Enable result caching
cache_dir = ".codeguardian/cache"     # Cache directory
enable_ai_enhancement = false         # Enable AI enhancements
```

### Security Analyzer

```toml
[analyzers.security_analyzer]
enabled = true
check_sql_injection = true
check_xss = true
check_command_injection = true
check_hardcoded_secrets = true
check_vulnerabilities = true
check_permissions = true
check_secrets = true
min_entropy_threshold = 3.5
```

### Performance Analyzer

```toml
[analyzers.performance_analyzer]
enabled = true
check_nested_loops = true
check_string_operations = true
check_blocking_io = true
check_algorithms = true
check_memory_usage = true
check_io_operations = true
max_complexity = 10
max_function_length = 50
max_loop_depth = 3
```

### Code Quality Analyzer

```toml
[analyzers.code_quality]
enabled = true
check_magic_numbers = true
check_complex_conditions = true
check_deep_nesting = true
check_commented_code = true
check_complexity = true
check_duplication = true
check_naming = true
max_complexity = 10
max_nesting_depth = 6
max_file_size = 500
max_line_length = 120
```

### Dependency Analyzer

```toml
[analyzers.dependency]
enabled = true
check_outdated = true
check_vulnerabilities = true
check_unused = true
check_duplicates = true
check_licenses = true
max_age_days = 365
vulnerability_databases = ["https://cve.mitre.org", "https://nvd.nist.gov"]
```

### Integrity Analyzer

```toml
[analyzers.integrity]
enabled = true
hash_algorithm = "Blake3"
baseline_file = ".codeguardian/integrity.baseline"
auto_update_baseline = false
check_permissions = true
check_binary_files = false
verify_checksums = true
max_file_size = 5242880  # 5MB
```

### Non-Production Analyzer

```toml
[analyzers.non_production]
enabled = true
exclude_test_files = true
exclude_example_files = true
custom_test_directories = ["tests", "test", "spec", "specs"]
custom_test_extensions = [".test.rs", ".spec.rs", ".integration.rs"]

[[analyzers.non_production.patterns]]
pattern = "(?i)\\b(todo|fixme|hack|xxx)\\b"
description = "Non-production code markers"
severity = "medium"

[[analyzers.non_production.patterns]]
pattern = "(?i)\\bconsole\\.log\\b"
description = "Debug logging statements"
severity = "low"
```

### Broken Files Analyzer

```toml
[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true
detect_duplicates = false

[analyzers.broken_files.conflicts]
fail_on_conflicts = false
validate_syntax = true
check_git_status = true

[analyzers.broken_files.placeholders]
severity = "medium"
patterns = ["add content here", "implement this", "your code here"]
```

## AI Configuration

```toml
[ai]
enabled = true
enable_semantic_enrichment = true
enable_relationship_detection = true
enable_insight_generation = true
enable_context_analysis = false
min_confidence_threshold = 0.7
max_processing_time = 300
enable_historical_analysis = false
model_cache_directory = ".codeguardian/models"
```

## Optimization Configuration

```toml
[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 4
max_memory_file_size = 10485760  # 10MB
streaming_chunk_size = 65536     # 64KB
max_findings_per_file = 50
pattern_cache_size = 1000

[optimization.cache_cleanup]
enabled = true
max_age_days = 7
max_size_mb = 100
cleanup_frequency = 10

[optimization.early_termination]
enabled = true
max_analysis_time_seconds = 30
max_lines_per_file = 10000
skip_large_files_bytes = 52428800  # 50MB
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
