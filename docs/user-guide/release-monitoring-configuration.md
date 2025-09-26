# Release Monitoring Configuration Guide

This comprehensive guide covers the configuration of CodeGuardian's release monitoring system, which provides automated tracking and alerting for software releases, deployment metrics, and post-release issues.

## Overview

The release monitoring system integrates with GitHub to track release metrics including:
- Release success rates and deployment times
- Post-release issue tracking and analysis
- User adoption patterns based on download counts
- Automated alerting for release-related issues

## Prerequisites

- **Feature Flag**: Enable the `release-monitoring` feature when building CodeGuardian
- **GitHub Access**: Valid GitHub token with repository read access
- **Permissions**: Repository access to the target GitHub repository

## Basic Configuration

### Minimal Setup

```toml
# Basic release monitoring configuration
[release_monitoring]
repository = "your-org/your-repo"
max_releases_to_monitor = 10
post_release_issue_window_days = 30
metrics_storage_path = "release_metrics.json"
enable_real_time = false
monitoring_interval_seconds = 3600
```

### Quick Start Example

```toml
# Complete minimal configuration for release monitoring
[release_monitoring]
repository = "microsoft/vscode"
max_releases_to_monitor = 5
post_release_issue_window_days = 14
metrics_storage_path = "./metrics/release_metrics.json"
enable_real_time = true
monitoring_interval_seconds = 1800

# Enable performance monitoring with release alerts
[performance.monitoring]
enabled = true
metrics_collection = true
reporting_interval_seconds = 60

# Basic alert thresholds
[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.95
max_post_release_issues_warning = 3.0
max_deployment_time_warning = 30.0
```

## Advanced Configuration Options

### Core Release Monitoring Settings

```toml
[release_monitoring]
# GitHub repository in format "owner/repo"
repository = "your-org/your-repo"

# Number of recent releases to monitor (1-50)
max_releases_to_monitor = 20

# Days to look back for post-release issues (7-90)
post_release_issue_window_days = 30

# Path to store metrics data (relative or absolute)
metrics_storage_path = "./data/release_metrics.json"

# Enable real-time monitoring (requires dashboard feature)
enable_real_time = true

# Monitoring interval in seconds (300-86400)
monitoring_interval_seconds = 3600

# Custom GitHub API endpoint (for GitHub Enterprise)
github_api_url = "https://api.github.com"

# GitHub token (can also use CODEGUARDIAN_GITHUB_TOKEN env var)
github_token = "${CODEGUARDIAN_GITHUB_TOKEN}"
```

### Alert Thresholds Configuration

```toml
[performance.monitoring.alert_thresholds]
# Release success rate warning threshold (0.0-1.0)
release_success_rate_warning = 0.95

# Maximum post-release issues before warning (0.0-50.0)
max_post_release_issues_warning = 5.0

# Maximum deployment time in minutes before warning
max_deployment_time_warning = 60.0

# User adoption score warning threshold (0.0-10.0)
user_adoption_warning_threshold = 3.0

# Alert escalation time in minutes
alert_escalation_minutes = 30

# Cache performance warning threshold (0.0-1.0)
cache_hit_rate_warning = 0.7

# Memory reuse warning threshold (0.0-1.0)
memory_reuse_rate_warning = 0.5

# Maximum processing time before critical alert (milliseconds)
max_processing_time_critical = 2000.0
```

### Notification Configuration

#### Email Notifications

```toml
[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.email]
smtp_server = "smtp.gmail.com"
smtp_port = 587
username = "alerts@yourcompany.com"
password = "${SMTP_PASSWORD}"
from_address = "codeguardian@yourcompany.com"
to_addresses = [
    "devops@yourcompany.com",
    "release-team@yourcompany.com",
    "security@yourcompany.com"
]
use_tls = true
use_starttls = true
```

#### Slack Notifications

```toml
[performance.monitoring.notifications.slack]
webhook_url = "${SLACK_WEBHOOK_URL}"
channel = "#codeguardian-alerts"
username = "CodeGuardian Release Monitor"
icon_emoji = ":rocket:"
mention_users = ["@release-manager", "@devops-lead"]
```

