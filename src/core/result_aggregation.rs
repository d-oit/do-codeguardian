//! Result aggregation framework for combining and synthesizing results from multiple agents

use crate::core::swarm_types::{
    ConflictInfo, ConflictResolutionStrategy, ExecutionSummary, Priority, SwarmError,
    SwarmPerformanceMetrics, SwarmResults, TaskResult,
};
use crate::types::AnalysisResults;
use crate::types::Finding;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Result aggregator for combining multiple agent results
pub struct ResultAggregator {
    config: AggregationConfig,
    conflict_resolver: Arc<dyn ConflictResolver>,
}

impl ResultAggregator {
    pub fn new(config: AggregationConfig, conflict_resolver: Arc<dyn ConflictResolver>) -> Self {
        Self {
            config,
            conflict_resolver,
        }
    }

    /// Aggregate results from multiple tasks into a unified result set
    pub async fn aggregate_results(
        &self,
        task_results: Vec<TaskResult>,
        performance_metrics: SwarmPerformanceMetrics,
    ) -> Result<SwarmResults, SwarmError> {
        let mut all_findings = Vec::new();
        let mut conflicts = Vec::new();
        let mut finding_groups = HashMap::new();

        // Group findings by their unique identifier
        for result in &task_results {
            for finding in &result.findings {
                let key = self.generate_finding_key(finding);
                finding_groups
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push((finding.clone(), result.agent_id.clone()));
            }
        }

        // Process each group of findings
        for (key, findings_with_agents) in finding_groups {
            if findings_with_agents.len() == 1 {
                // No conflict - use the single finding
                all_findings.push(findings_with_agents[0].0.clone());
            } else {
                // Conflict detected - resolve it
                let conflict = self.create_conflict_info(&key, findings_with_agents);
                let resolved_finding = self
                    .conflict_resolver
                    .resolve_conflict(conflict.clone())
                    .await?;

                if let Some(finding) = resolved_finding {
                    all_findings.push(finding);
                }
                conflicts.push(conflict);
            }
        }

        // Sort findings by priority and severity
        all_findings.sort_by(|a, b| {
            let a_priority = self.get_finding_priority(a);
            let b_priority = self.get_finding_priority(b);
            b_priority.cmp(&a_priority) // Higher priority first
        });

        // Deduplicate findings based on configuration
        if self.config.deduplicate_findings {
            all_findings = self.deduplicate_findings(all_findings);
        }

        // Create execution summary
        let execution_summary = self.create_execution_summary(&task_results);

        Ok(SwarmResults {
            task_results,
            aggregated_findings: all_findings,
            conflicts,
            performance_metrics,
            execution_summary,
        })
    }

    /// Generate a unique key for grouping similar findings
    fn generate_finding_key(&self, finding: &Finding) -> String {
        // Create a key based on file, line, and rule type
        // This is a simplified implementation - in practice, you'd want more sophisticated
        // similarity detection based on the finding content
        format!(
            "{}:{}:{}:{}",
            finding.file.display(),
            finding.line,
            finding.rule,
            finding.category.as_deref().unwrap_or("none")
        )
    }

    /// Create conflict information for a group of conflicting findings
    fn create_conflict_info(
        &self,
        key: &str,
        findings_with_agents: Vec<(Finding, String)>,
    ) -> ConflictInfo {
        let findings: Vec<Finding> = findings_with_agents
            .iter()
            .map(|(f, _)| f.clone())
            .collect();
        let agent_ids: Vec<String> = findings_with_agents
            .iter()
            .map(|(_, a)| a.clone())
            .collect();

        ConflictInfo {
            finding_id: key.to_string(),
            conflicting_findings: findings,
            agent_ids,
            resolution_strategy: self.config.conflict_resolution_strategy,
        }
    }

    /// Get priority level for a finding
    fn get_finding_priority(&self, finding: &Finding) -> Priority {
        // Map finding severity to priority
        match finding.severity.as_str() {
            "critical" | "high" => Priority::Critical,
            "medium" => Priority::High,
            "low" => Priority::Medium,
            "info" => Priority::Low,
            _ => Priority::Medium,
        }
    }

