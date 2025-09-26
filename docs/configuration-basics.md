# Configuration Basics

CodeGuardian works out of the box with sensible defaults, but can be customized via `codeguardian.toml`:

## Basic Configuration

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true
compress_output = true
reports_subdirectory = "reports"
data_subdirectory = "data"
temp_subdirectory = "temp"
historical_subdirectory = "historical"
auto_archive = true
max_reports_kept = 10

[security]
enabled = true
fail_on_issues = false
min_severity = "low"
max_file_size = 1048576
entropy_threshold = 4.5
max_file_size_bytes = 1048576
vulnerability_threshold = "medium"
check_hardcoded_secrets = true
check_unsafe_code = true
check_dependencies = true
secret_patterns = [
    "(?i)(password|passwd|pwd)\\s*[:=]\\s*['\"][^'\"]{8,}['\"]",
    "(?i)(api[_-]?key|apikey)\\s*[:=]\\s*['\"][^'\"]{16,}['\"]",
    "(?i)(secret|token)\\s*[:=]\\s*['\"][^'\"]{16,}['\"]",
    "(?i)(private[_-]?key)\\s*[:=]\\s*['\"][^'\"]{32,}['\"]"
]
check_sql_injection = true
check_xss = true
check_command_injection = true

[git]
conventional_commits = true
commit_template = "{type}({scope}): {description}"
require_signed_commits = false

[analysis]
enabled = true
analyze_binaries = false
timeout_seconds = 300
parallel_processing = true
max_workers = 4
enable_caching = true
cache_dir = "build/.codeguardian/cache"
cache_expiration_days = 7
enable_ai_enhancement = false

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
capacity = 1000
expiration_seconds = 3600
eviction_policy = "lru"

# Memory pool settings for object reuse optimization
[performance.memory_pools]
enabled = true
findings_pool_size = 1000
strings_pool_size = 5000
pathbuf_pool_size = 2000
hashmap_pool_size = 500

# Enhanced cache settings for analysis result caching
[performance.enhanced_cache]
enabled = true
memory_limit_mb = 256
pool_integration = true
compression_enabled = false

# Performance monitoring settings
[performance.monitoring]
enabled = true
metrics_collection = true
reporting_interval_seconds = 60

[analyzers]
[analyzers.integrity]
enabled = true
hash_algorithm = "Blake3"
baseline_file = "build/.codeguardian/integrity.baseline"
auto_update_baseline = false
check_permissions = true
check_binary_files = false
verify_checksums = true
max_file_size = 1048576

[analyzers.lint_drift]
enabled = false
config_files = ["Cargo.toml", "package.json", ".eslintrc.json"]
baseline_file = "build/.codeguardian/lint_drift.baseline"
auto_update_baseline = false
strict_mode = false

[analyzers.non_production]
enabled = true
exclude_test_files = true
exclude_example_files = true
custom_test_directories = ["tests", "test", "spec", "specs", "__tests__", "testdata", "fixtures", "mocks"]
custom_test_extensions = [".test.rs", ".spec.rs", ".integration.rs", ".e2e.rs"]
fuzzy_test_patterns = [
    "(?i)test.*\\.rs$",
    "(?i)spec.*\\.rs$",
    "(?i).*test.*",
    "(?i).*spec.*"
]

[[analyzers.non_production.patterns]]
pattern = "(?i)\\b(todo|fixme|hack|xxx)\\b"
description = "Non-production code markers"
severity = "medium"

[[analyzers.non_production.patterns]]
pattern = "(?i)\\bconsole\\.log\\b"
description = "Debug logging statements"
severity = "low"

[analyzers.dependency]
enabled = true
check_outdated = true
check_vulnerabilities = true
check_unused = true
check_duplicates = true
check_licenses = true
max_age_days = 365
vulnerability_databases = ["https://cve.mitre.org", "https://nvd.nist.gov"]

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
patterns = [
    "add content here",
    "implement this",
    "your code here",
    "placeholder",
    "todo: implement",
    "fill in the details",
    "complete this",
    "add your logic"
]
custom_patterns = []

[analyzers.broken_files.duplicates]
enabled = true
min_lines = 10
focus_security = true
ignore_test_files = true
max_files_to_compare = 1000
enable_ml_similarity = true
similarity_threshold = 0.8
enable_github_prevention = true

