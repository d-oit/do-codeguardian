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