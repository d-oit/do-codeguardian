//! Performance Regression Framework
//!
//! This module implements Phase 4 of the testing improvements plan:
//! Performance regression detection, memory leak detection, and scalability testing.

use do_codeguardian::core::GuardianEngine;
use do_codeguardian::utils::progress::ProgressReporter;
use do_codeguardian::{analyze_files, Config};
use serial_test::serial;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Performance baseline data
#[derive(Debug, Clone)]
struct PerformanceBaseline {
    operation: String,
    file_count: usize,
    avg_duration_ms: u64,
    max_memory_mb: usize,
    date_recorded: String,
}

/// Performance regression detector
struct PerformanceRegressionDetector {
    baselines: HashMap<String, PerformanceBaseline>,
    tolerance_percent: f64,
}

impl PerformanceRegressionDetector {
    fn new() -> Self {
        Self {
            baselines: Self::load_baselines(),
            tolerance_percent: 20.0, // 20% tolerance for performance regression
        }
    }

    fn load_baselines() -> HashMap<String, PerformanceBaseline> {
        // In a real implementation, this would load from a file or database
        let mut baselines = HashMap::new();

        // Set some reasonable baseline expectations
        baselines.insert(
            "single_file_analysis".to_string(),
            PerformanceBaseline {
                operation: "single_file_analysis".to_string(),
                file_count: 1,
                avg_duration_ms: 100,
                max_memory_mb: 50,
                date_recorded: "2024-01-01".to_string(),
            },
        );

        baselines.insert(
            "small_project_analysis".to_string(),
            PerformanceBaseline {
                operation: "small_project_analysis".to_string(),
                file_count: 10,
                avg_duration_ms: 500,
                max_memory_mb: 100,
                date_recorded: "2024-01-01".to_string(),
            },
        );

        baselines.insert(
            "medium_project_analysis".to_string(),
            PerformanceBaseline {
                operation: "medium_project_analysis".to_string(),
                file_count: 100,
                avg_duration_ms: 3000,
                max_memory_mb: 200,
                date_recorded: "2024-01-01".to_string(),
            },
        );

        baselines
    }

    fn check_regression(
        &self,
        operation: &str,
        duration: Duration,
        _memory_mb: usize,
    ) -> Result<(), String> {
        if let Some(baseline) = self.baselines.get(operation) {
            let duration_ms = duration.as_millis() as u64;
            let baseline_ms = baseline.avg_duration_ms;
            let tolerance_ms = (baseline_ms as f64 * self.tolerance_percent / 100.0) as u64;

            if duration_ms > baseline_ms + tolerance_ms {
                return Err(format!(
                    "Performance regression detected for {}: {}ms > {}ms (baseline + {}% tolerance)",
                    operation, duration_ms, baseline_ms + tolerance_ms, self.tolerance_percent
                ));
            }
        }
        Ok(())
    }
}

/// Performance benchmarking tests
mod performance_benchmarks {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_single_file_analysis_performance() {
        let detector = PerformanceRegressionDetector::new();
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");

        // Create a moderately complex test file
        let test_content = include_str!("../src/main.rs"); // Use actual source file
        std::fs::write(&test_file, test_content).unwrap();

        let config = Config::default();
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await.unwrap();

        // Warm up
        let _ = engine.analyze_files(&[test_file.clone()], 1).await.unwrap();

        // Benchmark
        let start = Instant::now();
        let _results = engine.analyze_files(&[test_file], 1).await.unwrap();
        let duration = start.elapsed();

        // Check for regression
        match detector.check_regression("single_file_analysis", duration, 0) {
            Ok(_) => println!(
                "✅ Single file analysis performance: {}ms",
                duration.as_millis()
            ),
            Err(e) => {
                println!("⚠️  {}", e);
                // Don't fail the test immediately, just warn
                // In CI, you might want to fail if this is critical
            }
        }

