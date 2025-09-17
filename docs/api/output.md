# Output Systems API Documentation

This document provides comprehensive API documentation for CodeGuardian's output systems, including formatters, storage, metrics, and parallel processing capabilities.

## Table of Contents

- [Core Output API](#core-output-api)
- [Formatters API](#formatters-api)
- [Storage API](#storage-api)
- [Metrics API](#metrics-api)
- [Parallel Processing API](#parallel-processing-api)
- [Security API](#security-api)
- [Enterprise API](#enterprise-api)

## Core Output API

### OutputFormat Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Html,
    Markdown,
    Sarif,
    Yaml,
    Text,
}
```

Supported output formats with serialization support.

**Methods:**
- `content_type() -> &'static str` - Returns MIME content type
- `file_extension() -> &'static str` - Returns file extension
- `supports_streaming() -> bool` - Whether format supports streaming

### OutputResult Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputResult {
    pub content: String,
    pub metadata: OutputMetadata,
    pub properties: HashMap<String, serde_json::Value>,
}
```

Container for formatted output with metadata and properties.

**Methods:**
- `new(content: String, format: &str, config_hash: String) -> Self`
- `with_generation_time(time_ms: u64) -> Self`
- `with_property(key: String, value: serde_json::Value) -> Self`
- `with_validation_status(status: ValidationStatus) -> Self`
- `is_valid() -> bool`
- `validation_errors() -> &[String]`
- `validation_warnings() -> &[String]`

### OutputMetadata Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMetadata {
    pub schema_version: String,
    pub generated_at: DateTime<Utc>,
    pub config_hash: String,
    pub content_size_bytes: usize,
    pub generation_time_ms: u64,
    pub format: String,
    pub tool_metadata: ToolMetadata,
    pub validation_status: ValidationStatus,
}
```

Metadata about output generation process.

### OutputFormatter Trait

```rust
pub trait OutputFormatter: Send + Sync {
    fn format(&self, results: &AnalysisResults) -> Result<OutputResult>;
    fn content_type(&self) -> &'static str;
    fn supports_streaming(&self) -> bool { false }
    fn metadata(&self) -> FormatMetadata;
    fn file_extension(&self) -> &'static str;
    fn validate_output(&self, content: &str) -> Result<()> { Ok(()) }
    fn get_config_schema(&self) -> Option<serde_json::Value> { None }
    fn validate_input(&self, results: &AnalysisResults) -> Result<()>;
    fn schema_version(&self) -> &'static str { "1.0.0" }
    fn security_config(&self) -> SecurityConfig { SecurityConfig::default() }
}
```

Core trait that all output formatters must implement.

## Formatters API

### JSON Formatter

```rust
pub struct JsonFormatter {
    pub memory_manager: Option<std::sync::Arc<GlobalMemoryPools>>,
    pub pretty: bool,
    pub include_metadata: bool,
}
```

JSON output formatter with memory optimization support.

**Constructors:**
- `JsonFormatter::new() -> Self`
- `JsonFormatter::with_memory_manager(memory_manager: Arc<GlobalMemoryPools>) -> Self`
- `JsonFormatter::compact() -> Self`
- `JsonFormatter::minimal() -> Self`

**Features:**
- Pretty printing support
- Memory pool optimization
- Streaming support
- JSON schema validation

### HTML Formatter

```rust
pub struct HtmlFormatter {
    pub include_css: bool,
    pub include_js: bool,
    pub interactive: bool,
    pub dark_theme: bool,
    pub sanitize_content: bool,
}
```

HTML output formatter with security features.

**Constructors:**
- `HtmlFormatter::new() -> Self`
- `HtmlFormatter::minimal() -> Self`
- `HtmlFormatter::interactive() -> Self`

**Features:**
- XSS prevention via sanitization
- Content Security Policy headers
- CSS styling (optional)
- JavaScript integration (optional)
- Dark theme support
- Interactive features

### Markdown Formatter

```rust
pub struct MarkdownFormatter {
    pub include_table_of_contents: bool,
    pub include_summary: bool,
    pub include_code_blocks: bool,
    pub sanitize_content: bool,
}
```

Markdown output formatter for documentation.

**Features:**
- Table of contents generation
- Summary sections
- Code block formatting
- Content sanitization

### SARIF Formatter

```rust
pub struct SarifFormatter {
    pub include_suppressions: bool,
    pub include_context: bool,
    pub tool_version: Option<String>,
}
```

SARIF (Static Analysis Results Interchange Format) formatter for security tools.

**Features:**
- SARIF 2.1.0 compliance
- Tool metadata inclusion
- Suppression support
- Context information

### YAML Formatter

```rust
pub struct YamlFormatter {
    pub indent_size: usize,
    pub include_comments: bool,
}
```

YAML output formatter with customization options.

**Features:**
- Configurable indentation
- Comment inclusion
- Human-readable format

## Storage API

### StorageConfig Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_directory: PathBuf,
    pub organization_strategy: OrganizationStrategy,
    pub enable_compression: bool,
    pub max_results_per_directory: usize,
    pub enable_indexing: bool,
    pub retention_days: Option<u32>,
    pub enable_deduplication: bool,
}
```

Configuration for result storage and organization.

### OrganizationStrategy Enum

```rust
pub enum OrganizationStrategy {
    ByDate,
    ByProject,
    Hybrid,
    HierarchicalTimeBased,
    Custom(String),
}
```

Strategies for organizing stored results.

### StorageIndex Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIndex {
    pub version: String,
    pub last_updated: DateTime<Utc>,
    pub by_id: HashMap<String, ResultMetadata>,
    pub by_project: HashMap<String, Vec<String>>,
    pub by_repository: HashMap<String, Vec<String>>,
    pub by_date: HashMap<String, Vec<String>>,
    pub by_tags: HashMap<String, Vec<String>>,
    pub search_index: HashMap<String, Vec<String>>,
}
```

Multi-dimensional index for fast result retrieval.

**Methods:**
- `add_result(metadata: ResultMetadata)`
- `remove_result(id: &str) -> Result<()>`
- `find_by_project(project: &str) -> Vec<&ResultMetadata>`
- `find_by_date_range(start: &str, end: &str) -> Vec<&ResultMetadata>`
- `find_by_repository(repository: &str) -> Vec<&ResultMetadata>`
- `find_by_tags(tags: &[String]) -> Vec<&ResultMetadata>`
- `get_statistics() -> StorageStatistics`

### HierarchicalResultsOrganizer

```rust
pub struct HierarchicalResultsOrganizer {
    config: StorageConfig,
    index: StorageIndex,
}
```

Advanced organizer with hierarchical time-based storage.

**Methods:**
- `new(config: StorageConfig) -> Self`
- `store_result(result: &OutputResult, metadata: ResultMetadata) -> Result<PathBuf>`
- `retrieve_result(id: &str) -> Result<Option<OutputResult>>`
- `cleanup_expired(retention_days: u32) -> Result<usize>`
- `search(query: &SearchQuery) -> Result<Vec<SearchResult>>`

## Metrics API

### OutputMetricsService

```rust
pub struct OutputMetricsService {
    collector: OutputMetricsCollector,
    monitor: RealTimeMonitor,
    reporter: AutomatedReporter,
    alert_manager: AlertManager,
    trend_analyzer: TrendAnalyzer,
}
```

Main service for metrics collection and monitoring.

**Methods:**
- `record_output_metrics(results: &AnalysisResults, output_result: &OutputResult, format: &str, generation_time_ms: u64) -> Result<()>`
- `generate_report(time_range: Option<(DateTime<Utc>, DateTime<Utc>)>) -> Result<MetricsReport>`
- `get_health_status() -> Result<SystemHealth>`

### OutputMetrics Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMetrics {
    pub timestamp: DateTime<Utc>,
    pub format: String,
    pub functionality: FunctionalityMetrics,
    pub performance: PerformanceMetrics,
    pub security: SecurityMetrics,
    pub user_experience: UserExperienceMetrics,
}
```

Comprehensive metrics for output operations.

### AlertManager

```rust
pub struct AlertManager {
    rules: Vec<AlertRule>,
    active_alerts: HashMap<String, Alert>,
}
```

Manages alerts based on configurable rules.

**Methods:**
- `add_rule(rule: AlertRule)`
- `check_alerts(metrics: &OutputMetrics) -> Result<Option<Alert>>`
- `get_active_alerts() -> Result<Vec<Alert>>`
- `resolve_alert(alert_id: &str) -> Result<()>`

## Parallel Processing API

### ParallelOutputProcessor

```rust
pub struct ParallelOutputProcessor {
    config: ParallelOutputConfig,
    thread_pool: Option<rayon::ThreadPool>,
}
```

Processor for parallel output format generation.

**Methods:**
- `new() -> Result<Self>`
- `with_config(config: ParallelOutputConfig) -> Result<Self>`
- `process_multiple_formats(results: &AnalysisResults, formats: Vec<OutputFormat>) -> Result<HashMap<OutputFormat, OutputResult>>`
- `process_concurrent_pipeline(results: &AnalysisResults, formats: Vec<OutputFormat>) -> Result<ConcurrentPipelineResult>`

### ParallelOutputConfig

```rust
#[derive(Debug, Clone)]
pub struct ParallelOutputConfig {
    pub max_concurrent_formats: usize,
    pub chunk_size: usize,
    pub max_parallel_chunks: usize,
    pub thread_pool_size: usize,
}
```

Configuration for parallel processing behavior.

### ChunkParallelProcessor

```rust
pub struct ChunkParallelProcessor<T> {
    chunk_size: usize,
    max_concurrent: usize,
}
```

Processor for chunk-level parallelization.

**Methods:**
- `process_chunks_sync<F, R>(data: &[T], processor: F) -> Result<Vec<R>>`
- `process_chunks_async<F, R>(data: &[T], processor: F) -> Result<Vec<R>>`

## Security API

### SecurityConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub sanitize_html: bool,
    pub max_content_size: usize,
    pub max_files: usize,
    pub enable_csp: bool,
    pub allow_external_resources: bool,
    pub validate_inputs: bool,
    pub enable_compression: bool,
}
```

Security configuration for output formatters.

### Sanitization Functions

```rust
pub fn sanitize_html(content: &str, config: Option<&SecurityConfig>) -> Result<String>
pub fn generate_csp_header() -> String
```

HTML sanitization and CSP generation utilities.

## Enterprise API

### EnterpriseManager

```rust
pub struct EnterpriseManager {
    config: EnterpriseConfig,
    tenants: HashMap<String, Tenant>,
    audit_log: Vec<AuditEvent>,
}
```

Manager for enterprise features and multi-tenancy.

**Methods:**
- `new(config: EnterpriseConfig) -> Self`
- `create_tenant(name: String, config: TenantConfig) -> Result<String>`
- `process_output_for_tenant(tenant_id: &str, results: &AnalysisResults, format: OutputFormat) -> Result<OutputResult>`
- `get_audit_log(tenant_id: Option<&str>) -> Vec<AuditEvent>`

### EnterpriseConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    pub max_tenants: usize,
    pub enable_audit_logging: bool,
    pub enable_encryption: bool,
    pub retention_policy: RetentionPolicy,
    pub compliance_mode: ComplianceMode,
}
```

Configuration for enterprise features.

## Usage Examples

### Basic Output Formatting

```rust
use do_codeguardian::output::{create_formatter, OutputFormat};
use do_codeguardian::types::AnalysisResults;

let results = AnalysisResults::new("example".to_string());
let formatter = create_formatter(OutputFormat::Json);
let output = formatter.format(&results)?;
println!("Generated {} bytes of output", output.content.len());
```

### Parallel Multi-Format Generation

```rust
use do_codeguardian::output::{ParallelOutputProcessor, OutputFormat};

let processor = ParallelOutputProcessor::new()?;
let formats = vec![OutputFormat::Json, OutputFormat::Html, OutputFormat::Sarif];
let outputs = processor.process_multiple_formats(&results, formats).await?;

for (format, result) in outputs {
    println!("Generated {} output: {} bytes", format, result.content.len());
}
```

### Storage and Retrieval

```rust
use do_codeguardian::output::storage::{StorageConfig, HierarchicalResultsOrganizer};

let config = StorageConfig::default();
let organizer = HierarchicalResultsOrganizer::new(config);

// Store result
let path = organizer.store_result(&output_result, metadata).await?;

// Retrieve result
let retrieved = organizer.retrieve_result("result_id").await?;
```

### Metrics Collection

```rust
use do_codeguardian::output::metrics::OutputMetricsService;

let mut metrics_service = OutputMetricsService::new();
metrics_service.record_output_metrics(
    &results,
    &output_result,
    "json",
    generation_time_ms,
).await?;

// Generate report
let report = metrics_service.generate_report(None).await?;
println!("Success rate: {:.1}%", report.summary.success_rate);
```

## Error Handling

All API methods return `Result<T>` types. Common error scenarios:

- **ValidationError**: Input validation failures
- **FormatError**: Output formatting errors
- **StorageError**: Storage operation failures
- **SecurityError**: Security validation failures
- **ConcurrencyError**: Parallel processing errors

## Configuration

Output systems can be configured via:

1. **Programmatic Configuration**: Direct struct initialization
2. **TOML Configuration**: Via `codeguardian.toml`
3. **Environment Variables**: Runtime overrides
4. **CLI Flags**: Command-line options

## Performance Considerations

- Use memory pools for large JSON outputs
- Enable compression for storage
- Configure appropriate thread pool sizes
- Use streaming for large datasets
- Monitor metrics for optimization opportunities

## Security Best Practices

- Always enable input validation
- Use HTML sanitization for web outputs
- Implement Content Security Policies
- Validate file paths for traversal attacks
- Enable audit logging in enterprise environments
- Regularly review and update security configurations
