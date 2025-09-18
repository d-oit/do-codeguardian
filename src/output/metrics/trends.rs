//! # Trend Analysis
//!
//! Provides trend analysis capabilities for output metrics.

use super::types::*;
use super::TrendDirection;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trend analyzer for metrics
#[derive(Debug)]
pub struct TrendAnalyzer {
    trend_history: HashMap<String, Vec<TrendDataPoint>>,
    analysis_config: TrendAnalysisConfig,
}

#[derive(Debug, Clone)]
pub struct TrendAnalysisConfig {
    pub min_data_points: usize,
    pub trend_window_hours: u64,
    pub anomaly_threshold: f64,
    pub seasonal_analysis_enabled: bool,
}

impl Default for TrendAnalysisConfig {
    fn default() -> Self {
        Self {
            min_data_points: 10,
            trend_window_hours: 24,
            anomaly_threshold: 2.0,
            seasonal_analysis_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub analysis_period: (DateTime<Utc>, DateTime<Utc>),
    pub performance_trend: TrendDirection,
    pub success_rate_trend: TrendDirection,
    pub security_incidents_trend: TrendDirection,
    pub user_satisfaction_trend: TrendDirection,
    pub memory_usage_trend: TrendDirection,
    pub predictions: Vec<MetricPrediction>,
    pub correlations: Vec<MetricCorrelation>,
    pub anomalies: Vec<MetricAnomaly>,
}

impl Default for TrendAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TrendAnalyzer {
    /// Create a new trend analyzer
    pub fn new() -> Self {
        Self {
            trend_history: HashMap::new(),
            analysis_config: TrendAnalysisConfig::default(),
        }
    }

    /// Update trends with new metrics
    pub async fn update_trends(&mut self, metrics: &OutputMetrics) -> Result<()> {
        // Update trend data for key metrics
        self.update_trend_data(
            "generation_time_ms",
            MetricValue::Integer(metrics.performance.generation_time_ms as i64),
            metrics.timestamp,
        );
        self.update_trend_data(
            "success",
            MetricValue::Boolean(metrics.functionality.success),
            metrics.timestamp,
        );
        self.update_trend_data(
            "security_incidents",
            MetricValue::Integer(metrics.security.incidents_detected as i64),
            metrics.timestamp,
        );
        self.update_trend_data(
            "satisfaction_score",
            MetricValue::Float(metrics.user_experience.satisfaction_score),
            metrics.timestamp,
        );
        self.update_trend_data(
            "memory_usage",
            MetricValue::Integer(metrics.performance.memory_usage_bytes as i64),
            metrics.timestamp,
        );

        Ok(())
    }

    /// Analyze trends for a given time period
    pub async fn analyze_trends(&self, metrics_history: &[OutputMetrics]) -> Result<TrendAnalysis> {
        if metrics_history.is_empty() {
            return Ok(TrendAnalysis {
                analysis_period: (Utc::now(), Utc::now()),
                performance_trend: TrendDirection::Stable,
                success_rate_trend: TrendDirection::Stable,
                security_incidents_trend: TrendDirection::Stable,
                user_satisfaction_trend: TrendDirection::Stable,
                memory_usage_trend: TrendDirection::Stable,
                predictions: Vec::new(),
                correlations: Vec::new(),
                anomalies: Vec::new(),
            });
        }

        let start_time = metrics_history
            .iter()
            .map(|m| m.timestamp)
            .min()
            .unwrap_or(Utc::now());
        let end_time = metrics_history
            .iter()
            .map(|m| m.timestamp)
            .max()
            .unwrap_or(Utc::now());

        let performance_trend = self.analyze_performance_trend(metrics_history);
        let success_rate_trend = self.analyze_success_rate_trend(metrics_history);
        let security_incidents_trend = self.analyze_security_trend(metrics_history);
        let user_satisfaction_trend = self.analyze_satisfaction_trend(metrics_history);
        let memory_usage_trend = self.analyze_memory_trend(metrics_history);

        let predictions = self.generate_predictions(metrics_history).await?;
        let correlations = self.analyze_correlations(metrics_history);
        let anomalies = self.detect_trend_anomalies(metrics_history);

        Ok(TrendAnalysis {
            analysis_period: (start_time, end_time),
            performance_trend,
            success_rate_trend,
            security_incidents_trend,
            user_satisfaction_trend,
            memory_usage_trend,
            predictions,
            correlations,
            anomalies,
        })
    }

    /// Get trend data for a specific metric
    pub fn get_trend_data(&self, metric_name: &str) -> Vec<TrendDataPoint> {
        self.trend_history
            .get(metric_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Calculate trend direction for a series of values
    fn calculate_trend_direction(&self, values: &[f64]) -> TrendDirection {
        if values.len() < 2 {
            return TrendDirection::Stable;
        }

        // Simple linear regression to determine trend
        let n = values.len() as f64;
        let sum_x: f64 = (0..values.len()).map(|i| i as f64).sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = values.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_xx: f64 = (0..values.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x.powi(2));

        if slope.abs() < 0.01 {
            TrendDirection::Stable
        } else if slope > 0.0 {
            TrendDirection::Increasing
        } else {
            TrendDirection::Degrading
        }
    }

    fn analyze_performance_trend(&self, metrics: &[OutputMetrics]) -> TrendDirection {
        let generation_times: Vec<f64> = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms as f64)
            .collect();
        self.calculate_trend_direction(&generation_times)
    }

    fn analyze_success_rate_trend(&self, metrics: &[OutputMetrics]) -> TrendDirection {
        let success_rates: Vec<f64> = metrics
            .windows(10)
            .map(|window| {
                let success_count = window.iter().filter(|m| m.functionality.success).count();
                success_count as f64 / window.len() as f64
            })
            .collect();
        self.calculate_trend_direction(&success_rates)
    }

    fn analyze_security_trend(&self, metrics: &[OutputMetrics]) -> TrendDirection {
        let incident_counts: Vec<f64> = metrics
            .iter()
            .map(|m| m.security.incidents_detected as f64)
            .collect();
        self.calculate_trend_direction(&incident_counts)
    }

    fn analyze_satisfaction_trend(&self, metrics: &[OutputMetrics]) -> TrendDirection {
        let satisfaction_scores: Vec<f64> = metrics
            .iter()
            .map(|m| m.user_experience.satisfaction_score)
            .collect();
        self.calculate_trend_direction(&satisfaction_scores)
    }

    fn analyze_memory_trend(&self, metrics: &[OutputMetrics]) -> TrendDirection {
        let memory_usage: Vec<f64> = metrics
            .iter()
            .map(|m| m.performance.memory_usage_bytes as f64)
            .collect();
        self.calculate_trend_direction(&memory_usage)
    }

    async fn generate_predictions(
        &self,
        metrics: &[OutputMetrics],
    ) -> Result<Vec<MetricPrediction>> {
        let mut predictions = Vec::new();

        // Predict generation time
        if let Some(prediction) = self.predict_generation_time(metrics).await {
            predictions.push(prediction);
        }

        // Predict success rate
        if let Some(prediction) = self.predict_success_rate(metrics).await {
            predictions.push(prediction);
        }

        Ok(predictions)
    }

    async fn predict_generation_time(&self, metrics: &[OutputMetrics]) -> Option<MetricPrediction> {
        if metrics.len() < 5 {
            return None;
        }

        let recent_times: Vec<f64> = metrics
            .iter()
            .rev()
            .take(10)
            .rev()
            .map(|m| m.performance.generation_time_ms as f64)
            .collect();

        // Simple linear extrapolation
        let _n = recent_times.len() as f64;
        let slope = self.calculate_slope(&recent_times);
        let last_value = *recent_times.last()?;
        let next_value = last_value + slope;

        Some(MetricPrediction {
            metric_name: "generation_time_ms".to_string(),
            predicted_value: MetricValue::Float(next_value),
            confidence_level: 0.7,
            prediction_horizon_seconds: 3600, // 1 hour
            prediction_timestamp: Utc::now() + Duration::hours(1),
        })
    }

    async fn predict_success_rate(&self, metrics: &[OutputMetrics]) -> Option<MetricPrediction> {
        if metrics.len() < 10 {
            return None;
        }

        let recent_success_rates: Vec<f64> = metrics
            .windows(5)
            .map(|window| {
                let success_count = window.iter().filter(|m| m.functionality.success).count();
                success_count as f64 / window.len() as f64
            })
            .collect();

        let slope = self.calculate_slope(&recent_success_rates);
        let last_rate = *recent_success_rates.last()?;
        let next_rate = (last_rate + slope).clamp(0.0, 1.0);

        Some(MetricPrediction {
            metric_name: "success_rate".to_string(),
            predicted_value: MetricValue::Float(next_rate),
            confidence_level: 0.8,
            prediction_horizon_seconds: 3600,
            prediction_timestamp: Utc::now() + Duration::hours(1),
        })
    }

    fn analyze_correlations(&self, metrics: &[OutputMetrics]) -> Vec<MetricCorrelation> {
        let mut correlations = Vec::new();

        // Correlation between generation time and memory usage
        if let Some(corr) = self.calculate_correlation(
            metrics,
            |m| m.performance.generation_time_ms as f64,
            |m| m.performance.memory_usage_bytes as f64,
        ) {
            correlations.push(MetricCorrelation {
                metric_a: "generation_time_ms".to_string(),
                metric_b: "memory_usage_bytes".to_string(),
                correlation_coefficient: corr,
                significance_level: 0.05,
                time_window_seconds: 3600,
            });
        }

        // Correlation between success rate and satisfaction
        if let Some(corr) = self.calculate_correlation(
            metrics,
            |m| if m.functionality.success { 1.0 } else { 0.0 },
            |m| m.user_experience.satisfaction_score,
        ) {
            correlations.push(MetricCorrelation {
                metric_a: "success".to_string(),
                metric_b: "satisfaction_score".to_string(),
                correlation_coefficient: corr,
                significance_level: 0.05,
                time_window_seconds: 3600,
            });
        }

        correlations
    }

    fn detect_trend_anomalies(&self, metrics: &[OutputMetrics]) -> Vec<MetricAnomaly> {
        let mut anomalies = Vec::new();

        // Detect sudden spikes in generation time
        if let Some(anomaly) = self.detect_generation_time_spike(metrics) {
            anomalies.push(anomaly);
        }

        // Detect sudden drops in success rate
        if let Some(anomaly) = self.detect_success_rate_drop(metrics) {
            anomalies.push(anomaly);
        }

        anomalies
    }

    fn detect_generation_time_spike(&self, metrics: &[OutputMetrics]) -> Option<MetricAnomaly> {
        if metrics.len() < 5 {
            return None;
        }

        let recent_times: Vec<f64> = metrics
            .iter()
            .rev()
            .take(10)
            .rev()
            .map(|m| m.performance.generation_time_ms as f64)
            .collect();

        let mean = recent_times.iter().sum::<f64>() / recent_times.len() as f64;
        let variance = recent_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>()
            / recent_times.len() as f64;
        let std_dev = variance.sqrt();

        let latest = metrics.last()?;
        let latest_time = latest.performance.generation_time_ms as f64;

        if latest_time > mean + 3.0 * std_dev {
            Some(MetricAnomaly {
                metric_name: "generation_time_ms".to_string(),
                timestamp: latest.timestamp,
                expected_value: MetricValue::Float(mean),
                actual_value: MetricValue::Float(latest_time),
                deviation_score: (latest_time - mean) / std_dev,
                anomaly_type: AnomalyType::Spike,
            })
        } else {
            None
        }
    }

    fn detect_success_rate_drop(&self, metrics: &[OutputMetrics]) -> Option<MetricAnomaly> {
        if metrics.len() < 10 {
            return None;
        }

        let recent_window = &metrics[metrics.len().saturating_sub(10)..];
        let success_rate = recent_window
            .iter()
            .filter(|m| m.functionality.success)
            .count() as f64
            / recent_window.len() as f64;

        let older_window =
            &metrics[metrics.len().saturating_sub(20)..metrics.len().saturating_sub(10)];
        if older_window.is_empty() {
            return None;
        }

        let older_success_rate = older_window
            .iter()
            .filter(|m| m.functionality.success)
            .count() as f64
            / older_window.len() as f64;

        if older_success_rate > 0.9 && success_rate < 0.7 {
            Some(MetricAnomaly {
                metric_name: "success_rate".to_string(),
                timestamp: metrics.last()?.timestamp,
                expected_value: MetricValue::Float(older_success_rate),
                actual_value: MetricValue::Float(success_rate),
                deviation_score: (older_success_rate - success_rate) / older_success_rate,
                anomaly_type: AnomalyType::Drop,
            })
        } else {
            None
        }
    }

    fn update_trend_data(
        &mut self,
        metric_name: &str,
        value: MetricValue,
        timestamp: DateTime<Utc>,
    ) {
        let data_points = self
            .trend_history
            .entry(metric_name.to_string())
            .or_default();

        data_points.push(TrendDataPoint {
            timestamp,
            value: value.clone(),
            confidence_interval: None,
        });

        // Keep only recent data points
        let cutoff = Utc::now() - Duration::hours(self.analysis_config.trend_window_hours as i64);
        data_points.retain(|point| point.timestamp > cutoff);

        // Limit the number of data points
        if data_points.len() > 1000 {
            *data_points = data_points.iter().rev().take(500).rev().cloned().collect();
        }
    }

    fn calculate_slope(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        let n = values.len() as f64;
        let sum_x: f64 = (0..values.len()).map(|i| i as f64).sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = values.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_xx: f64 = (0..values.len()).map(|i| (i as f64).powi(2)).sum();

        (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x.powi(2))
    }

    fn calculate_correlation<F1, F2>(
        &self,
        metrics: &[OutputMetrics],
        extract_a: F1,
        extract_b: F2,
    ) -> Option<f64>
    where
        F1: Fn(&OutputMetrics) -> f64,
        F2: Fn(&OutputMetrics) -> f64,
    {
        if metrics.len() < 3 {
            return None;
        }

        let values_a: Vec<f64> = metrics.iter().map(&extract_a).collect();
        let values_b: Vec<f64> = metrics.iter().map(&extract_b).collect();

        let mean_a = values_a.iter().sum::<f64>() / values_a.len() as f64;
        let mean_b = values_b.iter().sum::<f64>() / values_b.len() as f64;

        let covariance = values_a
            .iter()
            .zip(values_b.iter())
            .map(|(a, b)| (a - mean_a) * (b - mean_b))
            .sum::<f64>()
            / values_a.len() as f64;

        let std_a = (values_a.iter().map(|a| (a - mean_a).powi(2)).sum::<f64>()
            / values_a.len() as f64)
            .sqrt();
        let std_b = (values_b.iter().map(|b| (b - mean_b).powi(2)).sum::<f64>()
            / values_b.len() as f64)
            .sqrt();

        if std_a == 0.0 || std_b == 0.0 {
            None
        } else {
            Some(covariance / (std_a * std_b))
        }
    }
}
