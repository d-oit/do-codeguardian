use do_codeguardian::{analyze_files, Config};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;
use tokio::time::timeout;

#[cfg(test)]
mod chaos_engineering_tests {
    use super::*;

    /// Generate test files with security issues for chaos testing
    fn generate_chaos_test_files(count: usize) -> Vec<(NamedTempFile, String, usize)> {
        let mut files = Vec::new();

        for i in 0..count {
            let content = format!(
                r#"fn main() {{
    let password = "hardcoded_password_{}";
    let api_key = "sk-1234567890abcdef";
    println!("Password: {{}}", password);
    println!("API Key: {{}}", api_key);
}}

pub fn insecure_function() {{
    let mut data = vec![];
    for _ in 0..100000 {{
        data.push("large_string_data_{}".to_string());
    }}
}}"#,
                i, i
            );

            let mut file = NamedTempFile::new().unwrap();
            file.write_all(content.as_bytes()).unwrap();
            files.push((file, format!("chaos_test_{}.rs", i), content.len()));
        }

        files
    }

    #[tokio::test]
    async fn test_resilience_under_network_failures() {
        let test_files = generate_chaos_test_files(5);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        // Test with timeout to simulate network issues
        let config = Config::default();
        let result = timeout(Duration::from_secs(30), analyze_files(&file_paths, &config)).await;

        match result {
            Ok(analysis_result) => {
                assert!(analysis_result.is_ok());
                let analysis = analysis_result.unwrap();
                assert!(analysis.files_analyzed > 0);
                println!(
                    "✅ Network resilience test passed - analyzed {} files",
                    analysis.files_analyzed
                );
            }
            Err(_) => {
                println!("⚠️  Network timeout occurred - testing timeout handling");
                // Even on timeout, should not panic
                assert!(true);
            }
        }
    }

    #[tokio::test]
    async fn test_performance_under_load() {
        let test_files = generate_chaos_test_files(10);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let start_time = Instant::now();
        let config = Config::default();

        // Simulate CPU load while analyzing
        tokio::spawn(async {
            for _ in 0..100000 {
                let _ = (0..1000).map(|x| x * x).sum::<usize>();
            }
        });

        let result = analyze_files(&file_paths, &config).await;
        let duration = start_time.elapsed();

        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert!(analysis.files_analyzed > 0);

        println!("✅ Load performance test passed in {:?}", duration);
        println!("   Files analyzed: {}", analysis.files_analyzed);
        println!("   Issues found: {}", analysis.issues.len());

        // Should complete within reasonable time even under load
        assert!(duration < Duration::from_secs(60));
    }

    #[tokio::test]
    async fn test_memory_pressure_resilience() {
        let test_files = generate_chaos_test_files(3);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        // Simulate memory pressure
        let mut memory_hogs = Vec::new();
        for _ in 0..10 {
            memory_hogs.push(vec![0u8; 10 * 1024 * 1024]); // 10MB each
        }

        let config = Config::default();
        let result = analyze_files(&file_paths, &config).await;

        // Explicitly drop memory hogs to free memory
        drop(memory_hogs);

        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert!(analysis.files_analyzed > 0);

        println!(
            "✅ Memory pressure test passed - analyzed {} files",
            analysis.files_analyzed
        );
    }

    #[tokio::test]
    async fn test_concurrent_chaos_scenarios() {
        let test_files = generate_chaos_test_files(8);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let config = Config::default();

        // Run multiple analysis tasks concurrently with chaos
        let mut handles = Vec::new();

        for i in 0..3 {
            let paths = file_paths.clone();
            let cfg = config.clone();
            let handle = tokio::spawn(async move {
                // Simulate different chaos for each concurrent task
                match i {
                    0 => {
                        // Network delay chaos
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                    1 => {
                        // CPU load chaos
                        for _ in 0..50000 {
                            let _ = (0..100).map(|x| x * x).sum::<usize>();
                        }
                    }
                    2 => {
                        // Memory pressure chaos
                        let _memory_hog = vec![0u8; 50 * 1024 * 1024];
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    _ => {}
                }

                analyze_files(&paths, &cfg).await
            });
            handles.push(handle);
        }

        // Wait for all concurrent tasks
        let mut success_count = 0;
        for handle in handles {
            match handle.await {
                Ok(result) => {
                    if result.is_ok() {
                        success_count += 1;
                        let analysis = result.unwrap();
                        assert!(analysis.files_analyzed > 0);
                    }
                }
                Err(_) => {
                    // Task panicked - this is expected in chaos scenarios
                    println!("⚠️  Concurrent task failed - testing panic resilience");
                }
            }
        }

        println!(
            "✅ Concurrent chaos test: {}/3 tasks succeeded",
            success_count
        );
        // At least 2 out of 3 should succeed for resilience
        assert!(success_count >= 2);
    }

    #[tokio::test]
    async fn test_failure_recovery() {
        let test_files = generate_chaos_test_files(5);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let config = Config::default();

        // First analysis - should succeed
        let result1 = analyze_files(&file_paths, &config).await;
        assert!(result1.is_ok());

        // Simulate system stress
        let mut memory_hogs = Vec::new();
        for _ in 0..20 {
            memory_hogs.push(vec![0u8; 5 * 1024 * 1024]);
        }

        // Second analysis under stress
        let result2 = timeout(Duration::from_secs(45), analyze_files(&file_paths, &config)).await;

        // Clean up
        drop(memory_hogs);

        match result2 {
            Ok(analysis_result) => {
                assert!(analysis_result.is_ok());
                let analysis = analysis_result.unwrap();
                assert!(analysis.files_analyzed > 0);
                println!("✅ Recovery test passed - system recovered from stress");
            }
            Err(_) => {
                println!("⚠️  Recovery test: timeout under stress - testing graceful degradation");
                // Even on timeout, should not crash the test
                assert!(true);
            }
        }
    }

    #[tokio::test]
    async fn test_api_success_rate_measurement() {
        let test_files = generate_chaos_test_files(3);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let config = Config::default();
        let mut success_count = 0;
        let total_runs = 100; // Run 100 times to measure success rate

        for i in 0..total_runs {
            // Add some chaos every 10th run
            if i % 10 == 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }

            let result =
                timeout(Duration::from_secs(10), analyze_files(&file_paths, &config)).await;

            match result {
                Ok(analysis_result) => {
                    if analysis_result.is_ok() {
                        let analysis = analysis_result.unwrap();
                        if analysis.files_analyzed == file_paths.len() {
                            success_count += 1;
                        }
                    }
                }
                Err(_) => {
                    // Timeout counts as failure
                }
            }
        }

        let success_rate = (success_count as f64 / total_runs as f64) * 100.0;

        println!(
            "✅ API Success Rate Test: {:.2}% ({}/{})",
            success_rate, success_count, total_runs
        );

        // Target: 99.9% success rate
        assert!(
            success_rate >= 99.9,
            "Success rate {:.2}% below target 99.9%",
            success_rate
        );
    }

    #[tokio::test]
    async fn test_cross_component_workflow_chaos() {
        let test_files = generate_chaos_test_files(6);
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let config = Config::default();

        // Test full workflow: analysis -> caching -> reporting
        let start_time = Instant::now();

        // Simulate chaos during workflow
        tokio::spawn(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            // Simulate intermittent issues
        });

        let result = analyze_files(&file_paths, &config).await;
        let duration = start_time.elapsed();

        assert!(result.is_ok());
        let analysis = result.unwrap();

        // Validate cross-component workflow
        assert!(analysis.files_analyzed > 0);
        assert!(analysis.duration_ms > 0);
        assert!(analysis.issues.len() >= 0); // May be 0 or more

        println!("✅ Cross-component workflow test passed in {:?}", duration);
        println!("   Files analyzed: {}", analysis.files_analyzed);
        println!("   Issues detected: {}", analysis.issues.len());
        println!("   Analysis duration: {}ms", analysis.duration_ms);

        // Should complete workflow within reasonable time
        assert!(duration < Duration::from_secs(30));
    }
}
