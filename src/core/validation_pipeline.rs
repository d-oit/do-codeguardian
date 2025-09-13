//! Multi-layer validation pipeline for security findings

use crate::types::Finding;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub confidence_threshold: f64,
    pub require_manual_review: bool,
    pub max_parallel_validations: usize,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            confidence_threshold: 0.7,
            require_manual_review: false,
            max_parallel_validations: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Validated,
    Enhanced,
    Dismissed,
    RequiresReview,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub finding: Finding,
    pub status: ValidationStatus,
    pub confidence: f64,
    pub validation_time_ms: u64,
    pub layers_applied: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub total_validations: usize,
    pub validated_count: usize,
    pub enhanced_count: usize,
    pub dismissed_count: usize,
    pub requires_review_count: usize,
    pub failed_count: usize,
    pub average_confidence: f64,
    pub average_validation_time_ms: f64,
}

impl Default for ValidationMetrics {
    fn default() -> Self {
        Self {
            total_validations: 0,
            validated_count: 0,
            enhanced_count: 0,
            dismissed_count: 0,
            requires_review_count: 0,
            failed_count: 0,
            average_confidence: 0.0,
            average_validation_time_ms: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValidationLayer {
    PatternMatching,
    ContextAnalysis,
    HistoricalValidation,
    CrossReferencing,
    ManualReview,
}

#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub file_path: PathBuf,
    pub project_type: String,
    pub codebase_size: usize,
    pub validation_history: HashMap<String, f64>, // rule -> accuracy
}

pub struct ValidationPipeline {
    config: ValidationConfig,
    layers: Vec<ValidationLayer>,
    metrics: ValidationMetrics,
}

impl ValidationPipeline {
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            config,
            layers: vec![
                ValidationLayer::PatternMatching,
                ValidationLayer::ContextAnalysis,
                ValidationLayer::HistoricalValidation,
                ValidationLayer::CrossReferencing,
            ],
            metrics: ValidationMetrics::default(),
        }
    }

    pub async fn validate_finding(
        &mut self,
        finding: Finding,
        context: ValidationContext,
    ) -> ValidationResult {
        let start_time = std::time::Instant::now();

        let mut confidence = 0.5; // Base confidence
        let mut layers_applied = 0;

        for layer in &self.layers {
            if self
                .apply_layer(layer.clone(), &finding, &context, &mut confidence)
                .await
            {
                layers_applied += 1;
            }
        }

        let status = self.determine_status(confidence);
        let validation_time = start_time.elapsed().as_millis() as u64;

        self.update_metrics(status.clone(), confidence, validation_time);

        ValidationResult {
            finding,
            status,
            confidence,
            validation_time_ms: validation_time,
            layers_applied,
        }
    }

    async fn apply_layer(
        &self,
        layer: ValidationLayer,
        finding: &Finding,
        context: &ValidationContext,
        confidence: &mut f64,
    ) -> bool {
        match layer {
            ValidationLayer::PatternMatching => {
                // Pattern matching validation
                *confidence += 0.1;
                true
            }
            ValidationLayer::ContextAnalysis => {
                // Context analysis validation
                *confidence += 0.05;
                true
            }
            ValidationLayer::HistoricalValidation => {
                // Historical validation
                if let Some(historical_accuracy) = context.validation_history.get(&finding.rule) {
                    *confidence += historical_accuracy * 0.2;
                }
                true
            }
            ValidationLayer::CrossReferencing => {
                // Cross-referencing validation
                *confidence += 0.08;
                true
            }
            ValidationLayer::ManualReview => {
                // Manual review would be handled separately
                false
            }
        }
    }

    fn determine_status(&self, confidence: f64) -> ValidationStatus {
        if confidence >= self.config.confidence_threshold {
            ValidationStatus::Validated
        } else if confidence >= self.config.confidence_threshold - 0.2 {
            ValidationStatus::Enhanced
        } else if confidence >= 0.3 {
            ValidationStatus::RequiresReview
        } else {
            ValidationStatus::Dismissed
        }
    }

    fn update_metrics(&mut self, status: ValidationStatus, confidence: f64, time_ms: u64) {
        self.metrics.total_validations += 1;
        self.metrics.average_confidence = (self.metrics.average_confidence
            * (self.metrics.total_validations - 1) as f64
            + confidence)
            / self.metrics.total_validations as f64;
        self.metrics.average_validation_time_ms = (self.metrics.average_validation_time_ms
            * (self.metrics.total_validations - 1) as f64
            + time_ms as f64)
            / self.metrics.total_validations as f64;

        match status {
            ValidationStatus::Validated => self.metrics.validated_count += 1,
            ValidationStatus::Enhanced => self.metrics.enhanced_count += 1,
            ValidationStatus::Dismissed => self.metrics.dismissed_count += 1,
            ValidationStatus::RequiresReview => self.metrics.requires_review_count += 1,
            ValidationStatus::Failed => self.metrics.failed_count += 1,
        }
    }

    pub fn get_metrics(&self) -> ValidationMetrics {
        self.metrics.clone()
    }
}
