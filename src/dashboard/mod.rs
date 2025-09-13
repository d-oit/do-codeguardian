//! Unified Duplicate Prevention Dashboard
//!
//! Provides a comprehensive web-based dashboard for monitoring duplicates
//! across all ecosystem components including code, issues, documentation, and configurations.

#[cfg(feature = "dashboard")]
pub mod server;

pub mod data;
pub mod reports;

use crate::types::Finding;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub refresh_interval_seconds: u64,
    pub max_history_days: u32,
    pub enable_real_time: bool,
    pub custom_views: Vec<CustomView>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "127.0.0.1".to_string(),
            port: 8080,
            refresh_interval_seconds: 30,
            max_history_days: 30,
            enable_real_time: true,
            custom_views: vec![
                CustomView::default_overview(),
                CustomView::default_security(),
                CustomView::default_performance(),
            ],
        }
    }
}

/// Custom dashboard view configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomView {
    pub name: String,
    pub description: String,
    pub filters: ViewFilters,
    pub charts: Vec<ChartConfig>,
    pub stakeholder_role: StakeholderRole,
}

impl CustomView {
    pub fn default_overview() -> Self {
        Self {
            name: "Overview".to_string(),
            description: "High-level duplicate prevention metrics".to_string(),
            filters: ViewFilters::default(),
            charts: vec![
                ChartConfig::duplicate_trends(),
                ChartConfig::prevention_effectiveness(),
                ChartConfig::system_health(),
            ],
            stakeholder_role: StakeholderRole::Manager,
        }
    }

    pub fn default_security() -> Self {
        Self {
            name: "Security Focus".to_string(),
            description: "Security-related duplicate detection and prevention".to_string(),
            filters: ViewFilters {
                severity_filter: Some(vec!["high".to_string(), "critical".to_string()]),
                category_filter: Some(vec!["security".to_string()]),
                ..Default::default()
            },
            charts: vec![
                ChartConfig::security_duplicates(),
                ChartConfig::vulnerability_trends(),
            ],
            stakeholder_role: StakeholderRole::SecurityTeam,
        }
    }

    pub fn default_performance() -> Self {
        Self {
            name: "Performance Metrics".to_string(),
            description: "System performance and processing metrics".to_string(),
            filters: ViewFilters::default(),
            charts: vec![
                ChartConfig::processing_times(),
                ChartConfig::resource_usage(),
            ],
            stakeholder_role: StakeholderRole::Developer,
        }
    }
}

/// Dashboard view filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewFilters {
    pub time_range: Option<TimeRange>,
    pub severity_filter: Option<Vec<String>>,
    pub category_filter: Option<Vec<String>>,
    pub repository_filter: Option<Vec<String>>,
    pub file_type_filter: Option<Vec<String>>,
}

impl Default for ViewFilters {
    fn default() -> Self {
        Self {
            time_range: Some(TimeRange::Last7Days),
            severity_filter: None,
            category_filter: None,
            repository_filter: None,
            file_type_filter: None,
        }
    }
}

/// Time range for dashboard views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeRange {
    Last24Hours,
    Last7Days,
    Last30Days,
    Last90Days,
    Custom { start: DateTime<Utc>, end: DateTime<Utc> },
}

/// Chart configuration for dashboard views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub chart_type: ChartType,
    pub title: String,
    pub data_source: DataSource,
    pub refresh_rate: Option<u64>,
}

impl ChartConfig {
    pub fn duplicate_trends() -> Self {
        Self {
            chart_type: ChartType::LineChart,
            title: "Duplicate Detection Trends".to_string(),
            data_source: DataSource::DuplicateMetrics,
            refresh_rate: Some(60),
        }
    }

    pub fn prevention_effectiveness() -> Self {
        Self {
            chart_type: ChartType::PieChart,
            title: "Prevention Effectiveness".to_string(),
            data_source: DataSource::PreventionStats,
            refresh_rate: Some(300),
        }
    }

    pub fn system_health() -> Self {
        Self {
            chart_type: ChartType::GaugeChart,
            title: "System Health".to_string(),
            data_source: DataSource::SystemMetrics,
            refresh_rate: Some(30),
        }
    }

    pub fn security_duplicates() -> Self {
        Self {
            chart_type: ChartType::BarChart,
            title: "Security Duplicate Detection".to_string(),
            data_source: DataSource::SecurityMetrics,
            refresh_rate: Some(120),
        }
    }

    pub fn vulnerability_trends() -> Self {
        Self {
            chart_type: ChartType::LineChart,
            title: "Vulnerability Trends".to_string(),
            data_source: DataSource::VulnerabilityMetrics,
            refresh_rate: Some(180),
        }
    }