#### Webhook Notifications

```toml
[performance.monitoring.notifications.webhook]
url = "https://your-monitoring-system.com/webhook/codeguardian"
method = "POST"
headers = [
    ["Authorization", "Bearer ${WEBHOOK_TOKEN}"],
    ["Content-Type", "application/json"],
    ["X-Source", "codeguardian"]
]
timeout_seconds = 30
retry_attempts = 3
retry_delay_seconds = 5
```

### Escalation Policies

```toml
[performance.monitoring.notifications.escalation]
# Time before escalating warning alerts (minutes)
warning_escalation_minutes = 30

# Time before escalating critical alerts (minutes)
critical_escalation_minutes = 15

# Maximum escalation level
max_escalation_level = 3

# Escalation contacts by level
escalation_contacts = [
    ["devops@yourcompany.com", "release-manager@yourcompany.com"],
    ["engineering-director@yourcompany.com", "security-lead@yourcompany.com"],
    ["ceo@yourcompany.com", "cto@yourcompany.com"]
]

# Auto-escalation based on alert severity
auto_escalate_critical = true
auto_escalate_warnings = false
```

## Environment-Specific Configurations

### Development Environment

```toml
# Development configuration - more verbose, shorter intervals
[release_monitoring]
repository = "your-org/your-repo"
max_releases_to_monitor = 3
post_release_issue_window_days = 7
metrics_storage_path = "./dev/release_metrics.json"
enable_real_time = true
monitoring_interval_seconds = 300  # 5 minutes

[performance.monitoring]
enabled = true
reporting_interval_seconds = 30

[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.90  # More lenient for dev
max_post_release_issues_warning = 2.0
max_deployment_time_warning = 15.0

[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.slack]
channel = "#dev-alerts"
webhook_url = "${DEV_SLACK_WEBHOOK}"
```

### Staging Environment

```toml
# Staging configuration - balanced monitoring
[release_monitoring]
repository = "your-org/your-repo-staging"
max_releases_to_monitor = 10
post_release_issue_window_days = 14
metrics_storage_path = "./staging/release_metrics.json"
enable_real_time = true
monitoring_interval_seconds = 900  # 15 minutes

[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.95
max_post_release_issues_warning = 3.0
max_deployment_time_warning = 45.0

[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.email]
to_addresses = ["staging-team@yourcompany.com", "qa@yourcompany.com"]
```

### Production Environment

```toml
# Production configuration - strict monitoring and alerting
[release_monitoring]
repository = "your-org/your-repo"
max_releases_to_monitor = 25
post_release_issue_window_days = 30
metrics_storage_path = "/var/lib/codeguardian/release_metrics.json"
enable_real_time = true
monitoring_interval_seconds = 1800  # 30 minutes

[performance.monitoring]
enabled = true
reporting_interval_seconds = 60

[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.98  # Strict success rate
max_post_release_issues_warning = 5.0
max_deployment_time_warning = 60.0
alert_escalation_minutes = 15  # Faster escalation

[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.email]
to_addresses = [
    "production@yourcompany.com",
    "incident-response@yourcompany.com",
    "executive-team@yourcompany.com"
]

[performance.monitoring.notifications.slack]
channel = "#production-alerts"
webhook_url = "${PROD_SLACK_WEBHOOK}"
mention_users = ["@oncall-engineer", "@release-coordinator"]

[performance.monitoring.notifications.escalation]
warning_escalation_minutes = 20
critical_escalation_minutes = 10
max_escalation_level = 5
```

## Dashboard Integration

### Basic Dashboard Configuration

```toml
[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080
refresh_interval_seconds = 30
max_history_days = 30
enable_real_time = true

# Release monitoring dashboard config
[dashboard.release_monitoring_config]
repository = "your-org/your-repo"
max_releases_to_monitor = 20
post_release_issue_window_days = 30
metrics_storage_path = "./dashboard/release_metrics.json"
enable_real_time = true
monitoring_interval_seconds = 3600

# Dashboard authentication (optional)
[dashboard.auth]
enabled = true
username = "admin"
password = "${DASHBOARD_PASSWORD}"
session_timeout_minutes = 60
```

