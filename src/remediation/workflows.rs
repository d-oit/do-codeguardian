//! Workflow management for automated remediation

use super::{RemediationWorkflow, WorkflowStatus};
use anyhow::Result;
use std::collections::HashMap;
use chrono::Utc;

/// Workflow manager for handling remediation workflows
pub struct WorkflowManager {
    workflows: HashMap<String, RemediationWorkflow>,
    max_concurrent: u32,
}

impl WorkflowManager {
    pub fn new(max_concurrent: u32) -> Self {
        Self {
            workflows: HashMap::new(),
            max_concurrent,
        }
    }

    /// Add a new workflow
    pub fn add_workflow(&mut self, workflow: RemediationWorkflow) -> Result<()> {
        let active_count = self.workflows.values()
            .filter(|w| matches!(w.status, WorkflowStatus::InProgress))
            .count() as u32;

        if active_count >= self.max_concurrent {
            return Err(anyhow::anyhow!("Maximum concurrent workflows reached"));
        }

        self.workflows.insert(workflow.id.clone(), workflow);
        Ok(())
    }

    /// Get workflow by ID
    pub fn get_workflow(&self, id: &str) -> Option<&RemediationWorkflow> {
        self.workflows.get(id)
    }

    /// Get mutable workflow by ID
    pub fn get_workflow_mut(&mut self, id: &str) -> Option<&mut RemediationWorkflow> {
        self.workflows.get_mut(id)
    }

    /// List all workflows
    pub fn list_workflows(&self) -> Vec<&RemediationWorkflow> {
        self.workflows.values().collect()
    }

    /// List workflows by status
    pub fn list_workflows_by_status(&self, status: &WorkflowStatus) -> Vec<&RemediationWorkflow> {
        self.workflows.values()
            .filter(|w| std::mem::discriminant(&w.status) == std::mem::discriminant(status))
            .collect()
    }

    /// Update workflow status
    pub fn update_workflow_status(&mut self, id: &str, status: WorkflowStatus) -> Result<()> {
        if let Some(workflow) = self.workflows.get_mut(id) {
            workflow.status = status;
            workflow.updated_at = Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Workflow not found: {}", id))
        }
    }

    /// Remove completed workflows older than specified days
    pub fn cleanup_old_workflows(&mut self, days: u32) -> usize {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        let initial_count = self.workflows.len();

        self.workflows.retain(|_, workflow| {
            !matches!(workflow.status, WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled)
                || workflow.updated_at > cutoff
        });

        initial_count - self.workflows.len()
    }
}
