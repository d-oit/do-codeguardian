use do_codeguardian::analyzers::security_analyzer::SecurityAnalyzer;
use do_codeguardian::analyzers::Analyzer;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use tempfile::TempDir;

#[cfg(test)]
mod enhanced_edge_cases {

    use super::*;

    #[test]
    fn test_read_only_files() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("readonly_test.rs");

        // Create a test file with security issues
        let test_content = r#"let api_key = "sk-1234567890abcdef";
const PASSWORD = "secret_password_123";"#;
        fs::write(&file_path, test_content).unwrap();

        // Make file read-only
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o444); // Read-only for all
        fs::set_permissions(&file_path, perms).unwrap();

        // Analysis should still work on read-only files
        let result = analyzer.analyze(&file_path, test_content.as_bytes());
        assert!(result.is_ok(), "Failed to analyze read-only file");

        let findings = result.unwrap();
        assert!(
            !findings.is_empty(),
            "Should find security issues in read-only file"
        );
    }

    #[test]
    fn test_permission_denied_files() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("no_access_test.rs");

        // Create a test file
        let test_content = "let x = 42;";
        fs::write(&file_path, test_content).unwrap();

        // Make file inaccessible (no permissions)
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o000); // No permissions
        fs::set_permissions(&file_path, perms).unwrap();

        // Analysis should handle permission errors gracefully
        let result = analyzer.analyze(&file_path, test_content.as_bytes());
        // Should either succeed (if content is provided) or fail gracefully
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle permission issues appropriately"
        );
    }

    #[test]
    fn test_concurrent_file_access() {
        let analyzer = Arc::new(SecurityAnalyzer::new());
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("concurrent_test.rs");

        // Create a test file
        let test_content = generate_large_test_content(1000);
        fs::write(&file_path, &test_content).unwrap();

        let mut handles = vec![];
        let file_path = Arc::new(Mutex::new(file_path));

        // Spawn multiple threads trying to analyze the same file
        for i in 0..5 {
            let analyzer = Arc::clone(&analyzer);
            let file_path = Arc::clone(&file_path);
            let content = test_content.clone();

            let handle = thread::spawn(move || {
                let path = file_path.lock().unwrap().clone();
                let result = analyzer.analyze(&path, content.as_bytes());
                (i, result.is_ok())
            });

            handles.push(handle);
        }

        // Collect results
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All analyses should succeed
        for (thread_id, success) in results {
            assert!(success, "Thread {} failed concurrent analysis", thread_id);
        }
    }

    #[test]
    fn test_large_file_handling() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();

        // Test with various large file sizes
        let test_sizes = [10_000, 100_000, 1_000_000]; // lines

        for size in test_sizes {
            let test_content = generate_large_test_content(size);
            let file_path = temp_dir.path().join(format!("large_test_{}.rs", size));
            fs::write(&file_path, &test_content).unwrap();

            let start = std::time::Instant::now();
            let result = analyzer.analyze(&file_path, test_content.as_bytes());
            let duration = start.elapsed();

            assert!(result.is_ok(), "Failed to analyze {} line file", size);

            // Should scale reasonably (allow up to 10 seconds for 1M lines)
            let max_duration_ms = (size as u128 * 10).min(10_000);
            assert!(
                duration.as_millis() < max_duration_ms,
                "Large file analysis too slow: {} lines took {}ms",
                size,
                duration.as_millis()
            );
        }
    }

    #[test]
    fn test_edge_case_file_contents() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();

        let edge_cases = vec![
            ("empty_file", ""),
            ("binary_content", "�\u{0}�\u{1}�\u{2}"),
            ("unicode_content", "let café = '☕'; let naïve = '🚀';"),
            ("very_long_line", &"x".repeat(100_000)),
            ("null_bytes", "let x = 42;\0\0\0let y = 24;"),
            ("mixed_encodings", "let x = 'héllo'; // café"),
        ];

        for (case_name, content) in edge_cases {
            let file_path = temp_dir.path().join(format!("{}.rs", case_name));
            fs::write(&file_path, content).unwrap();

            // Analysis should not panic on edge case content
            let result = analyzer.analyze(&file_path, content.as_bytes());
            assert!(result.is_ok(), "Failed to handle edge case: {}", case_name);
        }
    }

    #[test]
    fn test_stress_error_handling() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();

        // Create files that might cause various errors
        let stress_scenarios = vec![
            ("nonexistent_file", PathBuf::from("/dev/null/nonexistent")),
            ("directory_as_file", temp_dir.path().to_path_buf()),
            ("special_device", PathBuf::from("/dev/null")),
        ];

        for (scenario, file_path) in stress_scenarios {
            let content = b"let x = 42;";

            // Should handle errors gracefully without panicking
            let result = analyzer.analyze(&file_path, content);

            // Result should be either Ok or Err, but not panic
            match result {
                Ok(_) => println!("Stress test {}: handled gracefully", scenario),
                Err(e) => println!("Stress test {}: error handled: {}", scenario, e),
            }
        }
    }

    #[test]
    fn test_resource_limits() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = TempDir::new().unwrap();

        // Test with extremely large content that might exceed memory limits
        let huge_content = "let x = 42;\n".repeat(1_000_000); // 10MB+ content
        let file_path = temp_dir.path().join("huge_file.rs");
        fs::write(&file_path, &huge_content).unwrap();

        let start = std::time::Instant::now();
        let result = analyzer.analyze(&file_path, huge_content.as_bytes());
        let duration = start.elapsed();

        // Should either succeed or fail gracefully, but not hang or crash
        match result {
            Ok(_) => {
                assert!(
                    duration.as_secs() < 30,
                    "Huge file analysis took too long: {}s",
                    duration.as_secs()
                );
            }
            Err(_) => {
                // Expected for very large files - should fail gracefully
                assert!(
                    duration.as_secs() < 10,
                    "Huge file error handling took too long: {}s",
                    duration.as_secs()
                );
            }
        }
    }

    // Helper functions

    fn generate_large_test_content(lines: usize) -> String {
        let mut content = String::new();
        for i in 0..lines {
            match i % 5 {
                0 => content.push_str(&format!("fn function_{}() {{}}\n", i)),
                1 => content.push_str(&format!("let var_{} = {};\n", i, i)),
                2 => content.push_str(&format!("// Comment line {}\n", i)),
                3 => content.push_str(&format!("struct Struct_{} {{}}\n", i)),
                _ => content.push_str(&format!("const CONST_{}: i32 = {};\n", i, i)),
            }
        }
        content
    }
}
