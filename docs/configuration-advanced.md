# Advanced Configuration

This document covers advanced configuration options for CodeGuardian, including AI features, integrations, dashboard, and remediation settings.

## AI Configuration

CodeGuardian's AI features enhance analysis through semantic understanding and relationship detection.

### Basic Configuration

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
model_cache_directory = "build/.codeguardian/models"
```

### Configuration Options

- `enabled`: Enable/disable AI features (default: true)
- `enable_semantic_enrichment`: Enhance findings with semantic context and natural language descriptions
- `enable_relationship_detection`: Identify relationships between code elements and security findings
- `enable_insight_generation`: Generate actionable insights from analysis results
- `enable_context_analysis`: Perform deep contextual analysis of code patterns
- `min_confidence_threshold`: Minimum confidence level for AI-generated insights (0.0-1.0)
- `max_processing_time`: Maximum time in seconds for AI processing per file
- `enable_historical_analysis`: Use historical data to improve analysis accuracy
- `model_cache_directory`: Directory to cache AI models for performance

### Semantic Enrichment

When `enable_semantic_enrichment` is enabled, CodeGuardian uses AI to:

- Generate natural language descriptions of security findings
- Provide context-aware explanations of vulnerabilities
- Suggest remediation steps with detailed reasoning
- Enhance finding severity based on code context

### Relationship Detection

When `enable_relationship_detection` is enabled, CodeGuardian can:

- Identify dependencies between security issues
- Detect cascading vulnerabilities in related code
- Map attack vectors across multiple components
- Provide holistic security assessments

### Examples

```toml
# Minimal AI configuration
[ai]
enabled = true
enable_semantic_enrichment = true
enable_relationship_detection = false

