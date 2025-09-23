//! Performance Integration Tests
//!
//! Performance benchmarking and scaling tests for CodeGuardian.

use do_codeguardian::core::GuardianEngine;
use do_codeguardian::utils::progress::ProgressReporter;
use do_codeguardian::Config;
use std::time::Instant;
use tempfile::TempDir;

/// Performance integration tests
#[cfg(test)]
mod performance_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_analysis_performance_scaling() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::default();

        // Test with different numbers of files
        let file_counts = [1, 5, 10, 20];
        let mut times = Vec::new();

        for &count in &file_counts {
            let mut files = Vec::new();
            for i in 0..count {
                let file = temp_dir.path().join(format!("perf_test_{}.rs", i));
                std::fs::write(&file, "fn main() { println!(\"Hello\"); }").unwrap();
                files.push(file);
            }

            let mut engine = GuardianEngine::new(config.clone(), ProgressReporter::new(false))
                .await
                .unwrap();

            let start = Instant::now();
            let _results = engine.analyze_files(&files, 2).await.unwrap();
            let duration = start.elapsed();

            times.push((count, duration));
        }

        // Performance should scale reasonably (not exponentially)
        // This is a basic sanity check
        for (count, duration) in times {
            assert!(
                duration.as_secs() < 60,
                "Analysis of {} files should complete within 60s (took {}s)",
                count,
                duration.as_secs()
            );
        }
    }

    #[tokio::test]
    async fn test_parallel_vs_sequential_performance() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::default();

        // Create test files
        let mut files = Vec::new();
        for i in 0..10 {
            let file = temp_dir.path().join(format!("parallel_test_{}.rs", i));
            let content = format!("fn test_{}() {{ println!(\"test\"); }}", i);
            std::fs::write(&file, content).unwrap();
            files.push(file);
        }

        // Test sequential processing
        let mut engine_seq = GuardianEngine::new(config.clone(), ProgressReporter::new(false))
            .await
            .unwrap();
        let start_seq = Instant::now();
        let _results_seq = engine_seq.analyze_files(&files, 1).await.unwrap();
        let duration_seq = start_seq.elapsed();

        // Test parallel processing
        let mut engine_par = GuardianEngine::new(config, ProgressReporter::new(false))
            .await
            .unwrap();
        let start_par = Instant::now();
        let _results_par = engine_par.analyze_files(&files, 4).await.unwrap();
        let duration_par = start_par.elapsed();

        // Parallel should be faster or at least not significantly slower
        // Allow some tolerance for overhead
        let speedup_ratio = duration_seq.as_millis() as f64 / duration_par.as_millis() as f64;
        assert!(
            speedup_ratio >= 0.5,
            "Parallel processing should not be more than 2x slower (ratio: {})",
            speedup_ratio
        );
    }
}
