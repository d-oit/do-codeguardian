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

## Detailed Setup Guides

### Jira Integration Setup

#### Prerequisites
- Jira instance URL (Cloud or Server)
- API token or personal access token
- Project key and issue type permissions

#### Step-by-Step Setup

1. **Create API Token**
   ```bash
   # For Jira Cloud: Visit https://id.atlassian.com/manage-profile/security/api-tokens
   # For Jira Server: Create personal access token in user profile
   ```

2. **Configure CodeGuardian**
   ```toml
   [integrations.systems.jira]
   enabled = true
   base_url = "https://your-domain.atlassian.net"
   auth = { type = "basic", username = "your-email@domain.com", token = "api-token" }
   features = { issue_tracking = true, duplicate_detection = true, workflow_automation = true }
   ```

3. **Test Connection**
   ```bash
   codeguardian integrations --health-check jira
   ```

4. **Configure Project Mappings**
   ```toml
   [integrations.systems.jira.mappings]
   default_project = "SEC"
   issue_type_mappings = { "security" = "Bug", "duplicate" = "Task" }
   priority_mappings = { "critical" = "Highest", "high" = "High", "medium" = "Medium", "low" = "Low" }
   ```

#### Advanced Jira Configuration

```toml
[integrations.systems.jira]
# Connection settings
timeout_seconds = 30
retry_attempts = 3
batch_size = 50

# Custom fields mapping
[integrations.systems.jira.custom_fields]
severity_field = "customfield_10001"
component_field = "customfield_10002"
tags_field = "labels"

# Workflow settings
[integrations.systems.jira.workflow]
transition_on_resolve = "Done"
transition_on_close = "Closed"
auto_assign = true
default_assignee = "security-team"
```

### GitLab Integration Setup

#### Prerequisites
- GitLab instance URL
- Personal or project access token
- Repository permissions

#### Step-by-Step Setup

1. **Create Access Token**
   ```bash
   # GitLab.com: User Settings > Access Tokens
   # Self-hosted: Admin > Applications or User Settings > Access Tokens
   # Required scopes: api, read_repository, write_repository
   ```

2. **Configure CodeGuardian**
   ```toml
   [integrations.systems.gitlab]
   enabled = true
   base_url = "https://gitlab.com"
   auth = { type = "token", token = "glpat-xxxxxxxxxxxxxx" }
   features = { issue_tracking = true, workflow_automation = true, ci_cd = true }
   ```

3. **Test Connection**
   ```bash
   codeguardian integrations --health-check gitlab
   ```

4. **Configure Project Settings**
   ```toml
   [integrations.systems.gitlab.settings]
   default_project_id = 12345
   issue_labels = ["codeguardian", "security", "duplicate"]
   merge_request_labels = ["codeguardian-analysis"]
   pipeline_trigger_token = "${GITLAB_TRIGGER_TOKEN}"
   ```

#### Advanced GitLab Configuration

```toml
[integrations.systems.gitlab]
# API settings
api_version = "v4"
timeout_seconds = 30

# Project mappings
[integrations.systems.gitlab.project_mappings]
"frontend" = 12345
"backend" = 67890
"mobile" = 11111

# CI/CD integration
[integrations.systems.gitlab.ci_cd]
trigger_pipelines = true
pipeline_variables = { "CODEGUARDIAN_TRIGGER" = "true", "ANALYSIS_TYPE" = "security" }
wait_for_pipeline_completion = false
```

### GitHub Integration Setup

#### Prerequisites
- GitHub repository access
- Personal access token or GitHub App
- Repository permissions

#### Step-by-Step Setup

1. **Create Personal Access Token**
   ```bash
   # Visit: https://github.com/settings/tokens
   # Required scopes: repo, read:org, write:repo_hook, read:user
   ```

2. **Configure CodeGuardian**
   ```toml
   [integrations.systems.github]
   enabled = true
   base_url = "https://api.github.com"
   auth = { type = "token", token = "ghp_xxxxxxxxxxxxxxxxxxxx" }
   features = { issue_tracking = true, workflow_automation = true, webhooks = true }
   ```