### Advanced Dashboard Features

```toml
[dashboard]
enabled = true
host = "0.0.0.0"
port = 8080
refresh_interval_seconds = 15
max_history_days = 90
enable_real_time = true

# SSL/TLS configuration
[dashboard.tls]
enabled = true
cert_file = "/etc/ssl/certs/codeguardian.crt"
key_file = "/etc/ssl/private/codeguardian.key"

# Advanced release monitoring dashboard
[dashboard.release_monitoring_config]
repository = "your-org/your-repo"
max_releases_to_monitor = 50
post_release_issue_window_days = 60
metrics_storage_path = "/var/lib/codeguardian/dashboard_metrics.json"
enable_real_time = true
monitoring_interval_seconds = 1800

# Custom dashboard settings
[dashboard.customization]
theme = "dark"
timezone = "America/New_York"
date_format = "YYYY-MM-DD HH:mm:ss"
max_chart_points = 1000
enable_export = true
```

## Configuration Validation

### Validation Rules

CodeGuardian validates release monitoring configuration with these rules:

1. **Repository Format**: Must be in "owner/repo" format
2. **Numeric Ranges**:
   - `max_releases_to_monitor`: 1-50
   - `post_release_issue_window_days`: 7-90
   - `monitoring_interval_seconds`: 300-86400
3. **File Paths**: Must be writable and valid paths
4. **Alert Thresholds**: Must be within logical ranges
5. **GitHub Token**: Must be valid (if provided)

### Validation Commands

```bash
# Validate configuration file
codeguardian --config your-config.toml --validate

# Validate with verbose output
codeguardian --config your-config.toml --validate --verbose

# Check GitHub connectivity
codeguardian release-monitoring --repo your-org/your-repo --check-connection
```

### Common Validation Errors

```text
Error: Invalid repository format. Expected: owner/repo, Got: invalid-repo-format
Error: max_releases_to_monitor must be between 1 and 50, Got: 100
Error: monitoring_interval_seconds must be between 300 and 86400, Got: 100
Error: metrics_storage_path is not writable: /readonly/path/metrics.json
Error: GitHub token is invalid or expired
```

## Best Practices

### Security Best Practices

1. **Token Management**:
   ```toml
   # Use environment variables for sensitive data
   github_token = "${CODEGUARDIAN_GITHUB_TOKEN}"
   ```

2. **File Permissions**:
   ```bash
   # Set appropriate permissions for metrics storage
   chmod 600 release_metrics.json
   chown codeguardian:codeguardian release_metrics.json
   ```

3. **Network Security**:
   ```toml
   # Use HTTPS for webhook endpoints
   [performance.monitoring.notifications.webhook]
   url = "https://secure-endpoint.com/webhook"
   ```

### Performance Best Practices

1. **Monitoring Intervals**:
   ```toml
   # Balance between real-time updates and API rate limits
   monitoring_interval_seconds = 1800  # 30 minutes
   ```

2. **Resource Limits**:
   ```toml
   # Limit concurrent operations
   max_parallel_workers = 4
   max_findings_per_file = 50
   ```

3. **Caching Strategy**:
   ```toml
   # Enable caching for better performance
   enable_file_caching = true
   cache_expiration_days = 7
   ```

### Operational Best Practices

1. **Alert Tuning**:
   ```toml
   # Start with reasonable thresholds and adjust based on data
   release_success_rate_warning = 0.95
   max_post_release_issues_warning = 3.0
   ```

2. **Escalation Policies**:
   ```toml
   # Define clear escalation paths
   [performance.monitoring.notifications.escalation]
   warning_escalation_minutes = 30
   critical_escalation_minutes = 15
   ```

3. **Backup and Recovery**:
   ```toml
   # Enable data persistence and backup
   metrics_storage_path = "/var/lib/codeguardian/metrics/"
   enable_auto_backup = true
   backup_interval_hours = 24
   ```

