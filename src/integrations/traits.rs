//! Traits and interfaces for external system integrations

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Main trait for external system clients
#[async_trait]
pub trait ExternalSystemClient: Send + Sync {
    /// Get the system name
    fn system_name(&self) -> &str;

    /// Perform health check
    async fn health_check(&self) -> Result<SystemHealth>;

    /// Search for duplicate issues/content
    async fn search_duplicates(
        &self,
        query: &DuplicateSearchQuery,
    ) -> Result<Vec<DuplicateSearchResult>>;

    /// Create a new issue
    async fn create_issue(&self, issue: &IssueCreationRequest) -> Result<CreatedIssue>;

    /// Update an existing issue
    async fn update_issue(&self, issue_id: &str, update: &IssueUpdate) -> Result<()>;

    /// Close/resolve an issue
    async fn close_issue(&self, issue_id: &str, resolution: &IssueResolution) -> Result<()>;

    /// Trigger a workflow/pipeline
    async fn trigger_workflow(&self, request: &WorkflowTriggerRequest)
        -> Result<TriggeredWorkflow>;

    /// Generate system-specific report
    async fn generate_report(&self, request: &ReportRequest) -> Result<SystemReport>;

    /// Get system capabilities
    fn get_capabilities(&self) -> SystemCapabilities;
}

/// System health information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub response_time_ms: Option<u64>,
    pub last_error: Option<String>,
    pub features_available: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Query for searching duplicates
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

/// Result of duplicate search
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

/// Request to create a new issue
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

/// Created issue information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedIssue {
    pub id: String,
    pub key: Option<String>,
    pub url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Issue update request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssueUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<IssuePriority>,
    pub labels: Option<Vec<String>>,
    pub assignee: Option<String>,
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Issue resolution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueResolution {
    pub resolution_type: String,
    pub comment: Option<String>,
    pub duplicate_of: Option<String>,
}

/// Issue priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssuePriority {
    Critical,
    High,
    Medium,
    Low,
    Trivial,
}

/// Workflow trigger request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTriggerRequest {
    pub workflow_name: String,
    pub branch: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub trigger_reason: String,
}

/// Triggered workflow information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggeredWorkflow {
    pub id: String,
    pub name: String,
    pub url: String,
    pub status: String,
    pub triggered_at: DateTime<Utc>,
}

/// Report generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub report_type: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub filters: HashMap<String, serde_json::Value>,
    pub include_details: bool,
}

/// System-specific report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemReport {
    pub system_name: String,
    pub report_type: String,
    pub generated_at: DateTime<Utc>,
    pub total_issues: u64,
    pub duplicates_found: u64,
    pub duplicate_rate: f64,
    pub time_period: Option<TimePeriod>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub details: Vec<ReportDetail>,
}

/// Time period for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Detailed report information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDetail {
    pub category: String,
    pub count: u64,
    pub percentage: f64,
    pub items: Vec<ReportItem>,
}

/// Individual report item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportItem {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub score: Option<f64>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// System capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    pub supports_issue_creation: bool,
    pub supports_issue_updates: bool,
    pub supports_duplicate_search: bool,
    pub supports_workflow_triggers: bool,
    pub supports_reporting: bool,
    pub supports_webhooks: bool,
    pub supports_bulk_operations: bool,
    pub max_batch_size: Option<usize>,
    pub rate_limits: Option<RateLimitInfo>,
}

/// Rate limiting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
    pub current_usage: Option<u32>,
    pub reset_time: Option<DateTime<Utc>>,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub active: bool,
}

/// Webhook event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_type: String,
    pub system: String,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub signature: Option<String>,
}

/// Trait for systems that support webhooks
#[async_trait]
pub trait WebhookSupport: ExternalSystemClient {
    /// Register a webhook
    async fn register_webhook(&self, config: &WebhookConfig) -> Result<String>;

    /// Unregister a webhook
    async fn unregister_webhook(&self, webhook_id: &str) -> Result<()>;

    /// List registered webhooks
    async fn list_webhooks(&self) -> Result<Vec<WebhookInfo>>;

    /// Verify webhook signature
    fn verify_webhook_signature(&self, payload: &str, signature: &str, secret: &str) -> bool;
}

/// Webhook information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub id: String,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub last_triggered: Option<DateTime<Utc>>,
}

/// Alias for backwards compatibility
pub trait IntegrationSystem: ExternalSystemClient {}
impl<T: ExternalSystemClient> IntegrationSystem for T {}

/// Alias for backwards compatibility
pub trait BulkOperations: BulkOperationSupport {}
impl<T: BulkOperationSupport> BulkOperations for T {}

/// Trait for systems that support bulk operations
#[async_trait]
pub trait BulkOperationSupport: ExternalSystemClient {
    /// Create multiple issues in batch
    async fn create_issues_bulk(
        &self,
        issues: &[IssueCreationRequest],
    ) -> Result<Vec<BulkOperationResult>>;

    /// Update multiple issues in batch
    async fn update_issues_bulk(
        &self,
        updates: &[(String, IssueUpdate)],
    ) -> Result<Vec<BulkOperationResult>>;

    /// Search for duplicates across multiple queries
    async fn search_duplicates_bulk(
        &self,
        queries: &[DuplicateSearchQuery],
    ) -> Result<Vec<Vec<DuplicateSearchResult>>>;
}

/// Result of bulk operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    pub success: bool,
    pub item_id: Option<String>,
    pub error: Option<String>,
    pub operation_index: usize,
    pub message: Option<String>,
    pub data: Option<Value>,
}

/// Trait for advanced search capabilities
#[async_trait]
pub trait AdvancedSearchSupport: ExternalSystemClient {
    /// Perform semantic search using ML models
    async fn semantic_search(
        &self,
        query: &SemanticSearchQuery,
    ) -> Result<Vec<DuplicateSearchResult>>;

    /// Search with custom filters and sorting
    async fn advanced_search(&self, query: &AdvancedSearchQuery) -> Result<SearchResults>;

    /// Get search suggestions
    async fn get_search_suggestions(&self, partial_query: &str) -> Result<Vec<SearchSuggestion>>;
}

/// Semantic search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchQuery {
    pub text: String,
    pub context: Option<String>,
    pub similarity_threshold: f64,
    pub max_results: usize,
    pub include_metadata: bool,
}

/// Advanced search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSearchQuery {
    pub query: String,
    pub filters: HashMap<String, serde_json::Value>,
    pub sort_by: Vec<SortCriteria>,
    pub facets: Vec<String>,
    pub pagination: PaginationOptions,
}

/// Sort criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortCriteria {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Pagination options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationOptions {
    pub offset: usize,
    pub limit: usize,
}

/// Search results with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<DuplicateSearchResult>,
    pub total_count: usize,
    pub facets: HashMap<String, Vec<FacetValue>>,
    pub query_time_ms: u64,
    pub suggestions: Vec<SearchSuggestion>,
}

/// Facet value for search filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetValue {
    pub value: String,
    pub count: usize,
}

/// Search suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub score: f64,
    pub category: Option<String>,
}

/// Error types for integration operations
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