3. **Test Connection**
   ```bash
   codeguardian integrations --health-check github
   ```

4. **Configure Repository Settings**
   ```toml
   [integrations.systems.github.settings]
   default_repository = "your-org/your-repo"
   issue_labels = ["codeguardian", "security", "duplicate"]
   pull_request_labels = ["codeguardian-analysis"]
   create_draft_issues = false
   ```

#### Advanced GitHub Configuration

```toml
[integrations.systems.github]
# API settings
timeout_seconds = 30
retry_attempts = 3

# Repository mappings
[integrations.systems.github.repository_mappings]
"main-repo" = "your-org/main-repo"
"api" = "your-org/api-repo"
"web" = "your-org/web-repo"

# Webhook configuration
[integrations.systems.github.webhooks]
auto_register = true
webhook_secret = "${GITHUB_WEBHOOK_SECRET}"
events = ["push", "pull_request", "issues", "release"]
```

### Azure DevOps Integration Setup

#### Prerequisites
- Azure DevOps organization URL
- Personal access token
- Project permissions

#### Step-by-Step Setup

1. **Create Personal Access Token**
   ```bash
   # Visit: https://dev.azure.com/{organization}/_usersSettings/tokens
   # Required scopes: Build (Read & execute), Code (Read), Work Items (Read & write)
   ```

2. **Configure CodeGuardian**
   ```toml
   [integrations.systems.azure_devops]
   enabled = true
   base_url = "https://dev.azure.com/your-org"
   auth = { type = "token", token = "your-pat-token" }
   features = { issue_tracking = true, workflow_automation = true, boards = true }
   ```

3. **Test Connection**
   ```bash
   codeguardian integrations --health-check azure-devops
   ```

4. **Configure Project Settings**
   ```toml
   [integrations.systems.azure_devops.settings]
   default_project = "YourProject"
   area_path = "YourProject\\Security"
   iteration_path = "YourProject\\Sprint 1"
   work_item_type = "Bug"
   ```

#### Advanced Azure DevOps Configuration

```toml
[integrations.systems.azure_devops]
# API settings
api_version = "7.1"
timeout_seconds = 30

# Work item mappings
[integrations.systems.azure_devops.work_item_mappings]
security = "Bug"
duplicate = "Task"
enhancement = "Feature"

# Pipeline integration
[integrations.systems.azure_devops.pipelines]
trigger_builds = true
build_definition_id = 123
wait_for_completion = false
```

## System-Specific Examples

### Jira Examples

#### Creating Security Issues

```bash
# Create a security vulnerability issue
codeguardian integrations --create-issue \
  --system jira \
  --title "Critical SQL Injection Vulnerability" \
  --description "Found SQL injection in user authentication module" \
  --project "SEC" \
  --type "Bug" \
  --priority "Highest" \
  --labels "security,critical" \
  --assignee "security-team-lead"
```

#### Searching for Duplicate Issues

```bash
# Search for similar issues
codeguardian integrations --search-duplicates \
  --system jira \
  --query "SQL injection authentication" \
  --project "SEC" \
  --max-results 10
```

#### Updating Issue Status

```bash
# Transition issue to resolved
codeguardian integrations --update-issue \
  --system jira \
  --issue-key "SEC-123" \
  --transition "Resolve Issue" \
  --comment "Fixed in release v1.2.3"
```

### GitLab Examples

#### Creating Merge Requests

```bash
# Create merge request with analysis results
codeguardian integrations --create-merge-request \
  --system gitlab \
  --title "Security fixes for authentication module" \
  --description "Automated security analysis and fixes" \
  --source-branch "security-fixes" \
  --target-branch "main" \
  --labels "security,automated" \
  --assignee "security-reviewer"
```

#### Triggering CI Pipelines

```bash
# Trigger security scanning pipeline
codeguardian integrations --trigger-pipeline \
  --system gitlab \
  --project-id 12345 \
  --ref "main" \
  --variables "SCAN_TYPE=security,TRIGGER=codeguardian"
```

#### Managing Issues

