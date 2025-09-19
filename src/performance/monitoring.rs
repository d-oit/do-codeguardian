use crate::performance::{PerformanceMetrics, PerformanceAnalyzer};
use crate::dashboard::{DashboardService, DashboardMetrics, PerformanceMetrics as DashboardPerfMetrics, SystemHealth};
use crate::cache::regex_cache::SharedRegexCache;
use crate::performance::memory_pool::GlobalMemoryPools;
use anyhow::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};

/// Production monitoring service for performance optimizations
pub struct ProductionMonitoringService {
    metrics: Arc<PerformanceMetrics>,
    analyzer: PerformanceAnalyzer,
    dashboard: Option<Arc<DashboardService>>,
    regex_cache: Option<SharedRegexCache>,
    memory_pools: Option<Arc<GlobalMemoryPools>>,
    last_report_time: Instant,
    report_interval: Duration,
}

impl ProductionMonitoringService {
    /// Create a new monitoring service
    pub fn new(
        metrics: Arc<PerformanceMetrics>,
        dashboard: Option<Arc<DashboardService>>,
        regex_cache: Option<SharedRegexCache>,
        memory_pools: Option<Arc<GlobalMemoryPools>>,
    ) -> Self {
        Self {
            analyzer: PerformanceAnalyzer::new(Arc::clone(&metrics)),
            dashboard,
            regex_cache,
            memory_pools,
            metrics,
            last_report_time: Instant::now(),
            report_interval: Duration::from_secs(60), // Report every minute
        }
    }

    /// Record regex cache performance metrics
    pub fn record_regex_metrics(&self) {
        if let Some(cache) = &self.regex_cache {
            let stats = cache.stats();

            // Record compilation time savings
            let compilation_time_saved = stats.total_compilation_time_saved();
            if compilation_time_saved > 0 {
                self.metrics.record_optimization_time_saved(Duration::from_millis(compilation_time_saved));
            }

            // Update cache hit rate
            let hit_rate = stats.hit_rate();
            if hit_rate > 0.8 {
                self.metrics.record_cache_optimization_hit();
            }
        }
    }

    /// Record memory pool performance metrics
    pub fn record_memory_metrics(&self) {
        if let Some(pools) = &self.memory_pools {
            let stats = pools.memory_stats();

            // Update pool utilization
            let string_utilization = stats.string_pool.utilization_percentage();
            self.metrics.update_string_pool_utilization(string_utilization as usize);

            // Record reuse rate
            let reuse_rate = self.metrics.get_memory_pool_reuse_rate();
            if reuse_rate > 0.7 {
                self.metrics.record_memory_optimization_hit();
            }
        }
    }

    /// Generate comprehensive performance report
    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# CodeGuardian Production Performance Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now()));

        // Core performance metrics
        report.push_str("## Core Performance Metrics\n\n");
        report.push_str(&format!("- Files Processed: {}\n", self.metrics.total_files_processed.load(std::sync::atomic::Ordering::Relaxed)));
        report.push_str(&format!("- Average Processing Time: {:.2}ms\n", self.metrics.get_average_processing_time().as_millis()));
        report.push_str(&format!("- Cache Hit Rate: {:.1}%\n", self.metrics.get_cache_hit_rate() * 100.0));
        report.push_str(&format!("- Throughput: {:.1} files/sec\n", self.metrics.get_throughput_files_per_second()));

        // Regex cache metrics
        report.push_str("\n## Regex Cache Performance\n\n");
        if let Some(cache) = &self.regex_cache {
            let stats = cache.stats();
            report.push_str(&format!("- Cache Hit Rate: {:.1}%\n", stats.hit_rate() * 100.0));
            report.push_str(&format!("- Total Compilations: {}\n", stats.cache_misses));
            report.push_str(&format!("- Average Compilation Time: {:.1}ms\n", stats.average_compilation_time()));
            report.push_str(&format!("- Time Saved: {:.1}s\n", stats.total_compilation_time_saved() as f64 / 1000.0));
        } else {
            report.push_str("- Regex cache metrics not available\n");
        }

        // Memory pool metrics
        report.push_str("\n## Memory Pool Performance\n\n");
        if let Some(pools) = &self.memory_pools {
            let stats = pools.memory_stats();
            report.push_str(&format!("- String Pool Utilization: {:.1}%\n", stats.string_pool.utilization_percentage()));
            report.push_str(&format!("- Memory Reuse Rate: {:.1}%\n", self.metrics.get_memory_pool_reuse_rate() * 100.0));
            report.push_str(&format!("- Memory Pool Allocations: {}\n", self.metrics.memory_pool_allocations.load(std::sync::atomic::Ordering::Relaxed)));
            report.push_str(&format!("- Memory Pool Reuses: {}\n", self.metrics.memory_pool_reuses.load(std::sync::atomic::Ordering::Relaxed)));
        } else {
            report.push_str("- Memory pool metrics not available\n");
        }

