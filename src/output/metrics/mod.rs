//! # Success Metrics Framework for Output Systems
//!
//! This module provides comprehensive metrics collection, monitoring, and reporting
//! capabilities for CodeGuardian's output systems. It tracks functionality, performance,
//! security, and user experience metrics to ensure high-quality output generation.

pub mod alerts;
pub mod collector;
pub mod monitoring;
pub mod reporter;
pub mod threshold_tuning;
pub mod trends;
pub mod types;

pub use alerts::{AlertManager, AlertRule, AlertSeverity};
pub use collector::OutputMetricsCollector;
pub use monitoring::{MonitoringConfig, RealTimeMonitor};
pub use reporter::AutomatedReporter;
pub use threshold_tuning::{
    EnvironmentProfile, PerformanceRequirements, SecurityRequirements, ThresholdTuningManager,
    TuningRecommendation,
};
pub use trends::{TrendAnalysis, TrendAnalyzer};
pub use types::{
    FunctionalityMetrics, MetricType, MetricValue, OutputMetrics, PerformanceMetrics,
    SecurityMetrics, UserExperienceMetrics,
};

use crate::output::OutputResult;
use crate::types::AnalysisResults;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main metrics service for output systems
#[derive(Debug)]
pub struct OutputMetricsService {
    collector: OutputMetricsCollector,
    monitor: RealTimeMonitor,
    reporter: AutomatedReporter,
    alert_manager: AlertManager,
    trend_analyzer: TrendAnalyzer,
}

impl OutputMetricsService {
    /// Create a new metrics service
    pub fn new() -> Self {
        Self {
            collector: OutputMetricsCollector::new(),
            monitor: RealTimeMonitor::new(),
            reporter: AutomatedReporter::new(),
            alert_manager: AlertManager::new(),
            trend_analyzer: TrendAnalyzer::new(),
        }
    }

    /// Record metrics for an output generation operation
    pub async fn record_output_metrics(
        &mut self,
        results: &AnalysisResults,
        output_result: &OutputResult,
        format: &str,
        generation_time_ms: u64,
    ) -> Result<()> {
        // Collect metrics
        let metrics =
            self.collector
                .collect_metrics(results, output_result, format, generation_time_ms)?;

        // Record in real-time monitor
        self.monitor.record_metrics(metrics.clone()).await?;

        // Check for alerts
        if let Some(alert) = self.alert_manager.check_alerts(&metrics).await? {
            self.reporter.report_alert(&alert).await?;
        }

        // Update trend analysis
        self.trend_analyzer.update_trends(&metrics).await?;

        Ok(())
    }

    /// Generate comprehensive metrics report
    pub async fn generate_report(
        &self,
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    ) -> Result<MetricsReport> {
        let metrics_history = self.monitor.get_metrics_history(time_range).await?;
        let trends = self.trend_analyzer.analyze_trends(&metrics_history).await?;
        let alerts = self.alert_manager.get_recent_alerts(100).await?;

        Ok(MetricsReport {
            generated_at: Utc::now(),
            time_range,
            summary: self.generate_summary(&metrics_history),
            trends: trends.clone(),
            alerts,
            recommendations: self.generate_recommendations(&metrics_history, &trends),
        })
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> Result<SystemHealth> {
        let recent_metrics = self.monitor.get_recent_metrics(10).await?;
        let active_alerts = self.alert_manager.get_active_alerts().await?;

        Ok(SystemHealth {
            overall_status: self.calculate_overall_status(&recent_metrics, &active_alerts),
            component_statuses: self.calculate_component_statuses(&recent_metrics),
            active_alerts_count: active_alerts.len(),
            last_updated: Utc::now(),
        })
    }

    fn generate_summary(&self, metrics: &[OutputMetrics]) -> MetricsSummary {
        if metrics.is_empty() {
            return MetricsSummary::default();
        }

        let total_operations = metrics.len();
        let avg_generation_time = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms)
            .sum::<u64>() as f64
            / total_operations as f64;
        let success_rate = metrics.iter().filter(|m| m.functionality.success).count() as f64
            / total_operations as f64
            * 100.0;
        let avg_validation_score = metrics
            .iter()
            .map(|m| m.functionality.validation_score)
            .sum::<f64>()
            / total_operations as f64;
        let security_incidents = metrics
            .iter()
            .map(|m| m.security.incidents_detected)
            .sum::<u64>();
        let avg_user_satisfaction = metrics
            .iter()
            .map(|m| m.user_experience.satisfaction_score)
            .sum::<f64>()
            / total_operations as f64;

        MetricsSummary {
            total_operations,
            avg_generation_time,
            success_rate,
            avg_validation_score,
            security_incidents,
            avg_user_satisfaction,
        }
    }

    fn generate_recommendations(
        &self,
        _metrics: &[OutputMetrics],
        trend_analysis: &TrendAnalysis,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Performance recommendations
        if trend_analysis.performance_trend == TrendDirection::Degrading {
            recommendations.push(
                "Performance is degrading. Consider optimizing output generation.".to_string(),
            );
        }

        // Security recommendations
        if trend_analysis.security_incidents_trend == TrendDirection::Increasing {
            recommendations
                .push("Security incidents are increasing. Review output sanitization.".to_string());
        }

        // Success rate recommendations
        if trend_analysis.success_rate_trend == TrendDirection::Degrading {
            recommendations.push(
                "Output success rate is declining. Investigate format-specific issues.".to_string(),
            );
        }

        recommendations
    }

    pub fn calculate_overall_status(
        &self,
        metrics: &[OutputMetrics],
        alerts: &[Alert],
    ) -> HealthStatus {
        if alerts.iter().any(|a| a.severity == AlertSeverity::Critical) {
            return HealthStatus::Critical;
        }

        if metrics.is_empty() {
            return HealthStatus::Unknown;
        }

        let recent_success_rate = metrics.iter().filter(|m| m.functionality.success).count() as f64
            / metrics.len() as f64;

        if recent_success_rate >= 0.95 {
            HealthStatus::Healthy
        } else if recent_success_rate >= 0.85 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        }
    }

    pub fn calculate_component_statuses(
        &self,
        metrics: &[OutputMetrics],
    ) -> HashMap<String, HealthStatus> {
        let mut statuses = HashMap::new();

        if let Some(latest) = metrics.last() {
            statuses.insert(
                "functionality".to_string(),
                if latest.functionality.success {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Critical
                },
            );
            statuses.insert(
                "performance".to_string(),
                if latest.performance.generation_time_ms < 5000 {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Warning
                },
            );
            statuses.insert(
                "security".to_string(),
                if latest.security.incidents_detected == 0 {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Warning
                },
            );
            statuses.insert(
                "user_experience".to_string(),
                if latest.user_experience.satisfaction_score >= 4.0 {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Warning
                },
            );
        }

        statuses
    }
}

impl Default for OutputMetricsService {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive metrics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsReport {
    pub generated_at: DateTime<Utc>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub summary: MetricsSummary,
    pub trends: TrendAnalysis,
    pub alerts: Vec<Alert>,
    pub recommendations: Vec<String>,
}

/// Metrics summary
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsSummary {
    pub total_operations: usize,
    pub avg_generation_time: f64,
    pub success_rate: f64,
    pub avg_validation_score: f64,
    pub security_incidents: u64,
    pub avg_user_satisfaction: f64,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub component_statuses: HashMap<String, HealthStatus>,
    pub active_alerts_count: usize,
    pub last_updated: DateTime<Utc>,
}

/// Health status levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Alert structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
}

/// Trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    Increasing,
    Decreasing,
}
