# Configuration Presets

CodeGuardian includes several preset configurations optimized for different use cases:

## Preset Configurations

### Minimal
Essential security checks only - fastest execution for basic security scanning.

### Security (Recommended)
Comprehensive security analysis with all analyzers - balanced performance and coverage.

### CI
Optimized for continuous integration with fast execution and minimal resource usage.

## Using Presets

```bash
# Use preset configuration
codeguardian init --template security

# List available templates
codeguardian init --list

# Create custom configuration
codeguardian init --interactive
```

### Minimal Preset Configuration

Essential security checks only - fastest execution for basic security scanning.

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true
compress_output = true
max_reports_kept = 10

[security]
enabled = true
fail_on_issues = false
min_severity = "medium"
check_hardcoded_secrets = true
check_unsafe_code = true

[git]
conventional_commits = true

[analysis]
enabled = true
parallel_processing = false
max_workers = 1
enable_caching = false

[performance]
enabled = false

[analyzers.security_analyzer]
enabled = true
check_hardcoded_secrets = true
check_vulnerabilities = true
min_entropy_threshold = 3.5

[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true

[files]
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java"]
skip_binaries = true
max_file_size_bytes = 1048576

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]

[ai]
enabled = false

[optimization]
enable_optimized_analyzers = false
enable_file_caching = false

[retention]
enabled = false
```

### Security Preset Configuration

Comprehensive security analysis with all analyzers - balanced performance and coverage.

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true
compress_output = true
max_reports_kept = 10

[security]
enabled = true
fail_on_issues = false
min_severity = "low"
entropy_threshold = 4.0
check_hardcoded_secrets = true
check_unsafe_code = true
check_dependencies = true
check_sql_injection = true
check_xss = true
check_command_injection = true

[git]
conventional_commits = true
require_signed_commits = false

[analysis]
enabled = true
parallel_processing = true
max_workers = 4
enable_caching = true
cache_dir = "build/.codeguardian/cache"

[performance]
enabled = true
max_complexity = 15
max_function_length = 150

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

[analyzers.dependency]
enabled = true
check_vulnerabilities = true

[analyzers.integrity]
enabled = true
hash_algorithm = "Blake3"

[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true

[files]
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java", ".cpp", ".c", ".h"]
skip_binaries = true
max_file_size_bytes = 1048576

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]
enable_escalation = true
escalation_threshold = 5

[ai]
enabled = true
enable_semantic_enrichment = true
min_confidence_threshold = 0.7

[optimization]
enable_optimized_analyzers = true
enable_file_caching = true

[retention]
enabled = true
max_age_days = 30
max_size_mb = 1000
```

### CI Preset Configuration

Optimized for continuous integration with fast execution and minimal resource usage.

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true
compress_output = true
max_reports_kept = 5

[security]
enabled = true
fail_on_issues = true
min_severity = "medium"
check_hardcoded_secrets = true
check_unsafe_code = true

[git]
conventional_commits = true

[analysis]
enabled = true
parallel_processing = true
max_workers = 2
timeout_seconds = 300
enable_caching = true

[performance]
enabled = false

[analyzers.security_analyzer]
enabled = true
check_hardcoded_secrets = true
check_vulnerabilities = true

[analyzers.code_quality]
enabled = true
check_naming = true
check_complexity = true

[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true

[files]
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/", "test/", "tests/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java"]
skip_binaries = true
max_file_size_bytes = 1048576

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]

[integrations.github]
enabled = true
create_issues = false
comment_prs = true
min_severity = "high"

[ai]
enabled = false

[optimization]
enable_optimized_analyzers = true
max_parallel_workers = 2

[retention]
enabled = false
```

## Configuration Examples

### Security-Focused Configuration

```toml
[output]
directory = "build/analysis-results"
format = "sarif"
verbose = false
generate_summary = true
compress_output = true
max_reports_kept = 20

[security]
enabled = true
fail_on_issues = true
min_severity = "medium"
entropy_threshold = 4.0
max_file_size_bytes = 1048576
check_hardcoded_secrets = true
check_unsafe_code = true
check_dependencies = true
check_sql_injection = true
check_xss = true
check_command_injection = true

[git]
conventional_commits = true

[analysis]
enabled = true
parallel_processing = true
max_workers = 8
timeout_seconds = 600
enable_caching = true
enable_ai_enhancement = true

[performance]
enabled = false

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