## Integration with Main Configuration

### Merging Configurations

CodeGuardian supports configuration file merging for different environments:

```bash
# Base configuration
codeguardian --config codeguardian.toml

# Environment-specific overrides
codeguardian --config codeguardian.toml --config codeguardian.prod.toml

# Multiple configuration files
codeguardian --config base.toml --config release-monitoring.toml --config alerts.toml
```

### Configuration Inheritance

```toml
# base.toml - Common settings
[release_monitoring]
repository = "your-org/your-repo"
max_releases_to_monitor = 10

[performance.monitoring]
enabled = true

# prod-overrides.toml - Production specific
[release_monitoring]
max_releases_to_monitor = 25  # Override base value
monitoring_interval_seconds = 1800

[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.98  # Stricter in prod
```

## Troubleshooting

### Common Issues

1. **GitHub API Rate Limiting**:
   ```toml
   # Increase monitoring interval to reduce API calls
   monitoring_interval_seconds = 3600
   ```

2. **Metrics Storage Issues**:
   ```toml
   # Ensure storage path is writable
   metrics_storage_path = "./writable/path/metrics.json"
   ```

3. **Alert Spam**:
   ```toml
   # Adjust thresholds to reduce false positives
   max_post_release_issues_warning = 5.0
   alert_escalation_minutes = 60
   ```

### Debug Configuration

```toml
# Enable debug logging
[logging]
level = "debug"
enable_file_logging = true
log_file = "./logs/codeguardian.log"

# Enable performance profiling
[performance]
enable_profiling = true
profiling_output_path = "./profiles/"
```

## CLI Usage Examples

### Basic Release Monitoring

```bash
# Collect release metrics
codeguardian release-monitoring --repo your-org/your-repo collect

# Show current metrics
codeguardian release-monitoring --repo your-org/your-repo show

# Show trends over last 30 days
codeguardian release-monitoring --repo your-org/your-repo trends --days 30

# Export metrics to JSON
codeguardian release-monitoring --repo your-org/your-repo export --output metrics.json
```

### Advanced CLI Usage

```bash
# Use custom configuration
codeguardian --config custom-release-config.toml release-monitoring --repo your-org/your-repo collect

# Enable verbose output
codeguardian --verbose release-monitoring --repo your-org/your-repo show

# Run with specific feature flags
cargo run --features release-monitoring,dashboard -- release-monitoring --repo your-org/your-repo collect
```

## Conceptual Overview

### What is Release Monitoring?

Release monitoring in CodeGuardian is a comprehensive system designed to track, analyze, and alert on software release metrics and post-release issues. It provides insights into deployment success rates, issue patterns, and user adoption trends to help development teams maintain high-quality software delivery.

#### Key Concepts

- **Release Success Rate**: Percentage of successful deployments without critical issues
- **Post-Release Issues**: Bugs, security vulnerabilities, or performance problems discovered after release
- **User Adoption Metrics**: Download counts, usage statistics, and feedback indicators
- **Deployment Time**: Time from code commit to successful production deployment
- **Issue Window**: Configurable period after release to monitor for related problems

#### Architecture Components

1. **Data Collection Layer**: Integrates with GitHub API to gather release and issue data
2. **Metrics Processing Engine**: Analyzes collected data and calculates key performance indicators
3. **Alert System**: Triggers notifications based on configurable thresholds
4. **Dashboard Integration**: Provides real-time visualization of release metrics
5. **Historical Analysis**: Tracks trends and patterns over time

#### Benefits

- **Proactive Issue Detection**: Identify problems before they impact users
- **Quality Assurance**: Maintain consistent release quality standards
- **Performance Optimization**: Reduce deployment times and improve success rates
- **Stakeholder Communication**: Provide transparent metrics to management and teams
- **Continuous Improvement**: Use historical data to refine development processes

## Usage Tutorials

### Tutorial 1: Basic Release Monitoring Setup

#### Step 1: Configure Repository Access

