//! Performance optimization module for CodeGuardian
//!
//! This module provides performance monitoring, optimization utilities,
//! and profiling tools to ensure CodeGuardian maintains high performance.

pub mod memory_pool;
pub mod optimizer;
pub mod profiler;

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Performance metrics collector
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub total_files_processed: AtomicUsize,
    pub total_processing_time: AtomicU64, // nanoseconds
    pub cache_hits: AtomicUsize,
    pub cache_misses: AtomicUsize,
    pub memory_peak_usage: AtomicUsize, // bytes
    pub parallel_efficiency: AtomicU64, // percentage * 100
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_file_processed(&self, duration: Duration) {
        self.total_files_processed.fetch_add(1, Ordering::Relaxed);
        self.total_processing_time
            .fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn update_memory_usage(&self, bytes: usize) {
        self.memory_peak_usage.fetch_max(bytes, Ordering::Relaxed);
    }

    pub fn get_average_processing_time(&self) -> Duration {
        let total_files = self.total_files_processed.load(Ordering::Relaxed);
        let total_time = self.total_processing_time.load(Ordering::Relaxed);

        if total_files == 0 {
            Duration::from_nanos(0)
        } else {
            Duration::from_nanos(total_time / total_files as u64)
        }
    }

    pub fn get_cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;

        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    pub fn get_throughput_files_per_second(&self) -> f64 {
        let total_files = self.total_files_processed.load(Ordering::Relaxed);
        let total_time_secs =
            self.total_processing_time.load(Ordering::Relaxed) as f64 / 1_000_000_000.0;

        if total_time_secs == 0.0 {
            0.0
        } else {
            total_files as f64 / total_time_secs
        }
    }
}

/// Performance profiler for tracking operation timings
pub struct PerformanceProfiler {
    metrics: Arc<PerformanceMetrics>,
    start_time: Instant,
}

impl PerformanceProfiler {
    pub fn new(metrics: Arc<PerformanceMetrics>) -> Self {
        Self {
            metrics,
            start_time: Instant::now(),
        }
    }

    pub fn time_operation<F, R>(&self, operation: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        (result, duration)
    }

    pub fn profile_file_analysis<F, R>(&self, file_operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        let (result, duration) = self.time_operation(file_operation);
        self.metrics.record_file_processed(duration);
        result
    }
}

/// Performance optimization recommendations
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: OptimizationCategory,
    pub priority: Priority,
    pub description: String,
    pub estimated_improvement: String,
    pub implementation_effort: ImplementationEffort,
}

#[derive(Debug, Clone)]
pub enum OptimizationCategory {
    Memory,
    CPU,
    IO,
    Caching,
    Parallelization,
    Algorithm,
}

#[derive(Debug, Clone)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
}

/// Performance analyzer that provides optimization recommendations
pub struct PerformanceAnalyzer {
    metrics: Arc<PerformanceMetrics>,
}

impl PerformanceAnalyzer {
    pub fn new(metrics: Arc<PerformanceMetrics>) -> Self {
        Self { metrics }
    }

    pub fn analyze_performance(&self) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();

        // Analyze cache performance
        let cache_hit_rate = self.metrics.get_cache_hit_rate();
        if cache_hit_rate < 0.8 {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Caching,
                priority: Priority::High,
                description: format!(
                    "Cache hit rate is {:.1}%. Consider improving cache strategy.",
                    cache_hit_rate * 100.0
                ),
                estimated_improvement: "20-40% faster analysis".to_string(),
                implementation_effort: ImplementationEffort::Medium,
            });
        }

        // Analyze processing speed
        let avg_time = self.metrics.get_average_processing_time();
        if avg_time > Duration::from_millis(100) {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::CPU,
                priority: Priority::Medium,
                description: format!(
                    "Average file processing time is {:.1}ms. Consider algorithm optimization.",
                    avg_time.as_millis()
                ),
                estimated_improvement: "10-30% faster processing".to_string(),
                implementation_effort: ImplementationEffort::High,
            });
        }

        // Analyze memory usage
        let peak_memory = self.metrics.memory_peak_usage.load(Ordering::Relaxed);
        if peak_memory > 100 * 1024 * 1024 {
            // 100MB
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Memory,
                priority: Priority::Medium,
                description: format!(
                    "Peak memory usage is {:.1}MB. Consider streaming processing.",
                    peak_memory as f64 / (1024.0 * 1024.0)
                ),
                estimated_improvement: "50-70% less memory usage".to_string(),
                implementation_effort: ImplementationEffort::High,
            });
        }

        recommendations
    }

    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# CodeGuardian Performance Report\n\n");

        // Basic metrics
        report.push_str("## Performance Metrics\n\n");
        report.push_str(&format!(
            "- **Files Processed**: {}\n",
            self.metrics.total_files_processed.load(Ordering::Relaxed)
        ));
        report.push_str(&format!(
            "- **Average Processing Time**: {:.2}ms per file\n",
            self.metrics.get_average_processing_time().as_millis()
        ));
        report.push_str(&format!(
            "- **Throughput**: {:.1} files/second\n",
            self.metrics.get_throughput_files_per_second()
        ));
        report.push_str(&format!(
            "- **Cache Hit Rate**: {:.1}%\n",
            self.metrics.get_cache_hit_rate() * 100.0
        ));
        report.push_str(&format!(
            "- **Peak Memory Usage**: {:.1}MB\n\n",
            self.metrics.memory_peak_usage.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0)
        ));

        // Optimization recommendations
        let recommendations = self.analyze_performance();
        if !recommendations.is_empty() {
            report.push_str("## Optimization Recommendations\n\n");
            for (i, rec) in recommendations.iter().enumerate() {
                report.push_str(&format!(
                    "{}. **{:?} - {:?} Priority**\n",
                    i + 1,
                    rec.category,
                    rec.priority
                ));
                report.push_str(&format!("   - {}\n", rec.description));
                report.push_str(&format!(
                    "   - Estimated improvement: {}\n",
                    rec.estimated_improvement
                ));
                report.push_str(&format!(
                    "   - Implementation effort: {:?}\n\n",
                    rec.implementation_effort
                ));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics::new();

        // Record some operations
        metrics.record_file_processed(Duration::from_millis(50));
        metrics.record_file_processed(Duration::from_millis(30));
        metrics.record_cache_hit();
        metrics.record_cache_miss();

        // Check metrics
        assert_eq!(metrics.total_files_processed.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.cache_hits.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.cache_misses.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.get_cache_hit_rate(), 0.5);
    }

    #[test]
    fn test_performance_profiler() {
        let metrics = Arc::new(PerformanceMetrics::new());
        let profiler = PerformanceProfiler::new(Arc::clone(&metrics));

        // Profile an operation
        let result = profiler.profile_file_analysis(|| {
            thread::sleep(Duration::from_millis(10));
            42
        });

        assert_eq!(result, 42);
        assert_eq!(metrics.total_files_processed.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_performance_analyzer() {
        let metrics = Arc::new(PerformanceMetrics::new());
        let analyzer = PerformanceAnalyzer::new(Arc::clone(&metrics));

        // Simulate poor performance
        metrics.record_file_processed(Duration::from_millis(200));
        metrics.record_cache_miss();
        metrics.record_cache_miss();
        metrics.update_memory_usage(200 * 1024 * 1024); // 200MB

        let recommendations = analyzer.analyze_performance();
        assert!(!recommendations.is_empty());

        let report = analyzer.generate_performance_report();
        assert!(report.contains("Performance Report"));
        assert!(report.contains("Optimization Recommendations"));
    }
}
