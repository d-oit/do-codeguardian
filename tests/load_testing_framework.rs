//! Load Testing Framework for CodeGuardian
//!
//! This module provides comprehensive load testing capabilities to validate
//! performance, scalability, and reliability under various stress conditions.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinSet;
use tokio::time::sleep;

/// Load testing configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestConfig {
    /// Total number of concurrent operations
    pub concurrent_operations: usize,
    /// Duration to sustain the load
    pub test_duration: Duration,
    /// Number of operations per second to target
    pub target_ops_per_second: f64,
    /// Maximum number of retries for failed operations
    pub max_retries: usize,
    /// Timeout for individual operations
    pub operation_timeout: Duration,
    /// Whether to ramp up gradually or start at full load
    pub gradual_ramp_up: bool,
    /// Time to ramp up to full load
    pub ramp_up_duration: Duration,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            concurrent_operations: 50,
            test_duration: Duration::from_secs(60),
            target_ops_per_second: 10.0,
            max_retries: 3,
            operation_timeout: Duration::from_secs(30),
            gradual_ramp_up: true,
            ramp_up_duration: Duration::from_secs(10),
        }
    }
}

/// Load testing metrics collector
#[derive(Debug, Default)]
pub struct LoadTestMetrics {
    /// Total operations attempted
    pub operations_attempted: AtomicU64,
    /// Total operations completed successfully
    pub operations_successful: AtomicU64,
    /// Total operations failed
    pub operations_failed: AtomicU64,
    /// Total response time in milliseconds
    pub total_response_time_ms: AtomicU64,
    /// Peak memory usage in bytes
    pub peak_memory_usage: AtomicU64,
    /// Number of retries performed
    pub retries_performed: AtomicU64,
    /// Start time of the test
    pub test_start_time: RwLock<Option<Instant>>,
    /// End time of the test
    pub test_end_time: RwLock<Option<Instant>>,
}

