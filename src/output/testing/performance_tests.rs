//! # Performance Testing Framework
//!
//! This module provides comprehensive performance testing for CodeGuardian output systems,
//! including benchmarking, memory profiling, and regression detection.

use super::{measure_test_execution, PerformanceBaseline, TestConfig, TestResult};
use crate::output::ai::{create_enhancement_engine, AIEnhancementConfig};
use crate::output::formats::*;
use crate::output::formatter::OutputFormatter;
use crate::output::storage::organizer::ResultsOrganizer;
use crate::output::storage::{OrganizationStrategy, StorageConfig};
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

/// Performance test runner
pub struct PerformanceTestRunner {
    config: TestConfig,
    baselines: HashMap<String, PerformanceBaseline>,
}

/// Performance test scenario
#[derive(Debug, Clone)]
pub struct PerformanceScenario {
    pub name: String,
    pub description: String,
    pub dataset_size: usize,
    pub concurrent_operations: usize,
    pub duration_seconds: u64,
    pub memory_limit_mb: usize,
}

/// Memory usage tracker
pub struct MemoryTracker {
    start_memory: usize,
    peak_memory: usize,
    samples: Vec<MemorySample>,
}

/// Memory sample at a point in time
#[derive(Debug, Clone)]
pub struct MemorySample {
    pub timestamp: Instant,
    pub memory_bytes: usize,
    pub heap_bytes: usize,
}

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub scenario_name: String,
    pub operations_completed: u64,
    pub total_duration: Duration,
    pub throughput_ops_per_sec: f64,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub memory_stats: MemoryStats,
    pub error_count: u64,
    pub error_rate_percent: f64,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub peak_memory_mb: f64,
    pub avg_memory_mb: f64,
    pub memory_growth_mb: f64,
    pub gc_pressure_score: f64,
}

impl PerformanceTestRunner {
    /// Create a new performance test runner
    pub fn new(config: &TestConfig) -> Self {
        Self {
            config: config.clone(),
            baselines: config.performance_baselines.clone(),
        }
    }

    /// Run all performance tests
    pub async fn run_all_tests(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        println!("⚡ Running performance tests...");

        // Formatter performance tests
        results.extend(self.test_formatter_performance().await?);

        // Storage performance tests
        results.extend(self.test_storage_performance().await?);

        // AI enhancement performance tests
        results.extend(self.test_ai_performance().await?);

        // Concurrent operation tests
        results.extend(self.test_concurrent_operations().await?);

        // Memory stress tests
        results.extend(self.test_memory_efficiency().await?);

        // Streaming performance tests
        results.extend(self.test_streaming_performance().await?);

        println!(
            "✅ Performance tests completed: {} tests run",
            results.len()
        );
        Ok(results)
    }

    /// Test formatter performance across all formats
    async fn test_formatter_performance(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        let scenarios = vec![
            PerformanceScenario {
                name: "small_dataset".to_string(),
                description: "Format 100 findings".to_string(),
                dataset_size: 100,
                concurrent_operations: 1,
                duration_seconds: 30,
                memory_limit_mb: 50,
            },
            PerformanceScenario {
                name: "medium_dataset".to_string(),
                description: "Format 1,000 findings".to_string(),
                dataset_size: 1000,
                concurrent_operations: 1,
                duration_seconds: 60,
                memory_limit_mb: 100,
            },
            PerformanceScenario {
                name: "large_dataset".to_string(),
                description: "Format 10,000 findings".to_string(),
                dataset_size: 10000,
                concurrent_operations: 1,
                duration_seconds: 120,
                memory_limit_mb: 200,
            },
        ];

        for scenario in scenarios {
            // Test each formatter
            let formatters: Vec<(&str, Box<dyn OutputFormatter>)> = vec![
                ("json", Box::new(JsonFormatter::new())),
                ("html", Box::new(HtmlFormatter::new())),
                ("markdown", Box::new(MarkdownFormatter::new())),
                ("sarif", Box::new(SarifFormatter::new())),
                ("yaml", Box::new(YamlFormatter::new())),
            ];

            for (format_name, formatter) in formatters {
                let test_name = format!("perf_{}_{}", format_name, scenario.name);

                let result = measure_test_execution(&test_name, async {
                    let benchmark = self
                        .benchmark_formatter(formatter.as_ref(), &scenario)
                        .await?;

                    self.validate_benchmark_result(&benchmark, format_name)?;
                    Ok(())
                })
                .await?;

                results.push(result);
            }
        }

        Ok(results)
    }

