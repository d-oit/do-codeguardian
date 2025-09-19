# Integrations API Documentation

This document provides comprehensive API documentation for CodeGuardian's external system integrations, introduced in v0.2.0.

## Table of Contents

- [Integration Manager](#integration-manager)
- [Supported Systems](#supported-systems)
- [Authentication](#authentication)
- [Duplicate Detection](#duplicate-detection)
- [Issue Management](#issue-management)
- [Workflow Automation](#workflow-automation)
- [Reporting](#reporting)
- [Webhooks](#webhooks)
- [Bulk Operations](#bulk-operations)
- [Health Monitoring](#health-monitoring)

## Integration Manager

### IntegrationManager

```rust
pub struct IntegrationManager {
    config: IntegrationsConfig,
    clients: HashMap<String, Box<dyn ExternalSystemClient>>,
}
```

**Methods:**
```rust
impl IntegrationManager {
    pub fn new(config: IntegrationsConfig) -> Self;
    pub async fn initialize(&mut self) -> Result<()>;
    pub async fn search_duplicates_across_systems(&self, query: &DuplicateSearchQuery) -> Result<Vec<DuplicateSearchResult>>;
    pub async fn create_issue_across_systems(&self, issue: &IssueCreationRequest) -> Result<Vec<IssueCreationResult>>;
    pub async fn trigger_workflows(&self, workflow_request: &WorkflowTriggerRequest) -> Result<Vec<WorkflowTriggerResult>>;
    pub async fn generate_unified_report(&self, report_request: &ReportRequest) -> Result<UnifiedReport>;
    pub async fn get_health_status(&self) -> Result<IntegrationHealthStatus>;
}
```

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

## Supported Systems

### Jira Integration

```rust
pub struct JiraClient {
    config: SystemConfig,
    client: reqwest::Client,
}
```

**Features:**
- Issue creation and management
- Duplicate detection across projects
- Workflow automation
- Custom field support

**Configuration:**
```toml
[integrations.jira]
enabled = true
base_url = "https://your-domain.atlassian.net"
username = "your-username"
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true }
```

### Confluence Integration

```rust
pub struct ConfluenceClient {
    config: SystemConfig,
    client: reqwest::Client,
}
```

**Features:**
- Documentation management
- Knowledge base integration
- Content duplicate detection
- Space management

**Configuration:**
```toml
[integrations.confluence]
enabled = true
base_url = "https://your-domain.atlassian.net/wiki"
features = { duplicate_detection = true, reporting = true, webhooks = true }
```

### Jenkins Integration

```rust
pub struct JenkinsClient {
    config: SystemConfig,
    client: reqwest::Client,
}
```

**Features:**
- Pipeline triggering
- Build status monitoring
- Job management
- Parameterized builds

**Configuration:**
```toml
[integrations.jenkins]
enabled = true
base_url = "https://jenkins.example.com"
features = { workflow_automation = true, reporting = true, webhooks = true }
```

### GitLab Integration

```rust
pub struct GitLabClient {
    config: SystemConfig,
    client: reqwest::Client,
}
```

**Features:**
- Issue management
- Merge request handling
- Pipeline automation
- Project management

**Configuration:**
```toml
[integrations.gitlab]
enabled = true
base_url = "https://gitlab.com"
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true }
```

### Bitbucket Integration

```rust
pub struct BitbucketClient {
    config: SystemConfig,
    client: reqwest::Client,
}
```

**Features:**
- Repository management
- Pull request handling
- Issue tracking
- Webhook support

**Configuration:**
```toml
[integrations.bitbucket]
enabled = true
base_url = "https://api.bitbucket.org/2.0"
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true }
```

### Azure DevOps Integration

```rust
pub struct AzureDevOpsClient {
    config: SystemConfig,
    client: reqwest::Client,
}
```

**Features:**
- Work item management
- Build pipeline integration
- Release management
- Test case management

**Configuration:**
```toml
[integrations.azure_devops]
enabled = true
base_url = "https://dev.azure.com/your-org"
features = { issue_tracking = true, workflow_automation = true, reporting = true }
```

## Authentication

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

### Authentication Examples

**Jira Basic Auth:**
```toml
[integrations.jira.auth]
BasicAuth = { username = "user@domain.com", token = "api-token" }
```

**GitLab Token Auth:**
```toml
[integrations.gitlab.auth]
Token = { token = "glpat-xxxxxxxxxxxxxx" }
```

**Jenkins API Key:**
```toml
[integrations.jenkins.auth]
ApiKey = { key = "jenkins-api-key" }
```

**Bitbucket OAuth:**
```toml
[integrations.bitbucket.auth]
OAuth = { client_id = "client-id", client_secret = "client-secret", access_token = "access-token" }
```

## Duplicate Detection

### DuplicateSearchQuery

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSearchQuery {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub labels: Vec<String>,
    pub project_key: Option<String>,
    pub issue_type: Option<String>,
    pub similarity_threshold: f64,
    pub max_results: usize,
    pub include_closed: bool,
}
```

### DuplicateSearchResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSearchResult {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub relevance_score: f64,
    pub similarity_score: f64,
    pub source_system: String,
    pub metadata: HashMap<String, serde_json::Value>,
}
```

### Cross-System Duplicate Search

```rust
use do_codeguardian::integrations::{IntegrationManager, DuplicateSearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = IntegrationManager::new(config);
    manager.initialize().await?;

    let query = DuplicateSearchQuery {
        title: "Security vulnerability in authentication".to_string(),
        description: Some("Users can bypass authentication".to_string()),
        labels: vec!["security".to_string(), "auth".to_string()],
        similarity_threshold: 0.8,
        max_results: 10,
        include_closed: false,
        ..Default::default()
    };

    let results = manager.search_duplicates_across_systems(&query).await?;

    println!("Found {} potential duplicates across systems", results.len());
    for result in results {
        println!("- {} ({}) - Similarity: {:.2}",
                result.title, result.source_system, result.similarity_score);
    }

    Ok(())
}
```

## Issue Management

### IssueCreationRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCreationRequest {
    pub title: String,
    pub description: String,
    pub issue_type: String,
    pub priority: IssuePriority,
    pub labels: Vec<String>,
    pub assignee: Option<String>,
    pub project_key: Option<String>,
    pub parent_issue: Option<String>,
    pub custom_fields: HashMap<String, serde_json::Value>,
}
```

### IssuePriority

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssuePriority {
    Critical,
    High,
    Medium,
    Low,
    Trivial,
}
```

### Issue Management Example

```rust
use do_codeguardian::integrations::{IntegrationManager, IssueCreationRequest, IssuePriority};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = IntegrationManager::new(config);
    manager.initialize().await?;

    let issue = IssueCreationRequest {
        title: "Security vulnerability found".to_string(),
        description: "Critical authentication bypass vulnerability detected".to_string(),
        issue_type: "Bug".to_string(),
        priority: IssuePriority::Critical,
        labels: vec!["security".to_string(), "critical".to_string()],
        assignee: Some("security-team".to_string()),
        project_key: Some("SEC".to_string()),
        custom_fields: HashMap::from([
            ("severity".to_string(), serde_json::json!("critical")),
            ("affected_version".to_string(), serde_json::json!("v1.2.3")),
        ]),
        ..Default::default()
    };

    let results = manager.create_issue_across_systems(&issue).await?;

    for result in results {
        if result.success {
            println!("Created issue in {}: {}", result.system, result.issue_id.unwrap());
        } else {
            eprintln!("Failed to create issue in {}: {}", result.system, result.error.unwrap());
        }
    }

    Ok(())
}
```

## Workflow Automation

### WorkflowTriggerRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTriggerRequest {
    pub workflow_name: String,
    pub branch: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub trigger_reason: String,
}
```

### WorkflowTriggerResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTriggerResult {
    pub system: String,
    pub success: bool,
    pub workflow_id: Option<String>,
    pub workflow_url: Option<String>,
    pub error: Option<String>,
}
```

### Workflow Automation Example

```rust
use do_codeguardian::integrations::{IntegrationManager, WorkflowTriggerRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = IntegrationManager::new(config);
    manager.initialize().await?;

    let workflow_request = WorkflowTriggerRequest {
        workflow_name: "security-scan".to_string(),
        branch: Some("main".to_string()),
        parameters: HashMap::from([
            ("scan_type".to_string(), serde_json::json!("full")),
            ("severity_threshold".to_string(), serde_json::json!("high")),
        ]),
        trigger_reason: "Automated security scan triggered by CodeGuardian".to_string(),
    };

    let results = manager.trigger_workflows(&workflow_request).await?;

    for result in results {
        if result.success {
            println!("Triggered workflow in {}: {}", result.system, result.workflow_id.unwrap());
        } else {
            eprintln!("Failed to trigger workflow in {}: {}", result.system, result.error.unwrap());
        }
    }

    Ok(())
}
```

## Reporting

### ReportRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub report_type: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub filters: HashMap<String, serde_json::Value>,
    pub include_details: bool,
}
```

### UnifiedReport

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedReport {
    pub generated_at: DateTime<Utc>,
    pub report_type: String,
    pub system_reports: HashMap<String, SystemReport>,
    pub summary: ReportSummary,
}
```

### Reporting Example

```rust
use do_codeguardian::integrations::{IntegrationManager, ReportRequest};
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = IntegrationManager::new(config);
    manager.initialize().await?;

    let report_request = ReportRequest {
        report_type: "duplicate_summary".to_string(),
        start_date: Some(Utc::now() - Duration::days(30)),
        end_date: Some(Utc::now()),
        filters: HashMap::from([
            ("severity".to_string(), serde_json::json!(["high", "critical"])),
            ("status".to_string(), serde_json::json!(["open"])),
        ]),
        include_details: true,
    };

    let report = manager.generate_unified_report(&report_request).await?;

    println!("Unified Report Summary:");
    println!("- Total systems: {}", report.summary.total_systems);
    println!("- Total issues: {}", report.summary.total_issues);
    println!("- Total duplicates: {}", report.summary.total_duplicates);
    println!("- Duplicate rate: {:.1}%", report.summary.duplicate_rate);

    for (system, system_report) in &report.system_reports {
        println!("{}: {} issues, {} duplicates",
                system, system_report.total_issues, system_report.duplicates_found);
    }

    Ok(())
}
```

## Webhooks

### WebhookConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub active: bool,
}
```

### WebhookEvent

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_type: String,
    pub system: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub signature: Option<String>,
}
```

### Webhook Support Trait

```rust
#[async_trait]
pub trait WebhookSupport: ExternalSystemClient {
    async fn register_webhook(&self, config: &WebhookConfig) -> Result<String>;
    async fn unregister_webhook(&self, webhook_id: &str) -> Result<()>;
    async fn list_webhooks(&self) -> Result<Vec<WebhookInfo>>;
    fn verify_webhook_signature(&self, payload: &str, signature: &str, secret: &str) -> bool;
}
```

### Webhook Example

```rust
use do_codeguardian::integrations::{WebhookConfig, WebhookSupport};

// Assuming we have a Jira client that implements WebhookSupport
let webhook_config = WebhookConfig {
    url: "https://codeguardian.example.com/webhooks/jira".to_string(),
    events: vec![
        "issue_created".to_string(),
        "issue_updated".to_string(),
        "issue_deleted".to_string(),
    ],
    secret: Some("webhook-secret".to_string()),
    active: true,
};

let webhook_id = jira_client.register_webhook(&webhook_config).await?;
println!("Registered webhook with ID: {}", webhook_id);

// Later, verify incoming webhook
let payload = r#"{"event":"issue_created","issue":{"id":"123","key":"PROJ-123"}}"#;
let signature = "sha256=abc123...";
let is_valid = jira_client.verify_webhook_signature(payload, &signature, "webhook-secret");
```

## Bulk Operations

### BulkOperationSupport Trait

```rust
#[async_trait]
pub trait BulkOperationSupport: ExternalSystemClient {
    async fn create_issues_bulk(&self, issues: &[IssueCreationRequest]) -> Result<Vec<BulkOperationResult>>;
    async fn update_issues_bulk(&self, updates: &[(String, IssueUpdate)]) -> Result<Vec<BulkOperationResult>>;
    async fn search_duplicates_bulk(&self, queries: &[DuplicateSearchQuery]) -> Result<Vec<Vec<DuplicateSearchResult>>>;
}
```

### BulkOperationResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    pub success: bool,
    pub item_id: Option<String>,
    pub error: Option<String>,
    pub operation_index: usize,
    pub message: Option<String>,
    pub data: Option<Value>,
}
```

### Bulk Operations Example

```rust
use do_codeguardian::integrations::{BulkOperationSupport, IssueCreationRequest};

// Assuming we have a client that implements BulkOperationSupport
let issues = vec![
    IssueCreationRequest {
        title: "Security issue 1".to_string(),
        description: "Description 1".to_string(),
        issue_type: "Bug".to_string(),
        priority: IssuePriority::High,
        ..Default::default()
    },
    IssueCreationRequest {
        title: "Security issue 2".to_string(),
        description: "Description 2".to_string(),
        issue_type: "Bug".to_string(),
        priority: IssuePriority::Medium,
        ..Default::default()
    },
];

let results = bulk_client.create_issues_bulk(&issues).await?;

let successful = results.iter().filter(|r| r.success).count();
println!("Successfully created {}/{} issues", successful, issues.len());

for (i, result) in results.iter().enumerate() {
    if !result.success {
        eprintln!("Failed to create issue {}: {}", i + 1, result.error.as_ref().unwrap());
    }
}
```

## Health Monitoring

### SystemHealth

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub response_time_ms: Option<u64>,
    pub last_error: Option<String>,
    pub features_available: Vec<String>,
}
```

### HealthStatus

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}
```

### IntegrationHealthStatus

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationHealthStatus {
    pub overall_status: HealthStatus,
    pub system_health: HashMap<String, SystemHealth>,
    pub healthy_systems: usize,
    pub total_systems: usize,
    pub last_checked: DateTime<Utc>,
}
```

### Health Monitoring Example

```rust
use do_codeguardian::integrations::IntegrationManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = IntegrationManager::new(config);
    manager.initialize().await?;

    let health_status = manager.get_health_status().await?;

    println!("Integration Health Status: {:?}", health_status.overall_status);
    println!("Healthy systems: {}/{}", health_status.healthy_systems, health_status.total_systems);

    for (system_name, health) in &health_status.system_health {
        println!("{}: {:?} ({:?}ms)",
                system_name,
                health.status,
                health.response_time_ms);

        if let Some(error) = &health.last_error {
            eprintln!("  Last error: {}", error);
        }
    }

    Ok(())
}
```

## Advanced Search

### AdvancedSearchSupport Trait

```rust
#[async_trait]
pub trait AdvancedSearchSupport: ExternalSystemClient {
    async fn semantic_search(&self, query: &SemanticSearchQuery) -> Result<Vec<DuplicateSearchResult>>;
    async fn advanced_search(&self, query: &AdvancedSearchQuery) -> Result<SearchResults>;
    async fn get_search_suggestions(&self, partial_query: &str) -> Result<Vec<SearchSuggestion>>;
}
```

### SemanticSearchQuery

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchQuery {
    pub text: String,
    pub context: Option<String>,
    pub similarity_threshold: f64,
    pub max_results: usize,
    pub include_metadata: bool,
}
```

### Advanced Search Example

```rust
use do_codeguardian::integrations::{AdvancedSearchSupport, SemanticSearchQuery};

// Assuming we have a client that implements AdvancedSearchSupport
let semantic_query = SemanticSearchQuery {
    text: "authentication bypass vulnerability".to_string(),
    context: Some("web application security".to_string()),
    similarity_threshold: 0.7,
    max_results: 20,
    include_metadata: true,
};

let results = advanced_client.semantic_search(&semantic_query).await?;

println!("Found {} semantically similar issues", results.len());
for result in results {
    println!("- {} (similarity: {:.2})", result.title, result.similarity_score);
}
```

## Error Handling

### IntegrationError

```rust
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("System unavailable: {0}")]
    SystemUnavailable(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
```

## Configuration Examples

### Complete Integration Configuration

```toml
[integrations]
enabled = true
default_timeout_seconds = 30
retry_attempts = 3
batch_size = 50

[integrations.jira]
enabled = true
base_url = "https://company.atlassian.net"
auth = { BasicAuth = { username = "user@company.com", token = "jira-api-token" } }
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true, reporting = true, webhooks = true }
rate_limits = { requests_per_minute = 60, burst_limit = 10 }

[integrations.confluence]
enabled = true
base_url = "https://company.atlassian.net/wiki"
auth = { BasicAuth = { username = "user@company.com", token = "confluence-api-token" } }
features = { duplicate_detection = true, reporting = true, webhooks = true }
rate_limits = { requests_per_minute = 60, burst_limit = 10 }

[integrations.gitlab]
enabled = true
base_url = "https://gitlab.company.com"
auth = { Token = { token = "glpat-xxxxxxxxxxxxxx" } }
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true, reporting = true, webhooks = true }
rate_limits = { requests_per_minute = 300, burst_limit = 50 }

[integrations.jenkins]
enabled = true
base_url = "https://jenkins.company.com"
auth = { ApiKey = { key = "jenkins-api-key" } }
features = { workflow_automation = true, reporting = true, webhooks = true }
rate_limits = { requests_per_minute = 120, burst_limit = 20 }

[integrations.bitbucket]
enabled = true
base_url = "https://api.bitbucket.org/2.0"
auth = { OAuth = { client_id = "bitbucket-client-id", client_secret = "bitbucket-client-secret", access_token = "bitbucket-access-token" } }
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true, reporting = true, webhooks = true }
rate_limits = { requests_per_minute = 1000, burst_limit = 100 }

[integrations.azure_devops]
enabled = true
base_url = "https://dev.azure.com/company"
auth = { Token = { token = "azure-devops-pat" } }
features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true, reporting = true, webhooks = true }
rate_limits = { requests_per_minute = 300, burst_limit = 50 }
```

## Best Practices

1. **Rate Limiting**: Respect API rate limits and configure appropriate limits
2. **Error Handling**: Implement proper error handling and retry logic
3. **Authentication**: Use secure authentication methods (tokens over passwords)
4. **Monitoring**: Monitor integration health and performance
5. **Testing**: Test integrations in staging environments first
6. **Documentation**: Document custom fields and system-specific configurations
7. **Security**: Store sensitive credentials securely (environment variables, secret management)
8. **Bulk Operations**: Use bulk operations for efficiency when available
9. **Webhooks**: Implement webhook signature verification for security
10. **Caching**: Cache frequently accessed data to reduce API calls