[analyzers.broken_files.duplicates.cache]
enabled = true
max_size_mb = 100
expiration_hours = 24
max_entries = 1000

[analyzers.duplicate_analyzer]
enabled = true
min_lines = 10
focus_security = true
ignore_test_files = true
max_files_to_compare = 1000
enable_ml_similarity = true
similarity_threshold = 0.8
enable_github_prevention = true

[analyzers.duplicate_analyzer.cache]
enabled = true
max_size_mb = 100
expiration_hours = 24
max_entries = 1000

[files]
include_patterns = []
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/", "dist/", "build/", "*.min.js", "*.min.css", "vendor/", "third_party/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java", ".cpp", ".c", ".h", ".go", ".php", ".rb", ".cs", ".swift", ".kt", ".scala", ".html", ".css", ".json", ".xml", ".yaml", ".yml", ".toml", ".md", ".txt", ".sh", ".bat", ".ps1"]
skip_binaries = true
max_file_size_bytes = 1048576
skip_large_files = true

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]
enable_escalation = true
escalation_threshold = 5
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

Based on recent benchmark data, these optimizations provide:
- **Memory usage**: Average of 173-172 MB during analysis
- **Execution time**: Average of 1024-985 ms per analysis run
- **Cache hit rate**: 78-85% for improved performance
- **Memory reduction trends**: -0.15 to -0.41 MB/day over time
- **Pattern matching speeds**: 89-125 ms for typical operations
- **Object reuse rates**: 75-85% for efficient resource utilization
- **Real-time performance monitoring** with configurable reporting
- **Configurable cache sizes** based on available system resources

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
max_file_size_bytes = 1048576        # Maximum file size to analyze (1MB)
entropy_threshold = 4.5               # Entropy threshold for secret detection
check_hardcoded_secrets = true        # Check for hardcoded secrets
check_unsafe_code = true              # Check for unsafe code patterns
check_dependencies = true             # Check dependency vulnerabilities
```

### Git Configuration

CodeGuardian integrates with Git for commit analysis and validation.

```toml
[git]
conventional_commits = true
commit_template = "{type}({scope}): {description}"
require_signed_commits = false
```

#### Configuration Options

- `conventional_commits`: Enforce conventional commit message format
- `commit_template`: Template for commit messages (supports placeholders: {type}, {scope}, {description})
- `require_signed_commits`: Require commits to be GPG signed

#### Conventional Commits

When `conventional_commits` is enabled, CodeGuardian validates commit messages against the conventional commits specification:

- Format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- Scope: Optional component name in parentheses
- Description: Brief description of changes

#### Commit Templates

The `commit_template` allows customization of expected commit message formats:

```toml
# Standard conventional commits
commit_template = "{type}({scope}): {description}"

# Custom format
commit_template = "[{type}] {scope}: {description}"

# Simple format
commit_template = "{type}: {description}"
```

#### Signed Commits

When `require_signed_commits` is enabled, CodeGuardian verifies that commits are cryptographically signed using GPG keys, ensuring commit authenticity and integrity.

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
baseline_file = "build/.codeguardian/integrity.baseline"
auto_update_baseline = false
check_permissions = true
check_binary_files = false
verify_checksums = true
max_file_size = 1048576
```

### Lint Drift Analyzer

```toml
[analyzers.lint_drift]
enabled = false
config_files = ["Cargo.toml", "package.json", ".eslintrc.json"]
baseline_file = "build/.codeguardian/lint_drift.baseline"
auto_update_baseline = false
strict_mode = false
```

### Non-Production Analyzer

```toml
[analyzers.non_production]
enabled = true
exclude_test_files = true
exclude_example_files = true
custom_test_directories = ["tests", "test", "spec", "specs", "__tests__", "testdata", "fixtures", "mocks"]
custom_test_extensions = [".test.rs", ".spec.rs", ".integration.rs", ".e2e.rs"]
fuzzy_test_patterns = [
    "(?i)test.*\\.rs$",
    "(?i)spec.*\\.rs$",
    "(?i).*test.*",
    "(?i).*spec.*"
]

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
patterns = [
    "add content here",
    "implement this",
    "your code here",
    "placeholder",
    "todo: implement",
    "fill in the details",
    "complete this",
    "add your logic"
]
custom_patterns = []

[analyzers.broken_files.duplicates]
enabled = true
min_lines = 10
focus_security = true
ignore_test_files = true
max_files_to_compare = 1000
enable_ml_similarity = true
similarity_threshold = 0.8
enable_github_prevention = true

[analyzers.broken_files.duplicates.cache]
enabled = true
max_size_mb = 100
expiration_hours = 24
max_entries = 1000
```

