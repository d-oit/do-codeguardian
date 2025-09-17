//! # Output Testing Framework
//!
//! This module provides comprehensive testing capabilities for CodeGuardian output systems,
//! including unit tests, integration tests, performance regression tests, and security testing.

pub mod integration_tests;
pub mod performance_tests;
pub mod security_tests;
pub mod unit_tests;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Test configuration for output testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// Enable performance testing
    pub performance_testing: bool,
    /// Enable security testing
    pub security_testing: bool,
    /// Enable load testing
    pub load_testing: bool,
    /// Maximum test duration
    pub max_test_duration: Duration,
    /// Memory usage threshold for tests
    pub memory_threshold_mb: usize,
    /// Performance baseline requirements
    pub performance_baselines: HashMap<String, PerformanceBaseline>,
}

/// Performance baseline for a specific test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
    /// Maximum memory usage in MB
    pub max_memory_usage_mb: usize,
    /// Minimum throughput (operations per second)
    pub min_throughput_ops: f64,
    /// Maximum error rate (percentage)
    pub max_error_rate_percent: f64,
}

/// Test result with metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test name
    pub test_name: String,
    /// Test status
    pub status: TestStatus,
    /// Execution time
    pub execution_time: Duration,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Performance metrics
    pub metrics: TestMetrics,
    /// Test artifacts
    pub artifacts: Vec<TestArtifact>,
}

/// Test execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Timeout,
}

/// Performance metrics collected during testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetrics {
    /// Operations per second
    pub throughput_ops: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// 95th percentile response time
    pub p95_response_time_ms: f64,
    /// 99th percentile response time
    pub p99_response_time_ms: f64,
    /// Error rate as percentage
    pub error_rate_percent: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// I/O operations per second
    pub io_ops_per_second: f64,
}

/// Test artifact (files, logs, reports)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestArtifact {
    /// Artifact name
    pub name: String,
    /// Artifact type
    pub artifact_type: ArtifactType,
    /// File path or content
    pub content: String,
    /// Size in bytes
    pub size_bytes: usize,
}

/// Types of test artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    LogFile,
    OutputFile,
    PerformanceReport,
    SecurityReport,
    MemoryDump,
    CoverageReport,
}

/// Comprehensive test suite for output systems
pub struct OutputTestSuite {
    config: TestConfig,
    results: Vec<TestResult>,
}

impl Default for TestConfig {
    fn default() -> Self {
        let mut baselines = HashMap::new();

        // JSON format baseline
        baselines.insert(
            "json_format".to_string(),
            PerformanceBaseline {
                max_execution_time_ms: 100,
                max_memory_usage_mb: 50,
                min_throughput_ops: 100.0,
                max_error_rate_percent: 0.1,
            },
        );

        // HTML format baseline
        baselines.insert(
            "html_format".to_string(),
            PerformanceBaseline {
                max_execution_time_ms: 200,
                max_memory_usage_mb: 75,
                min_throughput_ops: 50.0,
                max_error_rate_percent: 0.1,
            },
        );

        // Storage operations baseline
        baselines.insert(
            "storage_operations".to_string(),
            PerformanceBaseline {
                max_execution_time_ms: 500,
                max_memory_usage_mb: 100,
                min_throughput_ops: 20.0,
                max_error_rate_percent: 0.05,
            },
        );

        Self {
            performance_testing: true,
            security_testing: true,
            load_testing: false, // Disabled by default due to resource requirements
            max_test_duration: Duration::from_secs(300), // 5 minutes
            memory_threshold_mb: 512,
            performance_baselines: baselines,
        }
    }
}