```bash
# Create issue with attachments
codeguardian integrations --create-issue \
  --system gitlab \
  --title "Code Quality Issues Found" \
  --description "Analysis results attached" \
  --labels "code-quality,analysis" \
  --weight 3 \
  --attachments "analysis-report.json,details.md"
```

### GitHub Examples

#### Creating Issues with Labels

```bash
# Create GitHub issue
codeguardian integrations --create-issue \
  --system github \
  --title "Security Vulnerability: XSS in Login Form" \
  --description "Cross-site scripting vulnerability detected" \
  --labels "security,critical,xss" \
  --assignees "security-team,frontend-lead" \
  --milestone "v1.3.0"
```

#### Managing Pull Requests

```bash
# Create pull request
codeguardian integrations --create-pull-request \
  --system github \
  --title "Fix security vulnerabilities" \
  --description "Automated fixes for detected vulnerabilities" \
  --head "security-fixes" \
  --base "main" \
  --draft false \
  --labels "security,automated-fix"
```

#### Working with Projects

```bash
# Add issue to project board
codeguardian integrations --add-to-project \
  --system github \
  --issue-number 123 \
  --project-id 456 \
  --column "To Do"
```

### Azure DevOps Examples

#### Creating Work Items

```bash
# Create work item
codeguardian integrations --create-work-item \
  --system azure-devops \
  --title "Security: Buffer Overflow in Network Handler" \
  --description "Buffer overflow vulnerability in network processing" \
  --type "Bug" \
  --area-path "Security\\Network" \
  --iteration-path "Sprint 5" \
  --priority 1 \
  --tags "security,critical,network"
```

#### Managing Boards

```bash
# Move work item on board
codeguardian integrations --move-work-item \
  --system azure-devops \
  --work-item-id 789 \
  --to-column "In Progress" \
  --reason "Starting investigation"
```

#### Linking Work Items

```bash
# Create work item link
codeguardian integrations --link-work-items \
  --system azure-devops \
  --source-id 123 \
  --target-id 456 \
  --link-type "Related"
```

## API Details

### Integration Manager API

#### Core Interface

```rust
use codeguardian::integrations::{IntegrationManager, IntegrationConfig, SystemType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = IntegrationConfig::from_file("integrations.toml").await?;

    // Create integration manager
    let manager = IntegrationManager::new(config).await?;

    // Initialize all systems
    manager.initialize_all().await?;

    // Search across systems
    let query = DuplicateSearchQuery {
        title: "authentication vulnerability".to_string(),
        description: Some("SQL injection in login".to_string()),
        similarity_threshold: 0.8,
        max_results: 10,
        ..Default::default()
    };

    let results = manager.search_duplicates_across_systems(&query).await?;
    println!("Found {} potential duplicates", results.len());

    Ok(())
}
```

#### System-Specific Clients

```rust
use codeguardian::integrations::jira::JiraClient;
use codeguardian::integrations::gitlab::GitLabClient;
use codeguardian::integrations::github::GitHubClient;

// Jira client
let jira_config = JiraConfig {
    base_url: "https://company.atlassian.net".to_string(),
    auth: BasicAuth {
        username: "user@company.com".to_string(),
        token: "api-token".to_string(),
    },
    ..Default::default()
};
let jira_client = JiraClient::new(jira_config).await?;

// GitLab client
let gitlab_config = GitLabConfig {
    base_url: "https://gitlab.com".to_string(),
    auth: TokenAuth {
        token: "glpat-xxxx".to_string(),
    },
    ..Default::default()
};
let gitlab_client = GitLabClient::new(gitlab_config).await?;

// GitHub client
let github_config = GitHubConfig {
    base_url: "https://api.github.com".to_string(),
    auth: TokenAuth {
        token: "ghp_xxxx".to_string(),
    },
    ..Default::default()
};
let github_client = GitHubClient::new(github_config).await?;
```

### REST API Endpoints

#### Integration Status

```http
GET /api/v1/integrations/status
```

