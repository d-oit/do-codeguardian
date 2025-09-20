# Release Monitoring Alerts and Notifications

This document describes the comprehensive release monitoring alert system implemented in CodeGuardian, which extends the existing performance monitoring infrastructure to include release-specific alerts and notifications.

## Overview

The release monitoring alert system provides:

1. **Release-specific alert types** for monitoring deployment success, issues, and delays
2. **Real-time monitoring integration** with the ReleaseMonitoringService
3. **Configurable alert thresholds** for different metrics
4. **Multi-channel notification system** (Email, Slack, Webhooks)
5. **Dashboard integration** for visualizing release alerts
6. **Automated alert escalation** and resolution workflows

## Architecture

### Core Components

#### 1. Extended Alert Types
The system extends the existing `AlertType` enum with release-specific alerts:

```rust
pub enum AlertType {
    // Existing performance alerts
    CachePerformance,
    MemoryEfficiency,
    ProcessingPerformance,
    SystemHealth,

    // New release monitoring alerts
    ReleaseSuccessRate,
    PostReleaseIssues,
    DeploymentDelay,
    UserAdoption,
}
```

#### 2. Enhanced Alert Structure
The `PerformanceAlert` struct now includes additional fields for release monitoring:

```rust
pub struct PerformanceAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub threshold: f64,
    pub current_value: f64,
    pub timestamp: DateTime<Utc>,
    pub release_tag: Option<String>,        // NEW: Specific release tag
    pub escalation_level: AlertEscalation,  // NEW: Escalation tracking
    pub acknowledged: bool,                 // NEW: Acknowledgment status
}
```

#### 3. Alert Thresholds Configuration
Configurable thresholds for all alert types:

```rust
pub struct AlertThresholdsConfig {
    pub cache_hit_rate_warning: f64,
    pub memory_reuse_rate_warning: f64,
    pub max_processing_time_critical: f64,
    pub release_success_rate_warning: f64,      // NEW
    pub max_post_release_issues_warning: f64,   // NEW
    pub max_deployment_time_warning: f64,       // NEW
    pub alert_escalation_minutes: u64,
}
```

#### 4. Notification System
Multi-channel notification support:

```rust
pub struct NotificationConfig {
    pub enabled: bool,
    pub email: Option<EmailConfig>,
    pub slack: Option<SlackConfig>,
    pub webhook: Option<WebhookConfig>,
    pub escalation: EscalationConfig,
}
```

## Alert Types

### Release-Specific Alerts

#### 1. Release Success Rate Alert
- **Trigger**: When overall release success rate falls below threshold (default: 95%)
- **Severity**: Warning
- **Message**: "Release success rate is low: X%"
- **Action**: Review release processes and identify failure patterns

#### 2. Post-Release Issues Alert
- **Trigger**: When average post-release issues exceed threshold (default: 5 issues)
- **Severity**: Warning/Critical (based on issue count)
- **Message**: "Average post-release issues is high: X" or "Release X has high post-release issues: Y"
- **Action**: Investigate release quality and testing processes

#### 3. Deployment Delay Alert
- **Trigger**: When deployment time exceeds threshold (default: 60 minutes)
- **Severity**: Warning/Critical (based on delay duration)
- **Message**: "Average deployment time is high: X minutes" or "Release X deployment time is critically high: Y minutes"
- **Action**: Review CI/CD pipeline performance and bottlenecks

#### 4. User Adoption Alert
- **Trigger**: When user adoption score falls below threshold (default: 5.0)
- **Severity**: Info
- **Message**: "User adoption score is low: X"
- **Action**: Review release communication and feature adoption

## Configuration

### Example Configuration

```toml
[performance.monitoring]
enabled = true
metrics_collection = true
reporting_interval_seconds = 60

[performance.monitoring.alert_thresholds]
cache_hit_rate_warning = 0.7
memory_reuse_rate_warning = 0.5
max_processing_time_critical = 2000.0
release_success_rate_warning = 0.95
max_post_release_issues_warning = 5.0
max_deployment_time_warning = 60.0
alert_escalation_minutes = 30

[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.email]
smtp_server = "smtp.gmail.com"
smtp_port = 587
username = "alerts@yourcompany.com"
password = "your-smtp-password"
from_address = "alerts@yourcompany.com"
to_addresses = ["devops@yourcompany.com", "security@yourcompany.com"]
use_tls = true

[performance.monitoring.notifications.slack]
webhook_url = "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK"
channel = "#codeguardian-alerts"
username = "CodeGuardian"
icon_emoji = ":warning:"

[performance.monitoring.notifications.webhook]
url = "https://your-monitoring-system.com/webhook"
headers = [
    ["Authorization", "Bearer your-api-token"],
    ["Content-Type", "application/json"]
]
timeout_seconds = 30

[performance.monitoring.notifications.escalation]
warning_escalation_minutes = 30
critical_escalation_minutes = 15
escalation_contacts = [
    "oncall-devops@yourcompany.com",
    "security-lead@yourcompany.com"
]
max_escalation_level = 3
```

## Integration Points

### 1. Performance Monitoring Service
The `ProductionMonitoringService` now integrates release monitoring:

