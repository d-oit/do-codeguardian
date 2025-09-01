use do_codeguardian::analyzers::{
    performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer, Analyzer,
};
use std::path::PathBuf;
use std::time::Instant;
use tempfile::tempdir;

/// Performance regression tests to ensure CodeGuardian maintains good performance
/// These tests establish baselines and catch performance regressions

#[test]
fn test_security_analyzer_performance_baseline() {
    let analyzer = SecurityAnalyzer::new();
    let temp_dir = tempdir().unwrap();

    // Create a standardized test file with known patterns
    let test_content = generate_security_test_content(1000); // 1000 lines
    let file_path = temp_dir.path().join("security_test.rs");
    std::fs::write(&file_path, &test_content).unwrap();

    let start = Instant::now();
    let findings = analyzer
        .analyze(&file_path, test_content.as_bytes())
        .unwrap();
    let duration = start.elapsed();

    // Performance baseline: should complete within 50ms for 1000 lines
    assert!(
        duration.as_millis() < 50,
        "Security analyzer performance regression: took {}ms (baseline: 50ms)",
        duration.as_millis()
    );

    // Should find some issues in our test content
    assert!(
        !findings.is_empty(),
        "Should find security issues in test content"
    );
}

#[test]
fn test_performance_analyzer_baseline() {
    let analyzer = PerformanceAnalyzer::new();
    let temp_dir = tempdir().unwrap();

    // Create a standardized test file with performance patterns
    let test_content = generate_performance_test_content(1000); // 1000 lines
    let file_path = temp_dir.path().join("performance_test.rs");
    std::fs::write(&file_path, &test_content).unwrap();

    let start = Instant::now();
    let findings = analyzer
        .analyze(&file_path, test_content.as_bytes())
        .unwrap();
    let duration = start.elapsed();

    // Performance baseline: should complete within 100ms for 1000 lines
    assert!(
        duration.as_millis() < 100,
        "Performance analyzer regression: took {}ms (baseline: 100ms)",
        duration.as_millis()
    );

    // Should find performance issues in our test content
    assert!(
        !findings.is_empty(),
        "Should find performance issues in test content"
    );
}

#[test]
fn test_large_file_performance() {
    let analyzer = SecurityAnalyzer::new();
    let temp_dir = tempdir().unwrap();

    // Test with a larger file (10k lines)
    let large_content = generate_security_test_content(10_000);
    let file_path = temp_dir.path().join("large_test.rs");
    std::fs::write(&file_path, &large_content).unwrap();

    let start = Instant::now();
    let result = analyzer.analyze(&file_path, large_content.as_bytes());
    let duration = start.elapsed();

    assert!(result.is_ok(), "Large file analysis failed");

    // Should scale reasonably: 10x lines should take <10x time
    assert!(
        duration.as_millis() < 500,
        "Large file analysis too slow: took {}ms",
        duration.as_millis()
    );
}

#[test]
fn test_memory_efficiency() {
    let analyzer = SecurityAnalyzer::new();
    let file_path = PathBuf::from("memory_test.rs");

    // Test with progressively larger files to ensure memory usage is reasonable
    for size_kb in [1, 10, 100, 1000] {
        let content = generate_repeated_content(size_kb * 1024);

        let start = Instant::now();
        let result = analyzer.analyze(&file_path, content.as_bytes());
        let duration = start.elapsed();

        assert!(result.is_ok(), "Memory test failed for {}KB", size_kb);

        // Memory efficiency: larger files shouldn't take exponentially longer
        let expected_max_ms = size_kb as u128 * 2; // 2ms per KB is reasonable
        assert!(
            duration.as_millis() < expected_max_ms,
            "Memory inefficiency detected: {}KB took {}ms (expected <{}ms)",
            size_kb,
            duration.as_millis(),
            expected_max_ms
        );
    }
}

#[test]
fn test_concurrent_analysis_performance() {
    use std::sync::Arc;
    use std::thread;

    let analyzer = Arc::new(SecurityAnalyzer::new());
    let temp_dir = tempdir().unwrap();

    // Create multiple test files
    let mut handles = vec![];
    for i in 0..4 {
        let analyzer = Arc::clone(&analyzer);
        let temp_dir = temp_dir.path().to_path_buf();

        let handle = thread::spawn(move || {
            let content = generate_security_test_content(500);
            let file_path = temp_dir.join(format!("concurrent_test_{}.rs", i));
            std::fs::write(&file_path, &content).unwrap();

            let start = Instant::now();
            let result = analyzer.analyze(&file_path, content.as_bytes());
            let duration = start.elapsed();

            (result.is_ok(), duration)
        });

        handles.push(handle);
    }

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All should succeed
    for (success, _) in &results {
        assert!(*success, "Concurrent analysis failed");
    }

    // None should take too long (concurrent access shouldn't cause major slowdowns)
    for (_, duration) in &results {
        assert!(
            duration.as_millis() < 200,
            "Concurrent analysis too slow: {}ms",
            duration.as_millis()
        );
    }
}

// Helper functions for generating test content

fn generate_security_test_content(lines: usize) -> String {
    let mut content = String::new();

    for i in 0..lines {
        match i % 10 {
            0 => content.push_str(&format!(
                "let api_key_{} = \"sk-1234567890abcdef{}\";\n",
                i, i
            )),
            1 => content.push_str(&format!(
                "const PASSWORD_{} = \"secret_password_{}\";\n",
                i, i
            )),
            2 => content.push_str(&format!(
                "// TODO: Remove this hardcoded token: token_{}\n",
                i
            )),
            3 => content.push_str(&format!(
                "let db_url = \"postgres://user:pass@localhost/db_{}\";\n",
                i
            )),
            4 => content.push_str(&format!(
                "fn function_{}() {{\n    println!(\"Function {}\");\n}}\n",
                i, i
            )),
            _ => content.push_str(&format!("// Regular comment line {}\n", i)),
        }
    }

    content
}

fn generate_performance_test_content(lines: usize) -> String {
    let mut content = String::new();

    for i in 0..lines {
        match i % 8 {
            0 => content.push_str(&format!(
                "fn nested_loop_{}() {{\n    for i in 0..10 {{\n        for j in 0..10 {{\n            println!(\"{{}}, {{}}\", i, j);\n        }}\n    }}\n}}\n",
                i
            )),
            1 => content.push_str(&format!(
                "fn string_concat_{}() {{\n    let mut s = String::new();\n    for i in 0..100 {{\n        s += &format!(\"item {{}}\", i);\n    }}\n}}\n",
                i
            )),
            2 => content.push_str(&format!(
                "async fn blocking_io_{}() {{\n    std::fs::read_to_string(\"file_{}.txt\").unwrap();\n}}\n",
                i, i
            )),
            3 => content.push_str(&format!(
                "fn inefficient_sort_{}() {{\n    let mut data = vec![1, 2, 3];\n    for _ in 0..10 {{\n        data.sort();\n    }}\n}}\n",
                i
            )),
            _ => content.push_str(&format!("// Regular function {}\nfn func_{}() {{}}\n", i, i)),
        }
    }

    content
}

fn generate_repeated_content(size_bytes: usize) -> String {
    let base_line = "let x = \"some content for memory testing\";\n";
    let lines_needed = size_bytes / base_line.len() + 1;
    base_line.repeat(lines_needed)
}
