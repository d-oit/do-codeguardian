//! Performance monitoring and metrics collection
//!
//! This module provides comprehensive performance monitoring capabilities
//! including metrics collection, alerting, and real-time dashboard functionality.

#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Performance metrics for monitoring optimization effectiveness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp_ms: u64,
    pub scan_duration: Duration,
    pub files_processed: usize,
    pub total_findings: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub parallel_workers: usize,
    pub streaming_operations: usize,
    pub memory_pool_stats: MemoryPoolStats,
    pub adaptive_parallelism_stats: AdaptiveParallelismStats,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryPoolStats {
    pub string_buffers_active: usize,
    pub findings_vectors_active: usize,
    pub total_allocations: usize,
    pub total_deallocations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveParallelismStats {
    pub current_workers: usize,
    pub load_score: f64,
    pub adjustments_made: usize,
    pub avg_load_score: f64,
}

impl Default for AdaptiveParallelismStats {
    fn default() -> Self {
        Self {
            current_workers: 0,
            load_score: 0.0,
            adjustments_made: 0,
            avg_load_score: 0.0,
        }
    }
}

/// Performance monitor for tracking optimization effectiveness
pub struct PerformanceMonitor {
    metrics_history: Arc<Mutex<Vec<PerformanceMetrics>>>,
    max_history_size: usize,
    start_time: Instant,
    alerts: Arc<Mutex<Vec<PerformanceAlert>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub timestamp_ms: u64,
    pub alert_type: AlertType,
    pub message: String,
    pub severity: AlertSeverity,
    pub metric_value: f64,
    pub threshold_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighMemoryUsage,
    HighCpuUsage,
    SlowScanPerformance,
    LowCacheHitRate,
    HighErrorRate,
    MemoryPoolExhaustion,
    ParallelismInefficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl PerformanceMonitor {
    pub fn new(max_history_size: usize) -> Self {
        Self {
            metrics_history: Arc::new(Mutex::new(Vec::with_capacity(max_history_size))),
            max_history_size,
            start_time: Instant::now(),
            alerts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Record performance metrics
    pub fn record_metrics(&self, mut metrics: PerformanceMetrics) {
        let mut history = self.metrics_history.lock().unwrap();

        // Set timestamp
        metrics.timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Add new metrics
        history.push(metrics);

        // Maintain history size limit
        if history.len() > self.max_history_size {
            history.remove(0);
        }

        // Check for performance issues
        self.check_performance_thresholds(&history);
    }

    /// Get current performance summary
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        let history = self.metrics_history.lock().unwrap();

        if history.is_empty() {
            return PerformanceSummary::default();
        }

        let _latest = &history[history.len() - 1];
        let avg_scan_duration =
            history.iter().map(|m| m.scan_duration).sum::<Duration>() / history.len() as u32;

        let avg_files_per_second = if !history.is_empty() {
            let total_files: usize = history.iter().map(|m| m.files_processed).sum();
            let total_duration: Duration = history.iter().map(|m| m.scan_duration).sum();
            total_files as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        };

        let avg_cache_hit_rate = if !history.is_empty() {
            let total_hits: usize = history.iter().map(|m| m.cache_hits).sum();
            let total_requests: usize = history.iter().map(|m| m.cache_hits + m.cache_misses).sum();
            if total_requests > 0 {
                total_hits as f64 / total_requests as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        PerformanceSummary {
            total_scans: history.len(),
            average_scan_duration: avg_scan_duration,
            average_files_per_second: avg_files_per_second,
            average_cache_hit_rate: avg_cache_hit_rate,
            peak_memory_usage: history
                .iter()
                .map(|m| m.memory_usage_mb)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            peak_cpu_usage: history
                .iter()
                .map(|m| m.cpu_usage_percent)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            total_findings: history.iter().map(|m| m.total_findings).sum(),
            uptime: self.start_time.elapsed(),
        }
    }

    /// Get recent alerts
    pub fn get_recent_alerts(&self, limit: usize) -> Vec<PerformanceAlert> {
        let alerts = self.alerts.lock().unwrap();
        alerts.iter().rev().take(limit).cloned().collect()
    }

    /// Export metrics to JSON
    pub fn export_metrics_json(&self) -> Result<String, serde_json::Error> {
        let history = self.metrics_history.lock().unwrap();
        let summary = self.get_performance_summary();
        let alerts = self.alerts.lock().unwrap();

        let export_data = PerformanceExport {
            summary,
            metrics: history.clone(),
            alerts: alerts.clone(),
            export_timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };

        serde_json::to_string_pretty(&export_data)
    }

    /// Check performance thresholds and generate alerts
    fn check_performance_thresholds(&self, history: &[PerformanceMetrics]) {
        if history.is_empty() {
            return;
        }

        let latest = &history[history.len() - 1];

        // Memory usage alert
        if latest.memory_usage_mb > 1000.0 {
            // 1GB threshold
            self.add_alert(PerformanceAlert {
                timestamp_ms: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                alert_type: AlertType::HighMemoryUsage,
                message: format!("High memory usage: {:.1}MB", latest.memory_usage_mb),
                severity: AlertSeverity::High,
                metric_value: latest.memory_usage_mb,
                threshold_value: 1000.0,
            });
        }

        // CPU usage alert
        if latest.cpu_usage_percent > 90.0 {
            self.add_alert(PerformanceAlert {
                timestamp_ms: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                alert_type: AlertType::HighCpuUsage,
                message: format!("High CPU usage: {:.1}%", latest.cpu_usage_percent),
                severity: AlertSeverity::Medium,
                metric_value: latest.cpu_usage_percent,
                threshold_value: 90.0,
            });
        }

        // Slow scan performance alert
        if latest.scan_duration > Duration::from_secs(60) {
            self.add_alert(PerformanceAlert {
                timestamp_ms: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                alert_type: AlertType::SlowScanPerformance,
                message: format!("Slow scan performance: {:?}", latest.scan_duration),
                severity: AlertSeverity::Medium,
                metric_value: latest.scan_duration.as_secs_f64(),
                threshold_value: 60.0,
            });
        }

        // Low cache hit rate alert
        let cache_hit_rate = if latest.cache_hits + latest.cache_misses > 0 {
            latest.cache_hits as f64 / (latest.cache_hits + latest.cache_misses) as f64
        } else {
            0.0
        };

        if cache_hit_rate < 0.5 && latest.files_processed > 10 {
            self.add_alert(PerformanceAlert {
                timestamp_ms: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                alert_type: AlertType::LowCacheHitRate,
                message: format!("Low cache hit rate: {:.1}%", cache_hit_rate * 100.0),
                severity: AlertSeverity::Low,
                metric_value: cache_hit_rate,
                threshold_value: 0.5,
            });
        }
    }

    fn add_alert(&self, alert: PerformanceAlert) {
        let mut alerts = self.alerts.lock().unwrap();
        alerts.push(alert);

        // Keep only recent alerts (last 100)
        if alerts.len() > 100 {
            alerts.remove(0);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_scans: usize,
    pub average_scan_duration: Duration,
    pub average_files_per_second: f64,
    pub average_cache_hit_rate: f64,
    pub peak_memory_usage: f64,
    pub peak_cpu_usage: f64,
    pub total_findings: usize,
    pub uptime: Duration,
}

impl Default for PerformanceSummary {
    fn default() -> Self {
        Self {
            total_scans: 0,
            average_scan_duration: Duration::from_secs(0),
            average_files_per_second: 0.0,
            average_cache_hit_rate: 0.0,
            peak_memory_usage: 0.0,
            peak_cpu_usage: 0.0,
            total_findings: 0,
            uptime: Duration::from_secs(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceExport {
    pub summary: PerformanceSummary,
    pub metrics: Vec<PerformanceMetrics>,
    pub alerts: Vec<PerformanceAlert>,
    pub export_timestamp_ms: u64,
}

/// Performance dashboard for real-time monitoring
pub struct PerformanceDashboard {
    monitor: Arc<PerformanceMonitor>,
    update_interval: Duration,
    enabled: Arc<Mutex<bool>>,
}

impl PerformanceDashboard {
    pub fn new(monitor: Arc<PerformanceMonitor>) -> Self {
        Self {
            monitor,
            update_interval: Duration::from_secs(5),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Start the performance dashboard
    pub async fn start(&self) {
        let monitor = Arc::clone(&self.monitor);
        let enabled = Arc::clone(&self.enabled);
        let interval = self.update_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if !*enabled.lock().unwrap() {
                    break;
                }

                let summary = monitor.get_performance_summary();
                let alerts = monitor.get_recent_alerts(5);

                Self::display_dashboard(&summary, &alerts);
            }
        });
    }

    /// Stop the performance dashboard
    pub fn stop(&self) {
        *self.enabled.lock().unwrap() = false;
    }

    fn display_dashboard(summary: &PerformanceSummary, alerts: &[PerformanceAlert]) {
        println!("\n=== Performance Dashboard ===");
        println!("Uptime: {:?}", summary.uptime);
        println!("Total Scans: {}", summary.total_scans);
        println!("Avg Scan Duration: {:?}", summary.average_scan_duration);
        println!("Avg Files/Second: {:.1}", summary.average_files_per_second);
        println!(
            "Avg Cache Hit Rate: {:.1}%",
            summary.average_cache_hit_rate * 100.0
        );
        println!("Peak Memory Usage: {:.1}MB", summary.peak_memory_usage);
        println!("Peak CPU Usage: {:.1}%", summary.peak_cpu_usage);
        println!("Total Findings: {}", summary.total_findings);

        if !alerts.is_empty() {
            println!("\n--- Recent Alerts ---");
            for alert in alerts.iter().rev().take(3) {
                let severity_icon = match alert.severity {
                    AlertSeverity::Low => "ℹ️",
                    AlertSeverity::Medium => "⚠️",
                    AlertSeverity::High => "🔴",
                    AlertSeverity::Critical => "🚨",
                };
                println!("{} {}", severity_icon, alert.message);
            }
        }
        println!("========================\n");
    }
}

/// Helper function to create a performance monitor
pub fn create_performance_monitor() -> Arc<PerformanceMonitor> {
    Arc::new(PerformanceMonitor::new(1000)) // Keep last 1000 metrics
}

/// Helper function to create a performance dashboard
pub fn create_performance_dashboard(monitor: Arc<PerformanceMonitor>) -> PerformanceDashboard {
    PerformanceDashboard::new(monitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new(10);

        let metrics = PerformanceMetrics {
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            scan_duration: Duration::from_secs(5),
            files_processed: 100,
            total_findings: 50,
            cache_hits: 80,
            cache_misses: 20,
            memory_usage_mb: 256.0,
            cpu_usage_percent: 45.0,
            parallel_workers: 4,
            streaming_operations: 10,
            memory_pool_stats: MemoryPoolStats::default(),
            adaptive_parallelism_stats: AdaptiveParallelismStats::default(),
        };

        monitor.record_metrics(metrics);

        let summary = monitor.get_performance_summary();
        assert_eq!(summary.total_scans, 1);
        assert_eq!(summary.total_findings, 50);
        assert_eq!(summary.average_files_per_second, 20.0);
    }

    #[test]
    fn test_performance_alerts() {
        let monitor = PerformanceMonitor::new(10);

        let metrics = PerformanceMetrics {
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            scan_duration: Duration::from_secs(120), // Over threshold
            files_processed: 100,
            total_findings: 50,
            cache_hits: 80,
            cache_misses: 20,
            memory_usage_mb: 256.0,
            cpu_usage_percent: 45.0,
            parallel_workers: 4,
            streaming_operations: 10,
            memory_pool_stats: MemoryPoolStats::default(),
            adaptive_parallelism_stats: AdaptiveParallelismStats::default(),
        };

        monitor.record_metrics(metrics);

        let alerts = monitor.get_recent_alerts(10);
        assert!(!alerts.is_empty());
        assert!(matches!(
            alerts[0].alert_type,
            AlertType::SlowScanPerformance
        ));
    }

    #[test]
    fn test_performance_dashboard() {
        let monitor = Arc::new(PerformanceMonitor::new(10));
        let dashboard = PerformanceDashboard::new(monitor);

        // Test that dashboard can be created and stopped
        dashboard.stop();
        assert!(!*dashboard.enabled.lock().unwrap());
    }
}
