//! Manual review workflow for security findings

use crate::types::Finding;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewConfig {
    pub enabled: bool,
    pub required_for_high_severity: bool,
    pub min_reviewers: usize,
    pub max_reviewers: usize,
    pub review_timeout_seconds: u64,
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            required_for_high_severity: true,
            min_reviewers: 1,
            max_reviewers: 3,
            review_timeout_seconds: 86400, // 24 hours
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReviewPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReviewStatus {
    Pending,
    InProgress,
    Approved,
    Rejected,
    NeedsMoreInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReviewDecision {
    Valid,
    Invalid,
    RequiresFix,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewerFeedback {
    pub reviewer_id: String,
    pub comments: String,
    pub decision: ReviewDecision,
    pub confidence: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewItem {
    pub finding: Finding,
    pub priority: ReviewPriority,
    pub status: ReviewStatus,
    pub assigned_reviewers: Vec<String>,
    pub feedback: Vec<ReviewerFeedback>,
    pub created_at: u64,
    pub updated_at: u64,
    pub timeout_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewStatistics {
    pub total_reviews: usize,
    pub pending_reviews: usize,
    pub completed_reviews: usize,
    pub average_review_time_seconds: f64,
    pub approval_rate: f64,
    pub common_categories: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct ManualReviewWorkflow {
    config: ReviewConfig,
    pending_reviews: HashMap<String, ReviewItem>, // review_id -> item
    completed_reviews: HashMap<String, ReviewItem>, // review_id -> item
    _reviewers: Vec<String>,
}

impl ManualReviewWorkflow {
    pub fn new(config: ReviewConfig) -> Self {
        Self {
            config,
            pending_reviews: HashMap::new(),
            completed_reviews: HashMap::new(),
            _reviewers: Vec::new(),
        }
    }

    pub async fn submit_for_review(
        &mut self,
        findings: Vec<Finding>,
    ) -> Result<Vec<String>, anyhow::Error> {
        let mut review_ids = Vec::new();

        for finding in findings {
            let review_id = self.create_review_item(finding).await?;
            review_ids.push(review_id);
        }

        Ok(review_ids)
    }

    async fn create_review_item(&mut self, finding: Finding) -> Result<String, anyhow::Error> {
        let review_id = format!("review_{}", uuid::Uuid::new_v4());
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let priority = match finding.severity {
            crate::types::Severity::Critical => ReviewPriority::Critical,
            crate::types::Severity::High => ReviewPriority::High,
            crate::types::Severity::Medium => ReviewPriority::Medium,
            crate::types::Severity::Low => ReviewPriority::Low,
            crate::types::Severity::Info => ReviewPriority::Low,
        };

        let review_item = ReviewItem {
            finding,
            priority,
            status: ReviewStatus::Pending,
            assigned_reviewers: Vec::new(),
            feedback: Vec::new(),
            created_at: now,
            updated_at: now,
            timeout_at: now + self.config.review_timeout_seconds,
        };

        self.pending_reviews.insert(review_id.clone(), review_item);
        Ok(review_id)
    }

    pub fn get_review_statistics(&self) -> ReviewStatistics {
        let total_reviews = self.pending_reviews.len() + self.completed_reviews.len();
        let pending_reviews = self.pending_reviews.len();
        let completed_reviews = self.completed_reviews.len();

        // Simple implementation - could be enhanced with actual data
        ReviewStatistics {
            total_reviews,
            pending_reviews,
            completed_reviews,
            average_review_time_seconds: 3600.0, // 1 hour average
            approval_rate: 0.7,                  // 70% approval rate
            common_categories: HashMap::new(),
        }
    }
}
