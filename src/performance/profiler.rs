//! Performance profiler for CodeGuardian
//!
//! Provides detailed performance profiling and bottleneck identification

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Detailed performance profiler with operation tracking
pub struct DetailedProfiler {
    operations: HashMap<String, OperationStats>,
    current_operation: Option<(String, Instant)>,
}

#[derive(Debug, Clone)]
pub struct OperationStats {
    pub total_time: Duration,
    pub call_count: u64,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl DetailedProfiler {
    pub fn new() -> Self {
        Self {
            operations: HashMap::new(),
            current_operation: None,
        }
    }

    pub fn start_operation(&mut self, name: &str) {
        self.current_operation = Some((name.to_string(), Instant::now()));
    }

    pub fn end_operation(&mut self) {
        if let Some((name, start_time)) = self.current_operation.take() {
            let duration = start_time.elapsed();
            self.record_operation(&name, duration);
        }
    }

    pub fn time_operation<F, R>(&mut self, name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        self.record_operation(name, duration);
        result
    }

    fn record_operation(&mut self, name: &str, duration: Duration) {
        let stats = self
            .operations
            .entry(name.to_string())
            .or_insert(OperationStats {
                total_time: Duration::from_nanos(0),
                call_count: 0,
                min_time: Duration::from_secs(u64::MAX),
                max_time: Duration::from_nanos(0),
            });

        stats.total_time += duration;
        stats.call_count += 1;
        stats.min_time = stats.min_time.min(duration);
        stats.max_time = stats.max_time.max(duration);
    }

    pub fn get_stats(&self, operation: &str) -> Option<&OperationStats> {
        self.operations.get(operation)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Performance Profile Report\n\n");

        let mut sorted_ops: Vec<_> = self.operations.iter().collect();
        sorted_ops.sort_by(|a, b| b.1.total_time.cmp(&a.1.total_time));

        report.push_str("| Operation | Total Time | Calls | Avg Time | Min Time | Max Time |\n");
        report.push_str("|-----------|------------|-------|----------|----------|----------|\n");

        for (name, stats) in sorted_ops {
            let avg_time = stats.total_time / stats.call_count as u32;
            report.push_str(&format!(
                "| {} | {:.2}ms | {} | {:.2}ms | {:.2}ms | {:.2}ms |\n",
                name,
                stats.total_time.as_millis(),
                stats.call_count,
                avg_time.as_millis(),
                stats.min_time.as_millis(),
                stats.max_time.as_millis()
            ));
        }

        report
    }

    pub fn identify_bottlenecks(&self) -> Vec<String> {
        let mut bottlenecks = Vec::new();
        let total_time: Duration = self.operations.values().map(|s| s.total_time).sum();

        for (name, stats) in &self.operations {
            let percentage =
                (stats.total_time.as_millis() as f64 / total_time.as_millis() as f64) * 100.0;
            if percentage > 20.0 {
                bottlenecks.push(format!(
                    "{}: {:.1}% of total time ({:.2}ms)",
                    name,
                    percentage,
                    stats.total_time.as_millis()
                ));
            }
        }

        bottlenecks
    }
}

impl Default for DetailedProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationStats {
    pub fn average_time(&self) -> Duration {
        if self.call_count == 0 {
            Duration::from_nanos(0)
        } else {
            self.total_time / self.call_count as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_profiler_basic_operations() {
        let mut profiler = DetailedProfiler::new();

        profiler.time_operation("test_op", || {
            thread::sleep(Duration::from_millis(10));
        });

        let stats = profiler.get_stats("test_op").unwrap();
        assert_eq!(stats.call_count, 1);
        assert!(stats.total_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_profiler_report_generation() {
        let mut profiler = DetailedProfiler::new();

        profiler.time_operation("fast_op", || {
            thread::sleep(Duration::from_millis(1));
        });

        profiler.time_operation("slow_op", || {
            thread::sleep(Duration::from_millis(5));
        });

        let report = profiler.generate_report();
        assert!(report.contains("Performance Profile Report"));
        assert!(report.contains("fast_op"));
        assert!(report.contains("slow_op"));
    }

    #[test]
    fn test_bottleneck_identification() {
        let mut profiler = DetailedProfiler::new();

        // Create a clear bottleneck
        for _ in 0..10 {
            profiler.time_operation("bottleneck", || {
                thread::sleep(Duration::from_millis(2));
            });
        }

        profiler.time_operation("fast", || {
            thread::sleep(Duration::from_millis(1));
        });

        let bottlenecks = profiler.identify_bottlenecks();
        assert!(!bottlenecks.is_empty());
        assert!(bottlenecks[0].contains("bottleneck"));
    }
}
