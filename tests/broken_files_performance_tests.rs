use codeguardian::analyzers::{
    ai_content_analyzer::AiContentAnalyzer, duplicate_analyzer::DuplicateAnalyzer,
    git_conflict_analyzer::GitConflictAnalyzer, Analyzer,
};
use std::path::Path;
use std::time::{Duration, Instant};

#[cfg(test)]
mod performance_tests {
    use super::*;

    const PERFORMANCE_THRESHOLD_MS: u128 = 1000; // 1 second max for large files
    const SMALL_FILE_THRESHOLD_MS: u128 = 100; // 100ms for small files

    #[test]
    fn test_git_conflict_analyzer_performance() {
        let analyzer = GitConflictAnalyzer::new();

        // Small file performance
        let small_content = generate_file_with_conflicts(100);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("small.rs"), small_content.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < SMALL_FILE_THRESHOLD_MS,
            "Small file analysis took too long: {:?}",
            duration
        );
        assert!(
            !findings.is_empty(),
            "Should detect conflicts in generated content"
        );

        // Large file performance
        let large_content = generate_file_with_conflicts(10000);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("large.rs"), large_content.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Large file analysis took too long: {:?}",
            duration
        );
        assert!(
            !findings.is_empty(),
            "Should detect conflicts in large content"
        );
    }

    #[test]
    fn test_ai_content_analyzer_performance() {
        let analyzer = AiContentAnalyzer::new();

        // Small file with AI content
        let small_content = generate_file_with_ai_content(100);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("small.rs"), small_content.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < SMALL_FILE_THRESHOLD_MS,
            "Small file analysis took too long: {:?}",
            duration
        );
        assert!(
            !findings.is_empty(),
            "Should detect AI content in generated content"
        );

        // Large file with AI content
        let large_content = generate_file_with_ai_content(5000);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("large.rs"), large_content.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Large file analysis took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_duplicate_analyzer_performance() {
        let analyzer = DuplicateAnalyzer::new().with_min_lines(5);

        // Small file with duplicates
        let small_content = generate_file_with_duplicates(50);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("small.rs"), small_content.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < SMALL_FILE_THRESHOLD_MS,
            "Small file analysis took too long: {:?}",
            duration
        );

        // Medium file with duplicates (duplicate analysis is more expensive)
        let medium_content = generate_file_with_duplicates(500);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("medium.rs"), medium_content.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Medium file analysis took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_memory_usage_large_files() {
        // Test that analyzers don't consume excessive memory with large files
        let analyzers: Vec<Box<dyn Analyzer>> = vec![
            Box::new(GitConflictAnalyzer::new()),
            Box::new(AiContentAnalyzer::new()),
            Box::new(DuplicateAnalyzer::new()),
        ];

        // Generate a very large file (1MB)
        let large_content = generate_large_file(1_000_000);

        for analyzer in analyzers {
            let start = Instant::now();
            let result = analyzer.analyze(Path::new("huge.rs"), large_content.as_bytes());
            let duration = start.elapsed();

            // Should complete without crashing
            assert!(
                result.is_ok(),
                "Analyzer {} failed on large file",
                analyzer.name()
            );

            // Should complete in reasonable time (adjust threshold as needed)
            assert!(
                duration.as_secs() < 10,
                "Analyzer {} took too long on large file: {:?}",
                analyzer.name(),
                duration
            );
        }
    }

    #[test]
    fn test_parallel_analysis_performance() {
        use std::sync::Arc;
        use std::thread;

        let analyzer = Arc::new(GitConflictAnalyzer::new());
        let test_content = Arc::new(generate_file_with_conflicts(1000));

        let start = Instant::now();

        // Simulate parallel analysis of multiple files
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let analyzer = Arc::clone(&analyzer);
                let content = Arc::clone(&test_content);

                thread::spawn(move || {
                    let path = format!("test_{}.rs", i);
                    analyzer
                        .analyze(Path::new(&path), content.as_bytes())
                        .unwrap()
                })
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        let duration = start.elapsed();

        // Parallel analysis should complete quickly
        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Parallel analysis took too long: {:?}",
            duration
        );

        // All threads should produce results
        assert_eq!(results.len(), 4);
        for result in results {
            assert!(!result.is_empty(), "Each thread should detect conflicts");
        }
    }

    #[test]
    fn test_regex_compilation_performance() {
        // Test that regex compilation doesn't significantly impact performance
        let iterations = 100;

        let start = Instant::now();
        for _ in 0..iterations {
            let _analyzer = GitConflictAnalyzer::new();
        }
        let git_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let _analyzer = AiContentAnalyzer::new();
        }
        let ai_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let _analyzer = DuplicateAnalyzer::new();
        }
        let dup_duration = start.elapsed();

        // Analyzer creation should be fast
        assert!(
            git_duration.as_millis() < 100,
            "Git analyzer creation too slow: {:?}",
            git_duration
        );
        assert!(
            ai_duration.as_millis() < 200,
            "AI analyzer creation too slow: {:?}",
            ai_duration
        );
        assert!(
            dup_duration.as_millis() < 100,
            "Duplicate analyzer creation too slow: {:?}",
            dup_duration
        );
    }

    #[test]
    fn test_worst_case_scenarios() {
        // Test performance with pathological inputs

        // Git conflicts: File with many conflict markers
        let many_conflicts = generate_file_with_many_conflicts(100);
        let analyzer = GitConflictAnalyzer::new();
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("conflicts.rs"), many_conflicts.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Many conflicts analysis took too long: {:?}",
            duration
        );
        assert!(findings.len() >= 100, "Should detect many conflicts");

        // AI content: File with many placeholder patterns
        let many_placeholders = generate_file_with_many_placeholders(100);
        let analyzer = AiContentAnalyzer::new();
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("placeholders.rs"), many_placeholders.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
            "Many placeholders analysis took too long: {:?}",
            duration
        );

        // Duplicates: File with many similar functions
        let many_duplicates = generate_file_with_many_similar_functions(50);
        let analyzer = DuplicateAnalyzer::new().with_min_lines(3);
        let start = Instant::now();
        let findings = analyzer
            .analyze(Path::new("duplicates.rs"), many_duplicates.as_bytes())
            .unwrap();
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < PERFORMANCE_THRESHOLD_MS * 2, // Allow more time for duplicates
            "Many duplicates analysis took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_incremental_analysis_performance() {
        // Test that analyzing similar content multiple times is efficient
        let base_content = generate_file_with_conflicts(1000);
        let analyzer = GitConflictAnalyzer::new();

        let mut total_duration = Duration::new(0, 0);
        let iterations = 10;

        for i in 0..iterations {
            // Slightly modify content each time
            let modified_content = format!("{}\n// Iteration {}\n", base_content, i);

            let start = Instant::now();
            let _findings = analyzer
                .analyze(Path::new("test.rs"), modified_content.as_bytes())
                .unwrap();
            total_duration += start.elapsed();
        }

        let avg_duration = total_duration / iterations;
        assert!(
            avg_duration.as_millis() < SMALL_FILE_THRESHOLD_MS,
            "Average incremental analysis too slow: {:?}",
            avg_duration
        );
    }

    #[test]
    fn test_file_type_filtering_performance() {
        // Test that file type checking is fast
        let analyzer = AiContentAnalyzer::new();
        let test_paths = generate_test_paths(1000);

        let start = Instant::now();
        for path in &test_paths {
            let _supported = analyzer.supports_file(Path::new(path));
        }
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < 50,
            "File type filtering too slow: {:?}",
            duration
        );
    }

    // Helper functions for generating test content

    fn generate_file_with_conflicts(lines: usize) -> String {
        let mut content = String::new();
        content.push_str("fn main() {\n");

        for i in 0..lines {
            if i % 20 == 0 {
                content.push_str(&format!(
                    "<<<<<<< HEAD\n    let var_{} = {};\n=======\n    let var_{} = {};\n>>>>>>> branch\n",
                    i, i, i, i + 1000
                ));
            } else {
                content.push_str(&format!("    let var_{} = {};\n", i, i));
            }
        }

        content.push_str("}\n");
        content
    }

    fn generate_file_with_ai_content(lines: usize) -> String {
        let mut content = String::new();
        let ai_patterns = vec![
            "// TODO: implement this",
            "// add content here",
            "// Generated by AI assistant",
            "// placeholder implementation",
            "unimplemented!()",
        ];

        for i in 0..lines {
            if i % 10 == 0 {
                let pattern = &ai_patterns[i % ai_patterns.len()];
                content.push_str(&format!("{}\n", pattern));
            }
            content.push_str(&format!(
                "fn function_{}() {{\n    println!(\"Function {}\");\n}}\n\n",
                i, i
            ));
        }

        content
    }

    fn generate_file_with_duplicates(functions: usize) -> String {
        let mut content = String::new();

        for i in 0..functions {
            // Create pairs of similar functions
            if i % 2 == 0 {
                content.push_str(&format!(
                    "fn authenticate_user_{}(user: &str, pass: &str) -> bool {{\n    let hash = hash_password(pass);\n    let stored = get_stored_hash(user);\n    hash == stored\n}}\n\n",
                    i
                ));
            } else {
                content.push_str(&format!(
                    "fn authenticate_user_{}(user: &str, pass: &str) -> bool {{\n    let hash = hash_password(pass);\n    let stored = get_stored_hash(user);\n    hash == stored\n}}\n\n",
                    i
                ));
            }
        }

        content
    }

    fn generate_large_file(target_size: usize) -> String {
        let mut content = String::new();
        let base_function = "fn test_function() {\n    println!(\"test\");\n    let x = 1;\n    let y = 2;\n    let z = x + y;\n}\n\n";

        while content.len() < target_size {
            content.push_str(base_function);
        }

        content
    }

    fn generate_file_with_many_conflicts(conflict_count: usize) -> String {
        let mut content = String::new();
        content.push_str("fn main() {\n");

        for i in 0..conflict_count {
            content.push_str(&format!(
                "<<<<<<< HEAD\n    let conflict_{} = \"version1\";\n=======\n    let conflict_{} = \"version2\";\n>>>>>>> branch\n",
                i, i
            ));
        }

        content.push_str("}\n");
        content
    }

    fn generate_file_with_many_placeholders(placeholder_count: usize) -> String {
        let mut content = String::new();
        let patterns = vec![
            "// TODO: implement this",
            "// add content here",
            "// your code here",
            "// placeholder",
            "// implement me",
        ];

        for i in 0..placeholder_count {
            let pattern = &patterns[i % patterns.len()];
            content.push_str(&format!("{}\nfn function_{}() {{}}\n\n", pattern, i));
        }

        content
    }

    fn generate_file_with_many_similar_functions(function_count: usize) -> String {
        let mut content = String::new();

        for i in 0..function_count {
            content.push_str(&format!(
                "fn security_function_{}(input: &str) -> bool {{\n    let validated = validate_input(input);\n    let sanitized = sanitize_input(input);\n    validated && sanitized\n}}\n\n",
                i
            ));
        }

        content
    }

    fn generate_test_paths(count: usize) -> Vec<String> {
        let extensions = vec!["rs", "js", "py", "java", "txt", "md", "png", "exe"];
        let mut paths = Vec::new();

        for i in 0..count {
            let ext = &extensions[i % extensions.len()];
            paths.push(format!("test_{}.{}", i, ext));
        }

        paths
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;

    #[test]
    fn benchmark_analyzer_comparison() {
        // Compare performance of different analyzers on the same content
        let test_content = generate_mixed_content(1000);

        let analyzers: Vec<(Box<dyn Analyzer>, &str)> = vec![
            (Box::new(GitConflictAnalyzer::new()), "git_conflict"),
            (Box::new(AiContentAnalyzer::new()), "ai_content"),
            (Box::new(DuplicateAnalyzer::new()), "duplicate"),
        ];

        let mut results = Vec::new();

        for (analyzer, name) in analyzers {
            let start = Instant::now();
            let findings = analyzer
                .analyze(Path::new("benchmark.rs"), test_content.as_bytes())
                .unwrap();
            let duration = start.elapsed();

            results.push((name, duration, findings.len()));

            println!(
                "Analyzer {}: {:?} ({} findings)",
                name,
                duration,
                findings.len()
            );
        }

        // All analyzers should complete within reasonable time
        for (name, duration, _) in &results {
            assert!(
                duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
                "Analyzer {} exceeded performance threshold: {:?}",
                name,
                duration
            );
        }
    }

    #[test]
    fn benchmark_scaling_behavior() {
        // Test how analyzers scale with file size
        let analyzer = GitConflictAnalyzer::new();
        let sizes = vec![100, 500, 1000, 2000, 5000];

        let mut prev_duration = Duration::new(0, 0);

        for size in sizes {
            let content = generate_file_with_conflicts(size);

            let start = Instant::now();
            let _findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .unwrap();
            let duration = start.elapsed();

            println!("Size {}: {:?}", size, duration);

            // Performance should scale reasonably (not exponentially)
            if prev_duration.as_millis() > 0 {
                let ratio = duration.as_millis() as f64 / prev_duration.as_millis() as f64;
                assert!(
                    ratio < 10.0,
                    "Performance scaling too poor: {}x slower",
                    ratio
                );
            }

            prev_duration = duration;
        }
    }

    fn generate_mixed_content(lines: usize) -> String {
        let mut content = String::new();

        for i in 0..lines {
            match i % 10 {
                0 => content.push_str(
                    "<<<<<<< HEAD\nconflict content\n=======\nother content\n>>>>>>> branch\n",
                ),
                1 => content.push_str("// TODO: implement this function\n"),
                2 => content.push_str("// Generated by AI assistant\n"),
                3 => content.push_str("fn do_something() { println!(\"generic\"); }\n"),
                4 => content.push_str("fn authenticate_user() { /* duplicate */ }\n"),
                5 => content.push_str("fn authenticate_user_copy() { /* duplicate */ }\n"),
                6 => content.push_str("unimplemented!()\n"),
                7 => content.push_str("// add content here\n"),
                8 => content.push_str("// placeholder implementation\n"),
                _ => content.push_str(&format!("fn normal_function_{}() {{}}\n", i)),
            }
        }

        content
    }
}
