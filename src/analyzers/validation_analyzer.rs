use crate::analyzers::Analyzer;

use crate::core::{
    ConfidenceScorer, ManualReviewWorkflow, ReviewConfig, ValidationConfig, ValidationPipeline,
};
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Validation analyzer that applies multi-layer validation to findings
pub struct ValidationAnalyzer {
    validation_pipeline: Arc<Mutex<ValidationPipeline>>,
    confidence_scorer: ConfidenceScorer,
    manual_review_workflow: Arc<Mutex<ManualReviewWorkflow>>,
    config: ValidationConfig,
    enabled: bool,
}

impl ValidationAnalyzer {
    /// Create a new validation analyzer
    pub fn new() -> Self {
        let config = ValidationConfig::default();
        let validation_pipeline = ValidationPipeline::new(config.clone());
        let confidence_scorer = ConfidenceScorer::new();
        let review_config = ReviewConfig::default();
        let manual_review_workflow = ManualReviewWorkflow::new(review_config);

        Self {
            validation_pipeline: Arc::new(Mutex::new(validation_pipeline)),
            confidence_scorer,
            manual_review_workflow: Arc::new(Mutex::new(manual_review_workflow)),
            config,
            enabled: true,
        }
    }

    /// Create validation analyzer with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        let validation_pipeline = ValidationPipeline::new(config.clone());
        let confidence_scorer = ConfidenceScorer::new();
        let review_config = ReviewConfig::default();
        let manual_review_workflow = ManualReviewWorkflow::new(review_config);

        Self {
            validation_pipeline: Arc::new(Mutex::new(validation_pipeline)),
            confidence_scorer,
            manual_review_workflow: Arc::new(Mutex::new(manual_review_workflow)),
            config,
            enabled: true,
        }
    }

    /// Enable or disable validation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Validate a batch of findings
    pub async fn validate_findings(&self, findings: Vec<Finding>) -> Result<Vec<Finding>> {
        if !self.enabled || findings.is_empty() {
            return Ok(findings);
        }

        let mut pipeline = self.validation_pipeline.lock().await;
        let mut validation_results = Vec::new();
        for finding in findings {
            let context = crate::core::ValidationContext {
                file_path: finding.file.clone(),
                project_type: "unknown".to_string(),
                codebase_size: 0,
                validation_history: std::collections::HashMap::new(),
            };
            let result = pipeline.validate_finding(finding, context).await;
            validation_results.push(result);
        }
        drop(pipeline);

        let mut validated_findings = Vec::new();
        let mut review_candidates = Vec::new();

        for result in validation_results {
            match result.status {
                crate::core::ValidationStatus::Validated => {
                    // High confidence - include finding
                    let enhanced_finding = self.enhance_finding_with_validation(result);
                    validated_findings.push(enhanced_finding);
                }
                crate::core::ValidationStatus::Enhanced => {
                    // Enhanced finding - include with improvements
                    let enhanced_finding = self.enhance_finding_with_validation(result);
                    validated_findings.push(enhanced_finding);
                }
                crate::core::ValidationStatus::Dismissed => {
                    // Low confidence - likely false positive, exclude
                    tracing::debug!(
                        "Dismissed finding as likely false positive: {}",
                        result.finding.message
                    );
                }
                crate::core::ValidationStatus::RequiresReview => {
                    // Uncertain - send for manual review
                    review_candidates.push(result.finding);
                }
                crate::core::ValidationStatus::Failed => {
                    // Validation failed - include original finding with warning
                    let mut finding = result.finding;
                    finding.description = Some(format!(
                        "{}. Warning: Validation failed.",
                        finding.description.unwrap_or_default()
                    ));
                    validated_findings.push(finding);
                }
            }
        }

        // Submit uncertain findings for manual review
        if !review_candidates.is_empty() {
            let mut review_workflow = self.manual_review_workflow.lock().await;
            match review_workflow.submit_for_review(review_candidates).await {
                Ok(review_ids) => {
                    tracing::info!(
                        "Submitted {} findings for manual review: {:?}",
                        review_ids.len(),
                        review_ids
                    );
                }
                Err(e) => {
                    tracing::error!("Failed to submit findings for manual review: {}", e);
                }
            }
        }

        Ok(validated_findings)
    }

    /// Enhance finding with validation results
    fn enhance_finding_with_validation(
        &self,
        validation_result: crate::core::ValidationResult,
    ) -> Finding {
        let mut finding = validation_result.finding;

        // Add confidence score to description
        let confidence_info = format!(
            " (Confidence: {:.1}%)",
            validation_result.confidence * 100.0
        );
        finding.description = Some(format!(
            "{}{}",
            finding.description.unwrap_or_default(),
            confidence_info
        ));

        // Adjust severity based on confidence
        if validation_result.confidence < 0.5 {
            finding.severity = match finding.severity {
                Severity::Critical => Severity::High,
                Severity::High => Severity::Medium,
                Severity::Medium => Severity::Low,
                severity => severity,
            };
        }

        finding
    }

    /// Get validation metrics
    pub async fn get_validation_metrics(&self) -> crate::core::ValidationMetrics {
        let pipeline = self.validation_pipeline.lock().await;
        pipeline.get_metrics().clone()
    }

    /// Get review statistics
    pub async fn get_review_statistics(&self) -> crate::core::ReviewStatistics {
        let workflow = self.manual_review_workflow.lock().await;
        workflow.get_review_statistics()
    }

    /// Reset validation metrics
    pub async fn reset_metrics(&self) {
        let mut pipeline = self.validation_pipeline.lock().await;
        pipeline.reset_metrics();
    }

    /// Update confidence scorer with feedback
    pub fn update_confidence_baseline(&mut self, category: &str, accuracy: f64) {
        self.confidence_scorer
            .update_baseline(category.to_string(), accuracy);
    }

    /// Get confidence threshold recommendations
    pub fn get_threshold_recommendations(
        &self,
        findings: &[Finding],
    ) -> crate::core::ThresholdRecommendations {
        self.confidence_scorer
            .get_threshold_recommendations(findings)
    }
}