impl OutputTestSuite {
    /// Create a new test suite
    pub fn new(config: TestConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// Run all tests in the suite
    pub async fn run_all_tests(&mut self) -> Result<TestSuiteReport> {
        println!("🧪 Starting comprehensive output testing suite...");

        // Unit tests
        self.run_unit_tests().await?;

        // Integration tests
        self.run_integration_tests().await?;

        // Performance tests
        if self.config.performance_testing {
            self.run_performance_tests().await?;
        }

        // Security tests
        if self.config.security_testing {
            self.run_security_tests().await?;
        }

        // Load tests
        if self.config.load_testing {
            self.run_load_tests().await?;
        }

        self.generate_report()
    }

    /// Run unit tests for output formatters
    async fn run_unit_tests(&mut self) -> Result<()> {
        println!("📋 Running unit tests...");

        let unit_test_runner = unit_tests::UnitTestRunner::new();
        let unit_results = unit_test_runner.run_all_tests().await?;

        self.results.extend(unit_results);
        Ok(())
    }

    /// Run integration tests for output pipelines
    async fn run_integration_tests(&mut self) -> Result<()> {
        println!("🔗 Running integration tests...");

        let integration_test_runner = integration_tests::IntegrationTestRunner::new();
        let integration_results = integration_test_runner.run_all_tests().await?;

        self.results.extend(integration_results);
        Ok(())
    }

    /// Run performance regression tests
    async fn run_performance_tests(&mut self) -> Result<()> {
        println!("⚡ Running performance tests...");

        let performance_test_runner = performance_tests::PerformanceTestRunner::new(&self.config);
        let performance_results = performance_test_runner.run_all_tests().await?;

        self.results.extend(performance_results);
        Ok(())
    }

    /// Run security tests for output generation
    async fn run_security_tests(&mut self) -> Result<()> {
        println!("🔒 Running security tests...");

        let security_test_runner = security_tests::SecurityTestRunner::new();
        let security_results = security_test_runner.run_all_tests().await?;

        self.results.extend(security_results);
        Ok(())
    }

    /// Run load tests
    async fn run_load_tests(&mut self) -> Result<()> {
        println!("📈 Running load tests...");

        // Load tests implementation would go here
        Ok(())
    }

    /// Generate comprehensive test report
    fn generate_report(&self) -> Result<TestSuiteReport> {
        let total_tests = self.results.len();
        let passed_tests = self
            .results
            .iter()
            .filter(|r| r.status == TestStatus::Passed)
            .count();
        let failed_tests = self
            .results
            .iter()
            .filter(|r| r.status == TestStatus::Failed)
            .count();
        let skipped_tests = self
            .results
            .iter()
            .filter(|r| r.status == TestStatus::Skipped)
            .count();

        let total_execution_time = self.results.iter().map(|r| r.execution_time).sum();

        let avg_memory_usage = if total_tests > 0 {
            self.results
                .iter()
                .map(|r| r.memory_usage_bytes)
                .sum::<usize>()
                / total_tests
        } else {
            0
        };

        let performance_regressions = self.check_performance_regressions();

        Ok(TestSuiteReport {
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            success_rate: if total_tests > 0 {
                (passed_tests as f64 / total_tests as f64) * 100.0
            } else {
                0.0
            },
            total_execution_time,
            avg_memory_usage_bytes: avg_memory_usage,
            performance_regressions,
            detailed_results: self.results.clone(),
            generated_at: chrono::Utc::now(),
        })
    }

    /// Check for performance regressions against baselines
    fn check_performance_regressions(&self) -> Vec<PerformanceRegression> {
        let mut regressions = Vec::new();

        for result in &self.results {
            if let Some(baseline) = self.config.performance_baselines.get(&result.test_name) {
                let execution_time_ms = result.execution_time.as_millis() as u64;
                let memory_usage_mb = result.memory_usage_bytes / (1024 * 1024);

                if execution_time_ms > baseline.max_execution_time_ms {
                    regressions.push(PerformanceRegression {
                        test_name: result.test_name.clone(),
                        metric: "execution_time".to_string(),
                        baseline_value: baseline.max_execution_time_ms as f64,
                        actual_value: execution_time_ms as f64,
                        regression_percent: ((execution_time_ms as f64
                            - baseline.max_execution_time_ms as f64)
                            / baseline.max_execution_time_ms as f64)
                            * 100.0,
                    });
                }

                if memory_usage_mb > baseline.max_memory_usage_mb {
                    regressions.push(PerformanceRegression {
                        test_name: result.test_name.clone(),
                        metric: "memory_usage".to_string(),
                        baseline_value: baseline.max_memory_usage_mb as f64,
                        actual_value: memory_usage_mb as f64,
                        regression_percent: ((memory_usage_mb as f64
                            - baseline.max_memory_usage_mb as f64)
                            / baseline.max_memory_usage_mb as f64)
                            * 100.0,
                    });
                }

                if result.metrics.throughput_ops < baseline.min_throughput_ops {
                    regressions.push(PerformanceRegression {
                        test_name: result.test_name.clone(),
                        metric: "throughput".to_string(),
                        baseline_value: baseline.min_throughput_ops,
                        actual_value: result.metrics.throughput_ops,
                        regression_percent: ((baseline.min_throughput_ops
                            - result.metrics.throughput_ops)
                            / baseline.min_throughput_ops)
                            * 100.0,
                    });
                }
            }
        }

        regressions
    }
}

/// Comprehensive test suite report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub success_rate: f64,
    pub total_execution_time: Duration,
    pub avg_memory_usage_bytes: usize,
    pub performance_regressions: Vec<PerformanceRegression>,
    pub detailed_results: Vec<TestResult>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Performance regression information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    pub test_name: String,
    pub metric: String,
    pub baseline_value: f64,
    pub actual_value: f64,
    pub regression_percent: f64,
}

impl Default for TestMetrics {
    fn default() -> Self {
        Self {
            throughput_ops: 0.0,
            avg_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
            error_rate_percent: 0.0,
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            io_ops_per_second: 0.0,
        }
    }
}

/// Helper function to measure execution time and memory usage
pub async fn measure_test_execution<F, T>(test_name: &str, test_fn: F) -> Result<TestResult>
where
    F: std::future::Future<Output = Result<T>>,
{
    let start_time = Instant::now();
    let start_memory = get_current_memory_usage();

    let result = test_fn.await;

    let execution_time = start_time.elapsed();
    let end_memory = get_current_memory_usage();
    let memory_usage_bytes = end_memory.saturating_sub(start_memory);

    let status = match result {
        Ok(_) => TestStatus::Passed,
        Err(_) => TestStatus::Failed,
    };

    let error_message = match result {
        Err(e) => Some(e.to_string()),
        Ok(_) => None,
    };

    Ok(TestResult {
        test_name: test_name.to_string(),
        status,
        execution_time,
        memory_usage_bytes,
        error_message,
        metrics: TestMetrics::default(),
        artifacts: Vec::new(),
    })
}

/// Get current memory usage (simplified implementation)
fn get_current_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real implementation, you would use system APIs to get actual memory usage
    std::process::id() as usize * 1024 // Placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_output_test_suite_creation() {
        let config = TestConfig::default();
        let test_suite = OutputTestSuite::new(config);

        assert_eq!(test_suite.results.len(), 0);
        assert!(test_suite.config.performance_testing);
    }

    #[tokio::test]
    async fn test_measure_execution() {
        let result = measure_test_execution("test_execution", async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok(())
        })
        .await
        .unwrap();

        assert_eq!(result.test_name, "test_execution");
        assert_eq!(result.status, TestStatus::Passed);
        assert!(result.execution_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_performance_baseline_creation() {
        let config = TestConfig::default();

        assert!(config.performance_baselines.contains_key("json_format"));
        assert!(config.performance_baselines.contains_key("html_format"));
        assert!(config
            .performance_baselines
            .contains_key("storage_operations"));
    }
}
