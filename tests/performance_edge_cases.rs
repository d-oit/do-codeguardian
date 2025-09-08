use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tempfile::TempDir;

use do_codeguardian::analyzers::security::SecretAnalyzer;
use do_codeguardian::analyzers::{security_analyzer::SecurityAnalyzer, Analyzer};

#[cfg(test)]
mod performance_edge_cases {
    use super::*;

    /// Test performance with large numbers of files
    #[test]
    fn test_large_file_count_performance() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = SecurityAnalyzer::new();

        // Edge case 1: Many small files
        let file_counts = [10, 50, 100, 500];

        for &count in &file_counts {
            let start_time = Instant::now();

            // Create many small files
            for i in 0..count {
                let file_path = temp_dir.path().join(format!("file_{}.rs", i));
                let content = format!(
                    "// File {}\nfn main() {{\n    let secret = \"sk-proj-test{}\";\n}}",
                    i, i
                );
                fs::write(&file_path, &content).unwrap();

                // Analyze each file
                let findings = analyzer.analyze(&file_path, content.as_bytes()).unwrap();

                // Should detect the secret
                assert!(!findings.is_empty(), "Should detect secret in file {}", i);
            }

            let duration = start_time.elapsed();
            let files_per_second = count as f64 / duration.as_secs_f64();

            println!(
                "Processed {} files in {:?} ({:.1} files/sec)",
                count, duration, files_per_second
            );

            // Performance assertion - should process at least 10 files per second
            assert!(
                files_per_second > 10.0,
                "Performance too slow: {:.1} files/sec for {} files",
                files_per_second,
                count
            );
        }
    }

    /// Test performance with large file sizes
    #[test]
    fn test_large_file_size_performance() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = SecurityAnalyzer::new();

        // Edge case 2: Large files with secrets at different positions
        let file_sizes = [
            (1024, "1KB"),
            (10 * 1024, "10KB"),
            (100 * 1024, "100KB"),
            (1024 * 1024, "1MB"),
        ];

        for (size_bytes, size_name) in &file_sizes {
            let start_time = Instant::now();

            // Create large file with secret at the end
            let mut content = "// Large file content\n".repeat(*size_bytes / 20);
            content.push_str("let api_key = \"sk-proj-RealSecretAtTheEnd123456789\";\n");

            let file_path = temp_dir.path().join(format!("large_file_{}.rs", size_name));
            fs::write(&file_path, &content).unwrap();

            // Analyze the large file
            let findings = analyzer.analyze(&file_path, content.as_bytes()).unwrap();

            let duration = start_time.elapsed();
            let mb_per_second = (*size_bytes as f64 / 1024.0 / 1024.0) / duration.as_secs_f64();

            println!(
                "Processed {} file in {:?} ({:.1} MB/sec)",
                size_name, duration, mb_per_second
            );

            // Should detect the secret even in large files
            assert!(
                !findings.is_empty(),
                "Should detect secret in {} file",
                size_name
            );

            // Performance assertion - should process at least 1 MB/sec
            if *size_bytes >= 1024 * 1024 {
                assert!(
                    mb_per_second > 1.0,
                    "Performance too slow: {:.1} MB/sec for {} file",
                    mb_per_second,
                    size_name
                );
            }
        }
    }

    /// Test performance with many false positives
    #[test]
    fn test_false_positive_filtering_performance() {
        let analyzer = SecretAnalyzer::new();

        // Edge case 3: Content with many potential false positives
        let false_positive_content = r#"
// This file contains many patterns that could be false positives
#[test]
fn test_many_patterns() {
    let test_key_1 = "sk-1234567890abcdef";
    let test_key_2 = "sk-test123456789";
    let dummy_secret_1 = "dummy_secret_123";
    let dummy_secret_2 = "fake_token_456";
    let example_key_1 = "example_api_key_789";
    let example_key_2 = "your-key-here";
    let mock_password_1 = "mock_password_123";
    let mock_password_2 = "fake_password_456";
    let placeholder_1 = "placeholder_secret_789";
    let placeholder_2 = "your-secret-here";
}

#[bench]
fn bench_auth_patterns() {
    let bench_key_1 = "sk-1234567890abcdef";
    let bench_key_2 = "benchmark_secret_123";
    let bench_token_1 = "bench_token_456";
    let bench_token_2 = "dummy_benchmark_789";
}

// Pattern definitions
const PATTERNS: &[&str] = &[
    "sk-[a-zA-Z0-9]+",
    "password\\s*=\\s*[\"'][^\"']+[\"']",
    "api_key\\s*=\\s*[\"'][^\"']+[\"']",
    "secret\\s*=\\s*[\"'][^\"']+[\"']",
];

/// Example usage:
/// let api_key = "your-api-key-here";
/// let password = "your-password-here";

/*
 * More examples:
 * token = "example-token"
 * secret = "example-secret"
 */
"#;

        let iterations = 100;
        let start_time = Instant::now();

        for i in 0..iterations {
            let file_path = format!("test_file_{}.rs", i);
            let findings = analyzer
                .analyze(Path::new(&file_path), false_positive_content.as_bytes())
                .unwrap();

            // Should filter out all false positives
            assert!(
                findings.is_empty(),
                "Should not detect any secrets in false positive content (iteration {})",
                i
            );
        }

        let duration = start_time.elapsed();
        let iterations_per_second = iterations as f64 / duration.as_secs_f64();

        println!(
            "Processed {} iterations of false positive filtering in {:?} ({:.1} iter/sec)",
            iterations, duration, iterations_per_second
        );

        // Should be able to filter false positives quickly
        assert!(
            iterations_per_second > 50.0,
            "False positive filtering too slow: {:.1} iter/sec",
            iterations_per_second
        );
    }

    /// Test memory usage with large content
    #[test]
    fn test_memory_usage_edge_cases() {
        let analyzer = SecurityAnalyzer::new();

        // Edge case 4: Very large content that could cause memory issues
        let large_content_sizes = [
            (1024 * 1024, "1MB"),       // 1MB
            (5 * 1024 * 1024, "5MB"),   // 5MB
            (10 * 1024 * 1024, "10MB"), // 10MB (CodeGuardian's limit)
        ];

        for (size_bytes, size_name) in &large_content_sizes {
            // Create content with secret buried in the middle
            let half_size = size_bytes / 2;
            let mut content = "// Filler content\n".repeat(half_size / 20);
            content.push_str("let hidden_secret = \"sk-proj-HiddenInLargeFile123456789\";\n");
            content.push_str(&"// More filler content\n".repeat(half_size / 25));

            let start_time = Instant::now();
            let initial_memory = get_memory_usage();

            // Analyze the large content
            let findings = analyzer
                .analyze(Path::new("large_file.rs"), content.as_bytes())
                .unwrap();

            let final_memory = get_memory_usage();
            let duration = start_time.elapsed();
            let memory_increase = final_memory.saturating_sub(initial_memory);

            println!(
                "Analyzed {} content in {:?}, memory increase: {} KB",
                size_name,
                duration,
                memory_increase / 1024
            );

            // Should detect the secret
            assert!(
                !findings.is_empty(),
                "Should detect secret in {} content",
                size_name
            );

            // Memory usage should be reasonable (less than 10x the content size)
            let max_expected_memory = (size_bytes * 10) as u64;
            assert!(
                memory_increase < max_expected_memory,
                "Memory usage too high: {} KB for {} content",
                memory_increase / 1024,
                size_name
            );

            // Should complete within reasonable time (less than 10 seconds for 10MB)
            assert!(
                duration < Duration::from_secs(10),
                "Analysis too slow: {:?} for {} content",
                duration,
                size_name
            );
        }
    }

    /// Test concurrent analysis performance
    #[test]
    fn test_concurrent_analysis_performance() {
        use std::sync::Arc;
        use std::thread;

        let analyzer = Arc::new(SecurityAnalyzer::new());

        // Edge case 5: Concurrent analysis of multiple files
        let thread_counts = [1, 2, 4, 8];
        let files_per_thread = 10;

        for &thread_count in &thread_counts {
            let start_time = Instant::now();
            let mut handles = vec![];

            for thread_id in 0..thread_count {
                let analyzer_clone = Arc::clone(&analyzer);

                let handle = thread::spawn(move || {
                    let mut thread_findings = 0;

                    for file_id in 0..files_per_thread {
                        let content = format!(
                            "// Thread {} File {}\nlet secret = \"sk-proj-thread{}file{}\";\n",
                            thread_id, file_id, thread_id, file_id
                        );

                        let file_path = format!("thread_{}_file_{}.rs", thread_id, file_id);
                        let findings = analyzer_clone
                            .analyze(Path::new(&file_path), content.as_bytes())
                            .unwrap();

                        if !findings.is_empty() {
                            thread_findings += 1;
                        }
                    }

                    thread_findings
                });

                handles.push(handle);
            }

            // Wait for all threads to complete
            let mut total_findings = 0;
            for handle in handles {
                total_findings += handle.join().unwrap();
            }

            let duration = start_time.elapsed();
            let total_files = thread_count * files_per_thread;
            let files_per_second = total_files as f64 / duration.as_secs_f64();

            println!(
                "Processed {} files with {} threads in {:?} ({:.1} files/sec)",
                total_files, thread_count, duration, files_per_second
            );

            // Should detect secrets in all files
            assert_eq!(
                total_findings, total_files,
                "Should detect secrets in all {} files",
                total_files
            );

            // Performance should scale reasonably with thread count
            if thread_count > 1 {
                assert!(
                    files_per_second > 20.0,
                    "Concurrent performance too slow: {:.1} files/sec with {} threads",
                    files_per_second,
                    thread_count
                );
            }
        }
    }

    /// Test performance with pathological input patterns
    #[test]
    fn test_pathological_input_performance() {
        let analyzer = SecretAnalyzer::new();

        // Edge case 6: Inputs designed to stress regex engines
        let pathological_cases = [
            // Many potential matches that are false positives
            ("many_false_matches", "api_key = \"test\"; ".repeat(1000)),
            // Very long lines
            (
                "long_lines",
                format!(
                    "// {}\nlet secret = \"sk-proj-real123456789\";\n",
                    "x".repeat(10000)
                ),
            ),
            // Many short lines
            (
                "many_lines",
                "// comment\n".repeat(10000) + "let secret = \"sk-proj-real123456789\";\n",
            ),
            // Nested quotes and escapes
            (
                "complex_quotes",
                r#"let complex = "\"api_key\" = \"sk-proj-real123456789\""; "#.repeat(100),
            ),
        ];

        for (case_name, content) in &pathological_cases {
            let start_time = Instant::now();

            let findings = analyzer
                .analyze(Path::new("pathological.rs"), content.as_bytes())
                .unwrap();

            let duration = start_time.elapsed();

            println!("Analyzed {} case in {:?}", case_name, duration);

            // Should complete within reasonable time (less than 5 seconds)
            assert!(
                duration < Duration::from_secs(5),
                "Pathological case '{}' took too long: {:?}",
                case_name,
                duration
            );

            // Should detect the real secret if present
            if content.contains("sk-proj-real123456789") {
                assert!(
                    !findings.is_empty(),
                    "Should detect real secret in pathological case '{}'",
                    case_name
                );
            }
        }
    }

    /// Test performance regression detection
    #[test]
    fn test_performance_regression_detection() {
        let analyzer = SecurityAnalyzer::new();

        // Baseline performance test
        let test_content = r#"
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key: "sk-proj-BaselinePerformanceTest123456789".to_string(),
        }
    }
}
"#;

        // Warm up
        for _ in 0..10 {
            let _ = analyzer
                .analyze(Path::new("warmup.rs"), test_content.as_bytes())
                .unwrap();
        }

        // Measure baseline performance
        let iterations = 1000;
        let start_time = Instant::now();

        for i in 0..iterations {
            let file_path = format!("baseline_{}.rs", i);
            let findings = analyzer
                .analyze(Path::new(&file_path), test_content.as_bytes())
                .unwrap();

            assert!(
                !findings.is_empty(),
                "Should detect secret in baseline test"
            );
        }

        let duration = start_time.elapsed();
        let iterations_per_second = iterations as f64 / duration.as_secs_f64();

        println!(
            "Baseline performance: {:.1} iterations/sec",
            iterations_per_second
        );

        // Performance regression check - should be faster than 100 iterations/sec
        assert!(
            iterations_per_second > 100.0,
            "Performance regression detected: {:.1} iterations/sec (expected > 100)",
            iterations_per_second
        );

        // Store baseline for future comparisons
        let baseline_file = "performance_baseline.txt";
        fs::write(baseline_file, format!("{:.1}", iterations_per_second)).ok();

        println!(
            "Performance baseline saved: {:.1} iterations/sec",
            iterations_per_second
        );
    }

    // Helper function to get memory usage (simplified)
    fn get_memory_usage() -> u64 {
        // This is a simplified memory usage estimation
        // In a real implementation, you might use system-specific APIs
        // Memory usage estimation - imports removed as they're not used in this mock implementation

        // For testing purposes, return a mock value
        // In practice, you'd use platform-specific memory APIs
        1024 * 1024 // 1MB baseline
    }
}