# Full AI configuration with advanced features
[ai]
enabled = true
enable_semantic_enrichment = true
enable_relationship_detection = true
enable_insight_generation = true
enable_context_analysis = true
min_confidence_threshold = 0.8
max_processing_time = 600
enable_historical_analysis = true
model_cache_directory = "/var/cache/codeguardian/models"
```

## Optimization Configuration

CodeGuardian includes various optimization features to improve performance and resource usage.

### Basic Configuration

```toml
[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 4
max_memory_file_size = 1048576  # 1MB
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
skip_large_files_bytes = 1048576  # 1MB
```

### Configuration Options

#### Main Optimization Settings

- `enable_optimized_analyzers`: Use optimized analyzer implementations for better performance
- `enable_file_caching`: Cache analysis results for unchanged files
- `max_parallel_workers`: Maximum number of parallel analysis workers
- `max_memory_file_size`: Maximum file size to load into memory (bytes)
- `streaming_chunk_size`: Chunk size for streaming analysis (bytes)
- `max_findings_per_file`: Maximum findings to report per file (prevents output flooding)
- `pattern_cache_size`: Size of regex pattern cache for repeated patterns

#### Cache Cleanup Settings

- `enabled`: Enable automatic cache cleanup
- `max_age_days`: Maximum age of cached items in days
- `max_size_mb`: Maximum cache size in megabytes
- `cleanup_frequency`: How often to run cleanup (in analysis runs)

#### Early Termination Settings

- `enabled`: Enable early termination of slow analyses
- `max_analysis_time_seconds`: Maximum analysis time per file
- `max_lines_per_file`: Maximum lines to analyze per file
- `skip_large_files_bytes`: Skip files larger than this size

### Performance Optimizations

CodeGuardian implements several performance optimizations:

1. **File Caching**: Results for unchanged files are cached and reused
2. **Parallel Processing**: Multiple files are analyzed concurrently
3. **Streaming Analysis**: Large files are processed in chunks to reduce memory usage
4. **Pattern Caching**: Regex patterns are cached for repeated use
5. **Early Termination**: Long-running analyses are terminated to prevent hangs
6. **Memory Pools**: Object reuse reduces garbage collection overhead

### Examples

```toml
# High-performance configuration for large codebases
[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 8
max_memory_file_size = 5242880  # 5MB
streaming_chunk_size = 131072    # 128KB
max_findings_per_file = 100
pattern_cache_size = 2000

[optimization.cache_cleanup]
enabled = true
max_age_days = 14
max_size_mb = 500
cleanup_frequency = 5

[optimization.early_termination]
enabled = true
max_analysis_time_seconds = 60
max_lines_per_file = 20000
skip_large_files_bytes = 10485760  # 10MB

# Memory-constrained configuration
[optimization]
enable_optimized_analyzers = true
enable_file_caching = true
max_parallel_workers = 2
max_memory_file_size = 524288   # 512KB
streaming_chunk_size = 32768     # 32KB
max_findings_per_file = 25
pattern_cache_size = 500

[optimization.cache_cleanup]
enabled = true
max_age_days = 3
max_size_mb = 50
cleanup_frequency = 20
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
duplicate_prevention = true
semantic_similarity_threshold = 0.8
cross_repository_search = false
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
- `duplicate_prevention`: Enable duplicate issue prevention using semantic similarity
- `semantic_similarity_threshold`: Similarity threshold for duplicate detection (0.0-1.0)
- `cross_repository_search`: Search for duplicates across repositories (requires additional permissions)

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

## GitLab Integration

CodeGuardian supports integration with GitLab for automated issue creation and merge request commenting.

### Basic Configuration

```toml
[integrations.gitlab]
enabled = true
project = "group/project"
url = "https://gitlab.com"
token = "${CODEGUARDIAN_GITLAB_TOKEN}"
create_issues = true
issue_labels = ["security", "codeguardian"]
comment_mrs = true
min_severity = "high"
duplicate_prevention = true
semantic_similarity_threshold = 0.8
```

### Environment Variables

Set the GitLab token securely using environment variables:

```bash
export CODEGUARDIAN_GITLAB_TOKEN="your_gitlab_token_here"
```

### Configuration Options

- `enabled`: Enable/disable GitLab integration (default: false)
- `project`: GitLab project in "group/project" format
- `url`: GitLab instance URL (default: "https://gitlab.com")
- `token`: GitLab personal access token (supports environment variables)
- `create_issues`: Automatically create GitLab issues for findings
- `issue_labels`: Labels to apply to created issues
- `comment_mrs`: Comment on merge requests with findings
- `min_severity`: Minimum severity level to create issues/MR comments
- `duplicate_prevention`: Enable duplicate issue prevention using semantic similarity
- `semantic_similarity_threshold`: Similarity threshold for duplicate detection (0.0-1.0)

### Security Best Practices

1. **Use Environment Variables**: Never hardcode tokens in configuration files
2. **Token Permissions**: Use tokens with minimal required permissions (api, read_repository, write_repository)
3. **Project Access**: Limit token access to specific projects
4. **Token Rotation**: Regularly rotate GitLab tokens

### CI/CD Integration

For CI/CD pipelines, set the environment variable in your pipeline configuration:

```yaml
# GitLab CI
variables:
   CODEGUARDIAN_GITLAB_TOKEN: $GITLAB_TOKEN
```

## Retention Policy Configuration

CodeGuardian manages data retention to prevent disk space issues while maintaining analysis history.

### Basic Configuration

```toml
[retention]
enabled = true
results_dir = "build/analysis-results"
max_age_days = 30
max_size_mb = 1000
min_results_to_keep = 10
enable_integrity_check = true
integrity_check_frequency_days = 7
enable_auto_repair = false
backup_corrupted_files = true
backup_dir = "build/analysis-results/backup"
enable_integrity_reporting = true
integrity_report_path = "build/analysis-results/integrity-report.json"
```

### Configuration Options

- `enabled`: Enable/disable retention policy management
- `results_dir`: Directory containing analysis results
- `max_age_days`: Maximum age of results to keep (in days)
- `max_size_mb`: Maximum total size of results directory (in MB)
- `min_results_to_keep`: Minimum number of result files to retain
- `enable_integrity_check`: Enable integrity checking of stored results
- `integrity_check_frequency_days`: How often to check integrity (in days)
- `enable_auto_repair`: Automatically repair corrupted result files
- `backup_corrupted_files`: Create backups of files before repair attempts
- `backup_dir`: Directory to store backups of corrupted files
- `enable_integrity_reporting`: Generate integrity check reports
- `integrity_report_path`: Path to write integrity reports

### Data Retention Policies

CodeGuardian implements intelligent retention policies:

1. **Age-based Retention**: Remove results older than `max_age_days`
2. **Size-based Retention**: Remove oldest results when total size exceeds `max_size_mb`
3. **Minimum Retention**: Always keep at least `min_results_to_keep` results
4. **Integrity Protection**: Verify result file integrity before deletion
5. **Backup on Corruption**: Backup corrupted files before attempting repairs

### Examples

```toml
# Strict retention for CI/CD environments
[retention]
enabled = true
results_dir = "build/analysis-results"
max_age_days = 7
max_size_mb = 100
min_results_to_keep = 5
enable_integrity_check = true
integrity_check_frequency_days = 1
enable_auto_repair = true
backup_corrupted_files = true
backup_dir = "build/analysis-results/backup"
enable_integrity_reporting = true
integrity_report_path = "build/analysis-results/integrity-report.json"

# Relaxed retention for development
[retention]
enabled = true
results_dir = "analysis-results"
max_age_days = 90
max_size_mb = 5000
min_results_to_keep = 50
enable_integrity_check = false
integrity_check_frequency_days = 30
enable_auto_repair = false
backup_corrupted_files = false
enable_integrity_reporting = false
```

## Dashboard Configuration

```toml
[dashboard]
enabled = false
host = "127.0.0.1"
port = 8080
refresh_interval_seconds = 30
max_history_days = 30
enable_real_time = true
```

## Automated Remediation Configuration

```toml
[remediation]
enabled = false
auto_approve_low_risk = true
require_approval_threshold = "Medium"
max_concurrent_workflows = 5
timeout_minutes = 30

[remediation.integrations]
github_enabled = true
jira_enabled = false
slack_enabled = false
email_enabled = false

[remediation.notifications]
notify_on_start = true
notify_on_completion = true
notify_on_failure = true
notify_on_approval_needed = true
```

## Advanced Configuration

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
- `CODEGUARDIAN_PROJECT` - Project name for storage organization (fallback for --project-name)

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

For basic configuration options like output, security, analysis, and files, see [configuration-basics.md](configuration-basics.md).

For preset configurations and templates, see [configuration-presets.md](configuration-presets.md).

For CLI options, environment variables, and validation, see [cli-reference.md](cli-reference.md).