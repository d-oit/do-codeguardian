# 🔗 Integrations Guide

CodeGuardian supports seamless integration with external systems for enhanced duplicate prevention across your entire development ecosystem. This guide covers setup and usage of supported integrations.

## Overview

Integrations enable CodeGuardian to:

- Search for duplicates across issue trackers and documentation platforms
- Create issues automatically when duplicates are detected
- Trigger workflows and CI/CD pipelines
- Generate unified reports across all systems
- Synchronize duplicate prevention data

## Supported Systems

### Issue Trackers

- **Jira**: Comprehensive issue tracking and project management
- **GitLab Issues**: Integrated issue management with CI/CD
- **GitHub Issues**: Pull request and issue integration
- **Azure DevOps**: Microsoft DevOps platform integration

### Documentation Platforms

- **Confluence**: Knowledge base and documentation management
- **GitHub Wiki**: Repository documentation integration

### CI/CD Systems

- **Jenkins**: Build automation and workflow triggering
- **GitLab CI**: Integrated CI/CD pipeline management
- **GitHub Actions**: Workflow automation and deployment

### Repository Management

- **Bitbucket**: Git repository hosting and management
- **Azure Repos**: Enterprise repository management

## Configuration

### Basic Integration Setup

```toml
[integrations]
enabled = true
default_timeout_seconds = 30
retry_attempts = 3
batch_size = 50

# Configure individual systems
[integrations.systems.jira]
enabled = true
base_url = "https://your-domain.atlassian.net"
auth = { type = "basic", username = "user", token = "api-token" }
features = { issue_tracking = true, duplicate_detection = true }

[integrations.systems.gitlab]
enabled = true
base_url = "https://gitlab.com"
auth = { type = "token", token = "personal-access-token" }
features = { issue_tracking = true, workflow_automation = true }
```

### Authentication Methods

#### Basic Authentication

```toml
[integrations.systems.jira]
auth = { type = "basic", username = "your-email@domain.com", token = "api-token" }
```

#### Token Authentication

```toml
[integrations.systems.gitlab]
auth = { type = "token", token = "glpat-xxxxxxxxxxxxxx" }
```

#### OAuth Authentication

```toml
[integrations.systems.bitbucket]
auth = {
  type = "oauth",
  client_id = "your-client-id",
  client_secret = "your-client-secret",
  access_token = "access-token"
}
```

### Feature Configuration

Each system supports different features:

```toml
[integrations.systems.jira.features]
issue_tracking = true
duplicate_detection = true
workflow_automation = false
reporting = true
webhooks = true
```

## CLI Usage

### List Integrations

```bash
# List all available integrations
codeguardian integrations --list

# Check system health
codeguardian integrations --health-check jira
codeguardian integrations --health-check gitlab
```

### Search for Duplicates

```bash
# Search across all enabled systems
codeguardian integrations --search-duplicates "authentication bypass"

# Search with similarity threshold
codeguardian integrations --search-duplicates "SQL injection" --threshold 0.8
```

### Create Issues

```bash
# Create issue across multiple systems
codeguardian integrations --create-issue \
  --title "Critical Security Vulnerability Detected" \
  --description "Multiple instances of SQL injection vulnerability found" \
  --project "SEC" \
  --type "Bug"
```

### Generate Reports

```bash
# Generate unified duplicate report
codeguardian integrations --generate-report --report-type duplicates

# Generate system health report
codeguardian integrations --generate-report --report-type health
```

### Trigger Workflows

```bash
# Trigger CI/CD pipeline
codeguardian integrations --trigger-workflow "security-scan" \
  --workflow-params '{"branch": "main", "severity": "high"}'
```

### Configuration Management

```bash
# Initialize integration configuration
codeguardian integrations --init-config

# Enable specific integration
codeguardian integrations --enable jira

# Disable integration
codeguardian integrations --disable confluence
```

## Programmatic Usage

### Integration Manager

```rust
use codeguardian::integrations::{IntegrationManager, IntegrationsConfig};

let config = IntegrationsConfig::default();
let mut manager = IntegrationManager::new(config);

// Initialize all enabled integrations
manager.initialize().await?;

// Search for duplicates
let query = DuplicateSearchQuery {
    title: "security vulnerability".to_string(),
    similarity_threshold: 0.7,
    ..Default::default()
};
let results = manager.search_duplicates_across_systems(&query).await?;

// Create issues
let issue = IssueCreationRequest {
    title: "Duplicate Code Found".to_string(),
    description: "Similar code patterns detected".to_string(),
    issue_type: "Task".to_string(),
    ..Default::default()
};
let results = manager.create_issue_across_systems(&issue).await?;
```

### System-Specific Clients

```rust
use codeguardian::integrations::jira::JiraClient;

// Create Jira client
let config = SystemConfig::jira_default();
let client = JiraClient::new(config).await?;

// Search issues
let issues = client.search_issues("project = SEC AND status = Open").await?;

// Create issue
let issue = client.create_issue(&IssueCreationRequest {
    title: "CodeGuardian Alert".to_string(),
    description: "Duplicate prevention triggered".to_string(),
    project_key: Some("SEC".to_string()),
    ..Default::default()
}).await?;
```

