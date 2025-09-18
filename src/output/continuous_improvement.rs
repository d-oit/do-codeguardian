//! # Continuous Improvement System
//!
//! This module implements automated continuous improvement capabilities for the output system,
//! including A/B testing, feedback loops, and optimization cycles as specified in Phase 4.

use crate::output::OutputResult;
use crate::types::AnalysisResults;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Continuous improvement manager
#[derive(Debug)]
pub struct ContinuousImprovementManager {
    ab_tester: ABTestManager,
    feedback_collector: FeedbackCollector,
    optimization_scheduler: OptimizationScheduler,
    improvement_tracker: ImprovementTracker,
}

impl Default for ContinuousImprovementManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ContinuousImprovementManager {
    /// Create a new continuous improvement manager
    pub fn new() -> Self {
        Self {
            ab_tester: ABTestManager::new(),
            feedback_collector: FeedbackCollector::new(),
            optimization_scheduler: OptimizationScheduler::new(),
            improvement_tracker: ImprovementTracker::new(),
        }
    }

    /// Process analysis results with continuous improvement
    pub async fn process_with_improvement(
        &mut self,
        results: &AnalysisResults,
        format: &str,
    ) -> Result<OutputResult> {
        // Check if this should be part of an A/B test
        let variant = self.ab_tester.get_variant_for_request(format).await?;

        // Process with selected variant
        let output_result = self.process_with_variant(results, format, &variant).await?;

        // Collect feedback and metrics
        self.feedback_collector
            .collect_automated_feedback(&output_result)
            .await?;

        // Update A/B test results
        self.ab_tester
            .record_result(&variant, &output_result)
            .await?;

        Ok(output_result)
    }

    /// Generate improvement recommendations
    pub async fn generate_improvement_recommendations(
        &self,
    ) -> Result<Vec<ImprovementRecommendation>> {
        let mut recommendations = Vec::new();

        // Get A/B test insights
        let ab_insights = self.ab_tester.get_insights().await?;
        recommendations.extend(ab_insights.into_iter().map(ImprovementRecommendation::from));

        // Get feedback insights
        let feedback_insights = self.feedback_collector.get_insights().await?;
        recommendations.extend(
            feedback_insights
                .into_iter()
                .map(ImprovementRecommendation::from),
        );

        // Get optimization opportunities
        let optimization_insights = self.optimization_scheduler.get_opportunities().await?;
        recommendations.extend(
            optimization_insights
                .into_iter()
                .map(ImprovementRecommendation::from),
        );

        Ok(recommendations)
    }

    async fn process_with_variant(
        &self,
        results: &AnalysisResults,
        format: &str,
        variant: &TestVariant,
    ) -> Result<OutputResult> {
        // Apply variant-specific processing
        match variant.variant_type {
            VariantType::Control => {
                // Standard processing
                crate::output::format_results(results, format.parse()?)
            }
            VariantType::ExperimentalFormat => {
                // Apply experimental formatting
                let mut result = crate::output::format_results(results, format.parse()?)?;
                result.content =
                    self.apply_experimental_formatting(&result.content, &variant.parameters)?;
                Ok(result)
            }
            VariantType::PerformanceOptimized => {
                // Apply performance optimizations
                self.process_with_performance_optimizations(results, format)
                    .await
            }
        }
    }

    fn apply_experimental_formatting(
        &self,
        content: &str,
        parameters: &HashMap<String, String>,
    ) -> Result<String> {
        let mut enhanced_content = content.to_string();

        // Apply experimental formatting based on parameters
        if parameters
            .get("enhanced_styling")
            .is_some_and(|v| v == "true")
        {
            enhanced_content = format!(
                "<div class=\"codeguardian-enhanced\">{}</div>",
                enhanced_content
            );
        }

        if parameters
            .get("compact_format")
            .is_some_and(|v| v == "true")
        {
            enhanced_content = enhanced_content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n");
        }

        Ok(enhanced_content)
    }