```bash
# Create a GitHub personal access token with repo permissions
# Visit: https://github.com/settings/tokens
# Required scopes: repo, read:org

# Set environment variable
export CODEGUARDIAN_GITHUB_TOKEN=your_github_token_here
```

#### Step 2: Create Basic Configuration

```toml
# release-monitoring.toml
[release_monitoring]
repository = "your-org/your-repo"
max_releases_to_monitor = 10
post_release_issue_window_days = 30
enable_real_time = false
monitoring_interval_seconds = 3600

[performance.monitoring]
enabled = true
```

#### Step 3: Run Initial Collection

```bash
# Collect release metrics
codeguardian release-monitoring --config release-monitoring.toml collect

# View current metrics
codeguardian release-monitoring --config release-monitoring.toml show
```

#### Step 4: Set Up Basic Alerts

```toml
[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.95
max_post_release_issues_warning = 3.0

[performance.monitoring.notifications.email]
smtp_server = "smtp.gmail.com"
smtp_port = 587
username = "alerts@yourcompany.com"
password = "${SMTP_PASSWORD}"
to_addresses = ["devops@yourcompany.com"]
use_tls = true
```

### Tutorial 2: Advanced Monitoring with Dashboard

#### Step 1: Enable Dashboard Integration

```toml
[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080
enable_real_time = true

[dashboard.release_monitoring_config]
repository = "your-org/your-repo"
max_releases_to_monitor = 20
enable_real_time = true
```

#### Step 2: Configure Multiple Alert Channels

```toml
[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.slack]
webhook_url = "${SLACK_WEBHOOK_URL}"
channel = "#release-alerts"
username = "Release Monitor"

[performance.monitoring.notifications.email]
to_addresses = ["release-team@yourcompany.com", "management@yourcompany.com"]
```

#### Step 3: Start Monitoring Service

```bash
# Start dashboard with release monitoring
codeguardian dashboard --config advanced-monitoring.toml
```

#### Step 4: Access Dashboard

Open browser to `http://127.0.0.1:8080` and navigate to the Release Monitoring section.

### Tutorial 3: Custom Alert Policies

#### Step 1: Define Escalation Rules

```toml
[performance.monitoring.notifications.escalation]
warning_escalation_minutes = 30
critical_escalation_minutes = 15
max_escalation_level = 3

escalation_contacts = [
    ["devops@yourcompany.com"],
    ["engineering-director@yourcompany.com"],
    ["ceo@yourcompany.com"]
]
```

#### Step 2: Set Environment-Specific Thresholds

```toml
# Production - strict thresholds
[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.98
max_deployment_time_warning = 60.0

# Staging - relaxed thresholds
[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.90
max_deployment_time_warning = 120.0
```

#### Step 3: Test Alert System

```bash
# Trigger test alert
codeguardian release-monitoring --config alert-config.toml test-alerts
```

## Output Interpretation

### Release Metrics Output

When running `codeguardian release-monitoring show`, you'll see output like:

```json
{
  "repository": "your-org/your-repo",
  "total_releases": 15,
  "success_rate": 0.933,
  "average_deployment_time_minutes": 45.2,
  "post_release_issues": {
    "total": 8,
    "critical": 2,
    "high": 3,
    "medium": 2,
    "low": 1
  },
  "user_adoption_score": 7.8,
  "trends": {
    "success_rate_trend": "improving",
    "issue_trend": "stable",
    "deployment_time_trend": "improving"
  }
}
```

#### Key Metrics Explained

- **success_rate**: 0.933 = 93.3% of releases were successful
- **average_deployment_time_minutes**: 45.2 minutes from commit to production
- **post_release_issues**: Issues found within the monitoring window
- **user_adoption_score**: 0-10 scale based on download trends and feedback
- **trends**: Direction of metric changes (improving/stable/declining)

### Alert Output Interpretation

Alert notifications contain:

