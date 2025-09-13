//! Remediation CLI commands

use crate::config::Config;
use crate::remediation::{RemediationConfig, RemediationService, WorkflowStatus};
use crate::types::Finding;
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct RemediationArgs {
    /// Create remediation workflow from findings file
    #[arg(long)]
    pub create_workflow: Option<PathBuf>,

    /// List active workflows
    #[arg(long)]
    pub list_workflows: bool,

    /// Get workflow status
    #[arg(long)]
    pub workflow_status: Option<String>,

    /// Approve workflow
    #[arg(long)]
    pub approve: Option<String>,

    /// Reject workflow
    #[arg(long)]
    pub reject: Option<String>,

    /// Rejection reason (required with --reject)
    #[arg(long)]
    pub reason: Option<String>,

    /// Execute workflow immediately (bypass approval)
    #[arg(long)]
    pub force_execute: bool,

    /// Show workflow execution log
    #[arg(long)]
    pub show_log: Option<String>,

    /// Cancel workflow
    #[arg(long)]
    pub cancel: Option<String>,

    /// Enable auto-approval for low-risk workflows
    #[arg(long)]
    pub auto_approve_low_risk: bool,

    /// Set maximum concurrent workflows
    #[arg(long)]
    pub max_concurrent: Option<u32>,
}

pub async fn run(args: RemediationArgs, _config: &Config) -> Result<()> {
    let remediation_config = RemediationConfig {
        enabled: true,
        auto_approve_low_risk: args.auto_approve_low_risk,
        max_concurrent_workflows: args.max_concurrent.unwrap_or(5),
        ..Default::default()
    };

    let mut service = RemediationService::new(remediation_config);

    if let Some(findings_file) = args.create_workflow {
        create_workflow_from_file(&mut service, &findings_file).await?;
    } else if args.list_workflows {
        list_workflows(&service)?;
    } else if let Some(workflow_id) = args.workflow_status {
        show_workflow_status(&service, &workflow_id)?;
    } else if let Some(workflow_id) = args.approve {
        approve_workflow(&mut service, &workflow_id).await?;
    } else if let Some(workflow_id) = args.reject {
        let reason = args.reason.unwrap_or_else(|| "No reason provided".to_string());
        reject_workflow(&mut service, &workflow_id, &reason).await?;
    } else if let Some(workflow_id) = args.show_log {
        show_workflow_log(&service, &workflow_id)?;
    } else if let Some(workflow_id) = args.cancel {
        cancel_workflow(&mut service, &workflow_id).await?;
    } else {
        show_remediation_help();
    }

    Ok(())
}

async fn create_workflow_from_file(
    service: &mut RemediationService,
    findings_file: &PathBuf,
) -> Result<()> {
    tracing::info!("Creating remediation workflow from findings file: {:?}", findings_file);

    // Read findings from file
    let content = tokio::fs::read_to_string(findings_file).await?;
    let findings: Vec<Finding> = serde_json::from_str(&content)?;

    if findings.is_empty() {
        tracing::warn!("No findings found in file");
        return Ok(());
    }

    tracing::info!("Found {} findings to process", findings.len());

    // Create workflow
    let workflow_id = service
        .create_workflow_from_findings(findings, "cli-user".to_string())
        .await?;

    tracing::info!("Remediation workflow created: {}", workflow_id);

    // Show workflow details
    if let Some(workflow) = service.get_workflow(&workflow_id) {
        println!("\n📋 Workflow Created");
        println!("ID: {}", workflow.id);
        println!("Title: {}", workflow.title);
        println!("Status: {:?}", workflow.status);
        println!("Risk Level: {:?}", workflow.risk_level);
        println!("Actions: {} planned", workflow.actions.len());
        println!("Approval Required: {}", workflow.approval_required);

        if workflow.approval_required {
            println!("\n⚠️  This workflow requires approval before execution.");
            println!("Use: codeguardian remediation --approve {}", workflow_id);
        }
    }

    Ok(())
}

fn list_workflows(service: &RemediationService) -> Result<()> {
    let workflows = service.get_active_workflows();

    if workflows.is_empty() {
        println!("No active workflows found.");
        return Ok(());
    }

    println!("\n📋 Active Remediation Workflows\n");
    println!("{:<36} {:<30} {:<15} {:<10} {:<8}",
        "ID", "Title", "Status", "Risk", "Actions");
    println!("{}", "-".repeat(100));

    for workflow in workflows {
        let status_icon = match workflow.status {
            WorkflowStatus::Pending => "⏳",
            WorkflowStatus::InProgress => "🔄",
            WorkflowStatus::AwaitingApproval => "⚠️",
            WorkflowStatus::Approved => "✅",
            WorkflowStatus::Rejected => "❌",
            WorkflowStatus::Completed => "✅",
            WorkflowStatus::Failed => "💥",
            WorkflowStatus::Cancelled => "🚫",
        };

        println!("{} {:<35} {:<30} {:<15} {:<10} {:<8}",
            status_icon,
            &workflow.id[..8],
            truncate_string(&workflow.title, 28),
            format!("{:?}", workflow.status),
            format!("{:?}", workflow.risk_level),
            workflow.actions.len()
        );
    }

    println!("\nUse --workflow-status <id> to see detailed information about a workflow.");
    Ok(())
}