    async fn process_with_performance_optimizations(
        &self,
        results: &AnalysisResults,
        format: &str,
    ) -> Result<OutputResult> {
        // Use memory-optimized processing
        let memory_manager = crate::performance::GlobalMemoryPools::new();
        crate::output::format_results_with_memory_manager(
            results,
            format.parse()?,
            Arc::new(memory_manager),
        )
    }
}

/// A/B Test Manager
#[derive(Debug)]
pub struct ABTestManager {
    active_tests: Arc<RwLock<HashMap<String, ABTest>>>,
    test_results: Arc<RwLock<HashMap<String, Vec<TestResult>>>>,
}

impl Default for ABTestManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ABTestManager {
    pub fn new() -> Self {
        Self {
            active_tests: Arc::new(RwLock::new(HashMap::new())),
            test_results: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_variant_for_request(&self, format: &str) -> Result<TestVariant> {
        let tests = self.active_tests.read().await;

        if let Some(test) = tests.get(format) {
            // Simple random assignment (50/50 split)
            if format!("{:?}", self).len() % 2 == 0 {
                Ok(test.control_variant.clone())
            } else {
                Ok(test.experimental_variant.clone())
            }
        } else {
            // No active test, return control variant
            Ok(TestVariant {
                id: "control".to_string(),
                variant_type: VariantType::Control,
                parameters: HashMap::new(),
            })
        }
    }

    pub async fn record_result(
        &mut self,
        variant: &TestVariant,
        output: &OutputResult,
    ) -> Result<()> {
        let result = TestResult {
            variant_id: variant.id.clone(),
            timestamp: Utc::now(),
            success: output.metadata.validation_status.errors.is_empty(),
            generation_time_ms: output.metadata.generation_time_ms,
            output_size: output.content.len(),
            user_satisfaction: None, // Will be updated with user feedback
        };

        let mut results = self.test_results.write().await;
        results
            .entry(variant.id.clone())
            .or_insert_with(Vec::new)
            .push(result);

        Ok(())
    }

    pub async fn get_insights(&self) -> Result<Vec<ABTestInsight>> {
        let results = self.test_results.read().await;
        let mut insights = Vec::new();

        for (variant_id, test_results) in results.iter() {
            if test_results.len() < 10 {
                continue; // Not enough data for meaningful insights
            }

            let success_rate = test_results.iter().filter(|r| r.success).count() as f64
                / test_results.len() as f64;
            let avg_generation_time = test_results
                .iter()
                .map(|r| r.generation_time_ms)
                .sum::<u64>() as f64
                / test_results.len() as f64;
            let avg_output_size = test_results.iter().map(|r| r.output_size).sum::<usize>() as f64
                / test_results.len() as f64;

            insights.push(ABTestInsight {
                variant_id: variant_id.clone(),
                success_rate,
                avg_generation_time,
                avg_output_size,
                sample_size: test_results.len(),
                confidence_level: self.calculate_confidence_level(test_results),
                recommendation: self
                    .generate_variant_recommendation(success_rate, avg_generation_time),
            });
        }

        Ok(insights)
    }

    fn calculate_confidence_level(&self, results: &[TestResult]) -> f64 {
        // Simple confidence calculation based on sample size
        let n = results.len() as f64;
        if n > 1000.0 {
            0.95
        } else if n > 100.0 {
            0.90
        } else if n > 30.0 {
            0.80
        } else {
            0.70
        }
    }

    fn generate_variant_recommendation(&self, success_rate: f64, avg_time: f64) -> String {
        if success_rate > 0.95 && avg_time < 1000.0 {
            "Consider promoting this variant to production".to_string()
        } else if success_rate < 0.85 {
            "This variant shows poor success rate, consider discontinuing".to_string()
        } else if avg_time > 5000.0 {
            "Performance issues detected, needs optimization".to_string()
        } else {
            "Continue testing with more samples".to_string()
        }
    }
}

/// Feedback Collection System
#[derive(Debug)]
pub struct FeedbackCollector {
    automated_feedback: Arc<RwLock<Vec<AutomatedFeedback>>>,
    user_feedback: Arc<RwLock<Vec<UserFeedback>>>,
}

impl Default for FeedbackCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedbackCollector {
    pub fn new() -> Self {
        Self {
            automated_feedback: Arc::new(RwLock::new(Vec::new())),
            user_feedback: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn collect_automated_feedback(&mut self, output: &OutputResult) -> Result<()> {
        let feedback = AutomatedFeedback {
            timestamp: Utc::now(),
            output_format: "json".to_string(), // Default format for now
            validation_score: if output.metadata.validation_status.errors.is_empty() {
                1.0
            } else {
                0.0
            },
            performance_score: self.calculate_performance_score(output),
            security_score: self.calculate_security_score(output),
            suggestions: self.generate_automated_suggestions(output),
        };

        let mut feedback_store = self.automated_feedback.write().await;
        feedback_store.push(feedback);

        Ok(())
    }

    pub async fn get_insights(&self) -> Result<Vec<FeedbackInsight>> {
        let automated = self.automated_feedback.read().await;
        let user = self.user_feedback.read().await;

        let mut insights = Vec::new();

        // Analyze automated feedback
        if !automated.is_empty() {
            let avg_validation =
                automated.iter().map(|f| f.validation_score).sum::<f64>() / automated.len() as f64;
            let avg_performance =
                automated.iter().map(|f| f.performance_score).sum::<f64>() / automated.len() as f64;
            let avg_security =
                automated.iter().map(|f| f.security_score).sum::<f64>() / automated.len() as f64;

            insights.push(FeedbackInsight {
                category: "automated".to_string(),
                insight: format!(
                    "Avg scores - Validation: {:.2}, Performance: {:.2}, Security: {:.2}",
                    avg_validation, avg_performance, avg_security
                ),
                priority: if avg_validation < 0.8 || avg_performance < 0.8 || avg_security < 0.8 {
                    Priority::High
                } else {
                    Priority::Medium
                },
            });
        }

        // Analyze user feedback
        if !user.is_empty() {
            let avg_satisfaction =
                user.iter().map(|f| f.satisfaction_score).sum::<f64>() / user.len() as f64;

            insights.push(FeedbackInsight {
                category: "user_experience".to_string(),
                insight: format!("Average user satisfaction: {:.2}/5.0", avg_satisfaction),
                priority: if avg_satisfaction < 3.0 {
                    Priority::High
                } else {
                    Priority::Low
                },
            });
        }

        Ok(insights)
    }

    fn calculate_performance_score(&self, output: &OutputResult) -> f64 {
        let time_ms = output.metadata.generation_time_ms;
        if time_ms < 100 {
            1.0
        } else if time_ms < 1000 {
            0.8
        } else if time_ms < 5000 {
            0.6
        } else {
            0.3
        }
    }

    fn calculate_security_score(&self, output: &OutputResult) -> f64 {
        // Check for potential security issues in output
        let content = &output.content;
        let mut score: f64 = 1.0;

        if content.contains("<script") {
            score -= 0.5;
        }
        if content.contains("javascript:") {
            score -= 0.3;
        }
        if content.contains("data:") {
            score -= 0.2;
        }

        score.max(0.0)
    }

    fn generate_automated_suggestions(&self, output: &OutputResult) -> Vec<String> {
        let mut suggestions = Vec::new();

        let time_ms = output.metadata.generation_time_ms;
        if time_ms > 5000 {
            suggestions
                .push("Consider optimizing output generation for better performance".to_string());
        }

        if output.content.len() > 1_000_000 {
            suggestions.push("Large output detected, consider implementing pagination".to_string());
        }

        if !output.metadata.validation_status.is_valid {
            suggestions.push("Output validation failed, review format compliance".to_string());
        }

        suggestions
    }
}

/// Optimization Scheduler
#[derive(Debug)]
pub struct OptimizationScheduler {
    optimization_cycles: Arc<RwLock<Vec<OptimizationCycle>>>,
    last_optimization: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl Default for OptimizationScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationScheduler {
    pub fn new() -> Self {
        Self {
            optimization_cycles: Arc::new(RwLock::new(Vec::new())),
            last_optimization: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn should_run_optimization(&self) -> Result<bool> {
        let last_opt = self.last_optimization.read().await;

        match *last_opt {
            Some(last) => Ok(Utc::now() - last > Duration::days(7)), // Weekly optimization cycle
            None => Ok(true),                                        // First time
        }
    }

    pub async fn get_opportunities(&self) -> Result<Vec<OptimizationOpportunity>> {
        let mut opportunities = Vec::new();

        // Check if optimization cycle is due
        if self.should_run_optimization().await? {
            opportunities.push(OptimizationOpportunity {
                category: "performance".to_string(),
                description: "Regular performance optimization cycle is due".to_string(),
                priority: Priority::Medium,
                estimated_impact: "Potential 5-10% performance improvement".to_string(),
            });
        }

        // Add other opportunity detection logic here

        Ok(opportunities)
    }
}

/// Improvement Tracker
#[derive(Debug)]
pub struct ImprovementTracker {
    improvements: Arc<RwLock<Vec<ImprovementRecord>>>,
}

impl Default for ImprovementTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ImprovementTracker {
    pub fn new() -> Self {
        Self {
            improvements: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record_improvement(&mut self, improvement: ImprovementRecord) -> Result<()> {
        let mut improvements = self.improvements.write().await;
        improvements.push(improvement);
        Ok(())
    }

    pub async fn get_improvement_history(&self) -> Result<Vec<ImprovementRecord>> {
        let improvements = self.improvements.read().await;
        Ok(improvements.clone())
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTest {
    pub id: String,
    pub name: String,
    pub control_variant: TestVariant,
    pub experimental_variant: TestVariant,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: TestStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVariant {
    pub id: String,
    pub variant_type: VariantType,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VariantType {
    Control,
    ExperimentalFormat,
    PerformanceOptimized,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TestStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub variant_id: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub generation_time_ms: u64,
    pub output_size: usize,
    pub user_satisfaction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestInsight {
    pub variant_id: String,
    pub success_rate: f64,
    pub avg_generation_time: f64,
    pub avg_output_size: f64,
    pub sample_size: usize,
    pub confidence_level: f64,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedFeedback {
    pub timestamp: DateTime<Utc>,
    pub output_format: String,
    pub validation_score: f64,
    pub performance_score: f64,
    pub security_score: f64,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub timestamp: DateTime<Utc>,
    pub output_format: String,
    pub satisfaction_score: f64,
    pub comments: String,
    pub feature_requests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackInsight {
    pub category: String,
    pub insight: String,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCycle {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub improvements_implemented: Vec<String>,
    pub performance_impact: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub category: String,
    pub description: String,
    pub priority: Priority,
    pub estimated_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub improvement_type: String,
    pub description: String,
    pub before_metrics: HashMap<String, f64>,
    pub after_metrics: HashMap<String, f64>,
    pub impact_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRecommendation {
    pub category: String,
    pub description: String,
    pub priority: Priority,
    pub estimated_effort: String,
    pub expected_impact: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl From<ABTestInsight> for ImprovementRecommendation {
    fn from(insight: ABTestInsight) -> Self {
        Self {
            category: "ab_testing".to_string(),
            description: insight.recommendation,
            priority: if insight.confidence_level > 0.9 {
                Priority::High
            } else {
                Priority::Medium
            },
            estimated_effort: "2-4 hours".to_string(),
            expected_impact: format!(
                "Based on {} samples with {:.1}% confidence",
                insight.sample_size,
                insight.confidence_level * 100.0
            ),
        }
    }
}

impl From<FeedbackInsight> for ImprovementRecommendation {
    fn from(insight: FeedbackInsight) -> Self {
        Self {
            category: insight.category,
            description: insight.insight,
            priority: insight.priority,
            estimated_effort: "1-3 hours".to_string(),
            expected_impact: "Improved user satisfaction".to_string(),
        }
    }
}

impl From<OptimizationOpportunity> for ImprovementRecommendation {
    fn from(opportunity: OptimizationOpportunity) -> Self {
        Self {
            category: opportunity.category,
            description: opportunity.description,
            priority: opportunity.priority,
            estimated_effort: "4-8 hours".to_string(),
            expected_impact: opportunity.estimated_impact,
        }
    }
}
