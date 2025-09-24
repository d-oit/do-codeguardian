# CI/CD Notifications & Cost Tracking

This document describes the Slack notification system and cost tracking implementation for CodeGuardian's CI/CD pipelines.

## Overview

The system provides:
- **Slack Notifications**: Real-time notifications for CI/CD events (success, failure, deployment)
- **Cost Tracking**: Automated monitoring of GitHub Actions usage and costs
- **Dashboards**: Cost analysis reports and visualizations
- **Secure Configuration**: Proper secrets management and access controls

## Setup Instructions

### 1. Slack Configuration

#### Create a Slack App
1. Go to [Slack API](https://api.slack.com/apps)
2. Click "Create New App" → "From scratch"
3. Name your app (e.g., "CodeGuardian CI/CD")
4. Select your workspace

#### Configure Webhooks
1. In your Slack app, go to "Incoming Webhooks"
2. Enable incoming webhooks
3. Click "Add New Webhook to Workspace"
4. Select the channel for notifications (e.g., `#ci-cd`, `#deployments`)
5. Copy the webhook URL

#### Set Repository Secrets
Add the following secrets to your GitHub repository:

```bash
# For CI/CD notifications
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK/URL

# Optional: Different channels for different notification types
SLACK_CHANNEL_CI=#ci-cd
SLACK_CHANNEL_DEPLOYMENTS=#deployments
```

Or use repository variables for non-sensitive configuration:

```bash
# Repository Variables (GitHub Settings → Secrets and variables → Variables)
SLACK_CHANNEL=#ci-cd
```

### 2. Notification Templates

The system uses JSON templates for consistent notification formatting:

- `ci-success.json` - CI/CD pipeline success notifications
- `ci-failure.json` - CI/CD pipeline failure notifications  
- `deployment-success.json` - Deployment success notifications
- `deployment-failure.json` - Deployment failure notifications

Templates support variable substitution using `{{variable_name}}` syntax.

### 3. Cost Tracking Setup

Cost tracking runs automatically via the `cost-tracking.yml` workflow:

- **Schedule**: Daily at 6 AM UTC
- **Manual**: Via workflow dispatch
- **Analysis Periods**: 7d, 30d, 90d

#### Cost Estimation

The system estimates costs based on:
- **Linux runners**: $0.008 per minute
- **Windows/macOS runners**: $0.016 per minute
- **Storage**: Cache and artifact storage
- **Data transfer**: Bandwidth usage

### 4. Dashboard Generation

Generate cost analysis reports using the dashboard script:

```bash
# Generate text report for last 30 days
./scripts/cost-dashboard.sh 30d text

# Generate JSON data for external dashboards
./scripts/cost-dashboard.sh 7d json

# Generate HTML dashboard
./scripts/cost-dashboard.sh 90d html
```

## Usage

### Manual Notifications

Send notifications manually using the notification script:

```bash
# CI success notification
./scripts/send-slack-notification.sh ci-success \
  "https://hooks.slack.com/services/..." \
  "#ci-cd" \
  workflow_name="CI Pipeline" \
  branch="main" \
  commit_sha="abc123" \
  coverage_percent="95.2"

# Deployment notification
./scripts/send-slack-notification.sh deployment-success \
  "https://hooks.slack.com/services/..." \
  "#deployments" \
  version="v1.2.3" \
  environment="production" \
  deployed_by="github-user"
```

### Automated Notifications

Notifications are automatically sent for:

- **CI/CD Events**: Success/failure of the main CI pipeline
- **Deployments**: Success/failure of deployment workflows
- **Cost Alerts**: When costs exceed thresholds ($20 warning, $50 critical)

### Cost Monitoring

The cost tracking system provides:

- **Daily Reports**: Automated daily cost analysis
- **Threshold Alerts**: Notifications when costs are high
- **Issue Creation**: Automatic GitHub issues for cost optimization
- **Trend Analysis**: Historical cost tracking and trends

## Security Considerations

### Secrets Management
- Webhook URLs stored as GitHub secrets
- No sensitive data in notification payloads
- Channel names can be repository variables

### Access Controls
- Notifications only sent to configured channels
- Repository secrets require appropriate permissions
- Cost data access limited to repository collaborators

### Data Privacy
- No sensitive information in notifications
- Cost data aggregated and anonymized
- No personal information exposed

## Configuration Options

### Environment Variables
```bash
# Slack configuration
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/...
SLACK_CHANNEL=#ci-cd

# Cost thresholds
COST_WARNING_THRESHOLD=20
COST_CRITICAL_THRESHOLD=50
SUCCESS_RATE_THRESHOLD=90
```

### Workflow Customization
Modify workflow files to:
- Change notification triggers
- Add custom notification types
- Adjust cost analysis parameters
- Configure different channels per environment

## Troubleshooting

### Common Issues

**Notifications not sending:**
- Check webhook URL is valid and active
- Verify repository secrets are set correctly
- Check workflow permissions for Slack API access

**Cost data inaccurate:**
- Ensure GitHub CLI is authenticated
- Check repository access permissions
- Verify workflow run data is accessible

**Template errors:**
- Validate JSON syntax in templates
- Check variable names match template placeholders
- Ensure all required variables are provided

### Debugging

Enable debug logging by setting workflow environment variables:

```yaml
env:
  ACTIONS_STEP_DEBUG: true
  ACTIONS_RUNNER_DEBUG: true
```

Check workflow logs for detailed error messages and API responses.

## Cost Optimization Tips

Based on the cost tracking data, consider these optimizations:

1. **Cache Dependencies**: Use proper caching to reduce build times
2. **Parallel Jobs**: Optimize job concurrency and matrix strategies
3. **Self-hosted Runners**: For high-volume repositories
4. **Conditional Workflows**: Skip unnecessary jobs when possible
5. **Artifact Management**: Clean up old artifacts regularly

## Integration Examples

### External Dashboards
Use JSON output for integration with external monitoring:

```bash
# Generate JSON for Grafana/monitoring dashboards
./scripts/cost-dashboard.sh 30d json > cost_data.json
```

### Custom Notifications
Extend the notification system for additional events:

```bash
# Create custom template
cp .github/notification-templates/ci-success.json \
   .github/notification-templates/custom-event.json

# Modify template for your needs
# Send notification
./scripts/send-slack-notification.sh custom-event \
  "$WEBHOOK_URL" "#channel" \
  custom_field="value"
```

## Support

For issues with the notification system:
1. Check workflow run logs for errors
2. Verify configuration in repository settings
3. Test webhook URLs manually
4. Review Slack app permissions

For cost tracking issues:
1. Ensure GitHub CLI authentication
2. Check repository API access
3. Verify workflow permissions
4. Review cost calculation logic
