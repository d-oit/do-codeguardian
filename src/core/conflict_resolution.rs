//! Conflict resolution protocol for handling conflicting recommendations from multiple agents

use crate::core::swarm_types::{ConflictInfo, ConflictResolutionStrategy, Priority, SwarmError};
use crate::types::Finding;

use std::collections::HashMap;
use std::sync::Arc;

/// Conflict resolver that handles different resolution strategies
pub struct ConflictResolver {
    strategy: ConflictResolutionStrategy,
    agent_priorities: HashMap<String, Priority>,
    confidence_thresholds: HashMap<String, f64>,
}

impl ConflictResolver {
    pub fn new(
        strategy: ConflictResolutionStrategy,
        agent_priorities: HashMap<String, Priority>,
        confidence_thresholds: HashMap<String, f64>,
    ) -> Self {
        Self {
            strategy,
            agent_priorities,
            confidence_thresholds,
        }
    }

    /// Resolve conflicts in a set of findings
    pub async fn resolve_conflicts(
        &self,
        conflicts: Vec<ConflictInfo>,
    ) -> Result<Vec<ResolvedConflict>, SwarmError> {
        let mut resolved = Vec::new();

        for conflict in conflicts {
            let resolution = self.resolve_single_conflict(conflict).await?;
            resolved.push(resolution);
        }

        Ok(resolved)
    }

    /// Resolve a single conflict
    async fn resolve_single_conflict(
        &self,
        conflict: ConflictInfo,
    ) -> Result<ResolvedConflict, SwarmError> {
        match self.strategy {
            ConflictResolutionStrategy::PriorityBased => {
                self.resolve_priority_based(conflict).await
            }
            ConflictResolutionStrategy::ConfidenceBased => {
                self.resolve_confidence_based(conflict).await
            }
            ConflictResolutionStrategy::ConsensusBased => {
                self.resolve_consensus_based(conflict).await
            }
            ConflictResolutionStrategy::ManualReview => self.resolve_manual_review(conflict).await,
        }
    }

    /// Resolve conflict using priority-based strategy
    async fn resolve_priority_based(
        &self,
        conflict: ConflictInfo,
    ) -> Result<ResolvedConflict, SwarmError> {
        let mut best_finding = None;
        let mut best_priority = Priority::Low;
        let mut selected_agent = String::new();

        for (i, finding) in conflict.conflicting_findings.iter().enumerate() {
            let agent_id: &String = &conflict.agent_ids[i];
            let priority = self
                .agent_priorities
                .get(agent_id)
                .copied()
                .unwrap_or(Priority::Medium);

            if priority > best_priority {
                best_priority = priority;
                best_finding = Some(finding.clone());
                selected_agent = agent_id.clone();
            }
        }

        Ok(ResolvedConflict {
            original_conflict: conflict,
            resolved_finding: best_finding,
            resolution_method: "priority_based".to_string(),
            selected_agent: Some(selected_agent),
            confidence_score: None,
            requires_manual_review: false,
        })
    }

    /// Resolve conflict using confidence-based strategy
    async fn resolve_confidence_based(
        &self,
        conflict: ConflictInfo,
    ) -> Result<ResolvedConflict, SwarmError> {
        let mut best_finding = None;
        let mut best_confidence = 0.0;
        let mut selected_agent = String::new();

        for (i, finding) in conflict.conflicting_findings.iter().enumerate() {
            let agent_id: &String = &conflict.agent_ids[i];
            let confidence = self
                .confidence_thresholds
                .get(agent_id)
                .copied()
                .unwrap_or(0.5);

            // In practice, you'd extract confidence from the finding metadata
            // For now, we'll use agent confidence as a proxy
            if confidence > best_confidence {
                best_confidence = confidence;
                best_finding = Some(finding.clone());
                selected_agent = agent_id.clone();
            }
        }

        Ok(ResolvedConflict {
            original_conflict: conflict,
            resolved_finding: best_finding,
            resolution_method: "confidence_based".to_string(),
            selected_agent: Some(selected_agent),
            confidence_score: Some(best_confidence),
            requires_manual_review: false,
        })
    }