Response:
```json
{
  "status": "success",
  "data": {
    "systems": {
      "jira": {
        "enabled": true,
        "healthy": true,
        "last_check": "2024-01-15T10:30:00Z"
      },
      "gitlab": {
        "enabled": true,
        "healthy": false,
        "error": "Authentication failed"
      }
    }
  }
}
```

#### Search Duplicates

```http
POST /api/v1/integrations/search-duplicates
Content-Type: application/json

{
  "query": "SQL injection",
  "systems": ["jira", "gitlab"],
  "similarity_threshold": 0.7,
  "max_results": 20
}
```

Response:
```json
{
  "status": "success",
  "data": {
    "results": [
      {
        "system": "jira",
        "id": "SEC-123",
        "title": "SQL Injection in Login",
        "similarity": 0.89,
        "url": "https://company.atlassian.net/browse/SEC-123"
      }
    ]
  }
}
```

#### Create Issue

```http
POST /api/v1/integrations/create-issue
Content-Type: application/json

{
  "system": "jira",
  "title": "Security Vulnerability Found",
  "description": "Critical vulnerability detected",
  "project": "SEC",
  "issue_type": "Bug",
  "priority": "Highest"
}
```

Response:
```json
{
  "status": "success",
  "data": {
    "system": "jira",
    "id": "SEC-456",
    "url": "https://company.atlassian.net/browse/SEC-456",
    "created": true
  }
}
```

### Webhook Payloads

#### GitHub Webhook Payload

```json
{
  "event_type": "push",
  "repository": {
    "full_name": "your-org/your-repo",
    "html_url": "https://github.com/your-org/your-repo"
  },
  "commits": [
    {
      "id": "abc123",
      "message": "Fix security vulnerability",
      "author": {
        "name": "Developer",
        "email": "dev@company.com"
      }
    }
  ],
  "codeguardian": {
    "trigger_analysis": true,
    "analysis_type": "security"
  }
}
```

#### Jira Webhook Payload

```json
{
  "event_type": "issue_created",
  "issue": {
    "key": "SEC-123",
    "fields": {
      "summary": "Security Issue",
      "description": "New security vulnerability",
      "priority": {
        "name": "Highest"
      }
    }
  },
  "codeguardian": {
    "check_duplicates": true,
    "similarity_threshold": 0.8
  }
}
```

## Testing Guidance

### Unit Testing Integrations

#### Testing Configuration Loading

```rust
#[cfg(test)]
mod tests {
    use codeguardian::integrations::config::IntegrationConfig;

    #[tokio::test]
    async fn test_config_loading() {
        let config_str = r#"
            [integrations.systems.jira]
            enabled = true
            base_url = "https://test.atlassian.net"
            auth = { type = "basic", username = "test@test.com", token = "test-token" }
        "#;

        let config: IntegrationConfig = toml::from_str(config_str).unwrap();
        assert!(config.systems.jira.enabled);
        assert_eq!(config.systems.jira.base_url, "https://test.atlassian.net");
    }
}
```

#### Testing Client Connections

```rust
#[cfg(test)]
mod integration_tests {
    use codeguardian::integrations::jira::JiraClient;
    use mockito::mock;

    #[tokio::test]
    async fn test_jira_connection() {
        // Mock Jira API
        let _m = mock("GET", "/rest/api/2/serverInfo")
            .with_status(200)
            .with_body(r#"{"serverTitle": "Test Jira"}"#)
            .create();

        let config = JiraConfig {
            base_url: mockito::server_url(),
            auth: BasicAuth {
                username: "test".to_string(),
                token: "test".to_string(),
            },
            ..Default::default()
        };

        let client = JiraClient::new(config).await.unwrap();
        let health = client.health_check().await.unwrap();
        assert!(health.healthy);
    }
}
```

### Integration Testing

#### End-to-End Testing Setup