[analyzers.dependency]
enabled = true
check_vulnerabilities = true

[analyzers.integrity]
enabled = true
hash_algorithm = "Blake3"

[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true

[analyzers.performance_analyzer]
enabled = false

[analyzers.code_quality]
enabled = false

[files]
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java", ".cpp", ".c", ".h", ".go", ".php", ".rb"]
skip_binaries = true
max_file_size_bytes = 1048576

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]
enable_escalation = true
escalation_threshold = 5

[integrations.github]
enabled = true
create_issues = true
issue_labels = ["security", "codeguardian", "critical"]
comment_prs = true
min_severity = "high"
duplicate_prevention = true

[ai]
enabled = true
enable_semantic_enrichment = true
enable_relationship_detection = true
enable_insight_generation = true
min_confidence_threshold = 0.8
max_processing_time = 600

[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 8

[retention]
enabled = true
max_age_days = 30
max_size_mb = 1000
enable_integrity_check = true
```

### Performance-Focused Configuration

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true
compress_output = true
max_reports_kept = 10

[security]
enabled = false

[git]
conventional_commits = true

[analysis]
enabled = true
parallel_processing = true
max_workers = 16
enable_caching = true

[performance]
enabled = true
check_allocations = true
check_async_blocking = true
max_complexity = 15
max_function_length = 150
enable_profiling = true
max_memory_usage_mb = 4096
max_cpu_usage_percent = 90

[performance.monitoring]
enabled = true
metrics_collection = true
reporting_interval_seconds = 30

[analyzers.performance_analyzer]
enabled = true
check_nested_loops = true
check_string_operations = true
check_blocking_io = true
check_algorithms = true
check_memory_usage = true
check_io_operations = true
max_complexity = 15
max_function_length = 50
max_loop_depth = 3

[analyzers.code_quality]
enabled = true
check_complexity = true
check_deep_nesting = true
max_complexity = 15
max_nesting_depth = 6

[analyzers.security_analyzer]
enabled = false

[analyzers.broken_files]
enabled = false

[files]
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java", ".cpp", ".c"]
skip_binaries = true
max_file_size_bytes = 1048576

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]

[ai]
enabled = false

[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 16
max_memory_file_size = 1048576

[optimization.cache_cleanup]
enabled = true
max_age_days = 7

[optimization.early_termination]
enabled = true
max_analysis_time_seconds = 60

[retention]
enabled = true
max_age_days = 7
max_size_mb = 500
```

### CI/CD Configuration

```toml
[output]
directory = "build/analysis-results"
format = "json"
verbose = false
generate_summary = true
compress_output = true
max_reports_kept = 5

[security]
enabled = true
fail_on_issues = true
min_severity = "medium"
check_hardcoded_secrets = true
check_unsafe_code = true

[git]
conventional_commits = true

[analysis]
enabled = true
parallel_processing = true
max_workers = 4
timeout_seconds = 600
enable_caching = true

[performance]
enabled = false

[analyzers.security_analyzer]
enabled = true
check_hardcoded_secrets = true
check_vulnerabilities = true
check_sql_injection = true
check_xss = true
check_command_injection = true
min_entropy_threshold = 3.5

[analyzers.code_quality]
enabled = true
check_naming = true
check_complexity = true
check_duplication = true
max_complexity = 10
max_line_length = 120

[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true

[analyzers.dependency]
enabled = true
check_vulnerabilities = true

[files]
exclude_patterns = ["*.log", "*.tmp", "target/", "node_modules/", ".git/", "test/", "tests/", "spec/", "specs/"]
analyze_extensions = [".rs", ".js", ".ts", ".py", ".java", ".cpp", ".c"]
skip_binaries = true
max_file_size_bytes = 1048576

[severity]
custom_levels = ["info", "low", "medium", "high", "critical"]
enable_escalation = true
escalation_threshold = 5

[integrations.github]
enabled = true
create_issues = false
comment_prs = true
min_severity = "medium"
duplicate_prevention = true

[ai]
enabled = false

[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 4

[optimization.early_termination]
enabled = true
max_analysis_time_seconds = 300

[retention]
enabled = false
```

For basic configuration options like output, security, analysis, and files, see [configuration-basics.md](configuration-basics.md).

For advanced features like AI, integrations, dashboard, and remediation, see [configuration-advanced.md](configuration-advanced.md).

For CLI options, environment variables, and validation, see [cli-reference.md](cli-reference.md).