## CI/CD Integration

### GitHub Actions

```yaml
name: CodeGuardian Integration
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run CodeGuardian
        env:
          JIRA_TOKEN: ${{ secrets.JIRA_TOKEN }}
          GITLAB_TOKEN: ${{ secrets.GITLAB_TOKEN }}
        run: |
          codeguardian check . --format json --out results.json

      - name: Create Issues if Duplicates Found
        if: steps.analysis.outputs.has_duplicates == 'true'
        run: |
          codeguardian integrations --create-issue \
            --title "Duplicates Detected in ${{ github.ref }}" \
            --description "CodeGuardian found duplicate code patterns" \
            --project "CODEQUALITY"
```

### GitLab CI

```yaml
codeguardian_integrations:
  image: d-oit/codeguardian:latest
  script:
    - codeguardian check . --format json --out results.json
    - |
      if [ -s results.json ]; then
        codeguardian integrations --create-issue \
          --title "CodeGuardian: Duplicates Found" \
          --description "Review attached analysis results" \
          --project "Quality"
      fi
  artifacts:
    reports:
      codequality: results.json
  only:
    - merge_requests
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    stages {
        stage('CodeGuardian Analysis') {
            steps {
                sh 'codeguardian check . --format json --out results.json'
            }
        }

        stage('Integration Check') {
            steps {
                sh '''
                    codeguardian integrations --health-check jira
                    codeguardian integrations --health-check gitlab
                '''
            }
        }

        stage('Create Issues') {
            when {
                expression { fileExists 'results.json' }
            }
            steps {
                sh '''
                    codeguardian integrations --create-issue \
                      --title "Build ${BUILD_NUMBER}: Code Quality Issues" \
                      --description "CodeGuardian analysis completed" \
                      --project "DEVOPS"
                '''
            }
        }
    }
}
```

## Webhook Integration

### Receiving Webhooks

Configure webhooks to trigger CodeGuardian analysis:

```rust
use codeguardian::integrations::traits::WebhookHandler;

struct CustomWebhookHandler;

impl WebhookHandler for CustomWebhookHandler {
    async fn handle_webhook(&self, payload: &WebhookPayload) -> Result<()> {
        match payload.event_type.as_str() {
            "push" => {
                // Run analysis on push
                codeguardian::run_analysis(payload.repository.clone()).await?;
            }
            "issue_created" => {
                // Check for duplicate issues
                codeguardian::check_duplicate_issue(payload).await?;
            }
            _ => {}
        }
        Ok(())
    }
}
```

### Sending Webhooks

Trigger external systems via webhooks:

```rust
use codeguardian::integrations::traits::WebhookTrigger;

let webhook = WebhookTrigger {
    url: "https://jenkins.example.com/generic-webhook-trigger".to_string(),
    method: "POST".to_string(),
    headers: HashMap::new(),
    payload: serde_json::json!({
        "job": "codeguardian-analysis",
        "branch": "main"
    }),
};

client.trigger_webhook(&webhook).await?;
```

## Security Considerations

### Authentication Security

- Use API tokens instead of passwords
- Rotate tokens regularly
- Store credentials securely (environment variables, secret managers)
- Limit token permissions to necessary scopes

### Network Security

- Use HTTPS for all external communications
- Configure appropriate timeouts
- Implement rate limiting
- Validate SSL certificates

### Data Protection

- Encrypt sensitive data in transit and at rest
- Implement proper access controls
- Audit integration activities
- Comply with data protection regulations

## Troubleshooting

### Connection Issues

**Authentication failures**
- Verify credentials are correct
- Check token permissions
- Ensure API endpoints are accessible

**Timeout errors**
- Increase timeout values in configuration
- Check network connectivity
- Verify external service status

**Rate limiting**
- Implement exponential backoff
- Reduce request frequency
- Check API rate limits

### Configuration Issues

**Features not working**
- Verify feature flags are enabled
- Check system capabilities
- Review configuration syntax

**Integration not initializing**
- Check configuration validity
- Verify network access
- Review error logs

## Best Practices

### Configuration Management

1. **Use environment variables** for sensitive data
2. **Version control** integration configurations
3. **Test configurations** in staging environments
4. **Document custom fields** and mappings

### Monitoring and Maintenance

1. **Monitor health checks** regularly
2. **Set up alerts** for integration failures
3. **Review and update** configurations periodically
4. **Audit integration activities** for security

### Performance Optimization

1. **Configure appropriate batch sizes**
2. **Implement caching** for frequently accessed data
3. **Use parallel processing** when possible
4. **Monitor resource usage** and adjust limits

## Next Steps

- [Configuration Guide](../configuration.md) - Integration configuration options
- [API Reference](../api.md) - Integration API documentation
- [Dashboard Guide](dashboard.md) - Visual monitoring of integrations
