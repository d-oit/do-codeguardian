# Output Systems User Guide

This guide provides comprehensive information about CodeGuardian's output systems, including format options, configuration, and best practices for different use cases.

## Table of Contents

- [Overview](#overview)
- [Output Formats](#output-formats)
- [Configuration](#configuration)
- [Storage and Organization](#storage-and-organization)
- [Performance Optimization](#performance-optimization)
- [Security Features](#security-features)
- [Integration Examples](#integration-examples)
- [Troubleshooting](#troubleshooting)

## Overview

CodeGuardian's output systems provide flexible, secure, and high-performance options for generating analysis results in multiple formats. The system supports parallel processing, advanced storage organization, and comprehensive metrics collection.

### Key Features

- **Multiple Output Formats**: JSON, HTML, Markdown, SARIF, YAML, Text
- **Parallel Processing**: Concurrent format generation with configurable thread pools
- **Advanced Storage**: Hierarchical organization with indexing and search
- **Security-First**: XSS prevention, input validation, and CSP headers
- **Metrics & Monitoring**: Real-time performance tracking and alerting
- **Enterprise Ready**: Multi-tenancy, audit logging, and compliance features

## Output Formats

### JSON Format

The JSON format is the "source of truth" for CodeGuardian results, providing complete data preservation with optional pretty printing.

**Use Cases:**
- Programmatic processing
- CI/CD integration
- API consumption
- Data analysis and reporting

**Configuration Options:**
```toml
[output.json]
pretty = true
include_metadata = true
compact = false
```

**Example Usage:**
```bash
do-codeguardian check . --format json --out results.json
```

**Features:**
- Complete data preservation
- Streaming support for large datasets
- Memory pool optimization
- Schema validation

### HTML Format

HTML format provides rich, interactive reports with built-in security features and responsive design.

**Use Cases:**
- Web-based reporting
- Executive dashboards
- Documentation portals
- Stakeholder communication

**Configuration Options:**
```toml
[output.html]
include_css = true
include_js = false  # Security: disabled by default
interactive = false
dark_theme = false
sanitize_content = true
```

**Example Usage:**
```bash
do-codeguardian check . --format html --out report.html
```

**Features:**
- XSS prevention via ammonia sanitization
- Content Security Policy headers
- Responsive CSS styling
- Optional JavaScript for interactivity
- Dark theme support

### Markdown Format

Markdown format generates clean, readable documentation suitable for version control and documentation systems.

**Use Cases:**
- Documentation generation
- GitHub/GitLab integration
- Knowledge base articles
- Technical reports

**Configuration Options:**
```toml
[output.markdown]
include_table_of_contents = true
include_summary = true
include_code_blocks = true
sanitize_content = true
```

**Example Usage:**
```bash
do-codeguardian check . --format markdown --out analysis.md
```

**Features:**
- Table of contents generation
- Code syntax highlighting
- Summary sections
- Cross-reference linking

### SARIF Format

SARIF (Static Analysis Results Interchange Format) is the industry standard for security tool integration.

**Use Cases:**
- Security tool integration
- Compliance reporting
- IDE integration
- Enterprise security platforms

**Configuration Options:**
```toml
[output.sarif]
include_suppressions = true
include_context = true
tool_version = "1.0.0"
```

**Example Usage:**
```bash
do-codeguardian check . --format sarif --out security-results.sarif
```

**Features:**
- SARIF 2.1.0 compliance
- Tool metadata inclusion
- Suppression support
- Context information
- CWE mapping

### YAML Format

YAML format provides human-readable structured data with optional comments.

**Use Cases:**
- Configuration files
- Documentation
- Data exchange
- Kubernetes integration

**Configuration Options:**
```toml
[output.yaml]
indent_size = 2
include_comments = true
```

**Example Usage:**
```bash
do-codeguardian check . --format yaml --out results.yaml
```

## Configuration

### Basic Configuration

```toml
[output]
# Default output directory
directory = "analysis-results"

# Default output format
format = "json"

# Enable verbose output
verbose = false

# Generate summary reports
generate_summary = true

# Enable output compression
compress_output = true

# Maximum concurrent formats
max_concurrent_formats = 4

# Chunk size for parallel processing
chunk_size = 1000
```

### Advanced Configuration

```toml
[output]
# Storage configuration
[output.storage]
base_directory = "./analysis-results"
organization_strategy = "hierarchical_time_based"
enable_compression = true
max_results_per_directory = 1000
enable_indexing = true
retention_days = 365
enable_deduplication = true

# Metrics configuration
[output.metrics]
enable_collection = true
alert_thresholds = { generation_time_ms = 5000, validation_score = 0.8 }
enable_real_time_monitoring = true

# Security configuration
[output.security]
sanitize_html = true
max_content_size = 104857600  # 100MB
max_files = 10000
enable_csp = true
allow_external_resources = false
validate_inputs = true
enable_compression = true
```

### Environment Variables

```bash
# Output configuration
export CODEGUARDIAN_OUTPUT_DIR="./custom-results"
export CODEGUARDIAN_OUTPUT_FORMAT="json"
export CODEGUARDIAN_OUTPUT_COMPRESS="true"

# Performance tuning
export CODEGUARDIAN_MAX_CONCURRENT_FORMATS="8"
export CODEGUARDIAN_CHUNK_SIZE="500"

# Security settings
export CODEGUARDIAN_SANITIZE_HTML="true"
export CODEGUARDIAN_MAX_CONTENT_SIZE="52428800"  # 50MB
```

## Storage and Organization

### Organization Strategies

CodeGuardian supports multiple strategies for organizing analysis results:

#### Hierarchical Time-Based (Default)
```
analysis-results/
├── 2024/
│   ├── 09/
│   │   ├── 17/
│   │   │   ├── 10/
│   │   │   │   ├── abc123-project/
│   │   │   │   │   ├── results.json
│   │   │   │   │   ├── report.html
│   │   │   │   │   └── metadata.json
```

#### By Project
```
analysis-results/
├── my-project/
│   ├── 2024-09-17-results.json
│   ├── 2024-09-16-results.json
│   └── archive/
```

#### Hybrid Organization
```
analysis-results/
├── projects/
│   ├── my-project/
│   │   ├── 2024/
│   │   │   ├── 09/
│   │   │   │   ├── results.json
```

### Storage Features

- **Automatic Indexing**: Fast search and retrieval
- **Compression**: Reduce storage space by up to 60%
- **Deduplication**: Prevent duplicate result storage
- **Retention Policies**: Automatic cleanup of old results
- **Integrity Verification**: Checksum validation

### Search and Retrieval

```rust
use do_codeguardian::output::storage::{StorageConfig, HierarchicalResultsOrganizer, QueryCriteria};

// Initialize storage
let config = StorageConfig::default();
let organizer = HierarchicalResultsOrganizer::new(config);

// Search by project
let query = QueryCriteria {
    project: Some("my-project".to_string()),
    date_range: Some(("2024-09-01".to_string(), "2024-09-30".to_string())),
    ..Default::default()
};

let results = organizer.search(&query)?;
```

## Performance Optimization

### Parallel Processing

Enable parallel output generation for multiple formats:

```bash
# Generate multiple formats in parallel
do-codeguardian check . \
  --format json \
  --format html \
  --format sarif \
  --max-concurrent-formats 4
```

### Memory Optimization

```toml
[performance]
# Enable memory pools for JSON processing
memory_pool_enabled = true
memory_pool_size_mb = 512

# Configure streaming thresholds
streaming_threshold_mb = 100

# Optimize for large datasets
large_dataset_optimization = true
```

### Benchmarking

```bash
# Run performance benchmarks
cargo bench --bench optimization_benchmarks -- parallel_output_processing

# Profile memory usage
do-codeguardian check . --format json --memory-profile
```

### Performance Tips

1. **Use appropriate chunk sizes**: Balance between memory usage and parallelism
2. **Enable compression**: Reduces I/O and storage costs
3. **Configure thread pools**: Match thread count to CPU cores
4. **Use streaming**: For datasets larger than 100MB
5. **Monitor metrics**: Track performance trends over time

## Security Features

### Input Validation

CodeGuardian validates all inputs before processing:

- **Path traversal prevention**: Blocks `../` and absolute paths
- **Content size limits**: Prevents memory exhaustion attacks
- **File count limits**: Prevents resource exhaustion
- **Malicious content detection**: Identifies potentially harmful patterns

### Output Sanitization

```rust
use do_codeguardian::output::security::sanitize_html;

// Sanitize HTML content
let clean_html = sanitize_html(dirty_content, None)?;
```

### Content Security Policy

HTML outputs include CSP headers by default:

```html
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'none'; style-src 'self' 'unsafe-inline';">
```

### Security Configuration

```toml
[output.security]
# Enable HTML sanitization
sanitize_html = true

# Content size limits
max_content_size = 104857600  # 100MB
max_files = 10000

# CSP settings
enable_csp = true
allow_external_resources = false

# Input validation
validate_inputs = true

# Compression (security feature)
enable_compression = true
```

## Integration Examples

### CI/CD Integration

#### GitHub Actions
```yaml
name: Security Analysis
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Run CodeGuardian
        run: |
          do-codeguardian check . \
            --format json \
            --format sarif \
            --out results.json \
            --out security.sarif

      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: security.sarif
```

#### GitLab CI
```yaml
security_scan:
  stage: security
  script:
    - do-codeguardian check . --format json --out results.json
    - do-codeguardian check . --format sarif --out security.sarif
  artifacts:
    reports:
      sarif: security.sarif
    paths:
      - results.json
```

### API Integration

```python
import requests
import json

# Run analysis
response = requests.post('http://localhost:8080/analyze', json={
    'path': '/path/to/code',
    'formats': ['json', 'html']
})

if response.status_code == 200:
    results = response.json()

    # Process JSON results
    with open('results.json', 'w') as f:
        json.dump(results['json'], f, indent=2)

    # Save HTML report
    with open('report.html', 'w') as f:
        f.write(results['html'])
```

### Database Integration

```rust
use sqlx::PgPool;
use do_codeguardian::output::OutputResult;

async fn store_results(pool: &PgPool, result: &OutputResult) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO analysis_results (id, content, metadata, created_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(&result.metadata.config_hash)
    .bind(&result.content)
    .bind(serde_json::to_value(&result.metadata)?)
    .bind(result.metadata.generated_at)
    .execute(pool)
    .await?;

    Ok(())
}
```

### Monitoring Integration

```rust
use do_codeguardian::output::metrics::{OutputMetricsService, AlertRule, AlertSeverity};

let mut metrics_service = OutputMetricsService::new();

// Add custom alert rule
let alert_rule = AlertRule {
    name: "slow_generation".to_string(),
    condition: "generation_time_ms > 10000".to_string(),
    severity: AlertSeverity::Warning,
    message: "Output generation is taking too long".to_string(),
};

metrics_service.alert_manager.add_rule(alert_rule);

// Monitor performance
let health = metrics_service.get_health_status().await?;
println!("System health: {:?}", health.overall_status);
```

## Troubleshooting

### Common Issues

#### High Memory Usage
**Symptoms:** Out of memory errors, slow performance
**Solutions:**
- Reduce chunk size: `--chunk-size 500`
- Enable streaming: `--streaming-threshold 50`
- Increase memory limits: `--memory-limit 2048`

#### Slow Output Generation
**Symptoms:** Long generation times, timeouts
**Solutions:**
- Enable parallel processing: `--max-concurrent-formats 4`
- Use compact formats: `--format json --compact`
- Optimize thread pools: `--thread-pool-size 8`

#### Storage Issues
**Symptoms:** Storage full, slow retrieval
**Solutions:**
- Enable compression: `compress_output = true`
- Configure retention: `retention_days = 90`
- Clean up old results: `--cleanup-expired`

#### Security Warnings
**Symptoms:** CSP violations, sanitization errors
**Solutions:**
- Review CSP configuration
- Check HTML content for malicious patterns
- Update security settings

### Debug Mode

Enable debug output for troubleshooting:

```bash
# Enable debug logging
export RUST_LOG=debug
do-codeguardian check . --format json --verbose

# Profile performance
do-codeguardian check . --format json --profile

# Validate output
do-codeguardian check . --format json --validate-output
```

### Performance Monitoring

```bash
# Monitor system resources
do-codeguardian check . --format json --metrics

# Generate performance report
do-codeguardian report --from results.json --performance-report

# Check system health
do-codeguardian health --output-format json
```

### Log Analysis

```bash
# View recent logs
tail -f ~/.codeguardian/logs/output.log

# Search for errors
grep "ERROR" ~/.codeguardian/logs/*.log

# Analyze performance logs
grep "generation_time" ~/.codeguardian/logs/metrics.log | jq '.generation_time_ms' | sort -n
```

## Best Practices

### Production Deployment

1. **Use appropriate formats**: JSON for APIs, HTML for reports
2. **Enable security features**: Always enable sanitization and validation
3. **Configure monitoring**: Set up alerts and metrics collection
4. **Optimize performance**: Tune thread pools and memory settings
5. **Implement retention**: Configure automatic cleanup policies

### Development Workflow

1. **Use JSON for debugging**: Complete data for troubleshooting
2. **Enable verbose output**: Detailed logging during development
3. **Test multiple formats**: Ensure compatibility across formats
4. **Monitor performance**: Track metrics during development

### Enterprise Usage

1. **Enable enterprise features**: Multi-tenancy and audit logging
2. **Configure compliance**: Meet regulatory requirements
3. **Implement access controls**: Secure result access
4. **Set up monitoring**: Comprehensive alerting and reporting

### Performance Tuning

1. **Profile regularly**: Monitor performance trends
2. **Tune thread pools**: Match CPU core count
3. **Optimize memory**: Use pools and streaming
4. **Enable compression**: Reduce storage and transfer costs
5. **Monitor alerts**: Respond to performance degradation

This guide covers the essential aspects of CodeGuardian's output systems. For specific API documentation, see the [API Reference](../api/output.md).
