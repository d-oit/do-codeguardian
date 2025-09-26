# remediation

## Synopsis
Manage automated remediation workflows for security findings, enabling systematic fixing of identified issues with approval workflows and execution tracking.

## Description
The remediation command provides comprehensive workflow management for automated code fixes. It creates remediation workflows from analysis findings, manages approval processes, and executes fixes with proper risk assessment and rollback capabilities.

Key capabilities include:
- **Workflow Creation**: Generate remediation plans from security analysis results
- **Approval Workflows**: Multi-step approval process for high-risk changes
- **Risk Assessment**: Automatic risk level evaluation for remediation actions
- **Execution Tracking**: Detailed logging and status monitoring of fix execution
- **Rollback Support**: Safe rollback mechanisms for failed or problematic fixes
- **Concurrent Management**: Control maximum concurrent remediation operations

## Syntax
```bash
codeguardian remediation [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--create-workflow <FILE>` | Create remediation workflow from findings file | `PATH` | - | No |
| `--list-workflows` | List all active workflows | `FLAG` | `false` | No |
| `--workflow-status <ID>` | Show detailed status of specific workflow | `STRING` | - | No |
| `--approve <ID>` | Approve and execute workflow | `STRING` | - | No |
| `--reject <ID> --reason <MSG>` | Reject workflow with reason | `STRING` | - | No |
| `--reason <MSG>` | Rejection reason (required with --reject) | `STRING` | - | No |
| `--force-execute` | Execute workflow immediately (bypass approval) | `FLAG` | `false` | No |
| `--show-log <ID>` | Show execution log for workflow | `STRING` | - | No |
| `--cancel <ID>` | Cancel running workflow | `STRING` | - | No |
| `--auto-approve-low-risk` | Enable auto-approval for low-risk workflows | `FLAG` | `false` | No |
| `--max-concurrent <NUM>` | Set maximum concurrent workflows | `u32` | - | No |

## Examples

### Basic Usage
```bash
# Create remediation workflow from analysis results
codeguardian remediation --create-workflow analysis-results.json

# List all active workflows
codeguardian remediation --list-workflows

# Check status of specific workflow
codeguardian remediation --workflow-status workflow-123
```

### Workflow Management
```bash
# Approve a pending workflow
codeguardian remediation --approve workflow-123

# Reject workflow with reason
codeguardian remediation --reject workflow-123 --reason "Requires manual review"

# View execution log
codeguardian remediation --show-log workflow-123

# Cancel running workflow
codeguardian remediation --cancel workflow-123
```

### Advanced Configuration
```bash
# Enable auto-approval for low-risk fixes
codeguardian remediation --auto-approve-low-risk --max-concurrent 3

# Force execute high-risk workflow (bypass approval)
codeguardian remediation --force-execute --create-workflow critical-findings.json
```

### Integration with Analysis
```bash
# Complete workflow: analyze, create remediation, approve
codeguardian check --output findings.json src/
codeguardian remediation --create-workflow findings.json
codeguardian remediation --list-workflows
codeguardian remediation --approve <workflow-id>
```

## Workflow States

Remediation workflows progress through the following states:

- **Pending**: Workflow created, awaiting approval
- **AwaitingApproval**: Risk assessment completed, requires approval
- **Approved**: Approved for execution
- **InProgress**: Currently executing fixes
- **Completed**: All actions successfully executed
- **Rejected**: Workflow rejected (cannot be restarted)
- **Failed**: Execution failed (can be retried)
- **Cancelled**: Manually cancelled by user

## Risk Assessment

Workflows are automatically categorized by risk level:

- **Low Risk**: Simple fixes (typo corrections, formatting)
- **Medium Risk**: Code changes with limited impact
- **High Risk**: Significant changes requiring approval
- **Critical Risk**: Changes affecting core functionality

## Execution Process

1. **Analysis**: Review findings and generate remediation actions
2. **Risk Assessment**: Evaluate impact and risk level
3. **Approval**: Obtain approval for high-risk workflows
4. **Execution**: Apply fixes in safe, reversible manner
5. **Validation**: Verify fixes don't introduce new issues
6. **Logging**: Record all actions and outcomes

## Output Format

### Workflow Status Display
```
📋 Workflow Details

ID: workflow-123
Title: Security vulnerability fixes for authentication module
Description: Automated fixes for SQL injection and XSS vulnerabilities
Status: AwaitingApproval
Risk Level: High
Created: 2025-09-25 10:30:00 UTC
Created By: security-scanner

🎯 Impact Assessment
Files Affected: 3
Lines Changed: 15
Breaking Changes: No
Estimated Time: 2 minutes
Confidence: 92.3%

🔧 Planned Actions (3)
  1. Fix SQL injection in user authentication
  2. Sanitize XSS input in login form
  3. Update session handling
```

### Workflow List Display
```
📋 Active Remediation Workflows

ID                 Title                          Status             Risk   Actions
workflow-123       Auth fixes                    AwaitingApproval   High   3
workflow-124       Dependency updates            InProgress         Medium 5
workflow-125       Code formatting               Completed          Low    12
```

## Error Handling

### Common Errors
- **Workflow Not Found**: Specified workflow ID does not exist
- **Permission Denied**: Insufficient permissions for workflow operation
- **Invalid State**: Operation not allowed for current workflow state
- **Execution Failed**: Remediation action failed to apply
- **Approval Required**: High-risk workflow requires approval before execution

## Security Considerations
- **Approval Gates**: High-risk changes require explicit approval
- **Rollback Capability**: All changes can be safely rolled back
- **Audit Trail**: Complete logging of all workflow operations
- **Access Control**: Role-based permissions for workflow management
- **Validation**: Post-execution validation ensures fixes are correct

## Integration with CI/CD

### GitHub Actions Example
```yaml
- name: Run Security Analysis
  run: codeguardian check --output findings.json --format json

- name: Create Remediation Workflow
  run: codeguardian remediation --create-workflow findings.json
  continue-on-error: true

- name: Auto-approve Low Risk Fixes
  run: codeguardian remediation --auto-approve-low-risk --max-concurrent 2
```

### Jenkins Pipeline Example
```groovy
stage('Security Remediation') {
    steps {
        sh 'codeguardian check --output findings.json'
        sh 'codeguardian remediation --create-workflow findings.json'
        sh 'codeguardian remediation --auto-approve-low-risk'
    }
}
```

## See Also
- [`codeguardian check`](check.md) - Generate findings for remediation
- [`codeguardian report`](report.md) - Convert results to different formats
- [`codeguardian gh-issue`](gh-issue.md) - Create GitHub issues from findings