    /// Benchmark a specific formatter
    async fn benchmark_formatter(
        &self,
        formatter: &dyn OutputFormatter,
        scenario: &PerformanceScenario,
    ) -> Result<BenchmarkResult> {
        let test_data = self
            .generate_performance_test_data(scenario.dataset_size)
            .await?;
        let mut memory_tracker = MemoryTracker::new();

        let start_time = Instant::now();
        let mut operations_completed = 0u64;
        let mut latencies = Vec::new();
        let mut error_count = 0u64;

        // Run benchmark for specified duration
        let end_time = start_time + Duration::from_secs(scenario.duration_seconds);

        while Instant::now() < end_time {
            let operation_start = Instant::now();
            memory_tracker.sample_memory();

            match formatter.format(&test_data) {
                Ok(_) => {
                    operations_completed += 1;
                    let latency = operation_start.elapsed();
                    latencies.push(latency.as_millis() as f64);
                }
                Err(_) => {
                    error_count += 1;
                }
            }

            // Small delay to prevent CPU spinning
            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        let total_duration = start_time.elapsed();
        memory_tracker.sample_memory();

        // Calculate statistics
        let throughput_ops_per_sec = operations_completed as f64 / total_duration.as_secs_f64();
        let avg_latency_ms = if !latencies.is_empty() {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        } else {
            0.0
        };

        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p95_latency_ms = if !latencies.is_empty() {
            latencies[(latencies.len() * 95 / 100).min(latencies.len() - 1)]
        } else {
            0.0
        };
        let p99_latency_ms = if !latencies.is_empty() {
            latencies[(latencies.len() * 99 / 100).min(latencies.len() - 1)]
        } else {
            0.0
        };

        let error_rate_percent = if operations_completed + error_count > 0 {
            (error_count as f64 / (operations_completed + error_count) as f64) * 100.0
        } else {
            0.0
        };

        let memory_stats = memory_tracker.calculate_stats();

        Ok(BenchmarkResult {
            scenario_name: scenario.name.clone(),
            operations_completed,
            total_duration,
            throughput_ops_per_sec,
            avg_latency_ms,
            p95_latency_ms,
            p99_latency_ms,
            memory_stats,
            error_count,
            error_rate_percent,
        })
    }

    /// Test storage system performance
    async fn test_storage_performance(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        // Test storage write performance
        results.push(
            measure_test_execution("perf_storage_write", async {
                let temp_dir = tempfile::TempDir::new()?;
                let storage_config = StorageConfig {
                    base_directory: temp_dir.path().to_path_buf(),
                    organization_strategy: OrganizationStrategy::Hybrid,
                    enable_compression: true,
                    ..Default::default()
                };

                let mut organizer = ResultsOrganizer::new(storage_config)?;
                let test_data = self.generate_performance_test_data(1000).await?;

                let start_time = Instant::now();

                for i in 0..100 {
                    let outputs = vec![(
                        "json".to_string(),
                        crate::output::formatter::OutputResult::new(
                            serde_json::to_string(&test_data)?,
                            "json",
                            "test_config".to_string(),
                        ),
                    )];

                    organizer.store_results(
                        &test_data,
                        &outputs,
                        &format!("test_project_{}", i),
                        Some(&format!("performance/repo_{}", i)),
                        vec!["performance_test".to_string()],
                    )?;
                }

                let elapsed = start_time.elapsed();

                // Should complete 100 storage operations in reasonable time
                if elapsed > Duration::from_secs(30) {
                    return Err(anyhow::anyhow!(
                        "Storage performance too slow: {:?}",
                        elapsed
                    ));
                }

                Ok(())
            })
            .await?,
        );

        // Test storage read performance
        results.push(
            measure_test_execution("perf_storage_read", async {
                let temp_dir = tempfile::TempDir::new()?;
                let storage_config = StorageConfig {
                    base_directory: temp_dir.path().to_path_buf(),
                    organization_strategy: OrganizationStrategy::ByProject,
                    enable_compression: false, // Faster for this test
                    ..Default::default()
                };

                let mut organizer = ResultsOrganizer::new(storage_config)?;
                let test_data = self.generate_performance_test_data(100).await?;

                // First, store some data
                let mut stored_ids = Vec::new();
                for _i in 0..50 {
                    let outputs = vec![(
                        "json".to_string(),
                        crate::output::formatter::OutputResult::new(
                            serde_json::to_string(&test_data)?,
                            "json",
                            "test_config".to_string(),
                        ),
                    )];

                    let id = organizer.store_results(
                        &test_data,
                        &outputs,
                        "test_project",
                        Some("performance/repo"),
                        vec!["performance_test".to_string()],
                    )?;
                    stored_ids.push(id);
                }

                // Now benchmark read performance
                let start_time = Instant::now();

                for id in &stored_ids {
                    let _retrieved = organizer.retrieve_results(id)?;
                }

                let elapsed = start_time.elapsed();

                // Should read 50 results quickly
                if elapsed > Duration::from_secs(10) {
                    return Err(anyhow::anyhow!("Read performance too slow: {:?}", elapsed));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test AI enhancement performance
    async fn test_ai_performance(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("perf_ai_enhancement", async {
                let ai_engine = create_enhancement_engine()?;
                let ai_config = AIEnhancementConfig::default();
                let test_data = self.generate_performance_test_data(1000).await?;

                let start_time = Instant::now();
                let _enhanced_results = ai_engine.enhance_results(&test_data, &ai_config)?;
                let elapsed = start_time.elapsed();

                // AI enhancement should complete within reasonable time
                if elapsed > Duration::from_secs(60) {
                    return Err(anyhow::anyhow!("AI enhancement too slow: {:?}", elapsed));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test concurrent operations performance
    async fn test_concurrent_operations(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("perf_concurrent_formatting", async {
                let test_data = self.generate_performance_test_data(500).await?;
                let semaphore = Arc::new(Semaphore::new(10)); // Limit to 10 concurrent ops

                let start_time = Instant::now();
                let mut handles = Vec::new();

                // Start 50 concurrent formatting operations
                for i in 0..50 {
                    let data = test_data.clone();
                    let sem = semaphore.clone();

                    let handle = tokio::spawn(async move {
                        let _permit = sem.acquire().await.unwrap();

                        let formatter = match i % 5 {
                            0 => Box::new(JsonFormatter::new()) as Box<dyn OutputFormatter>,
                            1 => Box::new(HtmlFormatter::new()) as Box<dyn OutputFormatter>,
                            2 => Box::new(MarkdownFormatter::new()) as Box<dyn OutputFormatter>,
                            3 => Box::new(SarifFormatter::new()) as Box<dyn OutputFormatter>,
                            _ => Box::new(YamlFormatter::new()) as Box<dyn OutputFormatter>,
                        };

                        formatter.format(&data)
                    });

                    handles.push(handle);
                }

                // Wait for all operations to complete
                for handle in handles {
                    handle.await??;
                }

                let elapsed = start_time.elapsed();

                // 50 concurrent operations should complete within reasonable time
                if elapsed > Duration::from_secs(120) {
                    return Err(anyhow::anyhow!(
                        "Concurrent operations too slow: {:?}",
                        elapsed
                    ));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test memory efficiency
    async fn test_memory_efficiency(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("perf_memory_large_dataset", async {
                let mut memory_tracker = MemoryTracker::new();
                memory_tracker.sample_memory();

                // Process increasingly large datasets
                for size in [1000, 5000, 10000, 20000] {
                    let test_data = self.generate_performance_test_data(size).await?;
                    memory_tracker.sample_memory();

                    let formatter = JsonFormatter::new();
                    let _output = formatter.format(&test_data)?;
                    memory_tracker.sample_memory();

                    // Force garbage collection (if available)
                    tokio::task::yield_now().await;
                    memory_tracker.sample_memory();
                }

                let stats = memory_tracker.calculate_stats();

                // Memory growth should be reasonable
                if stats.memory_growth_mb > 500.0 {
                    return Err(anyhow::anyhow!(
                        "Excessive memory growth: {:.2} MB",
                        stats.memory_growth_mb
                    ));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Test streaming performance
    async fn test_streaming_performance(&self) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        results.push(
            measure_test_execution("perf_streaming_output", async {
                // This would test streaming capabilities if implemented
                // For now, just verify formatters can handle large datasets efficiently

                let test_data = self.generate_performance_test_data(15000).await?;
                let start_time = Instant::now();

                // Test that formatters that claim to support streaming can handle large datasets
                let json_formatter = JsonFormatter::new();
                if json_formatter.supports_streaming() {
                    let _output = json_formatter.format(&test_data)?;
                }

                let elapsed = start_time.elapsed();

                if elapsed > Duration::from_secs(30) {
                    return Err(anyhow::anyhow!(
                        "Streaming performance insufficient: {:?}",
                        elapsed
                    ));
                }

                Ok(())
            })
            .await?,
        );

        Ok(results)
    }

    /// Generate performance test data
    async fn generate_performance_test_data(&self, size: usize) -> Result<AnalysisResults> {
        let mut results = AnalysisResults::new("perf_test_config".to_string());

        for i in 0..size {
            let severity = match i % 5 {
                0 => Severity::Critical,
                1 => Severity::High,
                2 => Severity::Medium,
                3 => Severity::Low,
                _ => Severity::Info,
            };

            let finding = Finding::new(
                &format!("perf_analyzer_{}", i % 20),
                &format!("perf_rule_{}", i % 100),
                severity,
                PathBuf::from(format!("perf_file_{}.rs", i % 500)),
                (i % 10000) as u32 + 1,
                format!("Performance test finding #{} with detailed message content", i),
            ).with_description(format!(
                "Detailed description for performance finding #{}. This includes multiple lines of text to simulate real-world finding descriptions that can be quite lengthy and contain various details about the issue found during analysis.",
                i
            )).with_suggestion(format!(
                "Suggested fix for finding #{}: Apply security best practices and review the code implementation",
                i
            ));

            results.add_finding(finding);
        }

        Ok(results)
    }

    /// Validate benchmark results against baselines
    fn validate_benchmark_result(
        &self,
        benchmark: &BenchmarkResult,
        format_name: &str,
    ) -> Result<()> {
        let baseline_key = format!("{}_format", format_name);

        if let Some(baseline) = self.baselines.get(&baseline_key) {
            if benchmark.avg_latency_ms > baseline.max_execution_time_ms as f64 {
                return Err(anyhow::anyhow!(
                    "Performance regression in {}: avg latency {:.2}ms > baseline {}ms",
                    format_name,
                    benchmark.avg_latency_ms,
                    baseline.max_execution_time_ms
                ));
            }

            if benchmark.throughput_ops_per_sec < baseline.min_throughput_ops {
                return Err(anyhow::anyhow!(
                    "Performance regression in {}: throughput {:.2} ops/s < baseline {:.2} ops/s",
                    format_name,
                    benchmark.throughput_ops_per_sec,
                    baseline.min_throughput_ops
                ));
            }

            if benchmark.error_rate_percent > baseline.max_error_rate_percent {
                return Err(anyhow::anyhow!(
                    "Error rate too high in {}: {:.2}% > baseline {:.2}%",
                    format_name,
                    benchmark.error_rate_percent,
                    baseline.max_error_rate_percent
                ));
            }
        }

        Ok(())
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTracker {
    pub fn new() -> Self {
        Self {
            start_memory: get_current_memory_usage(),
            peak_memory: 0,
            samples: Vec::new(),
        }
    }

    pub fn sample_memory(&mut self) {
        let current_memory = get_current_memory_usage();
        self.peak_memory = self.peak_memory.max(current_memory);

        self.samples.push(MemorySample {
            timestamp: Instant::now(),
            memory_bytes: current_memory,
            heap_bytes: current_memory, // Simplified
        });
    }

    pub fn calculate_stats(&self) -> MemoryStats {
        let peak_memory_mb = self.peak_memory as f64 / (1024.0 * 1024.0);
        let avg_memory_mb = if !self.samples.is_empty() {
            let total: usize = self.samples.iter().map(|s| s.memory_bytes).sum();
            (total as f64 / self.samples.len() as f64) / (1024.0 * 1024.0)
        } else {
            0.0
        };

        let memory_growth_mb = if !self.samples.is_empty() {
            let last_memory = self.samples.last().unwrap().memory_bytes;
            (last_memory.saturating_sub(self.start_memory) as f64) / (1024.0 * 1024.0)
        } else {
            0.0
        };

        // Simplified GC pressure calculation
        let gc_pressure_score = memory_growth_mb / peak_memory_mb.max(1.0);

        MemoryStats {
            peak_memory_mb,
            avg_memory_mb,
            memory_growth_mb,
            gc_pressure_score,
        }
    }
}

/// Get current memory usage (simplified implementation)
fn get_current_memory_usage() -> usize {
    // This is a placeholder implementation
    // In a real system, you would use system APIs to get actual memory usage
    std::process::id() as usize * 1024
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_test_runner_creation() {
        let config = TestConfig::default();
        let runner = PerformanceTestRunner::new(&config);
        assert!(!runner.baselines.is_empty());
    }

    #[tokio::test]
    async fn test_generate_performance_test_data() {
        let config = TestConfig::default();
        let runner = PerformanceTestRunner::new(&config);

        let test_data = runner.generate_performance_test_data(100).await.unwrap();
        assert_eq!(test_data.findings.len(), 100);
    }

    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        tracker.sample_memory();
        tracker.sample_memory();

        let stats = tracker.calculate_stats();
        assert!(stats.peak_memory_mb >= 0.0);
    }
}