    /// Remove duplicate findings based on similarity
    fn deduplicate_findings(&self, findings: Vec<Finding>) -> Vec<Finding> {
        let mut unique_findings = Vec::new();
        let mut seen_keys = HashSet::new();

        for finding in findings {
            let key = format!(
                "{}:{}:{}",
                finding.file.display(),
                finding.line,
                finding.rule
            );

            if !seen_keys.contains(&key) {
                seen_keys.insert(key);
                unique_findings.push(finding);
            }
        }

        unique_findings
    }

    /// Create execution summary from task results
    fn create_execution_summary(&self, task_results: &[TaskResult]) -> ExecutionSummary {
        let total_tasks = task_results.len();
        let completed_tasks = task_results
            .iter()
            .filter(|r| matches!(r.status, crate::core::swarm_types::TaskStatus::Completed))
            .count();
        let failed_tasks = task_results
            .iter()
            .filter(|r| matches!(r.status, crate::core::swarm_types::TaskStatus::Failed))
            .count();
        let cancelled_tasks = task_results
            .iter()
            .filter(|r| matches!(r.status, crate::core::swarm_types::TaskStatus::Cancelled))
            .count();

        let total_findings: usize = task_results.iter().map(|r| r.findings.len()).sum();

        ExecutionSummary {
            total_tasks,
            completed_tasks,
            failed_tasks,
            cancelled_tasks,
            total_findings,
            unique_findings: 0,    // This would be calculated after deduplication
            conflicts_resolved: 0, // This would be calculated after conflict resolution
        }
    }

    /// Merge multiple AnalysisResults into a single result
    pub fn merge_analysis_results(
        &self,
        results: Vec<AnalysisResults>,
    ) -> Result<AnalysisResults, SwarmError> {
        if results.is_empty() {
            return Err(SwarmError::InternalError(
                "Cannot merge empty results".to_string(),
            ));
        }

        let mut merged = results[0].clone();

        // Merge findings from all results
        for result in results.iter().skip(1) {
            merged.findings.extend(result.findings.clone());
        }

        // Update summary statistics
        merged.summary.total_findings = merged.findings.len();
        merged.summary.total_files_scanned = results
            .iter()
            .map(|r| r.summary.total_files_scanned)
            .max()
            .unwrap_or(0);

        // Recalculate severity counts
        let mut severity_counts = HashMap::new();
        for finding in &merged.findings {
            *severity_counts.entry(finding.severity.clone()).or_insert(0) += 1;
        }

        merged.summary.findings_by_severity = severity_counts;

        Ok(merged)
    }

    /// Apply filtering and prioritization to findings
    pub fn filter_and_prioritize_findings(
        &self,
        findings: Vec<Finding>,
        filters: &FindingFilters,
    ) -> Vec<Finding> {
        findings
            .into_iter()
            .filter(|finding| self.matches_filters(finding, filters))
            .collect()
    }

    /// Check if a finding matches the given filters
    fn matches_filters(&self, finding: &Finding, filters: &FindingFilters) -> bool {
        // Check severity filter
        if let Some(min_severity) = &filters.min_severity {
            if !self.severity_matches(finding.severity.as_str(), min_severity) {
                return false;
            }
        }

        // Check category filter
        if let Some(categories) = &filters.categories {
            if let Some(cat) = &finding.category {
                if !categories.contains(cat) {
                    return false;
                }
            } else {
                // If finding has no category but filter requires categories, exclude
                return false;
            }
        }

        // Check file path filter
        if let Some(file_patterns) = &filters.file_patterns {
            let mut matches = false;
            for pattern in file_patterns {
                if finding.file.to_string_lossy().contains(pattern) {
                    matches = true;
                    break;
                }
            }
            if !matches {
                return false;
            }
        }

        // Check rule filter
        if let Some(rules) = &filters.rules {
            if !rules.contains(&finding.rule) {
                return false;
            }
        }

        true
    }

    /// Check if finding severity meets minimum requirement
    fn severity_matches(&self, finding_severity: &str, min_severity: &str) -> bool {
        let severity_levels = ["info", "low", "medium", "high", "critical"];
        let finding_level = severity_levels
            .iter()
            .position(|&s| s == finding_severity)
            .unwrap_or(0);
        let min_level = severity_levels
            .iter()
            .position(|&s| s == min_severity)
            .unwrap_or(0);

        finding_level >= min_level
    }

