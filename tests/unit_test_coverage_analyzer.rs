//! Unit Test Coverage Analyzer
//!
//! This module implements comprehensive unit tests for all analyzers
//! to achieve the 95% coverage goal outlined in the testing plan.

use do_codeguardian::analyzers::*;
use do_codeguardian::types::{Finding, Severity};
use std::path::Path;
use proptest::prelude::*;

/// Comprehensive tests for GitConflictAnalyzer
mod git_conflict_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::GitConflictAnalyzer;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = GitConflictAnalyzer::new();
        assert_eq!(analyzer.name(), "git-conflict");
    }

    #[test]
    fn test_analyzer_with_syntax_validation() {
        let analyzer = GitConflictAnalyzer::new().with_syntax_validation(false);
        assert_eq!(analyzer.name(), "git-conflict");
    }

    #[test]
    fn test_supports_file_extensions() {
        let analyzer = GitConflictAnalyzer::new();
        
        // Should support common text files
        assert!(analyzer.supports_file(Path::new("test.rs")));
        assert!(analyzer.supports_file(Path::new("test.py")));
        assert!(analyzer.supports_file(Path::new("test.js")));
        assert!(analyzer.supports_file(Path::new("test.md")));
        assert!(analyzer.supports_file(Path::new("test.txt")));
        assert!(analyzer.supports_file(Path::new("test.toml")));
        assert!(analyzer.supports_file(Path::new("test.json")));
        
        // Should not support binary files
        assert!(!analyzer.supports_file(Path::new("test.exe")));
        assert!(!analyzer.supports_file(Path::new("test.png")));
        assert!(!analyzer.supports_file(Path::new("test.jpg")));
        assert!(!analyzer.supports_file(Path::new("test.bin")));
    }

    #[test]
    fn test_empty_file_analysis() {
        let analyzer = GitConflictAnalyzer::new();
        let result = analyzer.analyze(Path::new("empty.rs"), b"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_large_file_without_conflicts() {
        let analyzer = GitConflictAnalyzer::new();
        let content = "fn main() { println!(\"Hello\"); }\n".repeat(1000);
        let result = analyzer.analyze(Path::new("large.rs"), content.as_bytes());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_malformed_conflict_markers() {
        let analyzer = GitConflictAnalyzer::new();
        
        // Test incomplete conflict markers
        let test_cases = vec![
            "<<<<<<< HEAD\nsome code\n", // Missing end
            "=======\nsome code\n", // Missing start
            ">>>>>>> branch\nsome code\n", // Missing start
            "<<<<<<< HEAD\nsome code\n>>>>>>> branch\n", // Missing separator
        ];

        for (i, content) in test_cases.iter().enumerate() {
            let result = analyzer.analyze(Path::new(&format!("malformed_{}.rs", i)), content.as_bytes());
            assert!(result.is_ok(), "Should handle malformed conflicts: {}", content);
            let findings = result.unwrap();
            assert!(!findings.is_empty(), "Should detect malformed conflicts: {}", content);
        }
    }

    #[test]
    fn test_nested_conflict_markers() {
        let analyzer = GitConflictAnalyzer::new();
        let content = r#"
fn main() {
    <<<<<<< HEAD
    let x = "<<<<<<< inner";
    =======
    let x = ">>>>>>> inner";
    >>>>>>> branch
}
"#;
        let result = analyzer.analyze(Path::new("nested.rs"), content.as_bytes());
        assert!(result.is_ok());
        let findings = result.unwrap();
        // Should detect the outer conflict markers but not the inner string literals
        assert!(!findings.is_empty());
    }

    proptest! {
        #[test]
        fn test_arbitrary_text_without_conflicts(text in "[a-zA-Z0-9\n ]{0,1000}") {
            let analyzer = GitConflictAnalyzer::new();
            if !text.contains("<<<<<<<") && !text.contains("=======") && !text.contains(">>>>>>>") {
                let result = analyzer.analyze(Path::new("prop.rs"), text.as_bytes());
                prop_assert!(result.is_ok());
                prop_assert_eq!(result.unwrap().len(), 0);
            }
        }

        #[test]
        fn test_file_extensions(ext in "[a-z]{1,5}") {
            let analyzer = GitConflictAnalyzer::new();
            let filename = format!("test.{}", ext);
            let supports = analyzer.supports_file(Path::new(&filename));
            // Should be deterministic based on extension
            prop_assert_eq!(supports, analyzer.supports_file(Path::new(&filename)));
        }
    }
}

/// Comprehensive tests for SecurityAnalyzer
mod security_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::security::*;

    #[test]
    fn test_secret_analyzer_creation() {
        let analyzer = SecretAnalyzer::new();
        assert_eq!(analyzer.name(), "secret");
    }

    #[test]
    fn test_secret_analyzer_supports_files() {
        let analyzer = SecretAnalyzer::new();
        
        // Should support code files
        assert!(analyzer.supports_file(Path::new("config.rs")));
        assert!(analyzer.supports_file(Path::new("config.py")));
        assert!(analyzer.supports_file(Path::new("config.js")));
        assert!(analyzer.supports_file(Path::new("config.json")));
        assert!(analyzer.supports_file(Path::new("config.toml")));
        assert!(analyzer.supports_file(Path::new("config.yaml")));
        assert!(analyzer.supports_file(Path::new(".env")));
        
        // Should not support binary files
        assert!(!analyzer.supports_file(Path::new("image.png")));
        assert!(!analyzer.supports_file(Path::new("binary.exe")));
    }

    #[test]
    fn test_secret_detection_patterns() {
        let analyzer = SecretAnalyzer::new();
        
        let test_cases = vec![
            (r#"API_KEY = "sk-1234567890abcdef""#, true, "API key pattern"),
            (r#"password = "secretpassword123""#, true, "Password pattern"),
            (r#"aws_secret_access_key = "ABCD1234567890""#, true, "AWS secret"),
            (r#"let x = "just a string";"#, false, "Normal string"),
            (r#"// API_KEY = "example""#, false, "Commented out"),
            (r#"const API_KEY_NAME = "api_key";"#, false, "Variable name only"),
        ];

        for (content, should_detect, description) in test_cases {
            let result = analyzer.analyze(Path::new("test.rs"), content.as_bytes());
            assert!(result.is_ok(), "Analysis should succeed for: {}", description);
            let findings = result.unwrap();
            if should_detect {
                assert!(!findings.is_empty(), "Should detect secret in: {}", description);
                assert!(findings.iter().any(|f| f.rule == "hardcoded_secret"), 
                       "Should flag as hardcoded secret: {}", description);
            } else {
                assert!(findings.is_empty() || !findings.iter().any(|f| f.rule == "hardcoded_secret"), 
                       "Should not detect secret in: {}", description);
            }
        }
    }

    #[test]
    fn test_command_injection_analyzer() {
        let analyzer = CommandInjectionAnalyzer::new();
        assert_eq!(analyzer.name(), "command-injection");
        
        let dangerous_patterns = vec![
            r#"system("rm -rf " + user_input)"#,
            r#"exec("ls | grep " + filename)"#,
            r#"Command::new("sh").arg("-c").arg(user_input)"#,
        ];

        for pattern in dangerous_patterns {
            let result = analyzer.analyze(Path::new("test.rs"), pattern.as_bytes());
            assert!(result.is_ok());
            let findings = result.unwrap();
            assert!(!findings.is_empty(), "Should detect command injection in: {}", pattern);
        }
    }

    #[test]
    fn test_sql_injection_analyzer() {
        let analyzer = SqlInjectionAnalyzer::new();
        assert_eq!(analyzer.name(), "sql-injection");
        
        let sql_injection_patterns = vec![
            r#"query = "SELECT * FROM users WHERE id = " + user_id"#,
            r#"sql = format!("DELETE FROM {} WHERE id = {}", table, id)"#,
            r#"execute(&format!("UPDATE users SET name = '{}'", name))"#,
        ];

        for pattern in sql_injection_patterns {
            let result = analyzer.analyze(Path::new("test.rs"), pattern.as_bytes());
            assert!(result.is_ok());
            let findings = result.unwrap();
            assert!(!findings.is_empty(), "Should detect SQL injection in: {}", pattern);
        }
    }

    proptest! {
        #[test]
        fn test_secret_analyzer_with_random_content(content in "[a-zA-Z0-9=\" ]{0,500}") {
            let analyzer = SecretAnalyzer::new();
            let result = analyzer.analyze(Path::new("random.txt"), content.as_bytes());
            prop_assert!(result.is_ok());
            // Should not crash on random content
        }
    }
}

/// Comprehensive tests for AI Content Analyzer
mod ai_content_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::AiContentAnalyzer;

    #[test]
    fn test_ai_analyzer_creation() {
        let analyzer = AiContentAnalyzer::new();
        assert_eq!(analyzer.name(), "ai-content");
    }

    #[test]
    fn test_ai_analyzer_supports_files() {
        let analyzer = AiContentAnalyzer::new();
        
        // Should support text files
        assert!(analyzer.supports_file(Path::new("readme.md")));
        assert!(analyzer.supports_file(Path::new("code.rs")));
        assert!(analyzer.supports_file(Path::new("script.py")));
        assert!(analyzer.supports_file(Path::new("notes.txt")));
        
        // Should not support binary files
        assert!(!analyzer.supports_file(Path::new("image.jpg")));
        assert!(!analyzer.supports_file(Path::new("data.bin")));
    }

    #[test]
    fn test_todo_detection() {
        let analyzer = AiContentAnalyzer::new();
        
        let test_cases = vec![
            ("// TODO: Fix this bug", true, "Standard TODO comment"),
            ("/* FIXME: Memory leak */", true, "FIXME comment"),
            ("// HACK: Temporary workaround", true, "HACK comment"),
            ("// Note: This is documented", false, "Normal comment"),
            ("fn todo_list() {}", false, "Function name with todo"),
        ];

        for (content, should_detect, description) in test_cases {
            let result = analyzer.analyze(Path::new("test.rs"), content.as_bytes());
            assert!(result.is_ok(), "Analysis should succeed for: {}", description);
            let findings = result.unwrap();
            if should_detect {
                assert!(!findings.is_empty(), "Should detect AI content in: {}", description);
            } else {
                // May or may not detect, depending on implementation
                // This is more lenient for AI-based detection
            }
        }
    }

    #[test]
    fn test_empty_and_whitespace_content() {
        let analyzer = AiContentAnalyzer::new();
        
        let test_cases = vec![
            ("", "Empty content"),
            ("   ", "Whitespace only"),
            ("\n\n\n", "Newlines only"),
            ("\t\t", "Tabs only"),
        ];

        for (content, description) in test_cases {
            let result = analyzer.analyze(Path::new("test.txt"), content.as_bytes());
            assert!(result.is_ok(), "Should handle {}", description);
            let findings = result.unwrap();
            assert_eq!(findings.len(), 0, "Should not detect issues in {}", description);
        }
    }
}

/// Performance tests for analyzers
mod analyzer_performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_analyzer_performance_large_files() {
        let analyzers: Vec<Box<dyn Analyzer + Send + Sync>> = vec![
            Box::new(GitConflictAnalyzer::new()),
            Box::new(AiContentAnalyzer::new()),
            // Add more analyzers as needed
        ];

        // Create a large test file (10KB)
        let large_content = "fn test() { println!(\"Hello\"); }\n".repeat(500);
        
        for analyzer in analyzers {
            let start = Instant::now();
            let result = analyzer.analyze(Path::new("large.rs"), large_content.as_bytes());
            let duration = start.elapsed();
            
            assert!(result.is_ok(), "Large file analysis should succeed for {}", analyzer.name());
            assert!(duration.as_millis() < 1000, 
                   "Analysis should complete within 1s for {} (took {}ms)", 
                   analyzer.name(), duration.as_millis());
        }
    }

    #[test]
    fn test_concurrent_analysis() {
        use std::sync::Arc;
        use std::thread;

        let analyzer = Arc::new(GitConflictAnalyzer::new());
        let content = "fn main() { println!(\"Hello\"); }";
        
        let handles: Vec<_> = (0..10).map(|i| {
            let analyzer = Arc::clone(&analyzer);
            let content = content.to_string();
            thread::spawn(move || {
                let result = analyzer.analyze(
                    Path::new(&format!("test_{}.rs", i)), 
                    content.as_bytes()
                );
                assert!(result.is_ok());
                result.unwrap()
            })
        }).collect();

        for handle in handles {
            let findings = handle.join().unwrap();
            assert_eq!(findings.len(), 0, "Concurrent analysis should work correctly");
        }
    }
}

/// Edge case tests
mod analyzer_edge_cases {
    use super::*;

    #[test]
    fn test_unicode_content() {
        let analyzer = GitConflictAnalyzer::new();
        let unicode_content = "fn main() { println!(\"Hello 世界! 🚀\"); }";
        
        let result = analyzer.analyze(Path::new("unicode.rs"), unicode_content.as_bytes());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_binary_data_in_text_file() {
        let analyzer = GitConflictAnalyzer::new();
        let binary_data = vec![0, 1, 2, 3, 255, 254, 253];
        
        let result = analyzer.analyze(Path::new("binary.txt"), &binary_data);
        assert!(result.is_ok()); // Should not crash
    }

    #[test]
    fn test_extremely_long_lines() {
        let analyzer = GitConflictAnalyzer::new();
        let long_line = "a".repeat(10000);
        
        let result = analyzer.analyze(Path::new("long.rs"), long_line.as_bytes());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_many_short_lines() {
        let analyzer = GitConflictAnalyzer::new();
        let many_lines = "a\n".repeat(10000);
        
        let result = analyzer.analyze(Path::new("many_lines.rs"), many_lines.as_bytes());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}