    /// Resolve conflict using consensus-based strategy
    async fn resolve_consensus_based(
        &self,
        conflict: ConflictInfo,
    ) -> Result<ResolvedConflict, SwarmError> {
        if conflict.conflicting_findings.is_empty() {
            return Ok(ResolvedConflict {
                original_conflict: conflict,
                resolved_finding: None,
                resolution_method: "consensus_no_findings".to_string(),
                selected_agent: None,
                confidence_score: None,
                requires_manual_review: false,
            });
        }

        // Group findings by similarity
        let mut finding_groups: HashMap<String, Vec<(Finding, String)>> = HashMap::new();
        for (i, finding) in conflict.conflicting_findings.iter().enumerate() {
            let key = self.generate_similarity_key(finding);
            finding_groups
                .entry(key)
                .or_default()
                .push((finding.clone(), conflict.agent_ids[i].clone()));
        }

        // Find the group with the most findings (consensus)
        let mut consensus_group: Vec<(Finding, String)> = Vec::new();
        let mut max_count = 0;

        for group in finding_groups.values() {
            if group.len() > max_count {
                max_count = group.len();
                consensus_group = group.clone();
            }
        }

        // If we have a clear consensus (more than one agent agrees)
        if max_count > 1 {
            let (finding, agent_id): &(Finding, String) = &consensus_group[0];
            let confidence = Some(max_count as f64 / conflict.conflicting_findings.len() as f64);
            Ok(ResolvedConflict {
                original_conflict: conflict,
                resolved_finding: Some(finding.clone()),
                resolution_method: "consensus_agreement".to_string(),
                selected_agent: Some(agent_id.clone()),
                confidence_score: confidence,
                requires_manual_review: false,
            })
        } else {
            // No consensus, fall back to manual review
            Ok(ResolvedConflict {
                original_conflict: conflict,
                resolved_finding: None,
                resolution_method: "consensus_fallback_manual".to_string(),
                selected_agent: None,
                confidence_score: None,
                requires_manual_review: true,
            })
        }
    }

    /// Resolve conflict requiring manual review
    async fn resolve_manual_review(
        &self,
        conflict: ConflictInfo,
    ) -> Result<ResolvedConflict, SwarmError> {
        // In a real implementation, this would queue the conflict for manual review
        // For now, we'll just mark it as requiring review
        Ok(ResolvedConflict {
            original_conflict: conflict,
            resolved_finding: None,
            resolution_method: "manual_review_required".to_string(),
            selected_agent: None,
            confidence_score: None,
            requires_manual_review: true,
        })
    }

    /// Generate a similarity key for grouping similar findings
    fn generate_similarity_key(&self, finding: &Finding) -> String {
        // Create a key based on key characteristics of the finding
        format!(
            "{}:{}:{}:{}",
            finding.category.as_deref().unwrap_or("none"),
            finding.rule,
            finding.file.display(),
            finding.line
        )
    }

    /// Validate that a resolution is acceptable
    pub fn validate_resolution(&self, resolution: &ResolvedConflict) -> Result<(), SwarmError> {
        // Check if the resolution meets quality criteria
        if resolution.resolved_finding.is_none() && !resolution.requires_manual_review {
            return Err(SwarmError::InternalError(
                "Invalid resolution: no finding and no manual review required".to_string(),
            ));
        }

        // Additional validation logic could go here
        // For example, checking if the selected agent is authorized for this type of finding

        Ok(())
    }

    /// Get statistics about conflict resolution
    pub fn get_resolution_stats(&self, resolutions: &[ResolvedConflict]) -> ResolutionStats {
        let total_conflicts = resolutions.len();
        let resolved_automatically = resolutions
            .iter()
            .filter(|r| r.resolved_finding.is_some() && !r.requires_manual_review)
            .count();
        let requires_manual_review = resolutions
            .iter()
            .filter(|r| r.requires_manual_review)
            .count();

        let mut method_counts = HashMap::new();
        for resolution in resolutions {
            *method_counts
                .entry(resolution.resolution_method.clone())
                .or_insert(0) += 1;
        }

        ResolutionStats {
            total_conflicts,
            resolved_automatically,
            requires_manual_review,
            resolution_methods: method_counts,
        }
    }
}