    pub fn processing_times() -> Self {
        Self {
            chart_type: ChartType::HistogramChart,
            title: "Processing Time Distribution".to_string(),
            data_source: DataSource::PerformanceMetrics,
            refresh_rate: Some(60),
        }
    }

    pub fn resource_usage() -> Self {
        Self {
            chart_type: ChartType::AreaChart,
            title: "Resource Usage".to_string(),
            data_source: DataSource::ResourceMetrics,
            refresh_rate: Some(30),
        }
    }
}

/// Chart types supported by the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    LineChart,
    BarChart,
    PieChart,
    AreaChart,
    GaugeChart,
    HistogramChart,
    HeatmapChart,
}

/// Data sources for dashboard charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    DuplicateMetrics,
    PreventionStats,
    SystemMetrics,
    SecurityMetrics,
    VulnerabilityMetrics,
    PerformanceMetrics,
    ResourceMetrics,
}

/// Stakeholder roles for customized views
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakeholderRole {
    Developer,
    SecurityTeam,
    Manager,
    QualityAssurance,
    DevOps,
}

/// Dashboard metrics aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub timestamp: DateTime<Utc>,
    pub duplicate_stats: DuplicateStats,
    pub prevention_stats: PreventionStats,
    pub system_health: SystemHealth,
    pub performance_metrics: PerformanceMetrics,
}

/// Duplicate detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateStats {
    pub total_duplicates_found: u64,
    pub duplicates_by_type: HashMap<String, u64>,
    pub duplicates_by_severity: HashMap<String, u64>,
    pub false_positive_rate: f64,
    pub detection_accuracy: f64,
}

/// Prevention effectiveness statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventionStats {
    pub duplicates_prevented: u64,
    pub duplicates_created: u64,
    pub prevention_rate: f64,
    pub time_saved_hours: f64,
    pub cost_savings_estimate: f64,
}

/// System health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub api_success_rate: f64,
    pub average_response_time_ms: f64,
    pub error_rate: f64,
    pub uptime_percentage: f64,
    pub active_connections: u32,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_processing_time_ms: f64,
    pub throughput_per_minute: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percentage: f64,
    pub queue_length: u32,
}

/// Dashboard service for managing metrics and views
pub struct DashboardService {
    config: DashboardConfig,
    metrics_history: Vec<DashboardMetrics>,
}