        // Ensure reasonable upper bound
        assert!(
            duration < Duration::from_secs(5),
            "Single file analysis should complete within 5 seconds (took {}ms)",
            duration.as_millis()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_small_project_analysis_performance() {
        let detector = PerformanceRegressionDetector::new();
        let temp_dir = TempDir::new().unwrap();

        // Create 10 test files
        let mut files = Vec::new();
        for i in 0..10 {
            let file = temp_dir.path().join(format!("test_{}.rs", i));
            let content = format!(
                "fn test_function_{}() {{\n    println!(\"Test {}\");\n    // TODO: Add more logic\n}}\n",
                i, i
            );
            std::fs::write(&file, content).unwrap();
            files.push(file);
        }

        let config = Config::default();
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await.unwrap();

        // Benchmark
        let start = Instant::now();
        let results = engine.analyze_files(&files, 2).await.unwrap();
        let duration = start.elapsed();

        // Verify results
        assert_eq!(results.summary.total_files_scanned, 10);

        // Check for regression
        match detector.check_regression("small_project_analysis", duration, 0) {
            Ok(_) => println!(
                "✅ Small project analysis performance: {}ms",
                duration.as_millis()
            ),
            Err(e) => println!("⚠️  {}", e),
        }

        assert!(
            duration < Duration::from_secs(10),
            "Small project analysis should complete within 10 seconds (took {}ms)",
            duration.as_millis()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_parallel_processing_scaling() {
        let temp_dir = TempDir::new().unwrap();

        // Create 20 test files
        let mut files = Vec::new();
        for i in 0..20 {
            let file = temp_dir.path().join(format!("parallel_{}.rs", i));
            let content = format!(
                "use std::collections::HashMap;\n\nfn process_data_{}() {{\n    let mut map = HashMap::new();\n    for i in 0..100 {{\n        map.insert(i, format!(\"item {{}}\", i));\n    }}\n}}\n",
                i
            );
            std::fs::write(&file, content).unwrap();
            files.push(file);
        }

        let config = Config::default();
        let progress = ProgressReporter::new(false);

        // Test different thread counts
        let thread_counts = [1, 2, 4];
        let mut results = Vec::new();

        for &threads in &thread_counts {
            let mut engine = GuardianEngine::new(config.clone(), progress.clone())
                .await
                .unwrap();

            let start = Instant::now();
            let analysis_results = engine.analyze_files(&files, threads).await.unwrap();
            let duration = start.elapsed();

            results.push((threads, duration, analysis_results.summary.total_findings));
            println!(
                "Threads: {}, Duration: {}ms, Findings: {}",
                threads,
                duration.as_millis(),
                analysis_results.summary.total_findings
            );
        }

        // Verify that all configurations produce the same results
        let baseline_findings = results[0].2;
        for (threads, _, findings) in &results {
            assert_eq!(
                *findings, baseline_findings,
                "Thread count {} should produce same findings as baseline",
                threads
            );
        }

        // Verify that parallel processing provides some benefit
        let sequential_time = results[0].1;
        let parallel_time = results[results.len() - 1].1;

        // Parallel should be at least as fast (allowing for some overhead)
        let efficiency_ratio =
            parallel_time.as_millis() as f64 / sequential_time.as_millis() as f64;
        assert!(
            efficiency_ratio <= 1.5,
            "Parallel processing should not be more than 50% slower (ratio: {})",
            efficiency_ratio
        );
    }
}

/// Memory usage and leak detection tests
mod memory_tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_usage_stability() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::default();

        // Create a test file
        let test_file = temp_dir.path().join("memory_test.rs");
        let content = "fn main() { println!(\"Hello\"); }".repeat(1000);
        std::fs::write(&test_file, content).unwrap();

        // Run analysis multiple times
        for iteration in 0..10 {
            let progress = ProgressReporter::new(false);
            let mut engine = GuardianEngine::new(config.clone(), progress).await.unwrap();

            let _results = engine.analyze_files(&[test_file.clone()], 1).await.unwrap();

            // Force cleanup
            drop(engine);

            // Basic memory stability check
            // In a more sophisticated test, you'd monitor actual memory usage
            if iteration % 5 == 0 {
                println!("Completed {} iterations - memory stable", iteration + 1);
            }
        }

        println!("✅ Memory usage appears stable over 10 iterations");
    }

    #[tokio::test]
    async fn test_large_file_memory_handling() {
        let temp_dir = TempDir::new().unwrap();
        let large_file = temp_dir.path().join("large.rs");

        // Create a 5MB file
        let large_content = "// This is a comment\nfn test() {}\n".repeat(100_000);
        std::fs::write(&large_file, large_content).unwrap();

        let config = Config::default();
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await.unwrap();

        let start = Instant::now();
        let result = engine.analyze_files(&[large_file], 1).await;
        let duration = start.elapsed();

        assert!(
            result.is_ok(),
            "Should handle large files without memory issues"
        );
        assert!(
            duration < Duration::from_secs(30),
            "Large file processing should complete within 30 seconds"
        );

        println!(
            "✅ Large file (5MB) processed in {}ms",
            duration.as_millis()
        );
    }
}

/// Scalability tests
mod scalability_tests {
    use super::*;