```text
🚨 RELEASE MONITORING ALERT 🚨

Repository: your-org/your-repo
Alert Type: Release Success Rate Warning
Current Value: 92.1%
Threshold: 95.0%
Status: Active

Recent Releases:
- v1.2.3: Success (2024-01-15)
- v1.2.2: Failed - Post-release issues detected (2024-01-10)
- v1.2.1: Success (2024-01-05)

Recommendations:
1. Review post-release issues for v1.2.2
2. Consider delaying next release until issues are resolved
3. Review deployment pipeline for potential improvements
```

#### Alert Severity Levels

- **Info**: Informational updates, no action required
- **Warning**: Potential issues, review recommended
- **Critical**: Immediate attention required, may impact production
- **Escalation**: Issue has been escalated to higher management

### Trend Analysis Output

```json
{
  "period": "30 days",
  "metrics": {
    "success_rate": {
      "current": 0.95,
      "previous": 0.92,
      "change": "+3.3%",
      "trend": "improving"
    },
    "deployment_time": {
      "current": 42.5,
      "previous": 48.1,
      "change": "-11.6%",
      "trend": "improving"
    }
  },
  "insights": [
    "Success rate improved by 3.3% this month",
    "Deployment time reduced by 11.6% through pipeline optimizations",
    "Post-release issues decreased by 25%"
  ]
}
```

## API Details

### Programmatic Access

CodeGuardian provides a Rust API for programmatic access to release monitoring:

```rust
use codeguardian::release_monitoring::{ReleaseMonitor, ReleaseMonitorConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = ReleaseMonitorConfig {
        repository: "your-org/your-repo".to_string(),
        max_releases_to_monitor: 20,
        post_release_issue_window_days: 30,
        ..Default::default()
    };

    // Initialize monitor
    let monitor = ReleaseMonitor::new(config).await?;

    // Collect metrics
    let metrics = monitor.collect_metrics().await?;
    println!("Success rate: {:.2}%", metrics.success_rate * 100.0);

    // Check thresholds
    let alerts = monitor.check_alerts(&metrics).await?;
    for alert in alerts {
        println!("Alert: {} - {}", alert.alert_type, alert.message);
    }

    Ok(())
}
```

### REST API Endpoints

When dashboard is enabled, the following REST endpoints are available:

#### GET /api/v1/release-monitoring/metrics

Returns current release metrics.

```json
{
  "status": "success",
  "data": {
    "repository": "your-org/your-repo",
    "metrics": { /* metrics object */ },
    "last_updated": "2024-01-15T10:30:00Z"
  }
}
```

#### GET /api/v1/release-monitoring/alerts

Returns active alerts.

```json
{
  "status": "success",
  "data": {
    "alerts": [
      {
        "id": "alert-123",
        "type": "warning",
        "message": "Release success rate below threshold",
        "created_at": "2024-01-15T09:15:00Z",
        "escalation_level": 1
      }
    ]
  }
}
```

#### POST /api/v1/release-monitoring/collect

Triggers manual metrics collection.

```json
{
  "status": "success",
  "message": "Metrics collection initiated"
}
```

### Webhook Integration

Configure webhooks to receive real-time alerts:

```rust
use codeguardian::integrations::webhooks::{WebhookConfig, WebhookClient};

let webhook_config = WebhookConfig {
    url: "https://your-monitoring-system.com/webhook".to_string(),
    secret: "your-webhook-secret".to_string(),
    events: vec!["alert.created".to_string(), "metrics.updated".to_string()],
};

let client = WebhookClient::new(webhook_config);
client.register_webhook().await?;
```

## Examples

### Example 1: Enterprise Release Monitoring