        // Optimization metrics
        report.push_str("\n## Optimization Effectiveness\n\n");
        report.push_str(&format!("- Optimizations Applied: {}\n", self.metrics.optimization_applications.load(std::sync::atomic::Ordering::Relaxed)));
        report.push_str(&format!("- Time Saved by Optimizations: {:.1}s\n", self.metrics.optimization_time_saved.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1_000_000_000.0));
        report.push_str(&format!("- Cache Optimization Hits: {}\n", self.metrics.cache_optimization_hits.load(std::sync::atomic::Ordering::Relaxed)));
        report.push_str(&format!("- Memory Optimization Hits: {}\n", self.metrics.memory_optimization_hits.load(std::sync::atomic::Ordering::Relaxed)));

        // Recommendations
        let recommendations = self.analyzer.analyze_performance();
        if !recommendations.is_empty() {
            report.push_str("\n## Performance Recommendations\n\n");
            for (i, rec) in recommendations.iter().enumerate() {
                report.push_str(&format!("{}. **{:?} - {:?} Priority**\n", i + 1, rec.category, rec.priority));
                report.push_str(&format!("   - {}\n", rec.description));
                report.push_str(&format!("   - Estimated improvement: {}\n", rec.estimated_improvement));
                report.push_str(&format!("   - Implementation effort: {:?}\n\n", rec.implementation_effort));
            }
        }

        report
    }

    /// Update dashboard with current metrics
    pub fn update_dashboard(&self) -> Result<()> {
        if let Some(dashboard) = &self.dashboard {
            let dashboard_metrics = DashboardMetrics {
                timestamp: Utc::now(),
                duplicate_stats: Default::default(), // Would be populated from actual duplicate detection
                prevention_stats: Default::default(), // Would be populated from actual prevention logic
                system_health: SystemHealth {
                    api_success_rate: 0.99, // Placeholder
                    average_response_time_ms: self.metrics.get_average_processing_time().as_millis() as f64,
                    error_rate: 0.01, // Placeholder
                    uptime_percentage: 99.9, // Placeholder
                    active_connections: 1, // Placeholder
                },
                performance_metrics: DashboardPerfMetrics {
                    average_processing_time_ms: self.metrics.get_average_processing_time().as_millis() as f64,
                    throughput_per_minute: self.metrics.get_throughput_files_per_second() * 60.0,
                    memory_usage_mb: self.metrics.memory_peak_usage.load(std::sync::atomic::Ordering::Relaxed) as f64 / (1024.0 * 1024.0),
                    cpu_usage_percentage: 0.0, // Would need system monitoring
                    queue_length: 0, // Placeholder
                },
            };

            let mut dashboard_mut = Arc::as_ref(dashboard).clone();
            dashboard_mut.update_metrics(dashboard_metrics);
        }

        Ok(())
    }

    /// Check for performance alerts
    pub fn check_alerts(&self) -> Vec<PerformanceAlert> {
        let mut alerts = Vec::new();

        // Check regex cache hit rate
        if let Some(cache) = &self.regex_cache {
            let hit_rate = cache.stats().hit_rate();
            if hit_rate < 0.7 {
                alerts.push(PerformanceAlert {
                    alert_type: AlertType::CachePerformance,
                    severity: AlertSeverity::Warning,
                    message: format!("Regex cache hit rate is low: {:.1}%", hit_rate * 100.0),
                    threshold: 0.7,
                    current_value: hit_rate,
                });
            }
        }

        // Check memory pool utilization
        let memory_reuse_rate = self.metrics.get_memory_pool_reuse_rate();
        if memory_reuse_rate < 0.5 {
            alerts.push(PerformanceAlert {
                alert_type: AlertType::MemoryEfficiency,
                severity: AlertSeverity::Warning,
                message: format!("Memory pool reuse rate is low: {:.1}%", memory_reuse_rate * 100.0),
                threshold: 0.5,
                current_value: memory_reuse_rate,
            });
        }

        // Check processing time
        let avg_time = self.metrics.get_average_processing_time();
        if avg_time > Duration::from_millis(2000) {
            alerts.push(PerformanceAlert {
                alert_type: AlertType::ProcessingPerformance,
                severity: AlertSeverity::Critical,
                message: format!("Average processing time is high: {:.1}ms", avg_time.as_millis()),
                threshold: 2000.0,
                current_value: avg_time.as_millis() as f64,
            });
        }

        alerts
    }

    /// Periodic monitoring update
    pub fn update(&mut self) -> Result<()> {
        // Record current metrics
        self.record_regex_metrics();
        self.record_memory_metrics();

        // Update dashboard if enough time has passed
        if self.last_report_time.elapsed() >= self.report_interval {
            self.update_dashboard()?;
            self.last_report_time = Instant::now();
        }

        Ok(())
    }
}

/// Performance alert structure
#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub threshold: f64,
    pub current_value: f64,
}

/// Types of performance alerts
#[derive(Debug, Clone)]
pub enum AlertType {
    CachePerformance,
    MemoryEfficiency,
    ProcessingPerformance,
    SystemHealth,
}

/// Alert severity levels
#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}