```rust
pub struct ProductionMonitoringService {
    // ... existing fields ...
    #[cfg(feature = "release-monitoring")]
    release_monitoring: Option<Arc<ReleaseMonitoringService>>,
    alert_thresholds: AlertThresholdsConfig,
    notification_service: Option<Arc<AlertNotificationService>>,
    // ... other fields ...
}
```

### 2. Dashboard Integration
Release alerts are integrated into the dashboard system:

- **Alert History**: Historical view of all alerts
- **Real-time Updates**: Live alert status and notifications
- **Escalation Tracking**: Visual representation of alert escalation levels
- **Release-specific Views**: Dedicated views for release monitoring alerts

### 3. Notification Channels

#### Email Notifications
- HTML-formatted alerts with detailed information
- Configurable recipients and escalation contacts
- SMTP configuration for various email providers

#### Slack Notifications
- Rich message formatting with severity colors
- Interactive buttons for alert acknowledgment
- Channel-specific routing for different alert types

#### Webhook Notifications
- JSON payload with complete alert information
- Custom headers for authentication
- Integration with external monitoring systems

## Alert Escalation

### Escalation Levels
1. **None**: Initial alert state
2. **Escalated**: Warning alerts after escalation timeout
3. **Critical**: Critical alerts or escalated warnings

### Escalation Policies
- **Warning Alerts**: Escalate after 30 minutes (configurable)
- **Critical Alerts**: Escalate after 15 minutes (configurable)
- **Maximum Escalation**: Configurable maximum escalation level
- **Escalation Contacts**: Separate contact lists for different escalation levels

## Usage Examples

### Basic Setup
```rust
use do_codeguardian::{
    config::{Config, AlertThresholdsConfig, NotificationConfig},
    performance::{
        monitoring::ProductionMonitoringService,
        PerformanceMetrics,
    },
};

let config = Config::from_file("codeguardian.toml")?;
let metrics = Arc::new(PerformanceMetrics::new());

let monitoring_service = ProductionMonitoringService::new(
    Arc::clone(&metrics),
    None, // dashboard
    None, // regex_cache
    None, // memory_pools
    None, // release_monitoring
    config.performance.monitoring.alert_thresholds,
    Some(config.performance.monitoring.notifications),
);

// Start monitoring loop
loop {
    monitoring_service.update().await?;
    tokio::time::sleep(Duration::from_secs(60)).await;
}
```

### Custom Alert Handling
```rust
// Check for alerts
let alerts = monitoring_service.check_alerts();

for alert in alerts {
    match alert.alert_type {
        AlertType::ReleaseSuccessRate => {
            // Handle release success rate alerts
            notify_release_team(alert).await?;
        }
        AlertType::PostReleaseIssues => {
            // Handle post-release issue alerts
            create_issue_investigation_ticket(alert).await?;
        }
        _ => {
            // Handle other alert types
        }
    }
}
```

## Testing

The system includes comprehensive tests:

```bash
# Run all performance monitoring tests
cargo test --features release-monitoring,dashboard --lib performance::monitoring

# Run specific alert tests
cargo test test_alert_thresholds_config
cargo test test_performance_alert_creation
```

## Future Enhancements

### Planned Features
1. **Alert Acknowledgment System**: UI for acknowledging and resolving alerts
2. **Alert Dependencies**: Alert relationships and cascading notifications
3. **Machine Learning Integration**: Predictive alerting based on historical patterns
4. **Alert Templates**: Customizable alert message templates
5. **Alert Routing**: Advanced routing rules based on alert type and severity

### Integration Opportunities
1. **PagerDuty Integration**: Direct integration with incident management systems
2. **Microsoft Teams**: Additional notification channel
3. **Jira Integration**: Automatic ticket creation for critical alerts
4. **Metrics Aggregation**: Integration with Prometheus/Grafana for advanced monitoring

## Troubleshooting

### Common Issues

#### Alerts Not Triggering
- Check alert threshold configuration
- Verify release monitoring service is properly configured
- Ensure notification service is enabled

#### Notifications Not Sending
- Verify notification channel configuration (SMTP, webhook URLs)
- Check network connectivity
- Review authentication credentials

#### High Alert Volume
- Adjust alert thresholds to reduce noise
- Implement alert suppression rules
- Review monitoring intervals

### Debug Mode
Enable debug logging to troubleshoot alert processing:

```bash
RUST_LOG=debug cargo run --features release-monitoring,dashboard
```

## Security Considerations

1. **Credential Protection**: Store notification credentials securely
2. **Network Security**: Use HTTPS for webhook endpoints
3. **Access Control**: Limit who can acknowledge and resolve alerts
4. **Audit Logging**: Log all alert actions for compliance
5. **Rate Limiting**: Prevent alert spam with rate limiting

## Performance Impact

The alert system is designed to minimize performance impact:

- **Async Processing**: All notifications are processed asynchronously
- **Efficient Storage**: Alert history uses efficient data structures
- **Configurable Intervals**: Monitoring intervals can be adjusted based on needs
- **Resource Bounds**: Memory usage is bounded and configurable

## Conclusion

The release monitoring alert system provides comprehensive monitoring and notification capabilities that integrate seamlessly with CodeGuardian's existing performance monitoring infrastructure. It enables teams to proactively identify and respond to release issues, improving overall software delivery quality and reliability.