impl Default for ValidationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer for ValidationAnalyzer {
    fn name(&self) -> &str {
        "validation"
    }

    fn analyze(&self, _file_path: &Path, _content: &[u8]) -> Result<Vec<Finding>> {
        // Validation analyzer doesn't analyze files directly
        // It validates findings from other analyzers
        Ok(Vec::new())
    }

    fn supports_file(&self, _file_path: &Path) -> bool {
        // Validation analyzer supports all files indirectly
        false
    }
}

/// Enhanced analyzer registry with validation integration
pub struct ValidatedAnalyzerRegistry {
    base_registry: crate::analyzers::AnalyzerRegistry,
    validation_analyzer: ValidationAnalyzer,
    validation_enabled: bool,
}

impl ValidatedAnalyzerRegistry {
    /// Create a new validated analyzer registry
    pub fn new(base_registry: crate::analyzers::AnalyzerRegistry) -> Self {
        Self {
            base_registry,
            validation_analyzer: ValidationAnalyzer::new(),
            validation_enabled: true,
        }
    }

    /// Enable or disable validation
    pub fn set_validation_enabled(&mut self, enabled: bool) {
        self.validation_enabled = enabled;
        self.validation_analyzer.set_enabled(enabled);
    }

    /// Analyze file with validation
    pub async fn analyze_file_with_validation(
        &self,
        file_path: &Path,
        content: &[u8],
    ) -> Result<Vec<Finding>> {
        // Run base analysis
        let findings = self.base_registry.analyze_file(file_path, content)?;

        // Apply validation if enabled
        if self.validation_enabled && !findings.is_empty() {
            self.validation_analyzer.validate_findings(findings).await
        } else {
            Ok(findings)
        }
    }

    /// Get validation metrics
    pub async fn get_validation_metrics(&self) -> crate::core::ValidationMetrics {
        self.validation_analyzer.get_validation_metrics().await
    }

    /// Get review statistics
    pub async fn get_review_statistics(&self) -> crate::core::ReviewStatistics {
        self.validation_analyzer.get_review_statistics().await
    }

    /// Update confidence baseline with feedback
    pub fn update_confidence_baseline(&mut self, category: &str, accuracy: f64) {
        self.validation_analyzer
            .update_confidence_baseline(category, accuracy);
    }

    /// Get threshold recommendations
    pub fn get_threshold_recommendations(
        &self,
        findings: &[Finding],
    ) -> crate::core::ThresholdRecommendations {
        self.validation_analyzer
            .get_threshold_recommendations(findings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_validation_analyzer() {
        let analyzer = ValidationAnalyzer::new();

        let findings = vec![Finding::new(
            "security",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test security finding".to_string(),
        )];

        let validated = analyzer.validate_findings(findings).await.unwrap();

        // Should have some findings (either validated or enhanced)
        assert!(!validated.is_empty());
    }

    #[tokio::test]
    async fn test_validated_registry() {
        let base_registry = crate::analyzers::AnalyzerRegistry::new();
        let registry = ValidatedAnalyzerRegistry::new(base_registry);

        // Test that validation can be enabled/disabled
        assert!(registry.validation_enabled);
    }

    #[test]
    fn test_finding_enhancement() {
        let analyzer = ValidationAnalyzer::new();

        let validation_result = crate::core::ValidationResult {
            finding: Finding::new(
                "security",
                "test_rule",
                Severity::High,
                PathBuf::from("test.rs"),
                10,
                "Original message".to_string(),
            ),
            status: crate::core::ValidationStatus::Enhanced,
            confidence: 0.85,
            validation_time_ms: 100,
            layers_applied: 2,
        };

        let enhanced = analyzer.enhance_finding_with_validation(validation_result);

        assert!(enhanced.description.unwrap().contains("Confidence: 85"));
        assert!(enhanced.suggestion.is_some());
    }
}