impl LoadTestMetrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn start_test(&self) {
        *self.test_start_time.write().await = Some(Instant::now());
    }

    pub async fn end_test(&self) {
        *self.test_end_time.write().await = Some(Instant::now());
    }

    pub fn record_operation_start(&self) {
        self.operations_attempted.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_operation_success(&self, response_time_ms: u64) {
        self.operations_successful.fetch_add(1, Ordering::Relaxed);
        self.total_response_time_ms
            .fetch_add(response_time_ms, Ordering::Relaxed);
    }

    pub fn record_operation_failure(&self) {
        self.operations_failed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_retry(&self) {
        self.retries_performed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn update_peak_memory(&self, memory_usage: u64) {
        let current_peak = self.peak_memory_usage.load(Ordering::Relaxed);
        if memory_usage > current_peak {
            self.peak_memory_usage
                .compare_exchange_weak(
                    current_peak,
                    memory_usage,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .ok();
        }
    }

    pub async fn generate_report(&self) -> LoadTestReport {
        let start_time = self.test_start_time.read().await;
        let end_time = self.test_end_time.read().await;

        let test_duration = match (*start_time, *end_time) {
            (Some(start), Some(end)) => end.duration_since(start),
            _ => Duration::from_secs(0),
        };

        let attempted = self.operations_attempted.load(Ordering::Relaxed);
        let successful = self.operations_successful.load(Ordering::Relaxed);
        let failed = self.operations_failed.load(Ordering::Relaxed);
        let total_response_time = self.total_response_time_ms.load(Ordering::Relaxed);

        LoadTestReport {
            test_duration,
            operations_attempted: attempted,
            operations_successful: successful,
            operations_failed: failed,
            success_rate: if attempted > 0 {
                successful as f64 / attempted as f64
            } else {
                0.0
            },
            average_response_time_ms: if successful > 0 {
                total_response_time as f64 / successful as f64
            } else {
                0.0
            },
            operations_per_second: if test_duration.as_secs() > 0 {
                successful as f64 / test_duration.as_secs() as f64
            } else {
                0.0
            },
            peak_memory_usage_mb: self.peak_memory_usage.load(Ordering::Relaxed) as f64
                / 1_000_000.0,
            retries_performed: self.retries_performed.load(Ordering::Relaxed),
        }
    }
}

/// Load testing report with comprehensive metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadTestReport {
    pub test_duration: Duration,
    pub operations_attempted: u64,
    pub operations_successful: u64,
    pub operations_failed: u64,
    pub success_rate: f64,
    pub average_response_time_ms: f64,
    pub operations_per_second: f64,
    pub peak_memory_usage_mb: f64,
    pub retries_performed: u64,
}

impl LoadTestReport {
    pub fn meets_performance_criteria(&self) -> bool {
        self.success_rate >= 0.999 && // 99.9% success rate
        self.average_response_time_ms <= 30_000.0 && // <30s average response time
        self.peak_memory_usage_mb <= 100.0 // <100MB peak memory usage
    }

    pub fn print_summary(&self) {
        println!("\n=== Load Test Report ===");
        println!("Test Duration: {:?}", self.test_duration);
        println!("Operations Attempted: {}", self.operations_attempted);
        println!("Operations Successful: {}", self.operations_successful);
        println!("Operations Failed: {}", self.operations_failed);
        println!("Success Rate: {:.2}%", self.success_rate * 100.0);
        println!(
            "Average Response Time: {:.2}ms",
            self.average_response_time_ms
        );
        println!("Operations per Second: {:.2}", self.operations_per_second);
        println!("Peak Memory Usage: {:.2}MB", self.peak_memory_usage_mb);
        println!("Retries Performed: {}", self.retries_performed);
        println!(
            "Performance Criteria Met: {}",
            self.meets_performance_criteria()
        );
        println!("========================\n");
    }
}

/// Generic load testing framework
pub struct LoadTestFramework {
    config: LoadTestConfig,
    metrics: Arc<LoadTestMetrics>,
}

impl LoadTestFramework {
    pub fn new(config: LoadTestConfig) -> Self {
        Self {
            config,
            metrics: LoadTestMetrics::new(),
        }
    }

    /// Execute a load test with the provided operation
    pub async fn execute_load_test<F, Fut, T, E>(
        &self,
        operation: F,
    ) -> anyhow::Result<LoadTestReport>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<T, E>> + Send,
        T: Send + 'static,
        E: Send + 'static,
    {
        self.metrics.start_test().await;

        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_operations));
        let operation = Arc::new(operation);
        let mut join_set = JoinSet::new();

        let test_start = Instant::now();
        let test_end = test_start + self.config.test_duration;

        // Rate limiting calculation
        let operations_per_second = self.config.target_ops_per_second;
        let operation_interval = Duration::from_secs_f64(1.0 / operations_per_second);

        // Spawn operation tasks
        while Instant::now() < test_end {
            let permit = semaphore.clone().acquire_owned().await?;
            let operation_clone = operation.clone();
            let metrics_clone = self.metrics.clone();
            let config_clone = self.config.clone();

            join_set.spawn(async move {
                let _permit = permit;
                Self::execute_single_operation(operation_clone, metrics_clone, config_clone).await;
            });

            // Rate limiting
            if operations_per_second > 0.0 {
                sleep(operation_interval).await;
            }
        }

        // Wait for all operations to complete
        while let Some(_) = join_set.join_next().await {}

        self.metrics.end_test().await;
        Ok(self.metrics.generate_report().await)
    }

    async fn execute_single_operation<F, Fut, T, E>(
        operation: Arc<F>,
        metrics: Arc<LoadTestMetrics>,
        config: LoadTestConfig,
    ) where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Send,
    {
        metrics.record_operation_start();

        let start_time = Instant::now();
        let mut retries = 0;

        loop {
            match tokio::time::timeout(config.operation_timeout, operation()).await {
                Ok(Ok(_)) => {
                    let response_time = start_time.elapsed().as_millis() as u64;
                    metrics.record_operation_success(response_time);
                    break;
                }
                Ok(Err(_)) | Err(_) => {
                    if retries < config.max_retries {
                        retries += 1;
                        metrics.record_retry();
                        sleep(Duration::from_millis(100 * retries as u64)).await;
                    // Exponential backoff
                    } else {
                        metrics.record_operation_failure();
                        break;
                    }
                }
            }
        }

        // Update memory usage (simplified - in real implementation would use proper memory profiling)
        if let Ok(memory_info) = std::process::Command::new("ps")
            .args(&["-o", "rss=", "-p"])
            .arg(std::process::id().to_string())
            .output()
        {
            if let Ok(memory_str) = String::from_utf8(memory_info.stdout) {
                if let Ok(memory_kb) = memory_str.trim().parse::<u64>() {
                    metrics.update_peak_memory(memory_kb * 1024); // Convert KB to bytes
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_test_framework_basic() {
        let config = LoadTestConfig {
            concurrent_operations: 5,
            test_duration: Duration::from_secs(2),
            target_ops_per_second: 2.0,
            ..Default::default()
        };

        let framework = LoadTestFramework::new(config);

        let report = framework
            .execute_load_test(|| async {
                sleep(Duration::from_millis(100)).await;
                Ok::<(), anyhow::Error>(())
            })
            .await
            .expect("Failed to execute load test");

        assert!(report.operations_attempted > 0);
        assert!(report.success_rate > 0.0);
        assert!(report.test_duration >= Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_load_test_metrics() {
        let metrics = LoadTestMetrics::new();

        metrics.start_test().await;
        metrics.record_operation_start();
        metrics.record_operation_success(100);
        metrics.record_operation_start();
        metrics.record_operation_failure();
        metrics.record_retry();
        metrics.update_peak_memory(50_000_000); // 50MB
        metrics.end_test().await;

        let report = metrics.generate_report().await;

        assert_eq!(report.operations_attempted, 2);
        assert_eq!(report.operations_successful, 1);
        assert_eq!(report.operations_failed, 1);
        assert_eq!(report.success_rate, 0.5);
        assert_eq!(report.average_response_time_ms, 100.0);
        assert_eq!(report.retries_performed, 1);
        assert_eq!(report.peak_memory_usage_mb, 50.0);
    }
}