    /// Generate summary statistics for aggregated results
    pub fn generate_summary_stats(&self, results: &SwarmResults) -> AggregationStats {
        let mut severity_counts = HashMap::new();
        let mut category_counts = HashMap::new();
        let mut agent_contributions = HashMap::new();

        for finding in &results.aggregated_findings {
            *severity_counts
                .entry(finding.severity.as_str().to_string())
                .or_insert(0) += 1;
            *category_counts
                .entry(
                    finding
                        .category
                        .clone()
                        .unwrap_or_else(|| "none".to_string()),
                )
                .or_insert(0) += 1;
        }

        for result in &results.task_results {
            *agent_contributions
                .entry(result.agent_id.clone())
                .or_insert(0) += result.findings.len();
        }

        AggregationStats {
            total_findings: results.aggregated_findings.len(),
            unique_findings: results.execution_summary.unique_findings,
            conflicts_found: results.conflicts.len(),
            conflicts_resolved: results.execution_summary.conflicts_resolved,
            severity_distribution: severity_counts,
            category_distribution: category_counts,
            agent_contributions,
            aggregation_time: results.performance_metrics.total_execution_time,
        }
    }
}

/// Configuration for result aggregation
#[derive(Debug, Clone)]
pub struct AggregationConfig {
    pub deduplicate_findings: bool,
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
    pub max_conflicts_to_resolve: usize,
    pub aggregation_timeout: std::time::Duration,
}

/// Filters for finding selection
#[derive(Debug, Clone)]
pub struct FindingFilters {
    pub min_severity: Option<String>,
    pub categories: Option<Vec<String>>,
    pub file_patterns: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
    pub max_results: Option<usize>,
}

/// Statistics from result aggregation
#[derive(Debug, Clone)]
pub struct AggregationStats {
    pub total_findings: usize,
    pub unique_findings: usize,
    pub conflicts_found: usize,
    pub conflicts_resolved: usize,
    pub severity_distribution: HashMap<String, usize>,
    pub category_distribution: HashMap<String, usize>,
    pub agent_contributions: HashMap<String, usize>,
    pub aggregation_time: std::time::Duration,
}

/// Trait for conflict resolution strategies
#[async_trait::async_trait]
pub trait ConflictResolver: Send + Sync {
    /// Resolve a conflict between multiple findings for the same issue
    async fn resolve_conflict(&self, conflict: ConflictInfo)
        -> Result<Option<Finding>, SwarmError>;
}

/// Priority-based conflict resolver
pub struct PriorityBasedResolver {
    agent_priorities: HashMap<String, Priority>,
}

impl PriorityBasedResolver {
    pub fn new(agent_priorities: HashMap<String, Priority>) -> Self {
        Self { agent_priorities }
    }
}

#[async_trait::async_trait]
impl ConflictResolver for PriorityBasedResolver {
    async fn resolve_conflict(
        &self,
        conflict: ConflictInfo,
    ) -> Result<Option<Finding>, SwarmError> {
        let mut best_finding = None;
        let mut best_priority = Priority::Low;

        for (finding, agent_id) in conflict
            .conflicting_findings
            .into_iter()
            .zip(conflict.agent_ids)
        {
            let priority = self
                .agent_priorities
                .get(&agent_id)
                .copied()
                .unwrap_or(Priority::Medium);

            if priority > best_priority {
                best_priority = priority;
                best_finding = Some(finding);
            }
        }

        Ok(best_finding)
    }
}

/// Consensus-based conflict resolver
pub struct ConsensusResolver;

#[async_trait::async_trait]
impl ConflictResolver for ConsensusResolver {
    async fn resolve_conflict(
        &self,
        conflict: ConflictInfo,
    ) -> Result<Option<Finding>, SwarmError> {
        if conflict.conflicting_findings.is_empty() {
            return Ok(None);
        }

        // For consensus, we'll use the most common finding (simplified)
        // In practice, this would involve more sophisticated similarity analysis
        Ok(Some(conflict.conflicting_findings[0].clone()))
    }
}
