# CI/CD Notifications & Cost Tracking Setup

This directory contains the notification and cost tracking workflows for CodeGuardian.

## Files Overview

### Workflows
- `cost-tracking.yml` - Automated cost monitoring and reporting
- `slack-notification.yml` - Reusable Slack notification workflow

### Templates
- `../notification-templates/ci-success.json` - CI success notification template
- `../notification-templates/ci-failure.json` - CI failure notification template  
- `../notification-templates/deployment-success.json` - Deployment success template
- `../notification-templates/deployment-failure.json` - Deployment failure template

### Scripts
- `../../scripts/send-slack-notification.sh` - Notification sending script
- `../../scripts/cost-dashboard.sh` - Cost analysis dashboard generator

## Quick Setup

### 1. Configure Slack Webhook
1. Create a Slack app at https://api.slack.com/apps
2. Enable incoming webhooks
3. Add webhook to your desired channel
4. Copy the webhook URL

### 2. Set Repository Secrets
In GitHub repository settings → Secrets and variables → Secrets:

```
SLACK_WEBHOOK_URL = https://hooks.slack.com/services/YOUR/WEBHOOK/URL
```

### 3. Set Repository Variables (Optional)
In GitHub repository settings → Secrets and variables → Variables:

```
SLACK_CHANNEL = #ci-cd
```

### 4. Test Setup
Run the cost tracking workflow manually to verify everything works:

```bash
gh workflow run cost-tracking.yml
```

## Features

✅ **Automated Notifications**
- CI/CD pipeline success/failure
- Deployment success/failure
- Cost threshold alerts

✅ **Cost Tracking**
- Daily automated reports
- Cost estimation and analysis
- Storage and bandwidth monitoring

✅ **Security**
- Secure webhook URL storage
- No sensitive data in notifications
- Proper access controls

✅ **Customization**
- Template-based notifications
- Configurable channels
- Customizable thresholds

## Usage Examples

### Manual Cost Report
```bash
./scripts/cost-dashboard.sh 30d text
```

### Manual Notification
```bash
./scripts/send-slack-notification.sh ci-success \
  "$SLACK_WEBHOOK_URL" "#ci-cd" \
  workflow_name="Test" branch="main"
```

See `../docs/ci-cd-notifications-cost-tracking.md` for detailed documentation.
