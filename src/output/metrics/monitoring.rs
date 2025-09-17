//! # Real-Time Monitoring
//!
//! Provides real-time monitoring capabilities for output metrics.

use super::types::*;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Real-time monitor for output metrics
#[derive(Debug)]
pub struct RealTimeMonitor {
    metrics_history: Arc<RwLock<VecDeque<OutputMetrics>>>,
    config: MonitoringConfig,
    subscribers: Arc<RwLock<Vec<Box<dyn MetricSubscriber>>>>,
}

#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub max_history_size: usize,
    pub retention_period_seconds: u64,
    pub enable_real_time_alerts: bool,
    pub sampling_interval_ms: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            max_history_size: 10000,
            retention_period_seconds: 7 * 24 * 60 * 60, // 7 days
            enable_real_time_alerts: true,
            sampling_interval_ms: 1000,
        }
    }
}

#[async_trait::async_trait]
pub trait MetricSubscriber: Send + Sync + std::fmt::Debug {
    async fn on_metric_update(&self, metrics: &OutputMetrics) -> Result<()>;
    async fn on_anomaly_detected(&self, anomaly: &MetricAnomaly) -> Result<()>;
}

impl RealTimeMonitor {
    /// Create a new real-time monitor
    pub fn new() -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(VecDeque::new())),
            config: MonitoringConfig::default(),
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Record new metrics
    pub async fn record_metrics(&self, metrics: OutputMetrics) -> Result<()> {
        let mut history = self.metrics_history.write().await;

        // Add new metrics
        history.push_back(metrics.clone());

        // Maintain history size
        while history.len() > self.config.max_history_size {
            history.pop_front();
        }

        // Clean up old metrics
        let cutoff = Utc::now() - Duration::seconds(self.config.retention_period_seconds as i64);
        while let Some(old_metric) = history.front() {
            if old_metric.timestamp < cutoff {
                history.pop_front();
            } else {
                break;
            }
        }

        drop(history);

        // Notify subscribers
        self.notify_subscribers(&metrics).await?;

        // Check for anomalies
        if let Some(anomaly) = self.detect_anomaly(&metrics).await? {
            self.notify_anomaly(&anomaly).await?;
        }

        Ok(())
    }

    /// Get recent metrics
    pub async fn get_recent_metrics(&self, count: usize) -> Result<Vec<OutputMetrics>> {
        let history = self.metrics_history.read().await;
        let start = history.len().saturating_sub(count);
        Ok(history.range(start..).cloned().collect())
    }

    /// Get metrics within a time range
    pub async fn get_metrics_history(
        &self,
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    ) -> Result<Vec<OutputMetrics>> {
        let history = self.metrics_history.read().await;

        match time_range {
            Some((start, end)) => Ok(history
                .iter()
                .filter(|m| m.timestamp >= start && m.timestamp <= end)
                .cloned()
                .collect()),
            None => Ok(history.iter().cloned().collect()),
        }
    }

    /// Get current metrics snapshot
    pub async fn get_current_snapshot(&self) -> Result<Option<OutputMetrics>> {
        let history = self.metrics_history.read().await;
        Ok(history.back().cloned())
    }

    /// Get metrics aggregated by time window
    pub async fn get_aggregated_metrics(
        &self,
        window_seconds: u64,
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    ) -> Result<Vec<AggregatedMetrics>> {
        let metrics = self.get_metrics_history(time_range).await?;
        if metrics.is_empty() {
            return Ok(Vec::new());
        }

        let window_duration = Duration::seconds(window_seconds as i64);
        let mut aggregated = Vec::new();

        // Sort metrics by timestamp
        let mut sorted_metrics = metrics;
        sorted_metrics.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let mut current_window_start = sorted_metrics[0].timestamp;
        let mut current_window_metrics = Vec::new();

        for metric in sorted_metrics {
            if metric.timestamp - current_window_start >= window_duration {
                if !current_window_metrics.is_empty() {
                    aggregated.push(
                        self.aggregate_metrics(&current_window_metrics, current_window_start),
                    );
                }
                current_window_start = metric.timestamp;
                current_window_metrics.clear();
            }
            current_window_metrics.push(metric);
        }

        // Add the last window
        if !current_window_metrics.is_empty() {
            aggregated.push(self.aggregate_metrics(&current_window_metrics, current_window_start));
        }

        Ok(aggregated)
    }

    /// Subscribe to metric updates
    pub async fn subscribe(&self, subscriber: Box<dyn MetricSubscriber>) -> Result<()> {
        let mut subscribers = self.subscribers.write().await;
        subscribers.push(subscriber);
        Ok(())
    }

    /// Get monitoring statistics
    pub async fn get_monitoring_stats(&self) -> Result<MonitoringStats> {
        let history = self.metrics_history.read().await;

        let total_metrics = history.len();
        let oldest_metric = history.front().map(|m| m.timestamp);
        let newest_metric = history.back().map(|m| m.timestamp);

        let time_span = match (oldest_metric, newest_metric) {
            (Some(oldest), Some(newest)) => Some(newest - oldest),
            _ => None,
        };

        Ok(MonitoringStats {
            total_metrics_recorded: total_metrics,
            monitoring_uptime: time_span,
            average_metrics_per_hour: if let Some(span) = time_span {
                let hours = span.num_hours() as f64;
                if hours > 0.0 {
                    total_metrics as f64 / hours
                } else {
                    0.0
                }
            } else {
                0.0
            },
            memory_usage_estimate: self.estimate_memory_usage(),
        })
    }

    async fn notify_subscribers(&self, metrics: &OutputMetrics) -> Result<()> {
        let subscribers = self.subscribers.read().await;
        for subscriber in subscribers.iter() {
            subscriber.on_metric_update(metrics).await?;
        }
        Ok(())
    }

    async fn notify_anomaly(&self, anomaly: &MetricAnomaly) -> Result<()> {
        let subscribers = self.subscribers.read().await;
        for subscriber in subscribers.iter() {
            subscriber.on_anomaly_detected(anomaly).await?;
        }
        Ok(())
    }

    async fn detect_anomaly(&self, current: &OutputMetrics) -> Result<Option<MetricAnomaly>> {
        let recent_metrics = self.get_recent_metrics(50).await?;
        if recent_metrics.len() < 10 {
            return Ok(None); // Not enough data for anomaly detection
        }

        // Check for generation time anomaly
        if let Some(anomaly) = self.detect_generation_time_anomaly(current, &recent_metrics) {
            return Ok(Some(anomaly));
        }

        // Check for success rate anomaly
        if let Some(anomaly) = self.detect_success_rate_anomaly(current, &recent_metrics) {
            return Ok(Some(anomaly));
        }

        // Check for memory usage anomaly
        if let Some(anomaly) = self.detect_memory_anomaly(current, &recent_metrics) {
            return Ok(Some(anomaly));
        }

        Ok(None)
    }

    fn detect_generation_time_anomaly(
        &self,
        current: &OutputMetrics,
        recent: &[OutputMetrics],
    ) -> Option<MetricAnomaly> {
        let recent_times: Vec<f64> = recent
            .iter()
            .map(|m| m.performance.generation_time_ms as f64)
            .collect();
        let mean = recent_times.iter().sum::<f64>() / recent_times.len() as f64;
        let variance = recent_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>()
            / recent_times.len() as f64;
        let std_dev = variance.sqrt();

        let current_time = current.performance.generation_time_ms as f64;
        let deviation = (current_time - mean).abs() / std_dev;

        if deviation > 3.0 {
            // 3 standard deviations
            Some(MetricAnomaly {
                metric_name: "generation_time_ms".to_string(),
                timestamp: current.timestamp,
                expected_value: MetricValue::Float(mean),
                actual_value: MetricValue::Float(current_time),
                deviation_score: deviation,
                anomaly_type: if current_time > mean {
                    AnomalyType::Spike
                } else {
                    AnomalyType::Drop
                },
            })
        } else {
            None
        }
    }

    fn detect_success_rate_anomaly(
        &self,
        current: &OutputMetrics,
        recent: &[OutputMetrics],
    ) -> Option<MetricAnomaly> {
        let success_count = recent.iter().filter(|m| m.functionality.success).count();
        let success_rate = success_count as f64 / recent.len() as f64;

        if !current.functionality.success && success_rate > 0.95 {
            Some(MetricAnomaly {
                metric_name: "success_rate".to_string(),
                timestamp: current.timestamp,
                expected_value: MetricValue::Boolean(true),
                actual_value: MetricValue::Boolean(false),
                deviation_score: 2.0,
                anomaly_type: AnomalyType::Drop,
            })
        } else {
            None
        }
    }

    fn detect_memory_anomaly(
        &self,
        current: &OutputMetrics,
        recent: &[OutputMetrics],
    ) -> Option<MetricAnomaly> {
        let recent_memory: Vec<f64> = recent
            .iter()
            .map(|m| m.performance.memory_usage_bytes as f64)
            .collect();
        let mean = recent_memory.iter().sum::<f64>() / recent_memory.len() as f64;
        let current_memory = current.performance.memory_usage_bytes as f64;

        if current_memory > mean * 2.0 {
            // More than double the average
            Some(MetricAnomaly {
                metric_name: "memory_usage_bytes".to_string(),
                timestamp: current.timestamp,
                expected_value: MetricValue::Float(mean),
                actual_value: MetricValue::Float(current_memory),
                deviation_score: (current_memory / mean) - 1.0,
                anomaly_type: AnomalyType::Spike,
            })
        } else {
            None
        }
    }

    fn aggregate_metrics(
        &self,
        metrics: &[OutputMetrics],
        window_start: DateTime<Utc>,
    ) -> AggregatedMetrics {
        let count = metrics.len();
        let avg_generation_time = metrics
            .iter()
            .map(|m| m.performance.generation_time_ms)
            .sum::<u64>() as f64
            / count as f64;
        let success_rate =
            metrics.iter().filter(|m| m.functionality.success).count() as f64 / count as f64;
        let avg_memory_usage = metrics
            .iter()
            .map(|m| m.performance.memory_usage_bytes)
            .sum::<u64>() as f64
            / count as f64;
        let total_security_incidents = metrics
            .iter()
            .map(|m| m.security.incidents_detected)
            .sum::<u64>();
        let avg_satisfaction = metrics
            .iter()
            .map(|m| m.user_experience.satisfaction_score)
            .sum::<f64>()
            / count as f64;

        AggregatedMetrics {
            window_start,
            window_end: metrics.last().map(|m| m.timestamp).unwrap_or(window_start),
            count,
            avg_generation_time,
            success_rate,
            avg_memory_usage,
            total_security_incidents,
            avg_satisfaction,
        }
    }

    fn estimate_memory_usage(&self) -> u64 {
        // Rough estimate based on history size
        let base_memory = 1024 * 1024; // 1MB base
        let per_metric_memory = 1024; // ~1KB per metric
        base_memory + (self.config.max_history_size as u64 * per_metric_memory)
    }
}

/// Aggregated metrics for a time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub count: usize,
    pub avg_generation_time: f64,
    pub success_rate: f64,
    pub avg_memory_usage: f64,
    pub total_security_incidents: u64,
    pub avg_satisfaction: f64,
}

/// Monitoring statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStats {
    pub total_metrics_recorded: usize,
    pub monitoring_uptime: Option<Duration>,
    pub average_metrics_per_hour: f64,
    pub memory_usage_estimate: u64,
}

/// WebSocket-based real-time metrics streamer
#[derive(Debug)]
pub struct MetricsStreamer {
    monitor: Arc<RealTimeMonitor>,
}

impl MetricsStreamer {
    pub fn new(monitor: Arc<RealTimeMonitor>) -> Self {
        Self { monitor }
    }

    /// Stream metrics in real-time (would integrate with WebSocket in real implementation)
    pub async fn stream_metrics(&self) -> Result<()> {
        // In a real implementation, this would set up WebSocket connections
        // and stream metrics to connected clients
        Ok(())
    }
}