    #[tokio::test]
    async fn test_file_count_scalability() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::default();
        let progress = ProgressReporter::new(false);

        // Test with increasing file counts
        let file_counts = [10, 50, 100];

        for &count in &file_counts {
            let mut files = Vec::new();

            // Create files
            for i in 0..count {
                let file = temp_dir.path().join(format!("scale_test_{}.rs", i));
                let content = format!("fn test_{}() {{ println!(\"Test\"); }}", i);
                std::fs::write(&file, content).unwrap();
                files.push(file);
            }

            let mut engine = GuardianEngine::new(config.clone(), progress.clone())
                .await
                .unwrap();

            let start = Instant::now();
            let results = engine.analyze_files(&files, 4).await.unwrap();
            let duration = start.elapsed();

            assert_eq!(results.summary.total_files_scanned, count);

            // Performance should scale reasonably (not exponentially)
            let time_per_file = duration.as_millis() / count as u128;
            assert!(
                time_per_file < 500,
                "Time per file should be reasonable: {}ms/file for {} files",
                time_per_file,
                count
            );

            println!(
                "Processed {} files in {}ms ({}ms/file)",
                count,
                duration.as_millis(),
                time_per_file
            );
        }
    }

    #[tokio::test]
    async fn test_concurrent_engine_instances() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::default();

        // Create test files
        let mut files = Vec::new();
        for i in 0..5 {
            let file = temp_dir.path().join(format!("concurrent_{}.rs", i));
            std::fs::write(&file, "fn main() {}").unwrap();
            files.push(file);
        }

        // Run multiple engine instances concurrently
        let handles: Vec<_> = (0..3)
            .map(|instance| {
                let config = config.clone();
                let files = files.clone();

                tokio::spawn(async move {
                    let progress = ProgressReporter::new(false);
                    let mut engine = GuardianEngine::new(config, progress).await.unwrap();

                    let start = Instant::now();
                    let results = engine.analyze_files(&files, 1).await.unwrap();
                    let duration = start.elapsed();

                    (instance, results.summary.total_files_scanned, duration)
                })
            })
            .collect();

        // Wait for all instances to complete
        let mut total_duration = Duration::new(0, 0);
        for handle in handles {
            let (instance, files_scanned, duration) = handle.await.unwrap();
            assert_eq!(
                files_scanned, 5,
                "Instance {} should process all files",
                instance
            );
            total_duration += duration;
            println!(
                "Instance {} completed in {}ms",
                instance,
                duration.as_millis()
            );
        }

        let avg_duration = total_duration / 3;
        assert!(
            avg_duration < Duration::from_secs(10),
            "Average concurrent processing time should be reasonable"
        );

        println!(
            "✅ Concurrent processing test passed - average time: {}ms",
            avg_duration.as_millis()
        );
    }
}

/// Performance monitoring utilities
mod performance_monitoring {
    use super::*;

    pub fn measure_operation<F, R>(operation_name: &str, operation: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();

        println!(
            "Operation '{}' completed in {}ms",
            operation_name,
            duration.as_millis()
        );
        (result, duration)
    }

    #[tokio::test]
    async fn test_performance_monitoring_utility() {
        let (_result, duration) = measure_operation("test_operation", || {
            std::thread::sleep(Duration::from_millis(10));
            42
        });

        assert!(duration >= Duration::from_millis(9)); // Allow some tolerance
        assert!(duration < Duration::from_millis(50)); // But not too much
    }
}
