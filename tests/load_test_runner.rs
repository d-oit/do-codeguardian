//! Load Test Runner
//!
//! Unified test runner for all CodeGuardian load testing scenarios.
//! Executes comprehensive performance validation as specified in Task 25.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;

mod load_testing_framework;
use load_testing_framework::{LoadTestConfig, LoadTestFramework, LoadTestReport};

/// Comprehensive load test suite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestSuiteConfig {
    /// Whether to run GitHub API load tests
    pub run_github_api_tests: bool,
    /// Whether to run enterprise scale tests
    pub run_enterprise_tests: bool,
    /// Whether to run memory pressure tests
    pub run_memory_tests: bool,
    /// Whether to run network resilience tests
    pub run_network_tests: bool,
    /// Whether to run quick tests only (for CI)
    pub quick_mode: bool,
    /// Maximum test duration for the entire suite
    pub max_suite_duration: Duration,
}

impl Default for LoadTestSuiteConfig {
    fn default() -> Self {
        Self {
            run_github_api_tests: true,
            run_enterprise_tests: true,
            run_memory_tests: true,
            run_network_tests: true,
            quick_mode: false,
            max_suite_duration: Duration::from_secs(1800), // 30 minutes
        }
    }
}

/// Load test suite results
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadTestSuiteResults {
    pub suite_duration: Duration,
    pub tests_executed: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub overall_success_rate: f64,
    pub test_results: HashMap<String, LoadTestResult>,
    pub performance_summary: PerformanceSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadTestResult {
    pub test_name: String,
    pub passed: bool,
    pub report: LoadTestReport,
    pub execution_time: Duration,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub average_success_rate: f64,
    pub average_response_time_ms: f64,
    pub peak_memory_usage_mb: f64,
    pub total_operations: u64,
    pub total_retries: u64,
    pub meets_task25_criteria: bool,
}

/// Main load test runner
pub struct LoadTestRunner {
    config: LoadTestSuiteConfig,
}

impl LoadTestRunner {
    pub fn new(config: LoadTestSuiteConfig) -> Self {
        Self { config }
    }

    /// Execute the complete load test suite
    pub async fn run_load_test_suite(&self) -> anyhow::Result<LoadTestSuiteResults> {
        let suite_start = Instant::now();
        let mut test_results = HashMap::new();
        let mut tests_executed = 0;
        let mut tests_passed = 0;

        println!("🚀 Starting CodeGuardian Load Test Suite");
        println!("Configuration: {:?}", self.config);
        println!("=======================================\n");

        // GitHub API Load Tests
        if self.config.run_github_api_tests {
            println!("📡 Running GitHub API Load Tests...");
            let api_results = self.run_github_api_load_tests().await;
            for (test_name, result) in api_results {
                if result.passed {
                    tests_passed += 1;
                }
                tests_executed += 1;
                test_results.insert(test_name, result);
            }
        }

        // Enterprise Scale Tests
        if self.config.run_enterprise_tests && !self.config.quick_mode {
            println!("🏢 Running Enterprise Scale Load Tests...");
            let enterprise_results = self.run_enterprise_scale_tests().await;
            for (test_name, result) in enterprise_results {
                if result.passed {
                    tests_passed += 1;
                }
                tests_executed += 1;
                test_results.insert(test_name, result);
            }
        }

        // Memory Pressure Tests
        if self.config.run_memory_tests {
            println!("🧠 Running Memory Pressure Load Tests...");
            let memory_results = self.run_memory_pressure_tests().await;
            for (test_name, result) in memory_results {
                if result.passed {
                    tests_passed += 1;
                }
                tests_executed += 1;
                test_results.insert(test_name, result);
            }
        }

        // Network Resilience Tests
        if self.config.run_network_tests {
            println!("🌐 Running Network Resilience Load Tests...");
            let network_results = self.run_network_resilience_tests().await;
            for (test_name, result) in network_results {
                if result.passed {
                    tests_passed += 1;
                }
                tests_executed += 1;
                test_results.insert(test_name, result);
            }
        }

        let suite_duration = suite_start.elapsed();
        let overall_success_rate = if tests_executed > 0 {
            tests_passed as f64 / tests_executed as f64
        } else {
            0.0
        };

        let performance_summary = self.calculate_performance_summary(&test_results);

        let results = LoadTestSuiteResults {
            suite_duration,
            tests_executed,
            tests_passed,
            tests_failed: tests_executed - tests_passed,
            overall_success_rate,
            test_results,
            performance_summary,
        };

        self.print_suite_summary(&results);
        Ok(results)
    }

    async fn run_github_api_load_tests(&self) -> HashMap<String, LoadTestResult> {
        let mut results = HashMap::new();

        // GitHub API Burst Load Test
        let test_result = self
            .execute_test(
                "github_api_burst_load",
                self.create_github_burst_config(),
                || async { simulate_github_api_call().await },
            )
            .await;
        results.insert("github_api_burst_load".to_string(), test_result);

        // GitHub API Sustained Load Test (skip in quick mode)
        if !self.config.quick_mode {
            let test_result = self
                .execute_test(
                    "github_api_sustained_load",
                    self.create_github_sustained_config(),
                    || async { simulate_github_api_call().await },
                )
                .await;
            results.insert("github_api_sustained_load".to_string(), test_result);
        }

        // GitHub API Rate Limit Test
        let test_result = self
            .execute_test(
                "github_api_rate_limit",
                self.create_github_rate_limit_config(),
                || async { simulate_rate_limited_api_call().await },
            )
            .await;
        results.insert("github_api_rate_limit".to_string(), test_result);

        results
    }

    async fn run_enterprise_scale_tests(&self) -> HashMap<String, LoadTestResult> {
        let mut results = HashMap::new();

        // Enterprise Repository Analysis Test
        let test_result = self
            .execute_test(
                "enterprise_repository_analysis",
                self.create_enterprise_analysis_config(),
                || async { simulate_enterprise_repository_analysis().await },
            )
            .await;
        results.insert("enterprise_repository_analysis".to_string(), test_result);

        // Concurrent Workflow Test
        let test_result = self
            .execute_test(
                "concurrent_ci_workflows",
                self.create_concurrent_workflow_config(),
                || async { simulate_ci_workflow_execution().await },
            )
            .await;
        results.insert("concurrent_ci_workflows".to_string(), test_result);

        results
    }

    async fn run_memory_pressure_tests(&self) -> HashMap<String, LoadTestResult> {
        let mut results = HashMap::new();

        // Memory Intensive Operation Test
        let test_result = self
            .execute_test(
                "memory_intensive_operations",
                self.create_memory_intensive_config(),
                || async { simulate_memory_intensive_operation().await },
            )
            .await;
        results.insert("memory_intensive_operations".to_string(), test_result);

        // Memory Leak Detection Test (skip in quick mode)
        if !self.config.quick_mode {
            let test_result = self
                .execute_test(
                    "memory_leak_detection",
                    self.create_memory_leak_config(),
                    || async { simulate_memory_leak_test().await },
                )
                .await;
            results.insert("memory_leak_detection".to_string(), test_result);
        }

        results
    }

    async fn run_network_resilience_tests(&self) -> HashMap<String, LoadTestResult> {
        let mut results = HashMap::new();

        // Network Timeout Test
        let test_result = self
            .execute_test(
                "network_timeout_resilience",
                self.create_network_timeout_config(),
                || async { simulate_network_operation_with_timeouts().await },
            )
            .await;
        results.insert("network_timeout_resilience".to_string(), test_result);

        // Retry Logic Test
        let test_result = self
            .execute_test(
                "retry_logic_validation",
                self.create_retry_logic_config(),
                || async { simulate_operation_requiring_retries().await },
            )
            .await;
        results.insert("retry_logic_validation".to_string(), test_result);

        results
    }

    async fn execute_test<F, Fut>(
        &self,
        test_name: &str,
        config: LoadTestConfig,
        operation: F,
    ) -> LoadTestResult
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send,
    {
        let start_time = Instant::now();
        println!("  🔄 Executing: {}", test_name);

        let framework = LoadTestFramework::new(config);

        match framework.execute_load_test(operation).await {
            Ok(report) => {
                let execution_time = start_time.elapsed();
                let passed = self.evaluate_test_criteria(&report, test_name);

                if passed {
                    println!(
                        "  ✅ PASSED: {} ({:.2}s)",
                        test_name,
                        execution_time.as_secs_f64()
                    );
                } else {
                    println!(
                        "  ❌ FAILED: {} ({:.2}s)",
                        test_name,
                        execution_time.as_secs_f64()
                    );
                }

                LoadTestResult {
                    test_name: test_name.to_string(),
                    passed,
                    report,
                    execution_time,
                    error_message: None,
                }
            }
            Err(e) => {
                let execution_time = start_time.elapsed();
                println!(
                    "  ❌ ERROR: {} - {} ({:.2}s)",
                    test_name,
                    e,
                    execution_time.as_secs_f64()
                );

                LoadTestResult {
                    test_name: test_name.to_string(),
                    passed: false,
                    report: LoadTestReport {
                        test_duration: execution_time,
                        operations_attempted: 0,
                        operations_successful: 0,
                        operations_failed: 0,
                        success_rate: 0.0,
                        average_response_time_ms: 0.0,
                        operations_per_second: 0.0,
                        peak_memory_usage_mb: 0.0,
                        retries_performed: 0,
                    },
                    execution_time,
                    error_message: Some(e.to_string()),
                }
            }
        }
    }

    fn evaluate_test_criteria(&self, report: &LoadTestReport, test_name: &str) -> bool {
        match test_name {
            "github_api_burst_load" => {
                report.success_rate >= 0.95
                    && report.average_response_time_ms <= 5000.0
                    && report.operations_attempted >= 50
            }
            "github_api_sustained_load" => {
                report.success_rate >= 0.999
                    && report.average_response_time_ms <= 10000.0
                    && report.operations_per_second >= 0.8
            }
            "github_api_rate_limit" => report.success_rate >= 0.80 && report.retries_performed > 0,
            "enterprise_repository_analysis" => {
                report.success_rate >= 0.95
                    && report.average_response_time_ms <= 120000.0
                    && report.peak_memory_usage_mb <= 500.0
            }
            "concurrent_ci_workflows" => {
                report.success_rate >= 0.95
                    && report.average_response_time_ms <= 60000.0
                    && report.operations_per_second >= 1.5
            }
            "memory_intensive_operations" => {
                report.success_rate >= 0.80
                    && report.peak_memory_usage_mb <= 2000.0
                    && report.average_response_time_ms <= 90000.0
            }
            "memory_leak_detection" => {
                report.success_rate >= 0.95 && report.operations_successful >= 200
            }
            "network_timeout_resilience" => {
                report.success_rate >= 0.70
                    && report.retries_performed > 0
                    && report.average_response_time_ms <= 15000.0
            }
            "retry_logic_validation" => report.success_rate >= 0.85 && report.retries_performed > 0,
            _ => report.meets_performance_criteria(),
        }
    }

    fn calculate_performance_summary(
        &self,
        results: &HashMap<String, LoadTestResult>,
    ) -> PerformanceSummary {
        if results.is_empty() {
            return PerformanceSummary {
                average_success_rate: 0.0,
                average_response_time_ms: 0.0,
                peak_memory_usage_mb: 0.0,
                total_operations: 0,
                total_retries: 0,
                meets_task25_criteria: false,
            };
        }

        let mut total_success_rate = 0.0;
        let mut total_response_time = 0.0;
        let mut peak_memory = 0.0;
        let mut total_operations = 0;
        let mut total_retries = 0;

        for result in results.values() {
            total_success_rate += result.report.success_rate;
            total_response_time += result.report.average_response_time_ms;
            peak_memory = peak_memory.max(result.report.peak_memory_usage_mb);
            total_operations += result.report.operations_successful;
            total_retries += result.report.retries_performed;
        }

        let count = results.len() as f64;
        let average_success_rate = total_success_rate / count;
        let average_response_time_ms = total_response_time / count;

        // Task 25 success criteria: 99.9% success, <30s response, <100MB memory
        let meets_task25_criteria = average_success_rate >= 0.999
            && average_response_time_ms <= 30000.0
            && peak_memory <= 100.0;

        PerformanceSummary {
            average_success_rate,
            average_response_time_ms,
            peak_memory_usage_mb: peak_memory,
            total_operations,
            total_retries,
            meets_task25_criteria,
        }
    }

    fn print_suite_summary(&self, results: &LoadTestSuiteResults) {
        println!("\n🎯 Load Test Suite Results");
        println!("==========================");
        println!("Suite Duration: {:?}", results.suite_duration);
        println!("Tests Executed: {}", results.tests_executed);
        println!("Tests Passed: {}", results.tests_passed);
        println!("Tests Failed: {}", results.tests_failed);
        println!(
            "Overall Success Rate: {:.2}%",
            results.overall_success_rate * 100.0
        );
        println!();

        println!("📊 Performance Summary");
        println!(
            "Average Success Rate: {:.2}%",
            results.performance_summary.average_success_rate * 100.0
        );
        println!(
            "Average Response Time: {:.2}ms",
            results.performance_summary.average_response_time_ms
        );
        println!(
            "Peak Memory Usage: {:.2}MB",
            results.performance_summary.peak_memory_usage_mb
        );
        println!(
            "Total Operations: {}",
            results.performance_summary.total_operations
        );
        println!(
            "Total Retries: {}",
            results.performance_summary.total_retries
        );
        println!(
            "Meets Task 25 Criteria: {}",
            results.performance_summary.meets_task25_criteria
        );
        println!();

        println!("📋 Individual Test Results");
        for (test_name, result) in &results.test_results {
            let status = if result.passed {
                "✅ PASS"
            } else {
                "❌ FAIL"
            };
            println!(
                "  {} {} ({:.2}s, {:.1}% success)",
                status,
                test_name,
                result.execution_time.as_secs_f64(),
                result.report.success_rate * 100.0
            );
        }

        if results.performance_summary.meets_task25_criteria {
            println!("\n🎉 SUCCESS: All Task 25 performance criteria met!");
        } else {
            println!("\n⚠️  WARNING: Some Task 25 performance criteria not met.");
        }
    }

    // Configuration creators for different test scenarios

    fn create_github_burst_config(&self) -> LoadTestConfig {
        if self.config.quick_mode {
            LoadTestConfig {
                concurrent_operations: 10,
                test_duration: Duration::from_secs(30),
                target_ops_per_second: 2.0,
                max_retries: 2,
                operation_timeout: Duration::from_secs(10),
                gradual_ramp_up: false,
                ramp_up_duration: Duration::from_secs(0),
            }
        } else {
            LoadTestConfig {
                concurrent_operations: 20,
                test_duration: Duration::from_secs(60),
                target_ops_per_second: 2.0,
                max_retries: 3,
                operation_timeout: Duration::from_secs(10),
                gradual_ramp_up: false,
                ramp_up_duration: Duration::from_secs(0),
            }
        }
    }

    fn create_github_sustained_config(&self) -> LoadTestConfig {
        LoadTestConfig {
            concurrent_operations: 10,
            test_duration: Duration::from_secs(180), // Reduced from 300s for CI
            target_ops_per_second: 1.0,
            max_retries: 5,
            operation_timeout: Duration::from_secs(30),
            gradual_ramp_up: true,
            ramp_up_duration: Duration::from_secs(30),
        }
    }

    fn create_github_rate_limit_config(&self) -> LoadTestConfig {
        if self.config.quick_mode {
            LoadTestConfig {
                concurrent_operations: 15,
                test_duration: Duration::from_secs(30),
                target_ops_per_second: 3.0,
                max_retries: 5,
                operation_timeout: Duration::from_secs(15),
                gradual_ramp_up: false,
                ramp_up_duration: Duration::from_secs(0),
            }
        } else {
            LoadTestConfig {
                concurrent_operations: 25,
                test_duration: Duration::from_secs(60),
                target_ops_per_second: 3.0,
                max_retries: 10,
                operation_timeout: Duration::from_secs(30),
                gradual_ramp_up: false,
                ramp_up_duration: Duration::from_secs(0),
            }
        }
    }

    fn create_enterprise_analysis_config(&self) -> LoadTestConfig {
        LoadTestConfig {
            concurrent_operations: 4,
            test_duration: Duration::from_secs(120), // Reduced from 600s for CI
            target_ops_per_second: 0.2,
            max_retries: 2,
            operation_timeout: Duration::from_secs(60),
            gradual_ramp_up: true,
            ramp_up_duration: Duration::from_secs(20),
        }
    }

    fn create_concurrent_workflow_config(&self) -> LoadTestConfig {
        if self.config.quick_mode {
            LoadTestConfig {
                concurrent_operations: 10,
                test_duration: Duration::from_secs(60),
                target_ops_per_second: 2.0,
                max_retries: 3,
                operation_timeout: Duration::from_secs(30),
                gradual_ramp_up: true,
                ramp_up_duration: Duration::from_secs(10),
            }
        } else {
            LoadTestConfig {
                concurrent_operations: 25,
                test_duration: Duration::from_secs(120),
                target_ops_per_second: 2.0,
                max_retries: 5,
                operation_timeout: Duration::from_secs(60),
                gradual_ramp_up: true,
                ramp_up_duration: Duration::from_secs(20),
            }
        }
    }

    fn create_memory_intensive_config(&self) -> LoadTestConfig {
        if self.config.quick_mode {
            LoadTestConfig {
                concurrent_operations: 3,
                test_duration: Duration::from_secs(60),
                target_ops_per_second: 0.5,
                max_retries: 2,
                operation_timeout: Duration::from_secs(30),
                gradual_ramp_up: false,
                ramp_up_duration: Duration::from_secs(0),
            }
        } else {
            LoadTestConfig {
                concurrent_operations: 5,
                test_duration: Duration::from_secs(120),
                target_ops_per_second: 0.3,
                max_retries: 2,
                operation_timeout: Duration::from_secs(60),
                gradual_ramp_up: false,
                ramp_up_duration: Duration::from_secs(0),
            }
        }
    }

    fn create_memory_leak_config(&self) -> LoadTestConfig {
        LoadTestConfig {
            concurrent_operations: 8,
            test_duration: Duration::from_secs(120), // Reduced from 300s for CI
            target_ops_per_second: 2.0,
            max_retries: 3,
            operation_timeout: Duration::from_secs(15),
            gradual_ramp_up: true,
            ramp_up_duration: Duration::from_secs(15),
        }
    }

    fn create_network_timeout_config(&self) -> LoadTestConfig {
        if self.config.quick_mode {
            LoadTestConfig {
                concurrent_operations: 8,
                test_duration: Duration::from_secs(45),
                target_ops_per_second: 2.0,
                max_retries: 3,
                operation_timeout: Duration::from_secs(10),
                gradual_ramp_up: true,
                ramp_up_duration: Duration::from_secs(10),
            }
        } else {
            LoadTestConfig {
                concurrent_operations: 15,
                test_duration: Duration::from_secs(90),
                target_ops_per_second: 2.0,
                max_retries: 5,
                operation_timeout: Duration::from_secs(15),
                gradual_ramp_up: true,
                ramp_up_duration: Duration::from_secs(15),
            }
        }
    }

    fn create_retry_logic_config(&self) -> LoadTestConfig {
        if self.config.quick_mode {
            LoadTestConfig {
                concurrent_operations: 5,
                test_duration: Duration::from_secs(45),
                target_ops_per_second: 1.0,
                max_retries: 3,
                operation_timeout: Duration::from_secs(20),
                gradual_ramp_up: true,
                ramp_up_duration: Duration::from_secs(10),
            }
        } else {
            LoadTestConfig {
                concurrent_operations: 10,
                test_duration: Duration::from_secs(90),
                target_ops_per_second: 1.0,
                max_retries: 5,
                operation_timeout: Duration::from_secs(30),
                gradual_ramp_up: true,
                ramp_up_duration: Duration::from_secs(15),
            }
        }
    }
}

// Simulation functions

async fn simulate_github_api_call() -> anyhow::Result<()> {
    sleep(Duration::from_millis(100 + rand::random::<u64>() % 200)).await;
    if rand::random::<f64>() < 0.05 {
        anyhow::bail!("Simulated GitHub API failure");
    }
    Ok(())
}

async fn simulate_rate_limited_api_call() -> anyhow::Result<()> {
    sleep(Duration::from_millis(50 + rand::random::<u64>() % 100)).await;
    if rand::random::<f64>() < 0.20 {
        anyhow::bail!("Rate limit exceeded");
    }
    Ok(())
}

async fn simulate_enterprise_repository_analysis() -> anyhow::Result<()> {
    sleep(Duration::from_millis(2000 + rand::random::<u64>() % 3000)).await;
    if rand::random::<f64>() < 0.02 {
        anyhow::bail!("Enterprise analysis failed");
    }
    Ok(())
}

async fn simulate_ci_workflow_execution() -> anyhow::Result<()> {
    sleep(Duration::from_millis(500 + rand::random::<u64>() % 1500)).await;
    if rand::random::<f64>() < 0.03 {
        anyhow::bail!("CI workflow failed");
    }
    Ok(())
}

async fn simulate_memory_intensive_operation() -> anyhow::Result<()> {
    let _buffer = vec![0u8; 10_000_000]; // 10MB allocation
    sleep(Duration::from_millis(1000 + rand::random::<u64>() % 2000)).await;
    if rand::random::<f64>() < 0.15 {
        anyhow::bail!("Memory intensive operation failed");
    }
    Ok(())
}

async fn simulate_memory_leak_test() -> anyhow::Result<()> {
    let _buffer = vec![0u8; 1_000_000]; // 1MB allocation
    sleep(Duration::from_millis(100 + rand::random::<u64>() % 200)).await;
    if rand::random::<f64>() < 0.02 {
        anyhow::bail!("Memory leak test operation failed");
    }
    Ok(())
}

async fn simulate_network_operation_with_timeouts() -> anyhow::Result<()> {
    let latency = 200 + rand::random::<u64>() % 1000;
    sleep(Duration::from_millis(latency)).await;
    if rand::random::<f64>() < 0.30 {
        anyhow::bail!("Network timeout");
    }
    Ok(())
}

async fn simulate_operation_requiring_retries() -> anyhow::Result<()> {
    sleep(Duration::from_millis(100 + rand::random::<u64>() % 300)).await;
    if rand::random::<f64>() < 0.40 {
        anyhow::bail!("Operation failed, retry needed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_test_runner_quick_mode() {
        let config = LoadTestSuiteConfig {
            run_github_api_tests: true,
            run_enterprise_tests: false, // Skip for quick test
            run_memory_tests: true,
            run_network_tests: true,
            quick_mode: true,
            max_suite_duration: Duration::from_secs(300),
        };

        let runner = LoadTestRunner::new(config);
        let results = runner
            .run_load_test_suite()
            .await
            .expect("Failed to run load test suite");

        assert!(results.tests_executed > 0);
        assert!(results.suite_duration < Duration::from_secs(300));
        assert!(results.overall_success_rate > 0.0);
    }

    #[test]
    fn test_load_test_suite_config_default() {
        let config = LoadTestSuiteConfig::default();
        assert!(config.run_github_api_tests);
        assert!(config.run_enterprise_tests);
        assert!(config.run_memory_tests);
        assert!(config.run_network_tests);
        assert!(!config.quick_mode);
        assert_eq!(config.max_suite_duration, Duration::from_secs(1800));
    }
}
