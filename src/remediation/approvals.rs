//! Approval workflow management

use super::{ApprovalRequest, RiskLevel};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Approval manager for handling workflow approvals
pub struct ApprovalManager {
    pending_approvals: HashMap<String, ApprovalRequest>,
    approval_history: Vec<ApprovalRecord>,
}

/// Record of approval decisions
#[derive(Debug, Clone)]
pub struct ApprovalRecord {
    pub workflow_id: String,
    pub decision: ApprovalDecision,
    pub decided_by: String,
    pub decided_at: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Approval decision types
#[derive(Debug, Clone)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    Escalated,
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            pending_approvals: HashMap::new(),
            approval_history: Vec::new(),
        }
    }

    /// Submit an approval request
    pub fn submit_approval_request(&mut self, request: ApprovalRequest) -> Result<()> {
        self.pending_approvals
            .insert(request.workflow_id.clone(), request);
        Ok(())
    }

    /// Get pending approval request
    pub fn get_pending_approval(&self, workflow_id: &str) -> Option<&ApprovalRequest> {
        self.pending_approvals.get(workflow_id)
    }

    /// List all pending approvals
    pub fn list_pending_approvals(&self) -> Vec<&ApprovalRequest> {
        self.pending_approvals.values().collect()
    }

    /// Approve a workflow
    pub fn approve_workflow(
        &mut self,
        workflow_id: &str,
        approved_by: String,
        reason: Option<String>,
    ) -> Result<()> {
        if let Some(_request) = self.pending_approvals.remove(workflow_id) {
            let record = ApprovalRecord {
                workflow_id: workflow_id.to_string(),
                decision: ApprovalDecision::Approved,
                decided_by: approved_by,
                decided_at: Utc::now(),
                reason,
            };
            self.approval_history.push(record);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "No pending approval found for workflow: {}",
                workflow_id
            ))
        }
    }

    /// Reject a workflow
    pub fn reject_workflow(
        &mut self,
        workflow_id: &str,
        rejected_by: String,
        reason: String,
    ) -> Result<()> {
        if let Some(_request) = self.pending_approvals.remove(workflow_id) {
            let record = ApprovalRecord {
                workflow_id: workflow_id.to_string(),
                decision: ApprovalDecision::Rejected,
                decided_by: rejected_by,
                decided_at: Utc::now(),
                reason: Some(reason),
            };
            self.approval_history.push(record);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "No pending approval found for workflow: {}",
                workflow_id
            ))
        }
    }

    /// Check if approval is required based on risk level
    pub fn is_approval_required(&self, risk_level: &RiskLevel, threshold: &RiskLevel) -> bool {
        risk_level >= threshold
    }

    /// Get approval history for a workflow
    pub fn get_approval_history(&self, workflow_id: &str) -> Vec<&ApprovalRecord> {
        self.approval_history
            .iter()
            .filter(|record| record.workflow_id == workflow_id)
            .collect()
    }

    /// Check for expired approval requests
    pub fn check_expired_approvals(&mut self) -> Vec<String> {
        let now = Utc::now();
        let mut expired = Vec::new();

        self.pending_approvals.retain(|workflow_id, request| {
            if let Some(deadline) = request.deadline {
                if now > deadline {
                    expired.push(workflow_id.clone());
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });

        expired
    }
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self::new()
    }
}