```toml
# enterprise-release-monitoring.toml
[release_monitoring]
repository = "enterprise-org/product-repo"
max_releases_to_monitor = 50
post_release_issue_window_days = 60
enable_real_time = true
monitoring_interval_seconds = 1800
github_api_url = "https://api.github.com"

[performance.monitoring]
enabled = true
reporting_interval_seconds = 300

[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.98
max_post_release_issues_warning = 5.0
max_deployment_time_warning = 45.0
user_adoption_warning_threshold = 4.0

[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.email]
smtp_server = "smtp.corp.company.com"
smtp_port = 587
username = "release-monitor@company.com"
from_address = "release-monitor@company.com"
to_addresses = [
    "release-engineering@company.com",
    "product-management@company.com",
    "executive-team@company.com"
]
use_tls = true

[performance.monitoring.notifications.slack]
webhook_url = "${SLACK_WEBHOOK_URL}"
channel = "#release-monitoring"
username = "Enterprise Release Monitor"

[performance.monitoring.notifications.escalation]
warning_escalation_minutes = 30
critical_escalation_minutes = 10
max_escalation_level = 4
escalation_contacts = [
    ["release-lead@company.com", "devops-manager@company.com"],
    ["engineering-director@company.com", "qa-director@company.com"],
    ["vp-engineering@company.com", "cpo@company.com"],
    ["ceo@company.com", "cto@company.com"]
]

[dashboard]
enabled = true
host = "0.0.0.0"
port = 8080
enable_real_time = true
max_history_days = 90

[dashboard.auth]
enabled = true
username = "${DASHBOARD_USER}"
password = "${DASHBOARD_PASS}"

[dashboard.tls]
enabled = true
cert_file = "/etc/ssl/certs/release-monitor.crt"
key_file = "/etc/ssl/private/release-monitor.key"
```

### Example 2: Open Source Project Monitoring

```toml
# oss-release-monitoring.toml
[release_monitoring]
repository = "open-source-org/tool-repo"
max_releases_to_monitor = 25
post_release_issue_window_days = 30
enable_real_time = true
monitoring_interval_seconds = 3600

[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.90
max_post_release_issues_warning = 10.0
max_deployment_time_warning = 120.0

[performance.monitoring.notifications]
enabled = true

[performance.monitoring.notifications.email]
to_addresses = ["maintainers@open-source.org", "security@open-source.org"]

[performance.monitoring.notifications.webhook]
url = "YOUR_SLACK_WEBHOOK_URL"
method = "POST"
headers = [["Content-Type", "application/json"]]
```

### Example 3: CI/CD Pipeline Integration

```yaml
# .github/workflows/release-monitoring.yml
name: Release Monitoring
on:
  release:
    types: [published]
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours

jobs:
  monitor:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run Release Monitoring
        env:
          CODEGUARDIAN_GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          codeguardian release-monitoring --config .codeguardian/release-monitoring.toml collect

      - name: Check for Alerts
        run: |
          alerts=$(codeguardian release-monitoring --config .codeguardian/release-monitoring.toml check-alerts --format json)
          if [ -n "$alerts" ]; then
            echo "alerts_found=true" >> $GITHUB_ENV
            echo "$alerts" > alerts.json
          fi

      - name: Create Issue on Alert
        if: env.alerts_found == 'true'
        uses: actions/github-script@v7
        with:
          script: |
            const alerts = require('./alerts.json');
            await github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: 'Release Monitoring Alert',
              body: `Alert details: ${JSON.stringify(alerts, null, 2)}`,
              labels: ['alert', 'release-monitoring']
            });
```

## Maintenance Topics

### Regular Maintenance Tasks

#### Weekly Tasks

1. **Review Alert History**
   ```bash
   # Check recent alerts
   codeguardian release-monitoring --config monitoring.toml alert-history --days 7
   ```

2. **Update Thresholds Based on Trends**
   ```bash
   # Analyze trends and adjust thresholds
   codeguardian release-monitoring --config monitoring.toml analyze-trends --period 30d
   ```

3. **Verify Integration Health**
   ```bash
   # Check GitHub API connectivity
   codeguardian release-monitoring --config monitoring.toml health-check
   ```

#### Monthly Tasks

1. **Archive Old Metrics**
   ```bash
   # Archive metrics older than 90 days
   codeguardian release-monitoring --config monitoring.toml archive --older-than 90d
   ```

2. **Review and Update Alert Policies**
   - Analyze false positive rates
   - Adjust escalation policies based on response times
   - Update contact lists for personnel changes

3. **Performance Optimization**
   ```bash
   # Optimize monitoring intervals based on usage patterns
   codeguardian release-monitoring --config monitoring.toml optimize-intervals
   ```

