# Configuration API Documentation

This document provides comprehensive API documentation for CodeGuardian's configuration management system.

## Table of Contents

- [Configuration Structure](#configuration-structure)
- [Core Configuration](#core-configuration)
- [Security Configuration](#security-configuration)
- [Output Configuration](#output-configuration)
- [Integration Configuration](#integration-configuration)
- [Dashboard Configuration](#dashboard-configuration)
- [Remediation Configuration](#remediation-configuration)
- [Relationship Configuration](#relationship-configuration)
- [Configuration Loading](#configuration-loading)
- [Validation](#validation)

## Configuration Structure

CodeGuardian uses a hierarchical TOML configuration system with support for environment variables and CLI overrides.

### Main Config Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub security: SecurityConfig,
    pub git: GitConfig,
    pub output: OutputConfig,
    pub integrations: IntegrationsConfig,
    pub dashboard: DashboardConfig,
    pub remediation: RemediationConfig,
    pub relationships: RelationshipConfig,
}
```

## Core Configuration

### SecurityConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_advanced_analysis: bool,
    pub max_file_size_mb: usize,
    pub max_files_per_scan: usize,
    pub enable_ml_detection: bool,
    pub confidence_threshold: f64,
    pub severity_threshold: Severity,
    pub enable_cwe_mapping: bool,
    pub custom_rules_path: Option<String>,
    pub ignore_patterns: Vec<String>,
    pub security_policies: Vec<SecurityPolicy>,
}
```

**Fields:**
- `enable_advanced_analysis`: Enable advanced security analysis features
- `max_file_size_mb`: Maximum file size to analyze (default: 10MB)
- `max_files_per_scan`: Maximum number of files per scan (default: 1000)
- `enable_ml_detection`: Enable machine learning-based detection
- `confidence_threshold`: Minimum confidence score (0.0-1.0)
- `severity_threshold`: Minimum severity level to report
- `enable_cwe_mapping`: Map findings to CWE identifiers
- `custom_rules_path`: Path to custom security rules
- `ignore_patterns`: File patterns to ignore during scanning
- `security_policies`: Custom security policies

**Example:**
```toml
[security]
enable_advanced_analysis = true
max_file_size_mb = 10
max_files_per_scan = 1000
enable_ml_detection = true
confidence_threshold = 0.7
severity_threshold = "medium"
enable_cwe_mapping = true
ignore_patterns = ["*.min.js", "vendor/*"]
```

### GitConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub enable_security_checks: bool,
    pub require_clean_tree: bool,
    pub max_commit_size_kb: u32,
    pub allowed_file_types: Vec<String>,
    pub blocked_file_patterns: Vec<String>,
    pub enable_commit_message_analysis: bool,
    pub require_signed_commits: bool,
    pub branch_protection_rules: Vec<BranchProtectionRule>,
}
```

**Example:**
```toml
[git]
enable_security_checks = true
require_clean_tree = true
max_commit_size_kb = 1024
allowed_file_types = ["rs", "toml", "md", "yml", "yaml"]
blocked_file_patterns = ["*.exe", "*.dll", "*.so"]
enable_commit_message_analysis = true
require_signed_commits = false
```

## Output Configuration

### OutputConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub pretty_print: bool,
    pub include_metadata: bool,
    pub output_directory: String,
    pub enable_compression: bool,
    pub max_output_size_mb: usize,
    pub enable_streaming: bool,
    pub custom_templates_path: Option<String>,
}
```

**Example:**
```toml
[output]
format = "json"
pretty_print = true
include_metadata = true
output_directory = "./output"
enable_compression = true
max_output_size_mb = 100
enable_streaming = false
```

## Integration Configuration (New in v0.2.0-alpha.5)

### IntegrationsConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsConfig {
    pub enabled: bool,
    pub systems: HashMap<String, SystemConfig>,
    pub default_timeout_seconds: u64,
    pub retry_attempts: u32,
    pub batch_size: usize,
}
```

### SystemConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub enabled: bool,
    pub base_url: String,
    pub auth: AuthConfig,
    pub features: SystemFeatures,
    pub rate_limits: RateLimits,
    pub custom_fields: HashMap<String, String>,
}
```

**Supported Systems:**
- `jira`: Issue tracking and project management
- `confluence`: Documentation and knowledge management
- `jenkins`: CI/CD pipeline automation
- `gitlab`: Git repository and CI/CD management
- `bitbucket`: Git repository hosting
- `azure_devops`: DevOps and work tracking

**Example:**
```toml
[integrations]
enabled = true
default_timeout_seconds = 30
retry_attempts = 3
batch_size = 50

[integrations.jira]
enabled = true
base_url = "https://your-domain.atlassian.net"
username = "your-username"
features = { issue_tracking = true, duplicate_detection = true }

[integrations.gitlab]
enabled = true
base_url = "https://gitlab.com"
features = { issue_tracking = true, workflow_automation = true }

[integrations.jenkins]
enabled = true
base_url = "https://jenkins.example.com"
features = { workflow_automation = true, reporting = true }
```

### AuthConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthConfig {
    BasicAuth { username: String, token: String },
    Token { token: String },
    ApiKey { key: String },
    OAuth { client_id: String, client_secret: String, access_token: String },
}
```

## Dashboard Configuration (New in v0.2.0-alpha.5)

### DashboardConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub refresh_interval_seconds: u64,
    pub max_history_days: u32,
    pub enable_real_time: bool,
    pub custom_views: Vec<CustomView>,
    pub authentication: Option<DashboardAuth>,
}
```

**Example:**
```toml
[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080
refresh_interval_seconds = 30
max_history_days = 30
enable_real_time = true

[[dashboard.custom_views]]
name = "Security Overview"
description = "Security-related duplicate detection"
filters = { category_filter = ["security"], severity_filter = ["high", "critical"] }
```

## Remediation Configuration (New in v0.2.0-alpha.5)

### RemediationConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationConfig {
    pub enabled: bool,
    pub auto_approve_low_risk: bool,
    pub require_approval_threshold: RiskLevel,
    pub max_concurrent_workflows: u32,
    pub timeout_minutes: u32,
    pub integrations: IntegrationConfig,
    pub notification_settings: NotificationConfig,
}
```

**Risk Levels:**
- `Low`: Minimal impact changes
- `Medium`: Moderate impact changes
- `High`: Significant impact changes
- `Critical`: High-risk changes requiring approval

**Example:**
```toml
[remediation]
enabled = true
auto_approve_low_risk = true
require_approval_threshold = "medium"
max_concurrent_workflows = 5
timeout_minutes = 30

[remediation.notification_settings]
notify_on_start = true
notify_on_completion = true
notify_on_failure = true
notification_channels = ["email", "slack"]
```

## Relationship Configuration (New in v0.2.0-alpha.5)

### RelationshipConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipConfig {
    pub enabled: bool,
    pub max_relationships_per_artifact: usize,
    pub relationship_ttl_days: u32,
    pub auto_discovery_enabled: bool,
    pub visualization_enabled: bool,
    pub impact_analysis_enabled: bool,
    pub supported_artifact_types: Vec<ArtifactType>,
    pub relationship_weights: HashMap<String, f64>,
}
```

**Artifact Types:**
- `SourceCode`
- `Documentation`
- `Configuration`
- `Issue`
- `PullRequest`
- `TestFile`
- `BuildScript`
- `DatabaseSchema`
- `ApiDefinition`
- `Deployment`
- `Monitoring`
- `Security`

**Relationship Types:**
- `Duplicate`: Exact or near-exact duplicates
- `Similar`: Similar but not identical
- `References`: Code or documentation references
- `DependsOn`: Dependency relationships
- `PartOf`: Composition relationships
- `RelatedTo`: General relationships
- `Implements`: Implementation relationships
- `Tests`: Test relationships

**Example:**
```toml
[relationships]
enabled = true
max_relationships_per_artifact = 50
relationship_ttl_days = 90
auto_discovery_enabled = true
visualization_enabled = true
impact_analysis_enabled = true
supported_artifact_types = ["SourceCode", "Documentation", "Issue", "TestFile"]

[relationships.relationship_weights]
Duplicate = 0.9
Similar = 0.7
References = 0.6
DependsOn = 0.8
```

## Configuration Loading

### Loading Methods

```rust
impl Config {
    pub fn from_file(path: &str) -> Result<Self>;
    pub fn from_env() -> Result<Self>;
    pub fn merge_with_env(mut self) -> Result<Self>;
    pub fn validate(&self) -> Result<()>;
}
```

### Configuration Priority

1. **Default values**: Built-in defaults
2. **Configuration file**: TOML file specified by `--config` or `CODEGUARDIAN_CONFIG`
3. **Environment variables**: Override specific values
4. **CLI flags**: Override for current execution

### Environment Variables

- `CODEGUARDIAN_CONFIG`: Path to configuration file
- `CODEGUARDIAN_SECURITY_MAX_FILE_SIZE_MB`: Override max file size
- `CODEGUARDIAN_OUTPUT_FORMAT`: Override output format
- `CODEGUARDIAN_DASHBOARD_HOST`: Override dashboard host
- `CODEGUARDIAN_DASHBOARD_PORT`: Override dashboard port

## Validation

### Configuration Validation

```rust
pub fn validate_config(config: &Config) -> Result<Vec<ValidationError>>;
```

**Validation Rules:**
- File paths must exist and be accessible
- URLs must be valid
- Numeric values must be within acceptable ranges
- Required fields must be present
- Authentication credentials must be properly formatted
- Integration configurations must be consistent

### Validation Errors

```rust
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub severity: ValidationSeverity,
}
```

**Severities:**
- `Error`: Configuration is invalid and cannot be used
- `Warning`: Configuration may work but has issues
- `Info`: Informational message about configuration

## Examples

### Basic Configuration

```toml
# codeguardian.toml
[security]
enable_advanced_analysis = true
max_file_size_mb = 10
severity_threshold = "medium"

[output]
format = "json"
pretty_print = true

[git]
enable_security_checks = true
require_clean_tree = true
```

### Advanced Configuration with Integrations

```toml
# codeguardian.toml
[security]
enable_advanced_analysis = true
enable_ml_detection = true
confidence_threshold = 0.8

[integrations]
enabled = true

[integrations.jira]
enabled = true
base_url = "https://company.atlassian.net"
features = { issue_tracking = true, duplicate_detection = true }

[integrations.gitlab]
enabled = true
base_url = "https://gitlab.company.com"
features = { issue_tracking = true, workflow_automation = true }

[dashboard]
enabled = true
host = "0.0.0.0"
port = 8080
enable_real_time = true

[remediation]
enabled = true
auto_approve_low_risk = true
require_approval_threshold = "high"

[relationships]
enabled = true
auto_discovery_enabled = true
visualization_enabled = true
```

### Environment-Based Configuration

```bash
export CODEGUARDIAN_CONFIG="./config/production.toml"
export CODEGUARDIAN_SECURITY_MAX_FILE_SIZE_MB=50
export CODEGUARDIAN_OUTPUT_FORMAT="sarif"
export CODEGUARDIAN_DASHBOARD_PORT=3000
```

### Programmatic Configuration

```rust
use do_codeguardian::Config;

let config = Config::default()
    .with_security(|security| {
        security.enable_advanced_analysis = true;
        security.max_file_size_mb = 20;
    })
    .with_output(|output| {
        output.format = OutputFormat::Sarif;
        output.pretty_print = false;
    })
    .with_dashboard(|dashboard| {
        dashboard.enabled = true;
        dashboard.port = 3000;
    });
```

## Migration Guide

### From v0.1.0 to v0.2.0-alpha.5

**New Configuration Sections:**
- `[integrations]`: External system integrations
- `[dashboard]`: Web dashboard configuration
- `[remediation]`: Automated remediation settings
- `[relationships]`: Relationship management

**Updated Sections:**
- `[security]`: Added ML detection and advanced analysis options
- `[output]`: Enhanced with streaming and compression options
- `[git]`: Added branch protection and commit analysis features

**Migration Example:**
```toml
# Old v0.1.0 configuration
[security]
max_file_size = 10

[output]
format = "json"

# New v0.2.0-alpha.5 configuration
[security]
max_file_size_mb = 10
enable_advanced_analysis = true

[output]
format = "json"
pretty_print = true

# New sections
[integrations]
enabled = false

[dashboard]
enabled = false

[remediation]
enabled = false

[relationships]
enabled = true
```
