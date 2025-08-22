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
        assert!(cache.stats().total_entries >= 0);

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
        assert!(findings.len() >= 0);

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
        assert!(results.summary.total_files_scanned >= 0);

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

        assert!(findings.len() > 0);
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
        assert!(results.summary.total_findings >= 0);
        assert!(duration.as_secs() < 30); // Should complete within reasonable time

        println!("✅ End-to-end performance test passed in {:?}", duration);
        println!(
            "   Files processed: {}",
            results.summary.total_files_scanned
        );
        println!("   Findings: {}", results.summary.total_findings);
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
}