fn show_workflow_status(service: &RemediationService, workflow_id: &str) -> Result<()> {
    let workflow = service
        .get_workflow(workflow_id)
        .ok_or_else(|| anyhow::anyhow!("Workflow not found: {}", workflow_id))?;

    println!("\n📋 Workflow Details\n");
    println!("ID: {}", workflow.id);
    println!("Title: {}", workflow.title);
    println!("Description: {}", workflow.description);
    println!("Status: {:?}", workflow.status);
    println!("Risk Level: {:?}", workflow.risk_level);
    println!("Created: {}", workflow.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Updated: {}", workflow.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Created By: {}", workflow.created_by);

    if let Some(approved_by) = &workflow.approved_by {
        println!("Approved By: {}", approved_by);
    }

    println!("\n🎯 Impact Assessment");
    println!("Files Affected: {}", workflow.estimated_impact.files_affected);
    println!("Lines Changed: {}", workflow.estimated_impact.lines_of_code_changed);
    println!("Breaking Changes: {}", workflow.estimated_impact.breaking_changes);
    println!("Estimated Time: {} minutes", workflow.estimated_impact.estimated_time_minutes);
    println!("Confidence: {:.1}%", workflow.estimated_impact.confidence_score * 100.0);

    println!("\n🔧 Planned Actions ({})", workflow.actions.len());
    for (i, action) in workflow.actions.iter().enumerate() {
        println!("  {}. {:?}", i + 1, action);
    }

    if !workflow.execution_log.is_empty() {
        println!("\n📝 Execution Log");
        for entry in &workflow.execution_log {
            println!("  [{}] {}: {}",
                entry.timestamp.format("%H:%M:%S"),
                entry.status.to_uppercase(),
                entry.message
            );
        }
    }

    Ok(())
}

async fn approve_workflow(service: &mut RemediationService, workflow_id: &str) -> Result<()> {
    tracing::info!("Approving workflow: {}", workflow_id);

    service
        .approve_workflow(workflow_id, "cli-user".to_string())
        .await?;

    println!("✅ Workflow {} approved and execution started", workflow_id);
    Ok(())
}

async fn reject_workflow(
    service: &mut RemediationService,
    workflow_id: &str,
    reason: &str,
) -> Result<()> {
    tracing::info!("Rejecting workflow: {} (reason: {})", workflow_id, reason);

    service
        .reject_workflow(workflow_id, "cli-user".to_string(), reason.to_string())
        .await?;

    println!("❌ Workflow {} rejected: {}", workflow_id, reason);
    Ok(())
}

fn show_workflow_log(service: &RemediationService, workflow_id: &str) -> Result<()> {
    let workflow = service
        .get_workflow(workflow_id)
        .ok_or_else(|| anyhow::anyhow!("Workflow not found: {}", workflow_id))?;

    if workflow.execution_log.is_empty() {
        println!("No execution log available for workflow: {}", workflow_id);
        return Ok(());
    }

    println!("\n📝 Execution Log for Workflow: {}\n", workflow_id);

    for entry in &workflow.execution_log {
        let status_icon = match entry.status.as_str() {
            "started" => "🔄",
            "completed" => "✅",
            "failed" => "❌",
            "rejected" => "🚫",
            _ => "ℹ️",
        };

        println!("{} [{}] {}: {}",
            status_icon,
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            entry.status.to_uppercase(),
            entry.message
        );

        if let Some(details) = &entry.details {
            println!("   Details: {}", details);
        }
    }

    Ok(())
}

async fn cancel_workflow(service: &mut RemediationService, workflow_id: &str) -> Result<()> {
    tracing::info!("Cancelling workflow: {}", workflow_id);

    // In a real implementation, this would properly cancel the workflow
    // For now, we'll just reject it with a cancellation reason
    service
        .reject_workflow(workflow_id, "cli-user".to_string(), "Cancelled by user".to_string())
        .await?;

    println!("🚫 Workflow {} cancelled", workflow_id);
    Ok(())
}

fn show_remediation_help() {
    println!("\n🔧 CodeGuardian Automated Remediation\n");
    println!("Available commands:");
    println!("  --create-workflow <file>     Create workflow from findings JSON file");
    println!("  --list-workflows             List all active workflows");
    println!("  --workflow-status <id>       Show detailed workflow information");
    println!("  --approve <id>               Approve and execute workflow");
    println!("  --reject <id> --reason <msg> Reject workflow with reason");
    println!("  --show-log <id>              Show workflow execution log");
    println!("  --cancel <id>                Cancel workflow");
    println!("  --auto-approve-low-risk      Enable auto-approval for low-risk workflows");
    println!("  --max-concurrent <n>         Set maximum concurrent workflows");
    println!("\nExample workflow creation:");
    println!("  codeguardian check --output findings.json src/");
    println!("  codeguardian remediation --create-workflow findings.json");
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