/// Result of resolving a conflict
#[derive(Debug, Clone)]
pub struct ResolvedConflict {
    pub original_conflict: ConflictInfo,
    pub resolved_finding: Option<Finding>,
    pub resolution_method: String,
    pub selected_agent: Option<String>,
    pub confidence_score: Option<f64>,
    pub requires_manual_review: bool,
}

/// Statistics about conflict resolution
#[derive(Debug, Clone)]
pub struct ResolutionStats {
    pub total_conflicts: usize,
    pub resolved_automatically: usize,
    pub requires_manual_review: usize,
    pub resolution_methods: HashMap<String, usize>,
}

/// Advanced conflict resolver with machine learning support
pub struct MLConflictResolver {
    base_resolver: ConflictResolver,
    ml_model: Option<Arc<dyn MLConflictPredictor>>,
}

impl MLConflictResolver {
    pub fn new(
        strategy: ConflictResolutionStrategy,
        agent_priorities: HashMap<String, Priority>,
        confidence_thresholds: HashMap<String, f64>,
        ml_model: Option<Arc<dyn MLConflictPredictor>>,
    ) -> Self {
        let base_resolver =
            ConflictResolver::new(strategy, agent_priorities, confidence_thresholds);
        Self {
            base_resolver,
            ml_model,
        }
    }

    /// Resolve conflicts using ML when available
    pub async fn resolve_conflicts_ml(
        &self,
        conflicts: Vec<ConflictInfo>,
    ) -> Result<Vec<ResolvedConflict>, SwarmError> {
        if let Some(model) = &self.ml_model {
            // Use ML to predict the best resolution for each conflict
            let mut resolved = Vec::new();

            for conflict in conflicts {
                let prediction = model.predict_resolution(&conflict).await?;
                let resolution = self.apply_ml_prediction(conflict, prediction).await?;
                resolved.push(resolution);
            }

            Ok(resolved)
        } else {
            // Fall back to base resolver
            self.base_resolver.resolve_conflicts(conflicts).await
        }
    }

    /// Apply ML prediction to resolve a conflict
    async fn apply_ml_prediction(
        &self,
        conflict: ConflictInfo,
        prediction: MLResolutionPrediction,
    ) -> Result<ResolvedConflict, SwarmError> {
        match prediction.recommended_action {
            MLAction::SelectAgent(agent_id) => {
                let finding = conflict
                    .conflicting_findings
                    .iter()
                    .zip(&conflict.agent_ids)
                    .find(|(_, id)| **id == agent_id)
                    .map(|(f, _)| f.clone());

                Ok(ResolvedConflict {
                    original_conflict: conflict,
                    resolved_finding: finding,
                    resolution_method: "ml_agent_selection".to_string(),
                    selected_agent: Some(agent_id),
                    confidence_score: Some(prediction.confidence),
                    requires_manual_review: false,
                })
            }
            MLAction::ManualReview => Ok(ResolvedConflict {
                original_conflict: conflict,
                resolved_finding: None,
                resolution_method: "ml_manual_review".to_string(),
                selected_agent: None,
                confidence_score: Some(prediction.confidence),
                requires_manual_review: true,
            }),
            MLAction::MergeFindings => {
                // Merge the findings (simplified - would need more sophisticated logic)
                let merged_finding = conflict.conflicting_findings[0].clone();
                Ok(ResolvedConflict {
                    original_conflict: conflict,
                    resolved_finding: Some(merged_finding),
                    resolution_method: "ml_merge".to_string(),
                    selected_agent: None,
                    confidence_score: Some(prediction.confidence),
                    requires_manual_review: false,
                })
            }
        }
    }
}

/// ML-based conflict predictor trait
#[async_trait::async_trait]
pub trait MLConflictPredictor: Send + Sync {
    /// Predict the best resolution for a conflict
    async fn predict_resolution(
        &self,
        conflict: &ConflictInfo,
    ) -> Result<MLResolutionPrediction, SwarmError>;
}

/// ML prediction result
#[derive(Debug, Clone)]
pub struct MLResolutionPrediction {
    pub recommended_action: MLAction,
    pub confidence: f64,
    pub reasoning: String,
}

/// ML recommended actions
#[derive(Debug, Clone)]
pub enum MLAction {
    SelectAgent(String), // Select finding from specific agent
    ManualReview,        // Requires manual review
    MergeFindings,       // Merge multiple findings
}