```rust
#[cfg(test)]
mod e2e_tests {
    use codeguardian::integrations::{IntegrationManager, IntegrationConfig};
    use std::env;

    #[tokio::test]
    #[ignore] // Requires real credentials
    async fn test_full_integration_flow() {
        // Load test configuration
        let config_path = env::var("TEST_INTEGRATION_CONFIG")
            .unwrap_or_else(|_| "tests/integration-config.toml".to_string());

        let config = IntegrationConfig::from_file(&config_path).await.unwrap();
        let manager = IntegrationManager::new(config).await.unwrap();

        // Test health checks
        let health_results = manager.health_check_all().await.unwrap();
        for (system, health) in health_results {
            println!("{}: {}", system, if health.healthy { "healthy" } else { "unhealthy" });
            assert!(health.healthy, "System {} is not healthy: {:?}", system, health.error);
        }

        // Test duplicate search
        let query = DuplicateSearchQuery {
            title: "test security issue".to_string(),
            ..Default::default()
        };

        let results = manager.search_duplicates_across_systems(&query).await.unwrap();
        assert!(results.len() >= 0); // Should not fail
    }
}
```

#### Testing Webhooks

```rust
#[cfg(test)]
mod webhook_tests {
    use codeguardian::integrations::webhooks::{WebhookHandler, WebhookPayload};
    use serde_json::json;

    struct TestWebhookHandler;

    impl WebhookHandler for TestWebhookHandler {
        async fn handle_webhook(&self, payload: &WebhookPayload) -> Result<()> {
            match payload.event_type.as_str() {
                "test_event" => {
                    // Simulate processing
                    println!("Processing test webhook: {}", payload.repository);
                    Ok(())
                }
                _ => Err(anyhow!("Unknown event type")),
            }
        }
    }

    #[tokio::test]
    async fn test_webhook_handling() {
        let handler = TestWebhookHandler;

        let payload = WebhookPayload {
            event_type: "test_event".to_string(),
            repository: "test-repo".to_string(),
            data: json!({"test": "data"}),
        };

        let result = handler.handle_webhook(&payload).await;
        assert!(result.is_ok());
    }
}
```

### Testing Best Practices

#### Mock External Services

```rust
use mockito::{mock, Matcher};

#[tokio::test]
async fn test_with_mocked_api() {
    // Mock external API responses
    let _server_mock = mock("GET", "/api/v1/issues")
        .match_query(Matcher::UrlEncoded("project".into(), "TEST".into()))
        .with_status(200)
        .with_body(r#"[{"key": "TEST-1", "fields": {"summary": "Test Issue"}}]"#)
        .create();

    // Test your integration logic
    let client = TestClient::new(mockito::server_url());
    let issues = client.get_issues("TEST").await.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TEST-1");
}
```

#### Test Data Management

```rust
#[cfg(test)]
mod test_data {
    use codeguardian::integrations::test_utils;

    #[tokio::test]
    async fn test_with_test_data() {
        // Create test issues/projects
        let test_data = test_utils::create_test_jira_data().await;

        // Run tests
        let results = run_integration_tests(&test_data).await;

        // Cleanup
        test_utils::cleanup_test_data(&test_data).await;

        assert!(results.all_passed);
    }
}
```

#### Performance Testing

```rust
#[cfg(test)]
mod performance_tests {
    use codeguardian::integrations::IntegrationManager;
    use std::time::Instant;

    #[tokio::test]
    async fn test_search_performance() {
        let manager = setup_test_manager().await;
        let query = create_large_search_query();

        let start = Instant::now();
        let results = manager.search_duplicates_across_systems(&query).await.unwrap();
        let duration = start.elapsed();

        // Assert performance requirements
        assert!(duration < std::time::Duration::from_secs(30),
                "Search took too long: {:?}", duration);
        assert!(results.len() > 0);
    }
}
```

### CI/CD Testing Integration

#### GitHub Actions Testing

```yaml
# .github/workflows/integration-tests.yml
name: Integration Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      jira:
        image: atlassian/jira-software:latest
        ports:
          - 8080:8080
      gitlab:
        image: gitlab/gitlab-ce:latest
        ports:
          - 80:80

    steps:
      - uses: actions/checkout@v4

      - name: Setup test environment
        run: |
          # Configure test instances
          ./scripts/setup-test-integrations.sh

      - name: Run integration tests
        run: cargo test --test integration_tests

      - name: Run performance tests
        run: cargo test --test performance_tests
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
