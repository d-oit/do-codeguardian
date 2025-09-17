# Remediation API Documentation

This document provides comprehensive API documentation for CodeGuardian's automated remediation workflows, introduced in v0.2.0-alpha.5.

## Table of Contents

- [Remediation Service](#remediation-service)
- [Remediation Workflows](#remediation-workflows)
- [Remediation Actions](#remediation-actions)
- [Approval System](#approval-system)
- [Risk Assessment](#risk-assessment)
- [Integration Support](#integration-support)
- [Workflow Execution](#workflow-execution)
- [Monitoring and Logging](#monitoring-and-logging)

## Remediation Service

### RemediationService

```rust
pub struct RemediationService {
    config: RemediationConfig,
    active_workflows: HashMap<String, RemediationWorkflow>,
    pending_approvals: HashMap<String, ApprovalRequest>,
}
```

**Methods:**
```rust
impl RemediationService {
    pub fn new(config: RemediationConfig) -> Self;
    pub async fn create_workflow_from_findings(&mut self, findings: Vec<Finding>, created_by: String) -> Result<String>;
    pub async fn execute_workflow(&mut self, workflow_id: &str) -> Result<()>;
    pub async fn approve_workflow(&mut self, workflow_id: &str, approved_by: String) -> Result<()>;
    pub async fn reject_workflow(&mut self, workflow_id: &str, rejected_by: String, reason: String) -> Result<()>;
    pub fn get_workflow(&self, workflow_id: &str) -> Option<&RemediationWorkflow>;
    pub fn get_active_workflows(&self) -> Vec<&RemediationWorkflow>;
}
```

### RemediationConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationConfig {
    pub enabled: bool,
    pub auto_approve_low_risk: bool,
    pub require_approval_threshold: RiskLevel,
    pub max_concurrent_workflows: u32,
    pub timeout_minutes: u32,
    pub integrations: IntegrationConfig,
    pub notification_settings: NotificationConfig,
}
```

## Remediation Workflows

### RemediationWorkflow

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationWorkflow {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: WorkflowStatus,
    pub risk_level: RiskLevel,
    pub actions: Vec<RemediationAction>,
    pub findings: Vec<Finding>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub approved_by: Option<String>,
    pub approval_required: bool,
    pub estimated_impact: ImpactAssessment,
    pub execution_log: Vec<ExecutionLogEntry>,
}
```

### WorkflowStatus

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    InProgress,
    AwaitingApproval,
    Approved,
    Rejected,
    Completed,
    Failed,
    Cancelled,
}
```

### Workflow Creation Example

```rust
use do_codeguardian::remediation::{RemediationService, RemediationConfig};
use do_codeguardian::types::Finding;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = RemediationConfig {
        enabled: true,
        auto_approve_low_risk: true,
        require_approval_threshold: RiskLevel::Medium,
        max_concurrent_workflows: 5,
        timeout_minutes: 30,
        ..Default::default()
    };

    let mut service = RemediationService::new(config);

    // Create findings from analysis
    let findings = vec![
        Finding {
            file: PathBuf::from("src/main.rs"),
            line: 25,
            category: Some("duplicate_code".to_string()),
            message: "Duplicate function implementation".to_string(),
            severity: Severity::Medium,
            ..Default::default()
        },
        // ... more findings
    ];

    // Create remediation workflow
    let workflow_id = service.create_workflow_from_findings(findings, "security-team".to_string()).await?;
    println!("Created remediation workflow: {}", workflow_id);

    Ok(())
}
```

## Remediation Actions

### RemediationAction

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationAction {
    MergeDuplicateCode {
        source_files: Vec<String>,
        target_file: String,
        merge_strategy: MergeStrategy,
    },
    RefactorDuplicateFunction {
        function_name: String,
        source_locations: Vec<CodeLocation>,
        target_location: CodeLocation,
    },
    CloseDuplicateIssue {
        issue_id: String,
        duplicate_of: String,
        comment: String,
    },
    ConsolidateDocumentation {
        source_docs: Vec<String>,
        target_doc: String,
        merge_sections: Vec<String>,
    },
    UpdateConfiguration {
        config_file: String,
        changes: HashMap<String, serde_json::Value>,
        backup_created: bool,
    },
    CreatePullRequest {
        title: String,
        description: String,
        branch_name: String,
        files_changed: Vec<String>,
    },
}
```

### MergeStrategy

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeStrategy {
    KeepFirst,
    KeepLast,
    KeepMostRecent,
    KeepMostComplex,
    Manual,
}
```

### CodeLocation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file_path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub function_name: Option<String>,
}
```

### Action Execution Example

```rust
use do_codeguardian::remediation::{RemediationAction, MergeStrategy, CodeLocation};

// Example: Merge duplicate code
let merge_action = RemediationAction::MergeDuplicateCode {
    source_files: vec![
        "src/utils.rs".to_string(),
        "src/helpers.rs".to_string(),
    ],
    target_file: "src/common.rs".to_string(),
    merge_strategy: MergeStrategy::KeepMostRecent,
};

// Example: Refactor duplicate function
let refactor_action = RemediationAction::RefactorDuplicateFunction {
    function_name: "validate_input".to_string(),
    source_locations: vec![
        CodeLocation {
            file_path: "src/auth.rs".to_string(),
            start_line: 15,
            end_line: 35,
            function_name: Some("validate_input".to_string()),
        },
        CodeLocation {
            file_path: "src/user.rs".to_string(),
            start_line: 22,
            end_line: 42,
            function_name: Some("validate_input".to_string()),
        },
    ],
    target_location: CodeLocation {
        file_path: "src/validation.rs".to_string(),
        start_line: 1,
        end_line: 21,
        function_name: Some("validate_input".to_string()),
    },
};
```

## Approval System

### ApprovalRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub workflow_id: String,
    pub requested_by: String,
    pub requested_at: DateTime<Utc>,
    pub justification: String,
    pub risk_assessment: RiskAssessment,
    pub reviewers: Vec<String>,
    pub deadline: Option<DateTime<Utc>>,
}
```

### RiskAssessment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk: RiskLevel,
    pub technical_risk: RiskLevel,
    pub business_risk: RiskLevel,
    pub security_risk: RiskLevel,
    pub mitigation_strategies: Vec<String>,
}
```

### RiskLevel

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}
```

### Approval Workflow Example

```rust
use do_codeguardian::remediation::{RemediationService, RiskAssessment, RiskLevel};

// Check if workflow requires approval
let workflow = service.get_workflow(&workflow_id).unwrap();
if workflow.approval_required {
    println!("Workflow {} requires approval", workflow_id);
    println!("Risk Level: {:?}", workflow.risk_level);
    println!("Estimated Impact: {} files, {} lines",
             workflow.estimated_impact.files_affected,
             workflow.estimated_impact.lines_of_code_changed);

    // Approve workflow
    service.approve_workflow(&workflow_id, "manager@example.com".to_string()).await?;
    println!("Workflow approved and execution started");
} else {
    println!("Workflow auto-approved and will execute automatically");
}
```

## Risk Assessment

### ImpactAssessment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub files_affected: u32,
    pub lines_of_code_changed: u32,
    pub tests_affected: u32,
    pub breaking_changes: bool,
    pub estimated_time_minutes: u32,
    pub confidence_score: f64,
}
```

### Risk Assessment Methods

```rust
impl RemediationService {
    fn assess_risk_level(&self, actions: &[RemediationAction]) -> RiskLevel;
    async fn assess_impact(&self, actions: &[RemediationAction]) -> Result<ImpactAssessment>;
    fn should_auto_approve(&self, risk_level: &RiskLevel) -> bool;
}
```

### Risk Assessment Example

```rust
use do_codeguardian::remediation::{RemediationService, RemediationAction, RiskLevel};

// Assess risk for a set of actions
let actions = vec![
    RemediationAction::MergeDuplicateCode { /* ... */ },
    RemediationAction::CloseDuplicateIssue { /* ... */ },
];

let risk_level = service.assess_risk_level(&actions);
let impact = service.assess_impact(&actions).await?;

println!("Risk Assessment:");
println!("- Risk Level: {:?}", risk_level);
println!("- Files Affected: {}", impact.files_affected);
println!("- Lines Changed: {}", impact.lines_of_code_changed);
println!("- Breaking Changes: {}", impact.breaking_changes);
println!("- Estimated Time: {} minutes", impact.estimated_time_minutes);
println!("- Confidence Score: {:.2}", impact.confidence_score);

let auto_approve = service.should_auto_approve(&risk_level);
println!("Auto-approve: {}", auto_approve);
```

## Integration Support

### IntegrationConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub github_enabled: bool,
    pub jira_enabled: bool,
    pub slack_enabled: bool,
    pub email_enabled: bool,
    pub webhook_urls: Vec<String>,
}
```

### NotificationConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub notify_on_start: bool,
    pub notify_on_completion: bool,
    pub notify_on_failure: bool,
    pub notify_on_approval_needed: bool,
    pub notification_channels: Vec<NotificationChannel>,
}
```

### NotificationChannel

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Slack,
    Teams,
    Webhook,
    InApp,
}
```

### Integration Example

```toml
[remediation]
enabled = true
auto_approve_low_risk = true
require_approval_threshold = "medium"

[remediation.integrations]
github_enabled = true
jira_enabled = true
slack_enabled = true

[remediation.notification_settings]
notify_on_start = true
notify_on_completion = true
notify_on_failure = true
notify_on_approval_needed = true
notification_channels = ["email", "slack"]
```

## Workflow Execution

### ExecutionLogEntry

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLogEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub status: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
```

### Workflow Execution Process

1. **Validation**: Validate workflow and actions
2. **Preparation**: Set up execution environment
3. **Execution**: Execute each action in sequence
4. **Verification**: Verify action results
5. **Cleanup**: Clean up temporary resources
6. **Notification**: Send completion notifications

### Execution Example

```rust
use do_codeguardian::remediation::RemediationService;

// Execute workflow
let workflow_id = "workflow-12345";
service.execute_workflow(workflow_id).await?;

// Monitor execution
let workflow = service.get_workflow(workflow_id).unwrap();
println!("Workflow Status: {:?}", workflow.status);
println!("Execution Log:");

for log_entry in &workflow.execution_log {
    println!("{} [{}] {}: {}",
             log_entry.timestamp.format("%H:%M:%S"),
             log_entry.status,
             log_entry.action,
             log_entry.message);
}
```

## Monitoring and Logging

### Workflow Monitoring

```rust
use do_codeguardian::remediation::RemediationService;

// Get all active workflows
let active_workflows = service.get_active_workflows();

println!("Active Workflows:");
for workflow in active_workflows {
    println!("- {}: {} ({:?})",
             workflow.id,
             workflow.title,
             workflow.status);
    println!("  Created: {}", workflow.created_at);
    println!("  Risk Level: {:?}", workflow.risk_level);
    println!("  Progress: {}/{} actions",
             workflow.execution_log.len(),
             workflow.actions.len());
}
```

### Execution Logging

```rust
// Detailed execution logging
let workflow = service.get_workflow("workflow-12345").unwrap();

println!("Execution Details for Workflow: {}", workflow.title);
println!("========================================");

for (i, log_entry) in workflow.execution_log.iter().enumerate() {
    println!("{}. [{}] {}", i + 1, log_entry.status, log_entry.action);
    println!("   Time: {}", log_entry.timestamp);
    println!("   Message: {}", log_entry.message);

    if let Some(details) = &log_entry.details {
        println!("   Details: {}", serde_json::to_string_pretty(details)?);
    }
    println!();
}
```

## CLI Integration

### Remediation Commands

```bash
# Create remediation workflow from findings
codeguardian remediate create findings.json

# List active workflows
codeguardian remediate list

# Get workflow status
codeguardian remediate status workflow-12345

# Approve workflow
codeguardian remediate approve workflow-12345

# Reject workflow
codeguardian remediate reject workflow-12345 "Reason for rejection"

# Monitor workflow execution
codeguardian remediate monitor workflow-12345
```

### CLI Examples

```bash
# Create workflow with custom configuration
codeguardian remediate create findings.json --auto-approve --timeout 60

# List workflows with filtering
codeguardian remediate list --status pending --risk-level high

# Get detailed workflow information
codeguardian remediate status workflow-12345 --verbose

# Approve multiple workflows
codeguardian remediate approve workflow-12345 workflow-67890 --comment "Approved after review"
```

## Configuration Examples

### Basic Configuration

```toml
[remediation]
enabled = true
auto_approve_low_risk = true
require_approval_threshold = "medium"
max_concurrent_workflows = 5
timeout_minutes = 30
```

### Advanced Configuration

```toml
[remediation]
enabled = true
auto_approve_low_risk = false
require_approval_threshold = "low"
max_concurrent_workflows = 10
timeout_minutes = 60

[remediation.integrations]
github_enabled = true
jira_enabled = true
slack_enabled = true
email_enabled = true
webhook_urls = [
    "https://webhook.example.com/remediation"
]

[remediation.notification_settings]
notify_on_start = true
notify_on_completion = true
notify_on_failure = true
notify_on_approval_needed = true
notification_channels = ["email", "slack", "webhook"]
```

## Action Implementations

### Actions Module

```rust
pub mod actions {
    pub async fn merge_duplicate_code(
        source_files: &[String],
        target_file: &str,
        merge_strategy: &MergeStrategy,
    ) -> Result<()>;

    pub async fn refactor_duplicate_function(
        function_name: &str,
        source_locations: &[CodeLocation],
        target_location: &CodeLocation,
    ) -> Result<()>;

    pub async fn close_duplicate_issue(
        issue_id: &str,
        duplicate_of: &str,
        comment: &str,
    ) -> Result<()>;

    pub async fn consolidate_documentation(
        source_docs: &[String],
        target_doc: &str,
        merge_sections: &[String],
    ) -> Result<()>;

    pub async fn update_configuration(
        config_file: &str,
        changes: &HashMap<String, serde_json::Value>,
        backup_created: bool,
    ) -> Result<()>;

    pub async fn create_pull_request(
        title: &str,
        description: &str,
        branch_name: &str,
        files_changed: &[String],
    ) -> Result<()>;
}
```

### Custom Action Implementation

```rust
use do_codeguardian::remediation::actions;

// Implement custom remediation action
pub async fn custom_security_fix(
    file_path: &str,
    vulnerability_type: &str,
    fix_strategy: &str,
) -> Result<()> {
    // Load file content
    let content = tokio::fs::read_to_string(file_path).await?;

    // Apply security fix based on vulnerability type
    let fixed_content = match vulnerability_type {
        "sql_injection" => fix_sql_injection(&content, fix_strategy)?,
        "xss" => fix_xss(&content, fix_strategy)?,
        "auth_bypass" => fix_auth_bypass(&content, fix_strategy)?,
        _ => return Err(anyhow::anyhow!("Unsupported vulnerability type")),
    };

    // Create backup
    let backup_path = format!("{}.backup", file_path);
    tokio::fs::copy(file_path, &backup_path).await?;

    // Write fixed content
    tokio::fs::write(file_path, fixed_content).await?;

    tracing::info!("Applied {} fix to {}", vulnerability_type, file_path);
    Ok(())
}
```

## Error Handling

### RemediationError

```rust
#[derive(Debug, thiserror::Error)]
pub enum RemediationError {
    #[error("Workflow validation failed: {0}")]
    ValidationError(String),

    #[error("Action execution failed: {0}")]
    ExecutionError(String),

    #[error("Approval required but not granted")]
    ApprovalRequired,

    #[error("Workflow timeout after {0} minutes")]
    TimeoutError(u32),

    #[error("Integration error: {0}")]
    IntegrationError(String),

    #[error("Risk assessment failed: {0}")]
    RiskAssessmentError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

### Error Handling Example

```rust
use do_codeguardian::remediation::{RemediationService, RemediationError};

match service.execute_workflow(&workflow_id).await {
    Ok(()) => {
        println!("Workflow executed successfully");
    }
    Err(RemediationError::ApprovalRequired) => {
        println!("Workflow requires approval before execution");
        // Request approval
        service.request_approval(&workflow_id).await?;
    }
    Err(RemediationError::ExecutionError(e)) => {
        eprintln!("Workflow execution failed: {}", e);
        // Log error details
        let workflow = service.get_workflow(&workflow_id).unwrap();
        for log_entry in &workflow.execution_log {
            if log_entry.status == "failed" {
                eprintln!("Failed action: {}", log_entry.message);
            }
        }
    }
    Err(RemediationError::TimeoutError(minutes)) => {
        eprintln!("Workflow timed out after {} minutes", minutes);
        // Cancel workflow
        service.cancel_workflow(&workflow_id, "Timeout".to_string()).await?;
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Performance Considerations

1. **Concurrent Workflows**: Limit `max_concurrent_workflows` based on system resources
2. **Timeout Management**: Set appropriate `timeout_minutes` to prevent hanging workflows
3. **Resource Cleanup**: Ensure proper cleanup of temporary files and resources
4. **Batch Processing**: Use bulk operations for multiple similar actions
5. **Caching**: Cache frequently accessed files and metadata
6. **Async Execution**: Use async execution for I/O-bound operations
7. **Progress Tracking**: Implement progress tracking for long-running workflows

## Security Considerations

1. **Approval Gates**: Always require approval for high-risk actions
2. **Audit Logging**: Log all workflow actions and approvals
3. **Access Control**: Implement proper access controls for workflow management
4. **Input Validation**: Validate all inputs and file paths
5. **Backup Creation**: Always create backups before making changes
6. **Rollback Support**: Implement rollback mechanisms for failed actions
7. **Secure Communication**: Use secure channels for integrations

## Best Practices

1. **Risk Assessment**: Always perform risk assessment before execution
2. **Testing**: Test remediation actions in staging environments first
3. **Monitoring**: Monitor workflow execution and failure rates
4. **Documentation**: Document all remediation actions and their impacts
5. **Version Control**: Ensure all changes are properly version controlled
6. **Communication**: Notify stakeholders about remediation activities
7. **Gradual Rollout**: Start with low-risk remediations and gradually increase scope
8. **Quality Gates**: Implement quality gates before and after remediation

## Integration with CI/CD

### GitHub Actions Integration

```yaml
name: Automated Remediation
on:
  push:
    branches: [ main ]

jobs:
  remediation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Run CodeGuardian Analysis
        run: codeguardian analyze . --output json > findings.json

      - name: Create Remediation Workflow
        run: |
          WORKFLOW_ID=$(codeguardian remediate create findings.json)
          echo "WORKFLOW_ID=$WORKFLOW_ID" >> $GITHUB_ENV

      - name: Wait for Approval (if required)
        run: |
          STATUS=$(codeguardian remediate status $WORKFLOW_ID --format json | jq -r .status)
          if [ "$STATUS" = "AwaitingApproval" ]; then
            echo "Workflow requires approval"
            exit 1
          fi

      - name: Execute Remediation
        run: codeguardian remediate execute $WORKFLOW_ID

      - name: Create Pull Request
        uses: peter-evans/create-pull-request@v4
        with:
          title: "Automated Remediation"
          body: "This PR contains automated fixes for detected issues"
```

### Jenkins Pipeline Integration

```groovy
pipeline {
    agent any

    stages {
        stage('Analysis') {
            steps {
                sh 'codeguardian analyze . --output json > findings.json'
            }
        }

        stage('Remediation') {
            steps {
                script {
                    def workflowId = sh(
                        script: 'codeguardian remediate create findings.json',
                        returnStdout: true
                    ).trim()

                    def status = sh(
                        script: "codeguardian remediate status ${workflowId} --format json",
                        returnStdout: true
                    ).trim()

                    if (status.contains('"status":"AwaitingApproval"')) {
                        input message: 'Approve remediation workflow?'
                        sh "codeguardian remediate approve ${workflowId}"
                    }

                    sh "codeguardian remediate execute ${workflowId}"
                }
            }
        }

        stage('Verification') {
            steps {
                sh 'codeguardian analyze . --output json > verification.json'
                sh 'codeguardian verify verification.json findings.json'
            }
        }
    }
}
```
