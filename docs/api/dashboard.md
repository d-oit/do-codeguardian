# Dashboard API Documentation

This document provides comprehensive API documentation for CodeGuardian's web-based dashboard, introduced in v0.2.0.

## Table of Contents

- [Dashboard Service](#dashboard-service)
- [Dashboard Configuration](#dashboard-configuration)
- [Metrics Collection](#metrics-collection)
- [Custom Views](#custom-views)
- [Real-time Updates](#real-time-updates)
- [Visualization](#visualization)
- [Authentication](#authentication)
- [API Endpoints](#api-endpoints)

## Dashboard Service

### DashboardService

```rust
pub struct DashboardService {
    config: DashboardConfig,
    metrics_history: Vec<DashboardMetrics>,
}
```

**Methods:**
```rust
impl DashboardService {
    pub fn new(config: DashboardConfig) -> Self;
    pub async fn start(&self) -> Result<()>;
    pub fn update_metrics(&mut self, metrics: DashboardMetrics);
    pub fn get_current_metrics(&self) -> Option<&DashboardMetrics>;
    pub fn get_metrics_for_range(&self, range: &TimeRange) -> Vec<&DashboardMetrics>;
    pub fn generate_report(&self, view: &CustomView) -> Result<DashboardReport>;
}
```

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

## Dashboard Configuration

### Basic Configuration

```toml
[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080
refresh_interval_seconds = 30
max_history_days = 30
enable_real_time = true
```

### Custom Views Configuration

```toml
[[dashboard.custom_views]]
name = "Security Overview"
description = "Security-related duplicate detection and prevention"
filters = { category_filter = ["security"], severity_filter = ["high", "critical"] }
charts = [
    { chart_type = "BarChart", title = "Security Duplicates by Type", data_source = "SecurityMetrics" },
    { chart_type = "LineChart", title = "Security Trends", data_source = "VulnerabilityMetrics" }
]
stakeholder_role = "SecurityTeam"

[[dashboard.custom_views]]
name = "Performance Metrics"
description = "System performance and processing metrics"
filters = {}
charts = [
    { chart_type = "LineChart", title = "Processing Time", data_source = "PerformanceMetrics" },
    { chart_type = "AreaChart", title = "Resource Usage", data_source = "ResourceMetrics" }
]
stakeholder_role = "Developer"
```

### Authentication Configuration

```toml
[dashboard.authentication]
enabled = true
auth_type = "basic"
users = [
    { username = "admin", password_hash = "$2b$12$..." },
    { username = "viewer", password_hash = "$2b$12$..." }
]
session_timeout_minutes = 60
```

## Metrics Collection

### DashboardMetrics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub timestamp: DateTime<Utc>,
    pub duplicate_stats: DuplicateStats,
    pub prevention_stats: PreventionStats,
    pub system_health: SystemHealth,
    pub performance_metrics: PerformanceMetrics,
}
```

### DuplicateStats

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateStats {
    pub total_duplicates_found: u64,
    pub duplicates_by_type: HashMap<String, u64>,
    pub duplicates_by_severity: HashMap<String, u64>,
    pub false_positive_rate: f64,
    pub detection_accuracy: f64,
}
```

### PreventionStats

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventionStats {
    pub duplicates_prevented: u64,
    pub duplicates_created: u64,
    pub prevention_rate: f64,
    pub time_saved_hours: f64,
    pub cost_savings_estimate: f64,
}
```

### SystemHealth

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub api_success_rate: f64,
    pub average_response_time_ms: f64,
    pub error_rate: f64,
    pub uptime_percentage: f64,
    pub active_connections: u32,
}
```

### PerformanceMetrics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_processing_time_ms: f64,
    pub throughput_per_minute: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percentage: f64,
    pub queue_length: u32,
}
```

### Metrics Collection Example

```rust
use do_codeguardian::dashboard::{DashboardService, DashboardMetrics, DuplicateStats};

let mut dashboard = DashboardService::new(dashboard_config);

// Collect metrics from analysis results
let duplicate_stats = DuplicateStats {
    total_duplicates_found: 150,
    duplicates_by_type: HashMap::from([
        ("code".to_string(), 120),
        ("documentation".to_string(), 20),
        ("configuration".to_string(), 10),
    ]),
    duplicates_by_severity: HashMap::from([
        ("high".to_string(), 30),
        ("medium".to_string(), 80),
        ("low".to_string(), 40),
    ]),
    false_positive_rate: 0.05,
    detection_accuracy: 0.92,
};

let metrics = DashboardMetrics {
    timestamp: Utc::now(),
    duplicate_stats,
    prevention_stats: PreventionStats {
        duplicates_prevented: 45,
        duplicates_created: 15,
        prevention_rate: 0.75,
        time_saved_hours: 120.5,
        cost_savings_estimate: 25000.0,
    },
    system_health: SystemHealth {
        api_success_rate: 0.98,
        average_response_time_ms: 250.0,
        error_rate: 0.02,
        uptime_percentage: 99.5,
        active_connections: 12,
    },
    performance_metrics: PerformanceMetrics {
        average_processing_time_ms: 450.0,
        throughput_per_minute: 150.0,
        memory_usage_mb: 256.0,
        cpu_usage_percentage: 35.0,
        queue_length: 5,
    },
};

dashboard.update_metrics(metrics);
```

## Custom Views

### CustomView

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomView {
    pub name: String,
    pub description: String,
    pub filters: ViewFilters,
    pub charts: Vec<ChartConfig>,
    pub stakeholder_role: StakeholderRole,
}
```

### ViewFilters

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewFilters {
    pub time_range: Option<TimeRange>,
    pub severity_filter: Option<Vec<String>>,
    pub category_filter: Option<Vec<String>>,
    pub repository_filter: Option<Vec<String>>,
    pub file_type_filter: Option<Vec<String>>,
}
```

### ChartConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub chart_type: ChartType,
    pub title: String,
    pub data_source: DataSource,
    pub refresh_rate: Option<u64>,
}
```

### Chart Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    LineChart,
    BarChart,
    PieChart,
    AreaChart,
    GaugeChart,
    HistogramChart,
    HeatmapChart,
}
```

### Data Sources

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    DuplicateMetrics,
    PreventionStats,
    SystemMetrics,
    SecurityMetrics,
    VulnerabilityMetrics,
    PerformanceMetrics,
    ResourceMetrics,
}
```

### Stakeholder Roles

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakeholderRole {
    Developer,
    SecurityTeam,
    Manager,
    QualityAssurance,
    DevOps,
}
```

### Custom View Example

```rust
use do_codeguardian::dashboard::{CustomView, ViewFilters, ChartConfig, ChartType, DataSource, StakeholderRole};

let security_view = CustomView {
    name: "Security Dashboard".to_string(),
    description: "Comprehensive security metrics and duplicate detection".to_string(),
    filters: ViewFilters {
        time_range: Some(TimeRange::Last7Days),
        severity_filter: Some(vec!["high".to_string(), "critical".to_string()]),
        category_filter: Some(vec!["security".to_string()]),
        repository_filter: None,
        file_type_filter: None,
    },
    charts: vec![
        ChartConfig {
            chart_type: ChartType::LineChart,
            title: "Security Issues Trend".to_string(),
            data_source: DataSource::SecurityMetrics,
            refresh_rate: Some(300), // 5 minutes
        },
        ChartConfig {
            chart_type: ChartType::BarChart,
            title: "Duplicates by Severity".to_string(),
            data_source: DataSource::DuplicateMetrics,
            refresh_rate: Some(600), // 10 minutes
        },
        ChartConfig {
            chart_type: ChartType::GaugeChart,
            title: "System Health".to_string(),
            data_source: DataSource::SystemMetrics,
            refresh_rate: Some(60), // 1 minute
        },
    ],
    stakeholder_role: StakeholderRole::SecurityTeam,
};
```

## Real-time Updates

### Real-time Configuration

```toml
[dashboard]
enable_real_time = true
refresh_interval_seconds = 30
```

### WebSocket Integration

```rust
use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "ws://localhost:8080/ws/metrics";
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Send subscription message
    let subscribe_msg = serde_json::json!({
        "type": "subscribe",
        "metrics": ["duplicate_stats", "system_health"]
    });
    write.send(subscribe_msg.to_string().into()).await?;

    // Listen for real-time updates
    while let Some(message) = read.next().await {
        match message? {
            tokio_tungstenite::tungstenite::Message::Text(text) => {
                let update: serde_json::Value = serde_json::from_str(&text)?;
                println!("Real-time update: {:?}", update);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## Visualization

### GraphVisualizer

```rust
pub struct GraphVisualizer {
    config: VisualizationConfig,
}
```

**Methods:**
```rust
impl GraphVisualizer {
    pub fn new() -> Self;
    pub async fn generate_visualization(&self, search_result: &RelationshipSearchResult) -> Result<GraphVisualization>;
    pub async fn export_to_format(&self, visualization: &GraphVisualization, format: ExportFormat) -> Result<Vec<u8>>;
}
```

### GraphVisualization

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVisualization {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
    pub metadata: VisualizationMetadata,
}
```

### Export Formats

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    GraphML,
    DOT,
    SVG,
    PNG,
}
```

### Visualization Example

```rust
use do_codeguardian::dashboard::GraphVisualizer;
use do_codeguardian::relationships::RelationshipSearchResult;

let visualizer = GraphVisualizer::new();

// Generate visualization from relationship data
let visualization = visualizer.generate_visualization(&search_result).await?;

// Export to different formats
let svg_data = visualizer.export_to_format(&visualization, ExportFormat::SVG).await?;
let png_data = visualizer.export_to_format(&visualization, ExportFormat::PNG).await?;
```

## Authentication

### DashboardAuth

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAuth {
    pub enabled: bool,
    pub auth_type: AuthType,
    pub users: Vec<User>,
    pub session_timeout_minutes: u32,
}
```

### AuthType

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    None,
    Basic,
    Token,
    OAuth,
}
```

### User

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub permissions: Vec<String>,
}
```

### UserRole

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Editor,
    Viewer,
}
```

### Authentication Example

```rust
use do_codeguardian::dashboard::{DashboardAuth, AuthType, User, UserRole};

// Configure authentication
let auth = DashboardAuth {
    enabled: true,
    auth_type: AuthType::Basic,
    users: vec![
        User {
            username: "admin".to_string(),
            password_hash: bcrypt::hash("admin_password", bcrypt::DEFAULT_COST)?,
            role: UserRole::Admin,
            permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
        },
        User {
            username: "viewer".to_string(),
            password_hash: bcrypt::hash("viewer_password", bcrypt::DEFAULT_COST)?,
            role: UserRole::Viewer,
            permissions: vec!["read".to_string()],
        },
    ],
    session_timeout_minutes: 60,
};
```

## API Endpoints

### REST API Endpoints

#### GET /api/metrics/current
Get current dashboard metrics.

**Response:**
```json
{
  "timestamp": "2025-09-17T10:00:00Z",
  "duplicate_stats": {
    "total_duplicates_found": 150,
    "duplicates_by_type": {"code": 120, "documentation": 20},
    "detection_accuracy": 0.92
  },
  "system_health": {
    "api_success_rate": 0.98,
    "uptime_percentage": 99.5
  }
}
```

#### GET /api/metrics/history?range=7d
Get historical metrics for the specified time range.

**Parameters:**
- `range`: Time range (1h, 24h, 7d, 30d, 90d)

#### GET /api/views
Get available dashboard views.

**Response:**
```json
[
  {
    "name": "Security Overview",
    "description": "Security-related metrics",
    "stakeholder_role": "SecurityTeam",
    "charts": [
      {
        "chart_type": "LineChart",
        "title": "Security Trends",
        "data_source": "SecurityMetrics"
      }
    ]
  }
]
```

#### POST /api/views/{name}/report
Generate a report for a specific view.

**Request Body:**
```json
{
  "time_range": "Last30Days",
  "format": "json"
}
```

#### GET /api/health
Get dashboard health status.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.2.1",
  "uptime_seconds": 3600,
  "active_connections": 5
}
```

### WebSocket Endpoints

#### /ws/metrics
Real-time metrics streaming.

**Subscription Message:**
```json
{
  "type": "subscribe",
  "metrics": ["duplicate_stats", "system_health"],
  "update_interval_seconds": 30
}
```

**Update Message:**
```json
{
  "type": "update",
  "timestamp": "2025-09-17T10:00:30Z",
  "data": {
    "duplicate_stats": {
      "total_duplicates_found": 152
    }
  }
}
```

#### /ws/views/{name}
Real-time view updates.

### Authentication Endpoints

#### POST /api/auth/login
Authenticate user.

**Request:**
```json
{
  "username": "admin",
  "password": "password"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2025-09-17T11:00:00Z",
  "user": {
    "username": "admin",
    "role": "Admin"
  }
}
```

#### POST /api/auth/logout
Logout user.

#### GET /api/auth/verify
Verify authentication token.

## Dashboard Report

### DashboardReport

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardReport {
    pub view_name: String,
    pub generated_at: DateTime<Utc>,
    pub summary: DashboardSummary,
    pub charts_data: HashMap<String, serde_json::Value>,
    pub recommendations: Vec<String>,
}
```

### DashboardSummary

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_duplicates_detected: u64,
    pub average_detection_accuracy: f64,
    pub average_prevention_rate: f64,
    pub total_time_saved_hours: f64,
    pub system_uptime: f64,
}
```

### Report Generation Example

```rust
use do_codeguardian::dashboard::{DashboardService, CustomView};

// Generate report for a specific view
let report = dashboard.generate_report(&security_view).await?;

println!("Dashboard Report for: {}", report.view_name);
println!("Generated at: {}", report.generated_at);
println!("Summary:");
println!("- Total duplicates: {}", report.summary.total_duplicates_detected);
println!("- Detection accuracy: {:.1}%", report.summary.average_detection_accuracy * 100.0);
println!("- Time saved: {:.1} hours", report.summary.total_time_saved_hours);

println!("Recommendations:");
for recommendation in &report.recommendations {
    println!("- {}", recommendation);
}
```

## Integration with CodeGuardian

### Automatic Metrics Collection

```rust
use do_codeguardian::dashboard::DashboardService;
use do_codeguardian::analysis::AnalysisResults;

// After analysis, update dashboard metrics
let analysis_results = analyze_files(&files, &config).await?;
let metrics = DashboardMetrics::from_analysis_results(&analysis_results);
dashboard.update_metrics(metrics);
```

### CLI Integration

```bash
# Start dashboard
codeguardian dashboard --host 0.0.0.0 --port 8080

# Start with custom config
codeguardian dashboard --config dashboard.toml

# Start in background
codeguardian dashboard --detach
```

## Performance Considerations

1. **Data Retention**: Configure `max_history_days` to balance storage and performance
2. **Refresh Intervals**: Adjust `refresh_interval_seconds` based on update frequency needs
3. **Real-time Updates**: Enable WebSocket compression for high-frequency updates
4. **Caching**: Implement client-side caching for static dashboard assets
5. **Database Optimization**: Use time-series database for metrics storage
6. **Connection Pooling**: Configure appropriate connection pool sizes
7. **Metrics Aggregation**: Pre-aggregate metrics for faster queries

## Security Considerations

1. **Authentication**: Always enable authentication in production
2. **HTTPS**: Use HTTPS/TLS for dashboard access
3. **Session Management**: Implement proper session timeout and invalidation
4. **CORS**: Configure CORS policies appropriately
5. **Rate Limiting**: Implement rate limiting for API endpoints
6. **Audit Logging**: Log all dashboard access and actions
7. **Data Sanitization**: Sanitize all user inputs and displayed data
8. **CSRF Protection**: Implement CSRF protection for state-changing operations

## Troubleshooting

### Common Issues

1. **Dashboard not starting**: Check port availability and configuration
2. **Real-time updates not working**: Verify WebSocket connection and firewall settings
3. **Metrics not updating**: Check analysis pipeline and error logs
4. **Authentication failures**: Verify user credentials and configuration
5. **Performance issues**: Monitor resource usage and adjust configuration

### Debug Mode

```toml
[dashboard]
debug_mode = true
log_level = "debug"
enable_metrics_logging = true
```

### Health Checks

```bash
# Check dashboard health
curl http://localhost:8080/api/health

# Check WebSocket connectivity
websocat ws://localhost:8080/ws/metrics
```

## Migration from v0.1.0

### Configuration Changes

```toml
# Old configuration (v0.1.0)
[web_interface]
enabled = true
port = 8080

# New configuration (v0.2.0)
[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080
refresh_interval_seconds = 30
max_history_days = 30
enable_real_time = true
```

### API Changes

- **Endpoint Changes**: `/api/v1/*` → `/api/*`
- **Authentication**: Added required authentication headers
- **Real-time**: New WebSocket endpoints for real-time updates
- **Views**: Enhanced view system with stakeholder roles

### Data Migration

```rust
// Migrate old metrics to new format
use do_codeguardian::dashboard::DashboardService;

let old_metrics = load_old_metrics()?;
let new_metrics = old_metrics.into_iter()
    .map(|old| DashboardMetrics::from_old_format(old))
    .collect::<Vec<_>>();

for metric in new_metrics {
    dashboard.update_metrics(metric);
}
```