impl DashboardService {
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            config,
            metrics_history: Vec::new(),
        }
    }

    /// Start the dashboard service
    #[cfg(feature = "dashboard")]
    pub async fn start(&self) -> Result<()> {
        server::start_dashboard_server(&self.config).await
    }

    /// Update dashboard metrics
    pub fn update_metrics(&mut self, metrics: DashboardMetrics) {
        self.metrics_history.push(metrics);

        // Cleanup old metrics based on retention policy
        let cutoff_date = Utc::now() - chrono::Duration::days(self.config.max_history_days as i64);
        self.metrics_history.retain(|m| m.timestamp > cutoff_date);
    }

    /// Get current metrics
    pub fn get_current_metrics(&self) -> Option<&DashboardMetrics> {
        self.metrics_history.last()
    }

    /// Get metrics for a time range
    pub fn get_metrics_for_range(&self, range: &TimeRange) -> Vec<&DashboardMetrics> {
        let (start, end) = match range {
            TimeRange::Last24Hours => (Utc::now() - chrono::Duration::hours(24), Utc::now()),
            TimeRange::Last7Days => (Utc::now() - chrono::Duration::days(7), Utc::now()),
            TimeRange::Last30Days => (Utc::now() - chrono::Duration::days(30), Utc::now()),
            TimeRange::Last90Days => (Utc::now() - chrono::Duration::days(90), Utc::now()),
            TimeRange::Custom { start, end } => (*start, *end),
        };

        self.metrics_history
            .iter()
            .filter(|m| m.timestamp >= start && m.timestamp <= end)
            .collect()
    }

    /// Generate dashboard report
    pub fn generate_report(&self, view: &CustomView) -> Result<DashboardReport> {
        let metrics = self.get_metrics_for_range(&view.filters.time_range.as_ref().unwrap_or(&TimeRange::Last7Days));

        Ok(DashboardReport {
            view_name: view.name.clone(),
            generated_at: Utc::now(),
            summary: self.generate_summary(&metrics),
            charts_data: self.generate_charts_data(&view.charts, &metrics)?,
            recommendations: self.generate_recommendations(&metrics),
        })
    }

    fn generate_summary(&self, metrics: &[&DashboardMetrics]) -> DashboardSummary {
        if metrics.is_empty() {
            return DashboardSummary::default();
        }

        let total_duplicates: u64 = metrics.iter().map(|m| m.duplicate_stats.total_duplicates_found).sum();
        let avg_accuracy: f64 = metrics.iter().map(|m| m.duplicate_stats.detection_accuracy).sum::<f64>() / metrics.len() as f64;
        let avg_prevention_rate: f64 = metrics.iter().map(|m| m.prevention_stats.prevention_rate).sum::<f64>() / metrics.len() as f64;

        DashboardSummary {
            total_duplicates_detected: total_duplicates,
            average_detection_accuracy: avg_accuracy,
            average_prevention_rate: avg_prevention_rate,
            total_time_saved_hours: metrics.iter().map(|m| m.prevention_stats.time_saved_hours).sum(),
            system_uptime: metrics.last().map(|m| m.system_health.uptime_percentage).unwrap_or(0.0),
        }
    }

    fn generate_charts_data(&self, charts: &[ChartConfig], metrics: &[&DashboardMetrics]) -> Result<HashMap<String, serde_json::Value>> {
        let mut charts_data = HashMap::new();

        for chart in charts {
            let data = match chart.data_source {
                DataSource::DuplicateMetrics => {
                    let data: Vec<_> = metrics.iter().map(|m| {
                        serde_json::json!({
                            "timestamp": m.timestamp,
                            "total": m.duplicate_stats.total_duplicates_found,
                            "accuracy": m.duplicate_stats.detection_accuracy
                        })
                    }).collect();
                    serde_json::Value::Array(data)
                },
                DataSource::PreventionStats => {
                    let data: Vec<_> = metrics.iter().map(|m| {
                        serde_json::json!({
                            "timestamp": m.timestamp,
                            "prevented": m.prevention_stats.duplicates_prevented,
                            "created": m.prevention_stats.duplicates_created,
                            "rate": m.prevention_stats.prevention_rate
                        })
                    }).collect();
                    serde_json::Value::Array(data)
                },
                DataSource::SystemMetrics => {
                    let latest = metrics.last().unwrap();
                    serde_json::json!({
                        "api_success_rate": latest.system_health.api_success_rate,
                        "response_time": latest.system_health.average_response_time_ms,
                        "uptime": latest.system_health.uptime_percentage
                    })
                },
                DataSource::PerformanceMetrics => {
                    let data: Vec<_> = metrics.iter().map(|m| {
                        serde_json::json!({
                            "timestamp": m.timestamp,
                            "processing_time": m.performance_metrics.average_processing_time_ms,
                            "throughput": m.performance_metrics.throughput_per_minute,
                            "memory": m.performance_metrics.memory_usage_mb
                        })
                    }).collect();
                    serde_json::Value::Array(data)
                },
                _ => serde_json::Value::Null,
            };

            charts_data.insert(chart.title.clone(), data);
        }

        Ok(charts_data)
    }

    fn generate_recommendations(&self, metrics: &[&DashboardMetrics]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(latest) = metrics.last() {
            // Check detection accuracy
            if latest.duplicate_stats.detection_accuracy < 0.95 {
                recommendations.push("Consider retraining ML models to improve detection accuracy".to_string());
            }

            // Check false positive rate
            if latest.duplicate_stats.false_positive_rate > 0.05 {
                recommendations.push("High false positive rate detected - review detection thresholds".to_string());
            }

            // Check system performance
            if latest.performance_metrics.average_processing_time_ms > 30000.0 {
                recommendations.push("Processing times are above target - consider performance optimization".to_string());
            }

            // Check prevention effectiveness
            if latest.prevention_stats.prevention_rate < 0.90 {
                recommendations.push("Prevention rate is below target - review prevention strategies".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("System is performing well - no immediate actions required".to_string());
        }

        recommendations
    }
}

/// Dashboard report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardReport {
    pub view_name: String,
    pub generated_at: DateTime<Utc>,
    pub summary: DashboardSummary,
    pub charts_data: HashMap<String, serde_json::Value>,
    pub recommendations: Vec<String>,
}

/// Dashboard summary metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_duplicates_detected: u64,
    pub average_detection_accuracy: f64,
    pub average_prevention_rate: f64,
    pub total_time_saved_hours: f64,
    pub system_uptime: f64,
}

impl Default for DashboardSummary {
    fn default() -> Self {
        Self {
            total_duplicates_detected: 0,
            average_detection_accuracy: 0.0,
            average_prevention_rate: 0.0,
            total_time_saved_hours: 0.0,
            system_uptime: 0.0,
        }
    }
}
