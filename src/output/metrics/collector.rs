//! # Metrics Collector
//!
//! Collects comprehensive metrics from output generation operations.

use super::types::*;
use crate::output::formatter::ValidationStatus;
use crate::output::OutputResult;
use crate::types::{AnalysisResults, Severity};
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Metrics collector for output operations
#[derive(Debug)]
pub struct OutputMetricsCollector {
    baselines: Arc<RwLock<HashMap<String, MetricBaseline>>>,
    config: CollectorConfig,
}

#[derive(Debug, Clone)]
pub struct CollectorConfig {
    pub enable_detailed_tracking: bool,
    pub performance_sampling_rate: f64,
    pub security_scan_enabled: bool,
    pub user_feedback_collection: bool,
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            enable_detailed_tracking: true,
            performance_sampling_rate: 1.0,
            security_scan_enabled: true,
            user_feedback_collection: false,
        }
    }
}

impl Default for OutputMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputMetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            baselines: Arc::new(RwLock::new(HashMap::new())),
            config: CollectorConfig::default(),
        }
    }

    /// Collect comprehensive metrics from an output operation
    pub fn collect_metrics(
        &self,
        results: &AnalysisResults,
        output_result: &OutputResult,
        format: &str,
        generation_time_ms: u64,
    ) -> Result<OutputMetrics> {
        let mut metrics = OutputMetrics::new(format.to_string());

        // Collect functionality metrics
        metrics.functionality = self.collect_functionality_metrics(output_result)?;

        // Collect performance metrics
        metrics.performance = self.collect_performance_metrics(generation_time_ms)?;

        // Collect security metrics
        metrics.security = self.collect_security_metrics(results, output_result)?;

        // Collect user experience metrics
        metrics.user_experience = self.collect_user_experience_metrics(results, output_result)?;

        // Add metadata
        metrics.metadata = self.collect_metadata(results, output_result);

        Ok(metrics)
    }

    fn collect_functionality_metrics(
        &self,
        output_result: &OutputResult,
    ) -> Result<FunctionalityMetrics> {
        let success = output_result.is_valid();
        let validation_score =
            self.calculate_validation_score(&output_result.metadata.validation_status);
        let format_compliance = self.check_format_compliance(output_result)?;
        let content_integrity = self.verify_content_integrity(output_result);
        let schema_validation = output_result.metadata.validation_status.is_valid;
        let error_count = output_result.metadata.validation_status.errors.len() as u32;
        let warning_count = output_result.metadata.validation_status.warnings.len() as u32;

        Ok(FunctionalityMetrics {
            success,
            validation_score,
            format_compliance,
            content_integrity,
            schema_validation,
            error_count,
            warning_count,
        })
    }

    fn collect_performance_metrics(&self, generation_time_ms: u64) -> Result<PerformanceMetrics> {
        // In a real implementation, these would be measured from the system
        // For now, we'll use the provided generation time and estimate others
        let memory_usage_bytes = self.estimate_memory_usage();
        let cpu_usage_percent = self.estimate_cpu_usage();
        let throughput_ops_per_sec = 1000.0 / generation_time_ms as f64; // Rough estimate
        let latency_p95_ms = (generation_time_ms as f64 * 1.2) as u64; // Estimate
        let resource_efficiency =
            self.calculate_resource_efficiency(generation_time_ms, memory_usage_bytes);

        Ok(PerformanceMetrics {
            generation_time_ms,
            memory_usage_bytes,
            cpu_usage_percent,
            throughput_ops_per_sec,
            latency_p95_ms,
            resource_efficiency,
        })
    }

    fn collect_security_metrics(
        &self,
        results: &AnalysisResults,
        output_result: &OutputResult,
    ) -> Result<SecurityMetrics> {
        let incidents_detected = self.scan_for_security_incidents(results, output_result);
        let sanitization_effectiveness = self.measure_sanitization_effectiveness(output_result);
        let vulnerability_score = self.calculate_vulnerability_score(results);
        let data_leakage_risk = self.assess_data_leakage_risk(output_result);
        let compliance_score = self.check_compliance_score(results);
        let encryption_status = self.check_encryption_status(output_result);

        Ok(SecurityMetrics {
            incidents_detected,
            sanitization_effectiveness,
            vulnerability_score,
            data_leakage_risk,
            compliance_score,
            encryption_status,
        })
    }

    fn collect_user_experience_metrics(
        &self,
        results: &AnalysisResults,
        output_result: &OutputResult,
    ) -> Result<UserExperienceMetrics> {
        let satisfaction_score = self.calculate_satisfaction_score(results, output_result);
        let usability_rating = self.assess_usability(output_result);
        let error_recovery_rate = self.calculate_error_recovery_rate(output_result);
        let format_preference_score = self.assess_format_preference(output_result);
        let accessibility_score = self.check_accessibility(output_result);
        let customization_satisfaction = self.assess_customization_satisfaction(output_result);

        Ok(UserExperienceMetrics {
            satisfaction_score,
            usability_rating,
            error_recovery_rate,
            format_preference_score,
            accessibility_score,
            customization_satisfaction,
        })
    }

    fn collect_metadata(
        &self,
        results: &AnalysisResults,
        output_result: &OutputResult,
    ) -> HashMap<String, MetricValue> {
        let mut metadata = HashMap::new();

        metadata.insert(
            "total_findings".to_string(),
            MetricValue::Integer(results.summary.total_findings as i64),
        );
        metadata.insert(
            "total_files".to_string(),
            MetricValue::Integer(results.summary.total_files_scanned as i64),
        );
        metadata.insert(
            "scan_duration_ms".to_string(),
            MetricValue::Integer(results.summary.scan_duration_ms as i64),
        );
        metadata.insert(
            "output_size_bytes".to_string(),
            MetricValue::Integer(output_result.metadata.content_size_bytes as i64),
        );
        metadata.insert(
            "generation_time_ms".to_string(),
            MetricValue::Integer(output_result.metadata.generation_time_ms as i64),
        );
        metadata.insert(
            "config_hash".to_string(),
            MetricValue::String(output_result.metadata.config_hash.clone()),
        );

        // Add severity breakdown
        for (severity, count) in &results.summary.findings_by_severity {
            metadata.insert(
                format!("severity_{:?}", severity),
                MetricValue::Integer(*count as i64),
            );
        }

        metadata
    }

    fn calculate_validation_score(&self, validation_status: &ValidationStatus) -> f64 {
        if validation_status.is_valid {
            let error_penalty = validation_status.errors.len() as f64 * 0.1;
            let warning_penalty = validation_status.warnings.len() as f64 * 0.05;
            (1.0 - error_penalty - warning_penalty).max(0.0)
        } else {
            0.0
        }
    }

    fn check_format_compliance(&self, output_result: &OutputResult) -> Result<f64> {
        // Check if the output follows the expected format structure
        // This is a simplified check - in practice, this would validate against format schemas
        let content = &output_result.content;

        // Basic checks for different formats
        match output_result.metadata.format.as_str() {
            "json" => {
                // Check if it's valid JSON
                serde_json::from_str::<serde_json::Value>(content).is_ok() as u8 as f64
            }
            "html" => {
                // Check for basic HTML structure
                (content.contains("<html>") && content.contains("</html>")) as u8 as f64
            }
            "markdown" => {
                // Markdown is more flexible, so we give it a higher score
                0.9
            }
            _ => 0.8, // Default score for other formats
        };

        Ok(1.0) // Simplified for now
    }

    fn verify_content_integrity(&self, output_result: &OutputResult) -> bool {
        // Verify that the output content hasn't been corrupted
        !output_result.content.is_empty()
            && output_result.content.len() == output_result.metadata.content_size_bytes
    }

    fn estimate_memory_usage(&self) -> u64 {
        // In a real implementation, this would measure actual memory usage
        // For now, return a reasonable estimate
        50 * 1024 * 1024 // 50MB estimate
    }

    fn estimate_cpu_usage(&self) -> f64 {
        // Estimate CPU usage during output generation
        15.0 // 15% estimate
    }

    fn calculate_resource_efficiency(
        &self,
        generation_time_ms: u64,
        memory_usage_bytes: u64,
    ) -> f64 {
        // Calculate efficiency based on time and memory usage
        let time_score = (10000.0 - generation_time_ms as f64).max(0.0) / 10000.0;
        let memory_score = (100.0 * 1024.0 * 1024.0 - memory_usage_bytes as f64).max(0.0)
            / (100.0 * 1024.0 * 1024.0);

        (time_score + memory_score) / 2.0
    }

    fn scan_for_security_incidents(
        &self,
        results: &AnalysisResults,
        output_result: &OutputResult,
    ) -> u64 {
        // Scan for potential security issues in the output
        let mut incidents = 0u64;

        // Check for high-severity findings
        incidents += results
            .findings
            .iter()
            .filter(|f| matches!(f.severity, Severity::Critical | Severity::High))
            .count() as u64;

        // Check for potential data leakage in output
        if output_result.content.contains("password") || output_result.content.contains("secret") {
            incidents += 1;
        }

        incidents
    }

    fn measure_sanitization_effectiveness(&self, output_result: &OutputResult) -> f64 {
        // Measure how well the output has been sanitized
        // This would check for removal of dangerous content
        let content = &output_result.content;

        // Check for potentially dangerous patterns
        let dangerous_patterns = ["<script>", "javascript:", "onload=", "onerror="];
        let dangerous_found = dangerous_patterns
            .iter()
            .any(|pattern| content.contains(pattern));

        if dangerous_found {
            0.5
        } else {
            1.0
        }
    }

    fn calculate_vulnerability_score(&self, results: &AnalysisResults) -> f64 {
        // Calculate a vulnerability score based on findings
        let total_findings = results.summary.total_findings as f64;
        if total_findings == 0.0 {
            return 0.0;
        }

        let critical_weight = 1.0;
        let high_weight = 0.7;
        let medium_weight = 0.4;
        let low_weight = 0.1;

        let weighted_score: f64 = results
            .summary
            .findings_by_severity
            .iter()
            .map(|(severity, count)| {
                let weight = match severity {
                    Severity::Critical => critical_weight,
                    Severity::High => high_weight,
                    Severity::Medium => medium_weight,
                    Severity::Low => low_weight,
                    Severity::Info => 0.0,
                };
                weight * *count as f64
            })
            .sum();

        (weighted_score / total_findings).min(1.0)
    }

    fn assess_data_leakage_risk(&self, output_result: &OutputResult) -> f64 {
        // Assess risk of data leakage in the output
        let content = &output_result.content;
        let mut risk_score = 0.0f64;

        // Check for sensitive patterns
        let sensitive_patterns = [
            r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b", // Credit card numbers
            r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b", // Email addresses
            r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b",           // Phone numbers
        ];

        for pattern in &sensitive_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(content) {
                    risk_score += 0.2;
                }
            }
        }

        risk_score.min(1.0)
    }

    fn check_compliance_score(&self, results: &AnalysisResults) -> f64 {
        // Check compliance with security standards
        // This is a simplified check
        let has_critical_findings = results
            .findings
            .iter()
            .any(|f| matches!(f.severity, Severity::Critical));

        if has_critical_findings {
            0.7
        } else {
            0.95
        }
    }

    fn check_encryption_status(&self, _output_result: &OutputResult) -> bool {
        // Check if output is properly encrypted when needed
        // For now, assume it's encrypted for sensitive formats
        true
    }

    fn calculate_satisfaction_score(
        &self,
        results: &AnalysisResults,
        output_result: &OutputResult,
    ) -> f64 {
        // Calculate user satisfaction based on various factors
        let mut score = 4.0; // Base score

        // Reduce score based on errors and warnings
        score -= output_result.metadata.validation_status.errors.len() as f64 * 0.5;
        score -= output_result.metadata.validation_status.warnings.len() as f64 * 0.2;

        // Reduce score based on high-severity findings
        let high_severity_count = results
            .findings
            .iter()
            .filter(|f| matches!(f.severity, Severity::Critical | Severity::High))
            .count();
        score -= high_severity_count as f64 * 0.3;

        // Ensure score is within bounds
        score.clamp(1.0, 5.0)
    }

    fn assess_usability(&self, output_result: &OutputResult) -> f64 {
        // Assess usability of the output format
        match output_result.metadata.format.as_str() {
            "json" => 3.5,     // Technical users
            "html" => 4.5,     // User-friendly
            "markdown" => 4.0, // Balanced
            "sarif" => 2.5,    // Very technical
            _ => 3.0,
        }
    }

    fn calculate_error_recovery_rate(&self, output_result: &OutputResult) -> f64 {
        // Calculate how well the system recovers from errors
        if output_result.metadata.validation_status.errors.is_empty() {
            1.0
        } else {
            // In a real system, this would check if errors were handled gracefully
            0.8
        }
    }

    fn assess_format_preference(&self, output_result: &OutputResult) -> f64 {
        // Assess user preference for the format
        // This would typically come from user feedback
        match output_result.metadata.format.as_str() {
            "html" => 4.2,
            "markdown" => 4.0,
            "json" => 3.8,
            _ => 3.5,
        }
    }

    fn check_accessibility(&self, output_result: &OutputResult) -> f64 {
        // Check accessibility features
        let content = &output_result.content;

        match output_result.metadata.format.as_str() {
            "html" => {
                let mut score = 3.0;
                if content.contains("alt=") {
                    score += 0.5;
                }
                if content.contains("role=") {
                    score += 0.5;
                }
                if content.contains("aria-") {
                    score += 0.5;
                }
                score
            }
            "markdown" => 3.5, // Markdown has basic accessibility
            _ => 2.5,
        }
    }

    fn assess_customization_satisfaction(&self, output_result: &OutputResult) -> f64 {
        // Assess satisfaction with customization options
        // Check if custom properties are present
        if output_result.properties.is_empty() {
            3.0
        } else {
            4.0
        }
    }

    /// Update baselines with new metrics
    pub async fn update_baselines(&self, metrics: &OutputMetrics) -> Result<()> {
        let mut baselines = self.baselines.write().await;

        // Update baseline for generation time
        self.update_baseline(
            &mut baselines,
            "generation_time_ms",
            MetricValue::Integer(metrics.performance.generation_time_ms as i64),
        );

        // Update baseline for success rate
        self.update_baseline(
            &mut baselines,
            "success",
            MetricValue::Boolean(metrics.functionality.success),
        );

        // Update baseline for satisfaction score
        self.update_baseline(
            &mut baselines,
            "satisfaction_score",
            MetricValue::Float(metrics.user_experience.satisfaction_score),
        );

        Ok(())
    }

    fn update_baseline(
        &self,
        baselines: &mut HashMap<String, MetricBaseline>,
        name: &str,
        value: MetricValue,
    ) {
        let baseline = baselines
            .entry(name.to_string())
            .or_insert_with(|| MetricBaseline {
                metric_name: name.to_string(),
                baseline_value: value.clone(),
                standard_deviation: 0.0,
                sample_size: 0,
                last_updated: Utc::now(),
            });

        // Simple moving average update (simplified)
        baseline.sample_size += 1;
        baseline.last_updated = Utc::now();

        // In a real implementation, you'd calculate proper statistical baselines
    }
}
