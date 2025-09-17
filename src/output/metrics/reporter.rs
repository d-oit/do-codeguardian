//! # Automated Metrics Reporter
//!
//! Provides automated reporting capabilities for metrics.

use super::types::*;
use super::{Alert, AlertSeverity, TrendDirection};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Automated reporter for metrics
#[derive(Debug)]
pub struct AutomatedReporter {
    config: ReporterConfig,
    report_history: Vec<ReportRecord>,
}

#[derive(Debug, Clone)]
pub struct ReporterConfig {
    pub report_interval_seconds: u64,
    pub retention_days: u32,
    pub enable_email_reports: bool,
    pub enable_slack_notifications: bool,
    pub report_formats: Vec<ReportFormat>,
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub critical_performance_threshold_ms: u64,
    pub warning_success_rate_threshold: f64,
    pub critical_security_incidents_threshold: u64,
    pub warning_memory_usage_threshold_mb: u64,
}

impl Default for ReporterConfig {
    fn default() -> Self {
        Self {
            report_interval_seconds: 3600, // 1 hour
            retention_days: 30,
            enable_email_reports: false,
            enable_slack_notifications: false,
            report_formats: vec![ReportFormat::Json, ReportFormat::Html],
            alert_thresholds: AlertThresholds {
                critical_performance_threshold_ms: 10000,
                warning_success_rate_threshold: 0.85,
                critical_security_incidents_threshold: 5,
                warning_memory_usage_threshold_mb: 500,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    Json,
    Html,
    Markdown,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub report_type: ReportType,
    pub format: ReportFormat,
    pub file_path: Option<String>,
    pub summary: ReportSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Daily,
    Weekly,
    Monthly,
    Alert,
    Anomaly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub title: String,
    pub description: String,
    pub key_metrics: HashMap<String, MetricValue>,
    pub alerts_count: usize,
    pub anomalies_count: usize,
}

impl AutomatedReporter {
    /// Create a new automated reporter
    pub fn new() -> Self {
        Self {
            config: ReporterConfig::default(),
            report_history: Vec::new(),
        }
    }

    /// Generate and save a comprehensive metrics report
    pub async fn generate_report(
        &mut self,
        metrics: &[OutputMetrics],
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
        report_type: ReportType,
    ) -> Result<String> {
        let report = self
            .create_comprehensive_report(metrics, time_range)
            .await?;
        let report_id = format!("report_{}_{}", report_type.as_str(), Utc::now().timestamp());

        // Generate reports in configured formats
        let formats = self.config.report_formats.clone();
        for format in formats {
            let file_path = self.save_report(&report, &format, &report_id).await?;
            self.record_report(
                &report_id,
                report_type.clone(),
                format.clone(),
                Some(file_path),
                &report.summary,
            );
        }

        // Clean up old reports
        self.cleanup_old_reports().await?;

        Ok(report_id)
    }

    /// Report an alert
    pub async fn report_alert(&mut self, alert: &Alert) -> Result<()> {
        // Log the alert
        println!("ALERT [{}]: {}", alert.severity.as_str(), alert.message);

        // Send notifications if enabled
        if self.config.enable_email_reports {
            self.send_email_alert(alert).await?;
        }

        if self.config.enable_slack_notifications {
            self.send_slack_alert(alert).await?;
        }

        // Generate alert report
        let alert_report = AlertReport {
            alert: alert.clone(),
            context: self.gather_alert_context(alert).await?,
            recommendations: self.generate_alert_recommendations(alert),
        };

        let report_id = format!("alert_{}_{}", alert.id, Utc::now().timestamp());
        let file_path = self.save_alert_report(&alert_report, &report_id).await?;
        self.record_report(
            &report_id,
            ReportType::Alert,
            ReportFormat::Json,
            Some(file_path),
            &alert_report.summary(),
        );

        Ok(())
    }

    /// Generate scheduled reports
    pub async fn generate_scheduled_reports(
        &mut self,
        metrics: &[OutputMetrics],
    ) -> Result<Vec<String>> {
        let mut report_ids = Vec::new();

        // Daily report
        if self.should_generate_daily_report() {
            let report_id = self
                .generate_report(metrics, Some(self.get_daily_range()), ReportType::Daily)
                .await?;
            report_ids.push(report_id);
        }

        // Weekly report
        if self.should_generate_weekly_report() {
            let report_id = self
                .generate_report(metrics, Some(self.get_weekly_range()), ReportType::Weekly)
                .await?;
            report_ids.push(report_id);
        }

        // Monthly report
        if self.should_generate_monthly_report() {
            let report_id = self
                .generate_report(metrics, Some(self.get_monthly_range()), ReportType::Monthly)
                .await?;
            report_ids.push(report_id);
        }

        Ok(report_ids)
    }

    /// Get report history
    pub fn get_report_history(&self, limit: Option<usize>) -> Vec<ReportRecord> {
        let mut history = self.report_history.clone();
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        match limit {
            Some(limit) => history.into_iter().take(limit).collect(),
            None => history,
        }
    }

    async fn create_comprehensive_report(
        &self,
        metrics: &[OutputMetrics],
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    ) -> Result<ComprehensiveReport> {
        let filtered_metrics = match time_range {
            Some((start, end)) => metrics
                .iter()
                .filter(|m| m.timestamp >= start && m.timestamp <= end)
                .cloned()
                .collect::<Vec<_>>(),
            None => metrics.to_vec(),
        };

        let summary = self.generate_report_summary(&filtered_metrics);
        let performance_analysis = self.analyze_performance(&filtered_metrics);
        let security_analysis = self.analyze_security(&filtered_metrics);
        let user_experience_analysis = self.analyze_user_experience(&filtered_metrics);
        let trends = self.analyze_trends(&filtered_metrics);
        let recommendations = self.generate_recommendations(&filtered_metrics);

        Ok(ComprehensiveReport {
            generated_at: Utc::now(),
            time_range,
            summary,
            performance_analysis,
            security_analysis,
            user_experience_analysis,
            trends,
            recommendations,
        })
    }

    fn generate_report_summary(&self, metrics: &[OutputMetrics]) -> ReportSummary {
        let total_operations = metrics.len();
        let successful_operations = metrics.iter().filter(|m| m.functionality.success).count();
        let success_rate = if total_operations > 0 {
            successful_operations as f64 / total_operations as f64 * 100.0
        } else {
            0.0
        };

        let avg_generation_time = if total_operations > 0 {
            metrics
                .iter()
                .map(|m| m.performance.generation_time_ms)
                .sum::<u64>() as f64
                / total_operations as f64
        } else {
            0.0
        };

        let total_security_incidents = metrics
            .iter()
            .map(|m| m.security.incidents_detected)
            .sum::<u64>();
        let avg_satisfaction = if total_operations > 0 {
            metrics
                .iter()
                .map(|m| m.user_experience.satisfaction_score)
                .sum::<f64>()
                / total_operations as f64
        } else {
            0.0
        };

        let mut key_metrics = HashMap::new();
        key_metrics.insert(
            "total_operations".to_string(),
            MetricValue::Integer(total_operations as i64),
        );
        key_metrics.insert("success_rate".to_string(), MetricValue::Float(success_rate));
        key_metrics.insert(
            "avg_generation_time_ms".to_string(),
            MetricValue::Float(avg_generation_time),
        );
        key_metrics.insert(
            "total_security_incidents".to_string(),
            MetricValue::Integer(total_security_incidents as i64),
        );
        key_metrics.insert(
            "avg_satisfaction".to_string(),
            MetricValue::Float(avg_satisfaction),
        );

        ReportSummary {
            title: "CodeGuardian Output Metrics Report".to_string(),
            description: format!("Analysis of {} output operations", total_operations),
            key_metrics,
            alerts_count: 0,    // Would be calculated based on actual alerts
            anomalies_count: 0, // Would be calculated based on detected anomalies
        }
    }

    fn analyze_performance(&self, metrics: &[OutputMetrics]) -> PerformanceAnalysis {
        let generation_times: Vec<u64> = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms)
            .collect();
        let memory_usage: Vec<u64> = metrics
            .iter()
            .map(|m| m.performance.memory_usage_bytes)
            .collect();
        let cpu_usage: Vec<f64> = metrics
            .iter()
            .map(|m| m.performance.cpu_usage_percent)
            .collect();

        let avg_generation_time =
            generation_times.iter().sum::<u64>() as f64 / generation_times.len() as f64;
        let p95_generation_time = self.calculate_percentile(&generation_times, 95);
        let max_generation_time = *generation_times.iter().max().unwrap_or(&0);

        let avg_memory_usage = memory_usage.iter().sum::<u64>() as f64 / memory_usage.len() as f64;
        let max_memory_usage = *memory_usage.iter().max().unwrap_or(&0);

        let avg_cpu_usage = cpu_usage.iter().sum::<f64>() / cpu_usage.len() as f64;

        PerformanceAnalysis {
            avg_generation_time,
            p95_generation_time,
            max_generation_time,
            avg_memory_usage,
            max_memory_usage,
            avg_cpu_usage,
            throughput_trend: self.calculate_trend(&generation_times),
            bottlenecks: self.identify_bottlenecks(metrics),
        }
    }

    fn analyze_security(&self, metrics: &[OutputMetrics]) -> SecurityAnalysis {
        let incidents: Vec<u64> = metrics
            .iter()
            .map(|m| m.security.incidents_detected)
            .collect();
        let sanitization_scores: Vec<f64> = metrics
            .iter()
            .map(|m| m.security.sanitization_effectiveness)
            .collect();
        let vulnerability_scores: Vec<f64> = metrics
            .iter()
            .map(|m| m.security.vulnerability_score)
            .collect();

        let total_incidents = incidents.iter().sum::<u64>();
        let avg_sanitization_score =
            sanitization_scores.iter().sum::<f64>() / sanitization_scores.len() as f64;
        let max_vulnerability_score = vulnerability_scores.iter().cloned().fold(0.0, f64::max);

        SecurityAnalysis {
            total_incidents,
            avg_sanitization_score,
            max_vulnerability_score,
            incident_trend: self.calculate_trend(&incidents),
            risk_assessment: self.assess_security_risk(metrics),
            compliance_status: self.check_compliance_status(metrics),
        }
    }

    fn analyze_user_experience(&self, metrics: &[OutputMetrics]) -> UserExperienceAnalysis {
        let satisfaction_scores: Vec<f64> = metrics
            .iter()
            .map(|m| m.user_experience.satisfaction_score)
            .collect();
        let usability_ratings: Vec<f64> = metrics
            .iter()
            .map(|m| m.user_experience.usability_rating)
            .collect();
        let error_recovery_rates: Vec<f64> = metrics
            .iter()
            .map(|m| m.user_experience.error_recovery_rate)
            .collect();

        let avg_satisfaction =
            satisfaction_scores.iter().sum::<f64>() / satisfaction_scores.len() as f64;
        let avg_usability = usability_ratings.iter().sum::<f64>() / usability_ratings.len() as f64;
        let avg_error_recovery =
            error_recovery_rates.iter().sum::<f64>() / error_recovery_rates.len() as f64;

        UserExperienceAnalysis {
            avg_satisfaction,
            avg_usability,
            avg_error_recovery,
            format_preferences: self.analyze_format_preferences(metrics),
            accessibility_score: self.calculate_accessibility_score(metrics),
            improvement_suggestions: self.generate_ux_improvements(metrics),
        }
    }

    fn analyze_trends(&self, _metrics: &[OutputMetrics]) -> TrendAnalysisSummary {
        // This would integrate with the TrendAnalyzer
        // For now, return a simplified version
        TrendAnalysisSummary {
            performance_trend: TrendDirection::Stable,
            quality_trend: TrendDirection::Improving,
            security_trend: TrendDirection::Stable,
            predictions: Vec::new(),
        }
    }

    fn generate_recommendations(&self, metrics: &[OutputMetrics]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let success_rate = metrics.iter().filter(|m| m.functionality.success).count() as f64
            / metrics.len() as f64;
        if success_rate < 0.9 {
            recommendations.push(
                "Improve output generation success rate by addressing validation errors"
                    .to_string(),
            );
        }

        let avg_generation_time = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms)
            .sum::<u64>() as f64
            / metrics.len() as f64;
        if avg_generation_time > 5000.0 {
            recommendations.push(
                "Optimize output generation performance to reduce processing time".to_string(),
            );
        }

        let total_incidents = metrics
            .iter()
            .map(|m| m.security.incidents_detected)
            .sum::<u64>();
        if total_incidents > 0 {
            recommendations.push(
                "Enhance security measures to reduce output sanitization incidents".to_string(),
            );
        }

        recommendations
    }

    async fn save_report(
        &self,
        report: &ComprehensiveReport,
        format: &ReportFormat,
        report_id: &str,
    ) -> Result<String> {
        let reports_dir = Path::new("reports");
        fs::create_dir_all(reports_dir)?;

        let file_name = format!("{}.{}", report_id, format.extension());
        let file_path = reports_dir.join(file_name);

        match format {
            ReportFormat::Json => {
                let json = serde_json::to_string_pretty(report)?;
                fs::write(&file_path, json)?;
            }
            ReportFormat::Html => {
                let html = self.generate_html_report(report);
                fs::write(&file_path, html)?;
            }
            ReportFormat::Markdown => {
                let markdown = self.generate_markdown_report(report);
                fs::write(&file_path, markdown)?;
            }
            ReportFormat::Pdf => {
                // PDF generation would require additional dependencies
                // For now, save as HTML
                let html = self.generate_html_report(report);
                fs::write(&file_path, html)?;
            }
        }

        Ok(file_path.to_string_lossy().to_string())
    }

    async fn save_alert_report(
        &self,
        alert_report: &AlertReport,
        report_id: &str,
    ) -> Result<String> {
        let alerts_dir = Path::new("reports/alerts");
        fs::create_dir_all(alerts_dir)?;

        let file_name = format!("{}.json", report_id);
        let file_path = alerts_dir.join(file_name);

        let json = serde_json::to_string_pretty(alert_report)?;
        fs::write(&file_path, json)?;

        Ok(file_path.to_string_lossy().to_string())
    }

    fn record_report(
        &mut self,
        id: &str,
        report_type: ReportType,
        format: ReportFormat,
        file_path: Option<String>,
        summary: &ReportSummary,
    ) {
        self.report_history.push(ReportRecord {
            id: id.to_string(),
            timestamp: Utc::now(),
            report_type,
            format,
            file_path,
            summary: summary.clone(),
        });
    }

    async fn cleanup_old_reports(&self) -> Result<()> {
        let _cutoff = Utc::now() - Duration::days(self.config.retention_days as i64);

        // Clean up report files
        if let Ok(entries) = fs::read_dir("reports") {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if modified.elapsed().unwrap_or_default()
                            > Duration::days(self.config.retention_days as i64)
                                .to_std()
                                .unwrap_or_default()
                        {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn send_email_alert(&self, _alert: &Alert) -> Result<()> {
        // Email sending implementation would go here
        // This would integrate with an email service
        Ok(())
    }

    async fn send_slack_alert(&self, _alert: &Alert) -> Result<()> {
        // Slack notification implementation would go here
        // This would integrate with Slack API
        Ok(())
    }

    async fn gather_alert_context(&self, _alert: &Alert) -> Result<HashMap<String, MetricValue>> {
        // Gather contextual information about the alert
        let mut context = HashMap::new();
        context.insert("system_load".to_string(), MetricValue::Float(0.75));
        context.insert("recent_errors".to_string(), MetricValue::Integer(3));
        Ok(context)
    }

    fn generate_alert_recommendations(&self, alert: &Alert) -> Vec<String> {
        match alert.severity {
            AlertSeverity::Critical => vec![
                "Immediate investigation required".to_string(),
                "Consider rolling back recent changes".to_string(),
            ],
            AlertSeverity::Warning => vec![
                "Monitor the situation closely".to_string(),
                "Review recent configuration changes".to_string(),
            ],
            AlertSeverity::Info => vec![
                "Log for awareness".to_string(),
                "Consider preventive measures".to_string(),
            ],
        }
    }

    fn should_generate_daily_report(&self) -> bool {
        // Check if it's time for daily report (simplified)
        true // Would check actual timing
    }

    fn should_generate_weekly_report(&self) -> bool {
        // Check if it's time for weekly report (simplified)
        false // Would check actual timing
    }

    fn should_generate_monthly_report(&self) -> bool {
        // Check if it's time for monthly report (simplified)
        false // Would check actual timing
    }

    fn get_daily_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let end = Utc::now();
        let start = end - Duration::days(1);
        (start, end)
    }

    fn get_weekly_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let end = Utc::now();
        let start = end - Duration::days(7);
        (start, end)
    }

    fn get_monthly_range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let end = Utc::now();
        let start = end - Duration::days(30);
        (start, end)
    }

    // Helper methods for analysis
    fn calculate_percentile(&self, values: &[u64], percentile: u8) -> u64 {
        if values.is_empty() {
            return 0;
        }

        let mut sorted = values.to_vec();
        sorted.sort();

        let index = (percentile as f64 / 100.0 * (sorted.len() - 1) as f64) as usize;
        sorted[index]
    }

    fn calculate_trend(&self, values: &[u64]) -> TrendDirection {
        if values.len() < 2 {
            return TrendDirection::Stable;
        }

        let first_half = &values[..values.len() / 2];
        let second_half = &values[values.len() / 2..];

        let first_avg = first_half.iter().sum::<u64>() as f64 / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<u64>() as f64 / second_half.len() as f64;

        let change = (second_avg - first_avg) / first_avg;

        if change.abs() < 0.05 {
            TrendDirection::Stable
        } else if change > 0.0 {
            TrendDirection::Increasing
        } else {
            TrendDirection::Degrading
        }
    }

    fn identify_bottlenecks(&self, _metrics: &[OutputMetrics]) -> Vec<String> {
        // Identify performance bottlenecks
        vec!["Output formatting".to_string()] // Simplified
    }

    fn assess_security_risk(&self, _metrics: &[OutputMetrics]) -> String {
        "Low".to_string() // Simplified risk assessment
    }

    fn check_compliance_status(&self, _metrics: &[OutputMetrics]) -> String {
        "Compliant".to_string() // Simplified compliance check
    }

    fn analyze_format_preferences(&self, _metrics: &[OutputMetrics]) -> HashMap<String, f64> {
        let mut preferences = HashMap::new();
        preferences.insert("json".to_string(), 0.4);
        preferences.insert("html".to_string(), 0.35);
        preferences.insert("markdown".to_string(), 0.25);
        preferences
    }

    fn calculate_accessibility_score(&self, _metrics: &[OutputMetrics]) -> f64 {
        4.2 // Simplified accessibility score
    }

    fn generate_ux_improvements(&self, _metrics: &[OutputMetrics]) -> Vec<String> {
        vec![
            "Improve error messages clarity".to_string(),
            "Add progress indicators for long operations".to_string(),
        ]
    }

    fn generate_html_report(&self, report: &ComprehensiveReport) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>CodeGuardian Metrics Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .metric {{ background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }}
        .alert {{ color: red; }}
        .success {{ color: green; }}
    </style>
</head>
<body>
    <h1>CodeGuardian Output Metrics Report</h1>
    <p>Generated at: {}</p>

    <h2>Summary</h2>
    <div class="metric">
        <p>Total Operations: {}</p>
        <p>Success Rate: {:.2}%</p>
        <p>Average Generation Time: {:.2}ms</p>
    </div>

    <h2>Recommendations</h2>
    <ul>
        {}
    </ul>
</body>
</html>"#,
            report.generated_at,
            report
                .summary
                .key_metrics
                .get("total_operations")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as u64,
            report
                .summary
                .key_metrics
                .get("success_rate")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            report
                .summary
                .key_metrics
                .get("avg_generation_time_ms")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            report
                .recommendations
                .iter()
                .map(|r| format!("<li>{}</li>", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn generate_markdown_report(&self, report: &ComprehensiveReport) -> String {
        format!(
            r#"# CodeGuardian Output Metrics Report

Generated at: {}

## Summary

- Total Operations: {}
- Success Rate: {:.2}%
- Average Generation Time: {:.2}ms

## Recommendations

{}
"#,
            report.generated_at,
            report
                .summary
                .key_metrics
                .get("total_operations")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as u64,
            report
                .summary
                .key_metrics
                .get("success_rate")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            report
                .summary
                .key_metrics
                .get("avg_generation_time_ms")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            report
                .recommendations
                .iter()
                .map(|r| format!("- {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

// Additional data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ComprehensiveReport {
    pub generated_at: DateTime<Utc>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub summary: ReportSummary,
    pub performance_analysis: PerformanceAnalysis,
    pub security_analysis: SecurityAnalysis,
    pub user_experience_analysis: UserExperienceAnalysis,
    pub trends: TrendAnalysisSummary,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub avg_generation_time: f64,
    pub p95_generation_time: u64,
    pub max_generation_time: u64,
    pub avg_memory_usage: f64,
    pub max_memory_usage: u64,
    pub avg_cpu_usage: f64,
    pub throughput_trend: TrendDirection,
    pub bottlenecks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    pub total_incidents: u64,
    pub avg_sanitization_score: f64,
    pub max_vulnerability_score: f64,
    pub incident_trend: TrendDirection,
    pub risk_assessment: String,
    pub compliance_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserExperienceAnalysis {
    pub avg_satisfaction: f64,
    pub avg_usability: f64,
    pub avg_error_recovery: f64,
    pub format_preferences: HashMap<String, f64>,
    pub accessibility_score: f64,
    pub improvement_suggestions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendAnalysisSummary {
    pub performance_trend: TrendDirection,
    pub quality_trend: TrendDirection,
    pub security_trend: TrendDirection,
    pub predictions: Vec<MetricPrediction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertReport {
    pub alert: Alert,
    pub context: HashMap<String, MetricValue>,
    pub recommendations: Vec<String>,
}

impl AlertReport {
    fn summary(&self) -> ReportSummary {
        ReportSummary {
            title: format!("Alert Report: {}", self.alert.message),
            description: format!("Alert triggered at {}", self.alert.timestamp),
            key_metrics: self.context.clone(),
            alerts_count: 1,
            anomalies_count: 0,
        }
    }
}

impl ReportFormat {
    fn extension(&self) -> &'static str {
        match self {
            ReportFormat::Json => "json",
            ReportFormat::Html => "html",
            ReportFormat::Markdown => "md",
            ReportFormat::Pdf => "pdf",
        }
    }
}

impl ReportType {
    fn as_str(&self) -> &'static str {
        match self {
            ReportType::Daily => "daily",
            ReportType::Weekly => "weekly",
            ReportType::Monthly => "monthly",
            ReportType::Alert => "alert",
            ReportType::Anomaly => "anomaly",
            ReportType::Custom => "custom",
        }
    }
}

impl AlertSeverity {
    fn as_str(&self) -> &'static str {
        match self {
            AlertSeverity::Critical => "CRITICAL",
            AlertSeverity::Warning => "WARNING",
            AlertSeverity::Info => "INFO",
        }
    }
}
