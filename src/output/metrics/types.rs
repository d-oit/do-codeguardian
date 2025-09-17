//! # Metrics Types
//!
//! Defines all the data structures and types used in the metrics framework.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive output metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMetrics {
    pub timestamp: DateTime<Utc>,
    pub format: String,
    pub functionality: FunctionalityMetrics,
    pub performance: PerformanceMetrics,
    pub security: SecurityMetrics,
    pub user_experience: UserExperienceMetrics,
    pub metadata: HashMap<String, MetricValue>,
}

/// Functionality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalityMetrics {
    pub success: bool,
    pub validation_score: f64,
    pub format_compliance: f64,
    pub content_integrity: bool,
    pub schema_validation: bool,
    pub error_count: u32,
    pub warning_count: u32,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub generation_time_ms: u64,
    pub memory_usage_bytes: u64,
    pub cpu_usage_percent: f64,
    pub throughput_ops_per_sec: f64,
    pub latency_p95_ms: u64,
    pub resource_efficiency: f64,
}

/// Security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub incidents_detected: u64,
    pub sanitization_effectiveness: f64,
    pub vulnerability_score: f64,
    pub data_leakage_risk: f64,
    pub compliance_score: f64,
    pub encryption_status: bool,
}

/// User experience metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExperienceMetrics {
    pub satisfaction_score: f64,
    pub usability_rating: f64,
    pub error_recovery_rate: f64,
    pub format_preference_score: f64,
    pub accessibility_score: f64,
    pub customization_satisfaction: f64,
}

/// Generic metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<MetricValue>),
    Object(HashMap<String, MetricValue>),
}

/// Metric type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    Functionality,
    Performance,
    Security,
    UserExperience,
    Custom,
}

/// Metric threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricThreshold {
    pub metric_name: String,
    pub warning_threshold: MetricValue,
    pub critical_threshold: MetricValue,
    pub direction: ThresholdDirection,
}

/// Threshold direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThresholdDirection {
    Above,
    Below,
    Equal,
    NotEqual,
}

/// Metric aggregation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregation {
    pub metric_name: String,
    pub aggregation_type: AggregationType,
    pub time_window_seconds: u64,
}

/// Aggregation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationType {
    Average,
    Sum,
    Min,
    Max,
    Count,
    Percentile95,
    Percentile99,
}

/// Metric baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricBaseline {
    pub metric_name: String,
    pub baseline_value: MetricValue,
    pub standard_deviation: f64,
    pub sample_size: usize,
    pub last_updated: DateTime<Utc>,
}

/// Metric trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: MetricValue,
    pub confidence_interval: Option<(f64, f64)>,
}

/// Metric correlation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCorrelation {
    pub metric_a: String,
    pub metric_b: String,
    pub correlation_coefficient: f64,
    pub significance_level: f64,
    pub time_window_seconds: u64,
}

/// Metric anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAnomaly {
    pub metric_name: String,
    pub timestamp: DateTime<Utc>,
    pub expected_value: MetricValue,
    pub actual_value: MetricValue,
    pub deviation_score: f64,
    pub anomaly_type: AnomalyType,
}

/// Anomaly type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    Spike,
    Drop,
    TrendChange,
    Seasonal,
    Outlier,
}

/// Metric prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPrediction {
    pub metric_name: String,
    pub predicted_value: MetricValue,
    pub confidence_level: f64,
    pub prediction_horizon_seconds: u64,
    pub prediction_timestamp: DateTime<Utc>,
}

/// Metric health score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricHealthScore {
    pub overall_score: f64,
    pub component_scores: HashMap<String, f64>,
    pub risk_factors: Vec<String>,
    pub recommendations: Vec<String>,
    pub last_calculated: DateTime<Utc>,
}

impl OutputMetrics {
    /// Create new output metrics
    pub fn new(format: String) -> Self {
        Self {
            timestamp: Utc::now(),
            format,
            functionality: FunctionalityMetrics::default(),
            performance: PerformanceMetrics::default(),
            security: SecurityMetrics::default(),
            user_experience: UserExperienceMetrics::default(),
            metadata: HashMap::new(),
        }
    }

    /// Calculate overall health score
    pub fn calculate_health_score(&self) -> f64 {
        let functionality_score = if self.functionality.success { 1.0 } else { 0.0 };
        let performance_score =
            (5000.0 - self.performance.generation_time_ms as f64).max(0.0) / 5000.0;
        let security_score = 1.0 - (self.security.incidents_detected as f64 / 10.0).min(1.0);
        let ux_score = self.user_experience.satisfaction_score / 5.0;

        (functionality_score + performance_score + security_score + ux_score) / 4.0
    }

    /// Check if metrics indicate a healthy state
    pub fn is_healthy(&self) -> bool {
        self.functionality.success
            && self.performance.generation_time_ms < 5000
            && self.security.incidents_detected == 0
            && self.user_experience.satisfaction_score >= 3.0
    }
}

impl Default for FunctionalityMetrics {
    fn default() -> Self {
        Self {
            success: true,
            validation_score: 1.0,
            format_compliance: 1.0,
            content_integrity: true,
            schema_validation: true,
            error_count: 0,
            warning_count: 0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            generation_time_ms: 0,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0.0,
            throughput_ops_per_sec: 0.0,
            latency_p95_ms: 0,
            resource_efficiency: 1.0,
        }
    }
}

impl Default for SecurityMetrics {
    fn default() -> Self {
        Self {
            incidents_detected: 0,
            sanitization_effectiveness: 1.0,
            vulnerability_score: 0.0,
            data_leakage_risk: 0.0,
            compliance_score: 1.0,
            encryption_status: true,
        }
    }
}

impl Default for UserExperienceMetrics {
    fn default() -> Self {
        Self {
            satisfaction_score: 4.0,
            usability_rating: 4.0,
            error_recovery_rate: 1.0,
            format_preference_score: 4.0,
            accessibility_score: 4.5,
            customization_satisfaction: 4.0,
        }
    }
}

impl MetricValue {
    /// Convert to f64 if possible
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            MetricValue::Float(f) => Some(*f),
            MetricValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Convert to string
    pub fn as_string(&self) -> String {
        match self {
            MetricValue::String(s) => s.clone(),
            MetricValue::Integer(i) => i.to_string(),
            MetricValue::Float(f) => f.to_string(),
            MetricValue::Boolean(b) => b.to_string(),
            MetricValue::Array(_) => "[array]".to_string(),
            MetricValue::Object(_) => "{object}".to_string(),
        }
    }
}
