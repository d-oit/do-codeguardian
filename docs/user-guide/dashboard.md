# 🖥️ Dashboard Guide

CodeGuardian provides a comprehensive web-based dashboard for monitoring duplicate prevention across all ecosystem components. This guide covers dashboard setup, configuration, and usage.

## Overview

The dashboard feature enables real-time monitoring and reporting of duplicate detection and prevention metrics across your entire codebase and integrated systems.

## Prerequisites

The dashboard requires the `dashboard` feature to be enabled:

```bash
# Enable dashboard feature
cargo build --features dashboard

# Or add to Cargo.toml features
[features]
default = ["dashboard"]
```

## Configuration

### Basic Dashboard Configuration

```toml
[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080
refresh_interval_seconds = 30
max_history_days = 30
enable_real_time = true

# Custom views for different stakeholders
[[dashboard.custom_views]]
name = "Overview"
description = "High-level duplicate prevention metrics"
# ... additional view configuration
```

### Advanced Configuration

```toml
[dashboard]
# Server settings
host = "0.0.0.0"  # Listen on all interfaces
port = 3000
refresh_interval_seconds = 60

# Data retention
max_history_days = 90
enable_real_time = true

# Security (when deployed)
# enable_https = true
# ssl_cert_path = "/path/to/cert.pem"
# ssl_key_path = "/path/to/key.pem"
```

## Starting the Dashboard

### Command Line

```bash
# Start dashboard server
codeguardian dashboard --start

# Start on specific host/port
codeguardian dashboard --start --host 0.0.0.0 --port 3000

# Enable real-time updates
codeguardian dashboard --start --real-time
```

### Programmatic Usage

```rust
use codeguardian::dashboard::{DashboardConfig, DashboardService};

let config = DashboardConfig {
    enabled: true,
    host: "127.0.0.1".to_string(),
    port: 8080,
    ..Default::default()
};

let service = DashboardService::new(config);
service.start().await?;
```

## Dashboard Views

### Pre-configured Views

1. **Overview** - High-level metrics and trends
2. **Security Focus** - Security-related duplicate detection
3. **Performance Metrics** - System performance and processing times

### Custom Views

Create stakeholder-specific views:

```toml
[[dashboard.custom_views]]
name = "Developer View"
description = "Code quality and performance metrics"
filters = { category_filter = ["code_quality", "performance"] }
charts = ["processing_times", "resource_usage"]

[[dashboard.custom_views]]
name = "Manager View"
description = "Business impact and ROI metrics"
filters = { time_range = "Last30Days" }
charts = ["prevention_effectiveness", "system_health"]
```

## API Endpoints

### Health Check

```bash
curl http://localhost:8080/api/health
```

### Metrics

```bash
# Current metrics
curl http://localhost:8080/api/metrics/current

# Historical metrics
curl "http://localhost:8080/api/metrics?time_range=7d"
```

### Views and Reports

```bash
# List available views
curl http://localhost:8080/api/views

# Generate report for specific view
curl "http://localhost:8080/api/views/Overview"
```

### Real-time Streaming

```bash
# Server-sent events for real-time updates
curl -N http://localhost:8080/api/stream/metrics
```

## Metrics and Charts

### Available Metrics

- **Duplicate Detection**: Total duplicates found, detection accuracy, false positive rate
- **Prevention Effectiveness**: Duplicates prevented, prevention rate, time/cost savings
- **System Health**: API success rate, response times, uptime percentage
- **Performance**: Processing times, throughput, resource usage

### Chart Types

- Line charts for trends over time
- Bar charts for categorical data
- Pie charts for proportions
- Gauge charts for health metrics
- Histograms for distributions

## Integration with CI/CD

### GitHub Actions

```yaml
- name: Start Dashboard
  run: |
    codeguardian dashboard --start --host 0.0.0.0 --port 8080 &
    sleep 5

- name: Run Analysis
  run: |
    codeguardian check . --format json --out results.json

- name: Update Dashboard
  run: |
    curl -X POST http://localhost:8080/api/metrics \
      -H "Content-Type: application/json" \
      -d @results.json
```

### Docker Deployment

```dockerfile
FROM d-oit/codeguardian:latest

# Enable dashboard feature
ENV CODEGUARDIAN_FEATURES=dashboard

# Configure dashboard
ENV CODEGUARDIAN_DASHBOARD_HOST=0.0.0.0
ENV CODEGUARDIAN_DASHBOARD_PORT=8080

EXPOSE 8080

CMD ["dashboard", "--start"]
```

## Security Considerations

### Access Control

- Run dashboard on localhost for development
- Use reverse proxy (nginx, caddy) for production
- Implement authentication when exposing publicly

### Data Protection

- Dashboard data may contain sensitive information
- Use HTTPS in production
- Implement proper CORS policies

## Troubleshooting

### Common Issues

**Dashboard not starting**
- Check if port 8080 is available
- Verify dashboard feature is enabled
- Check logs for configuration errors

**Metrics not updating**
- Ensure analysis is running and producing output
- Check API endpoints are accessible
- Verify time ranges and filters

**Performance issues**
- Reduce refresh interval for large datasets
- Enable caching and compression
- Monitor memory usage

## Next Steps

- [Configuration Guide](../configuration.md) - Detailed configuration options
- [API Reference](../api.md) - Complete API documentation
- [Integration Guide](integrations.md) - External system integrations