#### Quarterly Tasks

1. **Security Review**
   - Rotate GitHub tokens
   - Review access permissions
   - Update SSL certificates

2. **Process Improvement**
   - Analyze release success patterns
   - Identify bottlenecks in deployment pipeline
   - Implement process improvements based on metrics

### Troubleshooting Common Issues

#### Metrics Collection Failures

**Symptom**: "Failed to collect release metrics"

**Possible Causes**:
- GitHub API rate limiting
- Invalid repository permissions
- Network connectivity issues

**Solutions**:
```bash
# Check API rate limits
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/rate_limit

# Verify repository access
codeguardian release-monitoring --config config.toml test-connection

# Increase monitoring interval
[release_monitoring]
monitoring_interval_seconds = 7200  # Increase to 2 hours
```

#### Alert Spam

**Symptom**: Too many alerts, many false positives

**Solutions**:
```toml
# Adjust thresholds
[performance.monitoring.alert_thresholds]
release_success_rate_warning = 0.90  # More lenient
max_post_release_issues_warning = 10.0  # Higher threshold

# Implement alert cooldown
[performance.monitoring]
alert_cooldown_minutes = 60  # No duplicate alerts within 1 hour
```

#### Performance Issues

**Symptom**: Monitoring impacting system performance

**Solutions**:
```toml
# Reduce monitoring frequency
[release_monitoring]
monitoring_interval_seconds = 3600  # Less frequent checks

# Limit data collection
[release_monitoring]
max_releases_to_monitor = 10  # Fewer releases

# Enable caching
[performance.monitoring]
enable_caching = true
cache_ttl_seconds = 1800
```

### Backup and Recovery

#### Configuration Backup

```bash
# Backup configuration
cp release-monitoring.toml release-monitoring.toml.backup

# Backup metrics data
codeguardian release-monitoring --config config.toml export-metrics --output backup-metrics.json
```

#### Recovery Procedures

1. **Configuration Loss**
   ```bash
   # Restore from backup
   cp release-monitoring.toml.backup release-monitoring.toml
   ```

2. **Metrics Data Loss**
   ```bash
   # Re-collect recent data
   codeguardian release-monitoring --config config.toml collect --since 2024-01-01
   ```

3. **System Migration**
   ```bash
   # Export all data
   codeguardian release-monitoring --config old-config.toml export-all --output migration-data.json

   # Import to new system
   codeguardian release-monitoring --config new-config.toml import --input migration-data.json
   ```

### Monitoring System Health

#### Health Checks

```bash
# Overall system health
codeguardian release-monitoring --config config.toml health

# Component-specific checks
codeguardian release-monitoring --config config.toml check-api
codeguardian release-monitoring --config config.toml check-storage
codeguardian release-monitoring --config config.toml check-alerts
```

#### Performance Metrics

Monitor the monitoring system itself:

```json
{
  "monitoring_system_metrics": {
    "cpu_usage_percent": 5.2,
    "memory_usage_mb": 45.8,
    "api_calls_per_minute": 12.3,
    "error_rate_percent": 0.1,
    "average_response_time_ms": 250.5
  }
}
```

## Migration Guide

### Upgrading from Basic to Advanced Configuration

1. **Start with Basic Setup**:
   ```toml
   [release_monitoring]
   repository = "your-org/your-repo"
   ```

2. **Add Alert Thresholds**:
   ```toml
   [performance.monitoring.alert_thresholds]
   release_success_rate_warning = 0.95
   ```

3. **Configure Notifications**:
   ```toml
   [performance.monitoring.notifications]
   enabled = true
   ```

4. **Enable Dashboard Integration**:
   ```toml
   [dashboard]
   enabled = true
   ```

## Conclusion

This configuration guide provides comprehensive coverage of CodeGuardian's release monitoring system. Start with the basic configuration and gradually add advanced features as your monitoring needs evolve. Remember to regularly review and adjust alert thresholds based on your project's specific requirements and historical data patterns.
