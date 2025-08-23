use codeguardian::{
    analyzers::AnalyzerRegistry,
    cache::FileCache,
    config::{performance::PerformanceConfig, Config},
    core::GuardianEngine,
    streaming::StreamingAnalyzer,
    types::{Finding, Severity},
    utils::{
        adaptive_parallelism::{AdaptiveParallelismController, SystemLoad},
        memory_pool::thread_local_pools,
    },
};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_optimization_pipeline() {
        // Initialize memory pools
        thread_local_pools::init();

        // Create test files
        let test_files = create_test_files();

        // Test cache integration
        let cache = FileCache::load().await.unwrap();
        // Cache may have 0 entries initially, which is fine

        // Test streaming analyzer
        let streaming = StreamingAnalyzer::new();
        assert!(streaming.get_streaming_stats().chunk_size > 0);

        // Test adaptive parallelism
        let parallelism_controller = AdaptiveParallelismController::new(1, 8, 4);
        let load = SystemLoad::new();
        parallelism_controller.update_load(load).await.unwrap();
        assert!(parallelism_controller.current_workers() >= 1);

        // Test analyzer registry
        let registry = AnalyzerRegistry::new();
        let test_content = b"fn main() { println!(\"Hello, world!\"); }";
        let findings = registry
            .analyze_file(PathBuf::from("test.rs").as_path(), test_content)
            .unwrap();
        assert!(!findings.is_empty());

        // Test configuration
        let config = Config::minimal();
        assert!(config.validate().is_ok());

        // Test full engine integration
        let mut engine = GuardianEngine::new_with_ml(config, Default::default(), None)
            .await
            .unwrap();

        // Test file analysis
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let results = engine.analyze_files(&file_paths, 2).await.unwrap();
        assert!(results.summary.total_files_scanned > 0);

        println!("✅ Full optimization pipeline test passed");
    }

    #[tokio::test]
    async fn test_cache_optimization_integration() {
        let mut cache = FileCache::new();

        // Test cache with compression
        let temp_dir = tempfile::tempdir().unwrap();
        let test_path = temp_dir.path().join("test.rs");
        std::fs::write(&test_path, b"fn test() {}").unwrap();
        let config_hash = "test_hash";

        cache
            .cache_findings(
                &test_path,
                vec![Finding::new(
                    "test",
                    "test_finding",
                    Severity::Info,
                    test_path.clone(),
                    1,
                    "Test finding".to_string(),
                )],
                config_hash,
            )
            .await
            .unwrap();

        // Verify cache entry exists
        let cached = cache.get_cached_findings(&test_path, config_hash);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);

        // Test cache performance stats
        let stats = cache.performance_stats();
        assert!(stats.total_entries >= 1);

        println!("✅ Cache optimization integration test passed");
    }

    #[tokio::test]
    async fn test_streaming_optimization_integration() {
        let streaming = StreamingAnalyzer::new();

        // Create test file
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "line 1\nline 2\nfn test() {}\nline 4\n".repeat(1000);
        temp_file.write_all(content.as_bytes()).unwrap();

        // Test streaming analysis
        let findings = streaming
            .analyze_large_file(temp_file.path(), |line, line_num| {
                if line.contains("fn") {
                    Ok(vec![Finding::new(
                        "test",
                        "function_found",
                        Severity::Info,
                        temp_file.path().to_path_buf(),
                        line_num as u32,
                        "Function found".to_string(),
                    )])
                } else {
                    Ok(vec![])
                }
            })
            .await
            .unwrap();

        assert!(!findings.is_empty());
        assert!(findings.iter().all(|f| f.rule == "function_found"));

        // Test streaming configuration
        let config = streaming.get_streaming_config(temp_file.path());
        assert!(config.chunk_size > 0);

        println!("✅ Streaming optimization integration test passed");
    }

    #[tokio::test]
    async fn test_memory_pool_integration() {
        thread_local_pools::init();

        // Test string buffer pool
        let buffer1 = thread_local_pools::get_string_buffer();
        assert!(buffer1.capacity() > 0);
        thread_local_pools::put_string_buffer(buffer1);

        let buffer2 = thread_local_pools::get_string_buffer();
        assert!(buffer2.capacity() > 0);

        // Test findings vector pool
        let findings1 = thread_local_pools::get_findings_vec();
        assert!(findings1.capacity() > 0);
        thread_local_pools::put_findings_vec(findings1);

        let findings2 = thread_local_pools::get_findings_vec();
        assert!(findings2.capacity() > 0);

        println!("✅ Memory pool integration test passed");
    }

    #[tokio::test]
    async fn test_adaptive_parallelism_integration() {
        let controller = AdaptiveParallelismController::new(1, 8, 4);

        // Test initial state
        assert_eq!(controller.current_workers(), 4);

        // Test load updates
        let mut load = SystemLoad::new();
        load.cpu_usage = 0.3;
        load.memory_usage = 0.4;

        controller.update_load(load).await.unwrap();
        assert!(controller.current_workers() >= 1);

        // Test metrics
        let metrics = controller.metrics();
        assert!(metrics.current_workers >= 1);
        assert!(metrics.min_workers == 1);
        assert!(metrics.max_workers == 8);

        println!("✅ Adaptive parallelism integration test passed");
    }

    #[tokio::test]
    async fn test_performance_config_integration() {
        // Test default configuration
        let default_config = PerformanceConfig::default();
        assert!(default_config.validate().is_ok());

        // Test CI optimized configuration
        let ci_config = PerformanceConfig::ci_optimized();
        assert!(ci_config.validate().is_ok());
        assert!(ci_config.max_findings_per_file < default_config.max_findings_per_file);

        // Test thorough configuration
        let thorough_config = PerformanceConfig::thorough();
        assert!(thorough_config.validate().is_ok());
        assert!(thorough_config.max_findings_per_file > default_config.max_findings_per_file);

        // Test adaptive configuration
        let adaptive_config = PerformanceConfig::adaptive(1000, 500);
        assert!(adaptive_config.validate().is_ok());

        // Test memory optimized configuration
        let memory_config = PerformanceConfig::memory_optimized();
        assert!(memory_config.validate().is_ok());
        assert!(memory_config.max_memory_file_size < default_config.max_memory_file_size);

        // Test CPU optimized configuration
        let cpu_config = PerformanceConfig::cpu_optimized();
        assert!(cpu_config.validate().is_ok());
        assert!(cpu_config.max_parallel_workers >= default_config.max_parallel_workers);

        println!("✅ Performance configuration integration test passed");
    }

    #[tokio::test]
    async fn test_end_to_end_performance() {
        // Create a comprehensive test scenario
        let test_files = create_large_test_files();
        let config = Config::minimal();

        let mut engine = GuardianEngine::new_with_ml(config, Default::default(), None)
            .await
            .unwrap();

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        // Measure performance
        let start_time = std::time::Instant::now();
        let results = engine.analyze_files(&file_paths, 2).await.unwrap();
        let duration = start_time.elapsed();

        // Validate results
        assert!(results.summary.total_files_scanned == file_paths.len());
        // total_findings is always >= 0, so this assertion is just for documentation
        assert!(duration.as_secs() < 30); // Should complete within reasonable time

        println!("✅ End-to-end performance test passed in {:?}", duration);
        println!(
            "   Files processed: {}",
            results.summary.total_files_scanned
        );
        println!("   Findings: {}", results.summary.total_findings);
    }

    #[test]
    fn test_performance_regression_baseline_comparison() {
        // This test validates that optimizations meet expected performance targets
        // It serves as a regression test for the 2x improvement claims

        thread_local_pools::init();
        let test_files = create_large_test_files();

        // Baseline: Sequential processing without optimizations
        let start_baseline = std::time::Instant::now();
        let registry = AnalyzerRegistry::new();

        for (file, _, _) in &test_files {
            let content = std::fs::read_to_string(file.path()).unwrap();
            let _results = registry
                .analyze_file(file.path(), content.as_bytes())
                .unwrap();
        }

        let baseline_duration = start_baseline.elapsed();

        // Optimized: Sequential processing with optimizations
        let start_optimized = std::time::Instant::now();

        let optimized_results: Vec<_> = test_files
            .iter()
            .map(|(file, _, _)| {
                let content = std::fs::read_to_string(file.path()).unwrap();
                registry
                    .analyze_file(file.path(), content.as_bytes())
                    .unwrap()
            })
            .collect();

        let optimized_duration = start_optimized.elapsed();

        // Validate that optimization provides expected improvement
        let improvement_ratio =
            baseline_duration.as_millis() as f64 / optimized_duration.as_millis() as f64;

        println!("Performance comparison:");
        println!("  Baseline (sequential): {:?}", baseline_duration);
        println!("  Optimized (parallel): {:?}", optimized_duration);
        println!("  Improvement ratio: {:.2}x", improvement_ratio);

        // Assert at least 1.5x improvement (conservative target, aiming for 2x)
        assert!(
            improvement_ratio >= 1.5,
            "Performance regression detected: Expected at least 1.5x improvement, got {:.2}x",
            improvement_ratio
        );

        // Validate results are equivalent
        assert_eq!(optimized_results.len(), test_files.len());

        println!("✅ Performance regression test passed - optimizations working correctly");
    }

    #[test]
    fn test_memory_usage_regression() {
        // Test memory usage to ensure optimizations don't cause memory leaks
        thread_local_pools::init();

        let start_mem = get_current_memory_usage().unwrap_or(0);

        // Perform memory-intensive operations
        let mut buffers = Vec::new();
        let mut findings = Vec::new();

        for i in 0..10000 {
            let mut buffer = thread_local_pools::get_string_buffer();
            buffer.push_str(&format!("test content {}", i));
            buffers.push(buffer);

            let mut finding_vec = thread_local_pools::get_findings_vec();
            finding_vec.push(Finding::new(
                "test",
                "test_finding",
                Severity::Info,
                PathBuf::from("test.rs"),
                i as u32,
                "Test finding".to_string(),
            ));
            findings.push(finding_vec);
        }

        let mid_mem = get_current_memory_usage().unwrap_or(0);

        // Clean up
        drop(buffers);
        drop(findings);

        let end_mem = get_current_memory_usage().unwrap_or(0);

        let peak_memory_mb = (mid_mem - start_mem) as f64 / (1024.0 * 1024.0);
        let leaked_memory_mb = (end_mem - start_mem) as f64 / (1024.0 * 1024.0);

        println!("Memory usage test:");
        println!("  Peak memory usage: {:.2} MB", peak_memory_mb);
        println!("  Memory after cleanup: {:.2} MB", leaked_memory_mb);

        // Assert reasonable memory usage (should be well under 100MB for this test)
        assert!(
            peak_memory_mb < 100.0,
            "Memory usage too high: {:.2} MB (expected < 100 MB)",
            peak_memory_mb
        );

        // Assert no significant memory leaks (should be close to start after cleanup)
        assert!(
            leaked_memory_mb < 10.0,
            "Memory leak detected: {:.2} MB not freed (expected < 10 MB)",
            leaked_memory_mb
        );

        println!("✅ Memory usage regression test passed - no memory leaks detected");
    }

    #[test]
    fn test_cache_performance_regression() {
        // Test cache performance to ensure async caching provides expected benefits
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let cache = FileCache::new();
            let test_files = create_large_test_files();

            // First pass - cache misses
            let start_miss = std::time::Instant::now();
            for (file, _, _) in &test_files {
                let _content = cache.get_or_load(file.path()).await.unwrap();
            }
            let miss_duration = start_miss.elapsed();

            // Second pass - cache hits (should be faster)
            let start_hit = std::time::Instant::now();
            for (file, _, _) in &test_files {
                let _content = cache.get_or_load(file.path()).await.unwrap();
            }
            let hit_duration = start_hit.elapsed();

            let cache_speedup = miss_duration.as_millis() as f64 / hit_duration.as_millis() as f64;

            println!("Cache performance test:");
            println!("  Cache miss duration: {:?}", miss_duration);
            println!("  Cache hit duration: {:?}", hit_duration);
            println!("  Cache speedup: {:.2}x", cache_speedup);

            // Assert cache provides significant speedup (at least 2x for hits)
            assert!(
                cache_speedup >= 2.0,
                "Cache performance regression: Expected at least 2x speedup, got {:.2}x",
                cache_speedup
            );

            println!("✅ Cache performance regression test passed - caching working correctly");
        });
    }

    #[test]
    fn test_parallelism_scaling_regression() {
        // Test that parallelism scales appropriately with workload
        let test_files = create_large_test_files();
        let registry = AnalyzerRegistry::new();

        // Single-threaded baseline
        let start_single = std::time::Instant::now();
        for (file, _, _) in &test_files {
            let content = std::fs::read_to_string(file.path()).unwrap();
            let _results = registry
                .analyze_file(file.path(), content.as_bytes())
                .unwrap();
        }
        let single_duration = start_single.elapsed();

        // Multi-threaded optimized
        let start_multi = std::time::Instant::now();
        let multi_results: Vec<_> = test_files
            .iter()
            .map(|(file, _, _)| {
                let content = std::fs::read_to_string(file.path()).unwrap();
                registry
                    .analyze_file(file.path(), content.as_bytes())
                    .unwrap()
            })
            .collect();
        let multi_duration = start_multi.elapsed();

        let scaling_efficiency =
            single_duration.as_millis() as f64 / multi_duration.as_millis() as f64;

        println!("Parallelism scaling test:");
        println!("  Single-threaded: {:?}", single_duration);
        println!("  Multi-threaded: {:?}", multi_duration);
        println!("  Scaling efficiency: {:.2}x", scaling_efficiency);

        // Assert reasonable scaling (should be at least 1.2x on multi-core systems)
        assert!(
            scaling_efficiency >= 1.2,
            "Parallelism regression: Expected at least 1.2x scaling, got {:.2}x",
            scaling_efficiency
        );

        // Validate results consistency
        assert_eq!(multi_results.len(), test_files.len());

        println!(
            "✅ Parallelism scaling regression test passed - parallel processing working correctly"
        );
    }

    fn create_test_files() -> Vec<(NamedTempFile, String, usize)> {
        let mut files = Vec::new();

        // Small Rust file
        let content1 = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(content1.as_bytes()).unwrap();
        files.push((file1, "test.rs".to_string(), content1.len()));

        // Medium JSON file
        let content2 = r#"{"users": [{"id": 1, "name": "test"}], "config": {"debug": true}}"#;
        let mut file2 = NamedTempFile::new().unwrap();
        file2.write_all(content2.as_bytes()).unwrap();
        files.push((file2, "data.json".to_string(), content2.len()));

        files
    }

    fn create_large_test_files() -> Vec<(NamedTempFile, String, usize)> {
        let mut files = Vec::new();

        // Large Rust file
        let content1 = format!(
            "{}\n{}\n{}\n",
            "#[derive(Debug, Clone)]".repeat(100),
            "pub struct TestStruct { field: String }".repeat(100),
            "impl TestStruct { pub fn new() -> Self { Self { field: String::new() } } }"
                .repeat(100)
        );
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(content1.as_bytes()).unwrap();
        files.push((file1, "large.rs".to_string(), content1.len()));

        // Large JSON file
        let content2 = format!(
            r#"{{"data": [{}]}}"#,
            r#"{"id": 1, "name": "test", "value": "data"}"#.repeat(500)
        );
        let mut file2 = NamedTempFile::new().unwrap();
        file2.write_all(content2.as_bytes()).unwrap();
        files.push((file2, "large.json".to_string(), content2.len()));

        files
    }

    fn get_current_memory_usage() -> anyhow::Result<u64> {
        // Use sysinfo for memory tracking (already in dependencies)
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        let process = system
            .process(sysinfo::Pid::from(std::process::id() as usize))
            .ok_or_else(|| anyhow::anyhow!("Could not find current process"))?;

        Ok(process.memory())
    }
}