### Duplicate Analyzer

```toml
[analyzers.duplicate_analyzer]
enabled = true
min_lines = 10
focus_security = true
ignore_test_files = true
max_files_to_compare = 1000
enable_ml_similarity = true
similarity_threshold = 0.8
enable_github_prevention = true

[analyzers.duplicate_analyzer.cache]
enabled = true
max_size_mb = 100
expiration_hours = 24
max_entries = 1000
```

### File Configuration

```toml
[files]
include_patterns = []
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/", "dist/", "build/", "*.min.js", "*.min.css", "vendor/", "third_party/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java", ".cpp", ".c", ".h", ".go", ".php", ".rb", ".cs", ".swift", ".kt", ".scala", ".html", ".css", ".json", ".xml", ".yaml", ".yml", ".toml", ".md", ".txt", ".sh", ".bat", ".ps1"]
skip_binaries = true
max_file_size_bytes = 1048576
skip_large_files = true
```

### Severity Configuration

CodeGuardian allows customization of severity levels and escalation policies for findings.

```toml
[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]
enable_escalation = true
escalation_threshold = 5
```

#### Configuration Options

- `custom_levels`: Array of severity levels in ascending order of criticality
- `enable_escalation`: Enable automatic severity escalation based on finding patterns
- `escalation_threshold`: Number of similar findings required to trigger escalation

#### Custom Severity Levels

Define your own severity hierarchy to match organizational policies:

```toml
# Standard levels
custom_levels = ["info", "low", "medium", "high", "critical"]

# Extended levels
custom_levels = ["trace", "debug", "info", "warning", "error", "fatal"]

# Custom business levels
custom_levels = ["minor", "major", "critical", "blocker"]
```

#### Escalation Settings

When `enable_escalation` is enabled, CodeGuardian can automatically increase severity based on:

- **Frequency**: Multiple instances of the same issue type
- **Context**: Issues found in critical components
- **Patterns**: Security issues that form attack chains

The `escalation_threshold` determines how many similar findings are needed to trigger escalation:

```toml
# Escalate after 3 similar findings
escalation_threshold = 3

# More conservative escalation
escalation_threshold = 10
```

#### Examples

```toml
# Development environment - detailed reporting
[severity]
custom_levels = ["trace", "debug", "info", "low", "medium", "high", "critical"]
enable_escalation = false

# Production environment - focused on critical issues
[severity]
custom_levels = ["low", "medium", "high", "critical"]
enable_escalation = true
escalation_threshold = 2

# Compliance-focused configuration
[severity]
custom_levels = ["info", "minor", "major", "critical", "compliance"]
enable_escalation = true
escalation_threshold = 1
```

## Troubleshooting

### Common Configuration Issues

1. **Configuration file not found**
   - Ensure `codeguardian.toml` exists in the current directory or specify path with `--config`
   - Use `codeguardian init --default` to create a basic configuration

2. **Invalid configuration values**
   - Run `codeguardian check --config codeguardian.toml --verbose` to see validation errors
   - Check TOML syntax with a TOML validator

3. **Performance issues**
   - Reduce `max_workers` in `[analysis]` if memory usage is too high
   - Increase `max_memory_usage_mb` in `[performance]` for large codebases
   - Adjust `max_file_size_bytes` in `[files]` for very large files

4. **Analyzer-specific issues**
   - Check analyzer `enabled` flags in `[analyzers.*]` sections
   - Verify file patterns in `[files]` match your codebase
   - Review analyzer-specific settings for appropriate thresholds

For advanced features like AI, integrations, dashboard, and remediation, see [configuration-advanced.md](configuration-advanced.md).

For preset configurations and templates, see [configuration-presets.md](configuration-presets.md).

For CLI options, environment variables, and validation, see [cli-reference.md](cli-reference.md).