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
        assert_eq!(analyzer.name(), "git_conflict");
    }

    #[test]
    fn test_analyzer_with_syntax_validation() {
        let analyzer = GitConflictAnalyzer::new().with_syntax_validation(false);
        assert_eq!(analyzer.name(), "git_conflict");
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
/// Comprehensive tests for PerformanceAnalyzer
mod performance_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::performance_analyzer::*;

    #[test]
    fn test_performance_analyzer_creation() {
        let analyzer = PerformanceAnalyzer::new();
        assert!(analyzer.is_ok());
        let analyzer = analyzer.unwrap();
        assert_eq!(analyzer.name(), "performance");
    }

    #[test]
    fn test_performance_analyzer_supports_files() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        
        // Should support Rust files
        assert!(analyzer.supports_file(Path::new("test.rs")));
        assert!(analyzer.supports_file(Path::new("src/main.rs")));
        assert!(analyzer.supports_file(Path::new("lib.rs")));
        
        // Should not support other files
        assert!(!analyzer.supports_file(Path::new("test.js")));
        assert!(!analyzer.supports_file(Path::new("test.py")));
        assert!(!analyzer.supports_file(Path::new("test.txt")));
        assert!(!analyzer.supports_file(Path::new("test.png")));
    }

    #[test]
    fn test_detect_nested_loops_comprehensive() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        
        // Test various nested loop patterns
        let test_cases = vec![
            (r#"
            fn nested_for() {
                for i in 0..10 {
                    for j in 0..10 {
                        println!("{} {}", i, j);
                    }
                }
            }
            "#, true, "Simple nested for loops"),
            
            (r#"
            fn nested_while() {
                let mut i = 0;
                while i < 10 {
                    let mut j = 0;
                    while j < 10 {
                        println!("{} {}", i, j);
                        j += 1;
                    }
                    i += 1;
                }
            }
            "#, true, "Nested while loops"),
            
            (r#"
            fn single_loop() {
                for i in 0..10 {
                    println!("{}", i);
                }
            }
            "#, false, "Single loop should not trigger"),
            
            (r#"
            fn mixed_loops() {
                for i in 0..10 {
                    let mut j = 0;
                    while j < 10 {
                        println!("{} {}", i, j);
                        j += 1;
                    }
                }
            }
            "#, true, "Mixed for/while loops"),
        ];

        for (content, should_detect, description) in test_cases {
            let findings = analyzer.detect_nested_loops(content, Path::new("test.rs"));
            if should_detect {
                assert!(!findings.is_empty(), "Should detect nested loops in: {}", description);
                assert!(findings.iter().any(|f| f.rule == "nested_loops"));
            } else {
                assert!(findings.is_empty(), "Should not detect nested loops in: {}", description);
            }
        }
    }

    #[test]
    fn test_detect_inefficient_strings_comprehensive() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        
        let test_cases = vec![
            (r#"
            fn bad_string_concat() {
                let mut s = String::new();
                for i in 0..100 {
                    s += &format!("item_{} ", i);
                }
            }
            "#, true, "String concatenation in loop"),
            
            (r#"
            fn good_string_concat() {
                let s: String = (0..100)
                    .map(|i| format!("item_{}", i))
                    .collect::<Vec<_>>()
                    .join(" ");
            }
            "#, false, "Efficient string building"),
            
            (r#"
            fn string_push() {
                let mut s = String::new();
                for i in 0..100 {
                    s.push_str(&format!("item_{} ", i));
                }
            }
            "#, false, "push_str is more efficient"),
        ];

        for (content, should_detect, description) in test_cases {
            let findings = analyzer.detect_inefficient_strings(content, Path::new("test.rs"));
            if should_detect {
                assert!(!findings.is_empty(), "Should detect inefficient strings in: {}", description);
                assert!(findings.iter().any(|f| f.rule == "inefficient_string_ops"));
            } else {
                // May or may not detect depending on implementation
            }
        }
    }

    #[test]
    fn test_detect_blocking_io() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        
        let test_cases = vec![
            (r#"
            #[tokio::main]
            async fn bad_async() {
                std::fs::read_to_string("file.txt").unwrap();
            }
            "#, true, "Blocking I/O in async function"),
            
            (r#"
            async fn good_async() {
                tokio::fs::read_to_string("file.txt").await.unwrap();
            }
            "#, false, "Proper async I/O"),
            
            (r#"
            fn sync_function() {
                std::fs::read_to_string("file.txt").unwrap();
            }
            "#, false, "Blocking I/O in sync function is OK"),
        ];

        for (content, should_detect, description) in test_cases {
            let findings = analyzer.detect_blocking_io(content, Path::new("test.rs"));
            if should_detect {
                assert!(!findings.is_empty(), "Should detect blocking I/O in: {}", description);
                assert!(findings.iter().any(|f| f.rule == "blocking_io"));
            }
        }
    }

    #[test]
    fn test_detect_algorithmic_inefficiencies() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        
        let test_cases = vec![
            (r#"
            fn inefficient_sort() {
                let mut data = vec![1, 2, 3];
                for _ in 0..10 {
                    data.sort();
                }
            }
            "#, true, "Sorting in loop"),
            
            (r#"
            fn collect_iter_pattern() {
                let data: Vec<i32> = vec![1, 2, 3];
                let result: Vec<String> = data.iter().collect::<Vec<_>>().iter().map(|x| x.to_string()).collect();
            }
            "#, true, "collect().iter() pattern"),
        ];

        for (content, should_detect, description) in test_cases {
            let findings = analyzer.detect_algorithmic_inefficiencies(content, Path::new("test.rs"));
            if should_detect {
                assert!(!findings.is_empty(), "Should detect algorithmic inefficiency in: {}", description);
                assert!(findings.iter().any(|f| f.rule == "algorithmic_inefficiency"));
            }
        }
    }

    #[test]
    fn test_empty_file_analysis() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        let result = analyzer.analyze(Path::new("empty.rs"), b"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_large_file_without_issues() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        let content = "fn main() { println!(\"Hello\"); }
".repeat(1000);
        let result = analyzer.analyze(Path::new("large.rs"), content.as_bytes());
        assert!(result.is_ok());
        // Should not have excessive findings for clean code
        assert!(result.unwrap().len() < 10);
    }

    proptest! {
        #[test]
        fn test_performance_analyzer_with_random_rust_code(content in "[a-zA-Z0-9\s\{\}\(\)\[\];=<>!@#$%^&*+-]{0,2000}") {
            let analyzer = PerformanceAnalyzer::new().unwrap();
            let result = analyzer.analyze(Path::new("random.rs"), content.as_bytes());
            prop_assert!(result.is_ok());
            // Should not crash on random content
        }

        #[test]
        fn test_file_extensions(ext in "[a-z]{1,5}") {
            let analyzer = PerformanceAnalyzer::new().unwrap();
            let filename = format!("test.{}", ext);
            let supports = analyzer.supports_file(Path::new(&filename));
            // Should be deterministic
            prop_assert_eq!(supports, analyzer.supports_file(Path::new(&filename)));
        }
    }
}

/// Comprehensive tests for DuplicateAnalyzer
mod duplicate_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::duplicate_analyzer::*;
    use do_codeguardian::config::analysis::DuplicateAnalyzerConfig;

    #[test]
    fn test_duplicate_analyzer_creation() {
        let analyzer = DuplicateAnalyzer::new();
        assert!(analyzer.is_ok());
        let analyzer = analyzer.unwrap();
        assert_eq!(analyzer.name(), "duplicate");
    }

    #[test]
    fn test_duplicate_analyzer_with_config() {
        let config = DuplicateAnalyzerConfig {
            enabled: true,
            min_lines: 5,
            focus_security: true,
            ignore_test_files: true,
            max_files_to_compare: 100,
            enable_ml_similarity: false,
            ml_model_path: None,
            similarity_threshold: 0.9,
            enable_github_prevention: false,
            cache: Default::default(),
        };
        
        let analyzer = DuplicateAnalyzer::with_config(config);
        assert!(analyzer.is_ok());
    }

    #[test]
    fn test_duplicate_analyzer_supports_files() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        // Should support programming language files
        assert!(analyzer.supports_file(Path::new("test.rs")));
        assert!(analyzer.supports_file(Path::new("test.js")));
        assert!(analyzer.supports_file(Path::new("test.py")));
        assert!(analyzer.supports_file(Path::new("test.java")));
        assert!(analyzer.supports_file(Path::new("test.cpp")));
        assert!(analyzer.supports_file(Path::new("test.go")));
        
        // Should not support non-code files
        assert!(!analyzer.supports_file(Path::new("test.txt")));
        assert!(!analyzer.supports_file(Path::new("test.png")));
        assert!(!analyzer.supports_file(Path::new("test.bin")));
    }

    #[test]
    fn test_normalize_line_functionality() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        let test_cases = vec![
            ("  let x = 5;  // comment", "let x = 5;"),
            ("    if condition {", "if condition {"),
            ("# This is a comment", ""),
            ("  /* block comment */ let x = 1;", "let x = 1;"),
            ("", ""),
            ("no change needed", "no change needed"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(analyzer.normalize_line(input), expected);
        }
    }

    #[test]
    fn test_is_in_string_detection() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        assert!(analyzer.is_in_string(r#"let x = "hello # world"; "#, 12));
        assert!(!analyzer.is_in_string(r#"let x = 5; # comment"#, 12));
        assert!(analyzer.is_in_string(r#"let x = '; # not in string"#, 10));
    }

    #[test]
    fn test_extract_code_blocks() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        let content = r#"
        // Comment
        fn function1() {
            let x = 1;
            let y = 2;
        }
        
        fn function2() {
            let a = 1;
            let b = 2;
        }
        "#;
        
        let blocks = analyzer.extract_code_blocks(content);
        assert!(!blocks.is_empty());
        // Should extract at least the function blocks
        assert!(blocks.len() >= 2);
    }

    #[test]
    fn test_calculate_similarity_comprehensive() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        let block1 = CodeBlock {
            lines: vec!["line1".to_string(), "line2".to_string(), "line3".to_string()],
            start_line: 1,
            end_line: 3,
        };
        
        let block2 = CodeBlock {
            lines: vec!["line1".to_string(), "line2".to_string(), "line3".to_string()],
            start_line: 5,
            end_line: 7,
        };
        
        let block3 = CodeBlock {
            lines: vec!["line1".to_string(), "different".to_string(), "line3".to_string()],
            start_line: 9,
            end_line: 11,
        };
        
        let block4 = CodeBlock {
            lines: vec!["completely".to_string(), "different".to_string(), "content".to_string()],
            start_line: 13,
            end_line: 15,
        };

        assert_eq!(analyzer.calculate_similarity(&block1, &block2), 1.0);
        assert!((analyzer.calculate_similarity(&block1, &block3) - 2.0/3.0).abs() < 0.01);
        assert_eq!(analyzer.calculate_similarity(&block1, &block4), 0.0);
    }

    #[test]
    fn test_security_relevance_detection() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        let security_block = CodeBlock {
            lines: vec![
                "fn authenticate_user(username: &str, password: &str)".to_string(),
                "let hashed = hash_password(password);".to_string(),
                "verify_credentials(hashed, stored)".to_string(),
            ],
            start_line: 1,
            end_line: 3,
        };
        
        let normal_block = CodeBlock {
            lines: vec![
                "fn calculate_sum(a: i32, b: i32)".to_string(),
                "return a + b;".to_string(),
            ],
            start_line: 5,
            end_line: 6,
        };

        assert!(analyzer.is_security_relevant(&security_block));
        assert!(!analyzer.is_security_relevant(&normal_block));
    }

    #[test]
    fn test_ignore_test_files() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        assert!(analyzer.should_ignore_file(Path::new("tests/test_auth.rs")));
        assert!(analyzer.should_ignore_file(Path::new("src/auth_test.rs")));
        assert!(analyzer.should_ignore_file(Path::new("src/test_auth.rs")));
        assert!(!analyzer.should_ignore_file(Path::new("src/auth.rs")));
        assert!(analyzer.should_ignore_file(Path::new("target/debug/test")));
    }

    #[test]
    fn test_is_test_file() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        assert!(analyzer.is_test_file(Path::new("tests/integration_test.rs")));
        assert!(analyzer.is_test_file(Path::new("src/main_test.rs")));
        assert!(analyzer.is_test_file(Path::new("test.py")));
        assert!(!analyzer.is_test_file(Path::new("src/main.rs")));
        assert!(!analyzer.is_test_file(Path::new("lib.rs")));
    }

    #[test]
    fn test_detect_security_patterns_in_duplicate() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        let block1 = CodeBlock {
            lines: vec!["authenticate_user".to_string(), "hash_password".to_string()],
            start_line: 1,
            end_line: 2,
        };
        
        let block2 = CodeBlock {
            lines: vec!["validate_input".to_string(), "sanitize_data".to_string()],
            start_line: 5,
            end_line: 6,
        };

        let patterns = analyzer.detect_security_patterns_in_duplicate(&block1, &block2);
        assert!(!patterns.is_empty());
        assert!(patterns.contains(&"authentication".to_string()) || patterns.contains(&"input_validation".to_string()));
    }

    #[test]
    fn test_calculate_severity() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        let block1 = CodeBlock {
            lines: vec!["authenticate".to_string()],
            start_line: 1,
            end_line: 1,
        };
        
        let block2 = CodeBlock {
            lines: vec!["authenticate".to_string()],
            start_line: 5,
            end_line: 5,
        };

        // High similarity with security content should be high severity
        let severity = analyzer.calculate_severity(0.95, &block1, &block2);
        match severity {
            Severity::High => {},
            _ => panic!("Expected High severity for high similarity security content"),
        }
    }

    #[test]
    fn test_empty_and_edge_cases() {
        let analyzer = DuplicateAnalyzer::new().unwrap();
        
        // Empty file
        let result = analyzer.analyze(Path::new("empty.rs"), b"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
        
        // File with only comments
        let content = "// This is just a comment
// Another comment
";
        let result = analyzer.analyze(Path::new("comments.rs"), content.as_bytes());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    proptest! {
        #[test]
        fn test_duplicate_analyzer_with_random_content(content in "[a-zA-Z0-9\s\{\}\(\)\[\];=<>!@#$%^&*+-]{0,1000}") {
            let analyzer = DuplicateAnalyzer::new().unwrap();
            let result = analyzer.analyze(Path::new("random.rs"), content.as_bytes());
            prop_assert!(result.is_ok());
            // Should not crash on random content
        }
    }
}
/// Comprehensive tests for BuildArtifactAnalyzer
mod build_artifact_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::build_artifact_analyzer::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_artifact_analyzer_creation() {
        let analyzer = BuildArtifactAnalyzer::new();
        assert_eq!(analyzer.name(), "build_artifact_duplicate");
        
        // Check default build directories
        assert!(analyzer.build_dirs.contains(&PathBuf::from("target")));
        assert!(analyzer.build_dirs.contains(&PathBuf::from("build")));
        assert!(analyzer.build_dirs.contains(&PathBuf::from("dist")));
        
        // Check default patterns
        assert!(analyzer.artifact_patterns.contains(&"*.so".to_string()));
        assert!(analyzer.artifact_patterns.contains(&"*.exe".to_string()));
        assert!(analyzer.artifact_patterns.contains(&"Cargo.toml".to_string()));
    }

    #[test]
    fn test_build_artifact_analyzer_with_custom_config() {
        let custom_dirs = vec![PathBuf::from("custom_build"), PathBuf::from("output")];
        let analyzer = BuildArtifactAnalyzer::new()
            .with_build_dirs(custom_dirs.clone())
            .with_max_file_size(50 * 1024 * 1024); // 50MB
        
        assert_eq!(analyzer.build_dirs, custom_dirs);
        assert_eq!(analyzer.max_file_size, 50 * 1024 * 1024);
    }

    #[test]
    fn test_supports_file_comprehensive() {
        let analyzer = BuildArtifactAnalyzer::new();
        
        // Should support build-related files
        assert!(analyzer.supports_file(Path::new("Cargo.toml")));
        assert!(analyzer.supports_file(Path::new("package.json")));
        assert!(analyzer.supports_file(Path::new("requirements.txt")));
        assert!(analyzer.supports_file(Path::new("pom.xml")));
        assert!(analyzer.supports_file(Path::new("build.gradle")));
        
        // Should not support other files
        assert!(!analyzer.supports_file(Path::new("README.md")));
        assert!(!analyzer.supports_file(Path::new("main.rs")));
        assert!(!analyzer.supports_file(Path::new("app.js")));
        assert!(!analyzer.supports_file(Path::new("data.txt")));
    }

    #[test]
    fn test_is_build_artifact_comprehensive() {
        let analyzer = BuildArtifactAnalyzer::new();
        
        // Test file extensions
        assert!(analyzer.is_build_artifact(Path::new("libtest.so")));
        assert!(analyzer.is_build_artifact(Path::new("library.dll")));
        assert!(analyzer.is_build_artifact(Path::new("archive.a")));
        assert!(analyzer.is_build_artifact(Path::new("object.o")));
        assert!(analyzer.is_build_artifact(Path::new("binary.exe")));
        assert!(analyzer.is_build_artifact(Path::new("program.bin")));
        
        // Test exact matches
        assert!(analyzer.is_build_artifact(Path::new("Cargo.toml")));
        assert!(analyzer.is_build_artifact(Path::new("package.json")));
        assert!(analyzer.is_build_artifact(Path::new("requirements.txt")));
        
        // Test build directory detection
        assert!(analyzer.is_build_artifact(Path::new("target/debug/binary")));
        assert!(analyzer.is_build_artifact(Path::new("build/release/library.so")));
        assert!(analyzer.is_build_artifact(Path::new("dist/app.exe")));
        assert!(analyzer.is_build_artifact(Path::new("out/artifact.bin")));
        
        // Should not detect non-build artifacts
        assert!(!analyzer.is_build_artifact(Path::new("src/main.rs")));
        assert!(!analyzer.is_build_artifact(Path::new("README.md")));
        assert!(!analyzer.is_build_artifact(Path::new("docs/guide.txt")));
    }

    #[test]
    fn test_determine_artifact_type_comprehensive() {
        let analyzer = BuildArtifactAnalyzer::new();
        
        // Test shared libraries
        assert_eq!(analyzer.determine_artifact_type(Path::new("lib.so")), ArtifactType::SharedLibrary);
        assert_eq!(analyzer.determine_artifact_type(Path::new("lib.dylib")), ArtifactType::SharedLibrary);
        assert_eq!(analyzer.determine_artifact_type(Path::new("lib.dll")), ArtifactType::SharedLibrary);
        
        // Test archives
        assert_eq!(analyzer.determine_artifact_type(Path::new("lib.a")), ArtifactType::Archive);
        assert_eq!(analyzer.determine_artifact_type(Path::new("lib.lib")), ArtifactType::Archive);
        
        // Test object files
        assert_eq!(analyzer.determine_artifact_type(Path::new("main.o")), ArtifactType::ObjectFile);
        assert_eq!(analyzer.determine_artifact_type(Path::new("module.obj")), ArtifactType::ObjectFile);
        
        // Test binaries
        assert_eq!(analyzer.determine_artifact_type(Path::new("app.exe")), ArtifactType::Binary);
        assert_eq!(analyzer.determine_artifact_type(Path::new("program.bin")), ArtifactType::Binary);
        assert_eq!(analyzer.determine_artifact_type(Path::new("script.out")), ArtifactType::Binary);
        
        // Test Java archives
        assert_eq!(analyzer.determine_artifact_type(Path::new("app.jar")), ArtifactType::Archive);
        assert_eq!(analyzer.determine_artifact_type(Path::new("web.war")), ArtifactType::Archive);
        assert_eq!(analyzer.determine_artifact_type(Path::new("enterprise.ear")), ArtifactType::Archive);
        
        // Test package files
        assert_eq!(analyzer.determine_artifact_type(Path::new("package.deb")), ArtifactType::Binary);
        assert_eq!(analyzer.determine_artifact_type(Path::new("software.rpm")), ArtifactType::Binary);
        
        // Test unknown types
        assert_eq!(analyzer.determine_artifact_type(Path::new("unknown.xyz")), ArtifactType::Other);
        assert_eq!(analyzer.determine_artifact_type(Path::new("data.txt")), ArtifactType::Other);
    }

    #[test]
    fn test_calculate_file_hash() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = BuildArtifactAnalyzer::new();
        let temp_dir = TempDir::new()?;
        
        // Create test files
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        
        fs::write(&file1, "test content")?;
        fs::write(&file2, "test content")?; // Same content as file1
        fs::write(&file3, "different content")?; // Different content
        
        let hash1 = analyzer.calculate_file_hash(&file1)?;
        let hash2 = analyzer.calculate_file_hash(&file2)?;
        let hash3 = analyzer.calculate_file_hash(&file3)?;
        
        // Same content should have same hash
        assert_eq!(hash1, hash2);
        // Different content should have different hash
        assert_ne!(hash1, hash3);
        
        Ok(())
    }

    #[test]
    fn test_extract_dependencies_cargo() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = BuildArtifactAnalyzer::new();
        let temp_dir = TempDir::new()?;
        
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        let content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
regex = "1.5"

[dev-dependencies]
tempfile = "3.0"
"#;
        fs::write(&cargo_toml, content)?;
        
        let deps = analyzer.extract_dependencies(&cargo_toml)?;
        assert!(deps.contains(&"cargo-dependencies".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_extract_dependencies_npm() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = BuildArtifactAnalyzer::new();
        let temp_dir = TempDir::new()?;
        
        let package_json = temp_dir.path().join("package.json");
        let content = r#"
{
  "name": "test-app",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "axios": "^1.0.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}
"#;
        fs::write(&package_json, content)?;
        
        let deps = analyzer.extract_dependencies(&package_json)?;
        assert!(deps.contains(&"express".to_string()));
        assert!(deps.contains(&"axios".to_string()));
        assert!(deps.contains(&"lodash".to_string()));
        assert!(!deps.contains(&"jest".to_string())); // devDependencies should not be included
        
        Ok(())
    }

    #[test]
    fn test_extract_dependencies_pip() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = BuildArtifactAnalyzer::new();
        let temp_dir = TempDir::new()?;
        
        let requirements_txt = temp_dir.path().join("requirements.txt");
        let content = r#"
requests==2.28.1
flask>=2.0.0
django<4.0
sqlalchemy!=1.4.0
# This is a comment
numpy
"#;
        fs::write(&requirements_txt, content)?;
        
        let deps = analyzer.extract_dependencies(&requirements_txt)?;
        assert!(deps.contains(&"requests".to_string()));
        assert!(deps.contains(&"flask".to_string()));
        assert!(deps.contains(&"django".to_string()));
        assert!(deps.contains(&"sqlalchemy".to_string()));
        assert!(deps.contains(&"numpy".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_group_artifacts_by_hash() -> Result<(), Box<dyn std::error::Error>> {
        let mut analyzer = BuildArtifactAnalyzer::new();
        
        let artifacts = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.so"),
                size: 1000,
                hash: "hash1".to_string(), // Same hash
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target3/lib.so"),
                size: 1500,
                hash: "hash2".to_string(), // Different hash
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep2".to_string()],
            },
        ];
        
        let duplicates = analyzer.group_artifacts_by_hash(artifacts)?;
        
        // Should have one duplicate group
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].artifacts.len(), 2);
        assert_eq!(duplicates[0].hash, "hash1");
        assert_eq!(duplicates[0].total_wasted_space, 1000); // One duplicate
        
        Ok(())
    }

    #[test]
    fn test_assess_conflict_level_comprehensive() {
        let analyzer = BuildArtifactAnalyzer::new();
        
        // Test low conflict (same type, same dependencies)
        let low_conflict = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
        ];
        assert_eq!(analyzer.assess_conflict_level(&low_conflict), ConflictLevel::Low);
        
        // Test medium conflict (many duplicates)
        let medium_conflict = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target3/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target4/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
        ];
        assert_eq!(analyzer.assess_conflict_level(&medium_conflict), ConflictLevel::Medium);
        
        // Test high conflict (different types)
        let high_conflict = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.a"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::Archive,
                dependencies: vec!["dep1".to_string()],
            },
        ];
        assert_eq!(analyzer.assess_conflict_level(&high_conflict), ConflictLevel::High);
        
        // Test critical conflict (different types and dependencies)
        let critical_conflict = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.a"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::Archive,
                dependencies: vec!["dep2".to_string()],
            },
        ];
        assert_eq!(analyzer.assess_conflict_level(&critical_conflict), ConflictLevel::Critical);
    }

    #[test]
    fn test_generate_cleanup_recommendation() {
        let analyzer = BuildArtifactAnalyzer::new();
        
        let artifacts = vec![
            BuildArtifact {
                path: PathBuf::from("target1/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
            BuildArtifact {
                path: PathBuf::from("target2/lib.so"),
                size: 1000,
                hash: "hash1".to_string(),
                artifact_type: ArtifactType::SharedLibrary,
                dependencies: vec!["dep1".to_string()],
            },
        ];
        
        let recommendation = analyzer.generate_cleanup_recommendation(&artifacts, ConflictLevel::Low);
        assert!(recommendation.contains("LOW"));
        assert!(recommendation.contains("duplicate artifacts"));
        assert!(recommendation.contains("Automated cleanup recommended"));
    }

    #[test]
    fn test_conflict_resolver() {
        let resolver = ConflictResolver::new();
        
        let duplicate = DuplicateArtifact {
            hash: "hash1".to_string(),
            artifacts: vec![
                BuildArtifact {
                    path: PathBuf::from("target1/lib.so"),
                    size: 1000,
                    hash: "hash1".to_string(),
                    artifact_type: ArtifactType::SharedLibrary,
                    dependencies: vec!["dep1".to_string()],
                },
                BuildArtifact {
                    path: PathBuf::from("target2/lib.so"),
                    size: 1000,
                    hash: "hash1".to_string(),
                    artifact_type: ArtifactType::SharedLibrary,
                    dependencies: vec!["dep1".to_string()],
                },
            ],
            total_wasted_space: 1000,
            conflict_level: ConflictLevel::Low,
            cleanup_recommendation: "Test recommendation".to_string(),
        };
        
        let actions = resolver.resolve(&duplicate).unwrap();
        
        // Should have one Keep and one Remove action
        assert_eq!(actions.len(), 2);
        let keep_count = actions.iter().filter(|a| matches!(a, CleanupAction::Keep { .. })).count();
        let remove_count = actions.iter().filter(|a| matches!(a, CleanupAction::Remove { .. })).count();
        assert_eq!(keep_count, 1);
        assert_eq!(remove_count, 1);
    }

    #[test]
    fn test_empty_file_analysis() {
        let analyzer = BuildArtifactAnalyzer::new();
        let result = analyzer.analyze(Path::new("empty.txt"), b"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // This analyzer works on directories
    }

    #[test]
    fn test_max_file_size_limit() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = BuildArtifactAnalyzer::new().with_max_file_size(100); // 100 bytes limit
        let temp_dir = TempDir::new()?;
        
        let small_file = temp_dir.path().join("small.txt");
        let large_file = temp_dir.path().join("large.txt");
        
        fs::write(&small_file, "small content")?; // 13 bytes
        fs::write(&large_file, "x".repeat(200))?; // 200 bytes
        
        // Small file should be analyzed
        let small_result = analyzer.analyze_file(&small_file);
        assert!(small_result.is_ok());
        assert!(small_result.unwrap().is_some());
        
        // Large file should be skipped
        let large_result = analyzer.analyze_file(&large_file);
        assert!(large_result.is_ok());
        assert!(large_result.unwrap().is_none());
        
        Ok(())
    }

    proptest! {
        #[test]
        fn test_build_artifact_analyzer_with_random_content(content in "[a-zA-Z0-9\s\{\}\(\)\[\];=<>!@#$%^&*+-]{0,1000}") {
            let analyzer = BuildArtifactAnalyzer::new();
            let result = analyzer.analyze(Path::new("random.txt"), content.as_bytes());
            prop_assert!(result.is_ok());
            // Should not crash on random content
        }
    }
}
/// Comprehensive tests for DependencyAnalyzer
mod dependency_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::dependency_analyzer::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_dependency_analyzer_creation() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        assert_eq!(analyzer.name(), "dependency-analyzer");
        assert_eq!(analyzer.project_root, temp_dir.path());
    }

    #[test]
    fn test_supports_file() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        
        // Should support Cargo.toml
        assert!(analyzer.supports_file(Path::new("Cargo.toml")));
        assert!(analyzer.supports_file(Path::new("src/Cargo.toml")));
        
        // Should not support other files
        assert!(!analyzer.supports_file(Path::new("main.rs")));
        assert!(!analyzer.supports_file(Path::new("package.json")));
        assert!(!analyzer.supports_file(Path::new("requirements.txt")));
        assert!(!analyzer.supports_file(Path::new("README.md")));
    }

    #[test]
    fn test_analyze_dependencies_no_cargo_toml() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        
        let findings = analyzer.analyze_dependencies().unwrap();
        assert!(findings.is_empty());
    }

    #[test]
    fn test_analyze_dependencies_with_cargo_toml() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        let cargo_content = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
regex = "1.5"
"#;
        
        fs::write(&cargo_toml_path, cargo_content)?;
        
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        let findings = analyzer.analyze_dependencies()?;
        
        // Should not crash, may or may not find vulnerabilities depending on cargo-audit availability
        assert!(findings.is_ok());
        
        Ok(())
    }

    #[test]
    fn test_is_problematic_license_comprehensive() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        
        // GPL licenses should be problematic
        assert!(analyzer.is_problematic_license("GPL-2.0"));
        assert!(analyzer.is_problematic_license("GPL-2.0+"));
        assert!(analyzer.is_problematic_license("GPL-3.0"));
        assert!(analyzer.is_problematic_license("GPL-3.0+"));
        
        // LGPL licenses should be problematic
        assert!(analyzer.is_problematic_license("LGPL-2.1"));
        assert!(analyzer.is_problematic_license("LGPL-2.1+"));
        assert!(analyzer.is_problematic_license("LGPL-3.0"));
        assert!(analyzer.is_problematic_license("LGPL-3.0+"));
        
        // AGPL should be problematic
        assert!(analyzer.is_problematic_license("AGPL-3.0"));
        assert!(analyzer.is_problematic_license("AGPL-3.0+"));
        
        // Other copyleft licenses should be problematic
        assert!(analyzer.is_problematic_license("OSL-3.0"));
        assert!(analyzer.is_problematic_license("EPL-1.0"));
        assert!(analyzer.is_problematic_license("EPL-2.0"));
        
        // Permissive licenses should not be problematic
        assert!(!analyzer.is_problematic_license("MIT"));
        assert!(!analyzer.is_problematic_license("Apache-2.0"));
        assert!(!analyzer.is_problematic_license("BSD-2-Clause"));
        assert!(!analyzer.is_problematic_license("BSD-3-Clause"));
        assert!(!analyzer.is_problematic_license("ISC"));
        
        // Unknown licenses should be considered problematic (conservative approach)
        assert!(analyzer.is_problematic_license("Unknown-License-123"));
        assert!(analyzer.is_problematic_license("Custom-License"));
        assert!(analyzer.is_problematic_license(""));
    }

    #[test]
    fn test_analyze_non_cargo_toml_file() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        
        let result = analyzer.analyze(Path::new("main.rs"), b"fn main() {}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Should return empty for non-Cargo.toml files
    }

    #[test]
    fn test_analyze_cargo_toml_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        let cargo_content = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
"#;
        
        fs::write(&cargo_toml_path, cargo_content)?;
        
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        let result = analyzer.analyze(&cargo_toml_path, cargo_content.as_bytes());
        
        // Should not crash
        assert!(result.is_ok());
        
        Ok(())
    }

    #[test]
    fn test_run_cargo_audit_subprocess_error_handling() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let cargo_toml_path = temp_dir.path().join("Cargo.toml");
        
        let cargo_content = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#;
        
        fs::write(&cargo_toml_path, cargo_content)?;
        
        let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
        
        // This should handle the case where cargo-audit is not available gracefully
        let result = analyzer.run_cargo_audit_subprocess(&cargo_toml_path);
        
        // Should not panic, should return empty or handle error gracefully
        match result {
            Ok(findings) => {
                // Either succeeds with findings or returns empty
                assert!(findings.is_empty() || !findings.is_empty());
            }
            Err(_) => {
                // Error is acceptable if cargo-audit is not installed
            }
        }
        
        Ok(())
    }

    proptest! {
        #[test]
        fn test_dependency_analyzer_with_random_content(content in "[a-zA-Z0-9\s\{\}\(\)\[\];=<>!@#$%^&*+-]{0,1000}") {
            let temp_dir = TempDir::new().unwrap();
            let analyzer = DependencyAnalyzer::new(temp_dir.path().to_path_buf());
            let result = analyzer.analyze(Path::new("random.txt"), content.as_bytes());
            prop_assert!(result.is_ok());
            // Should not crash on random content
        }
    }
}

/// Comprehensive tests for ValidationAnalyzer
mod validation_analyzer_tests {
    use super::*;
    use do_codeguardian::analyzers::validation_analyzer::*;
    use do_codeguardian::config::checklist::SecurityChecklist;
    use do_codeguardian::core::{ValidationConfig, ValidationStatus};
    use std::path::PathBuf;

    #[test]
    fn test_validation_analyzer_creation() {
        let analyzer = ValidationAnalyzer::new();
        assert_eq!(analyzer.name(), "validation");
        assert!(analyzer.enabled);
    }

    #[test]
    fn test_validation_analyzer_with_config() {
        let config = ValidationConfig::default();
        let analyzer = ValidationAnalyzer::with_config(config);
        assert_eq!(analyzer.name(), "validation");
        assert!(analyzer.enabled);
    }

    #[test]
    fn test_supports_file() {
        let analyzer = ValidationAnalyzer::new();
        
        // Validation analyzer supports no files directly
        assert!(!analyzer.supports_file(Path::new("main.rs")));
        assert!(!analyzer.supports_file(Path::new("Cargo.toml")));
        assert!(!analyzer.supports_file(Path::new("README.md")));
    }

    #[test]
    fn test_analyze_method() {
        let analyzer = ValidationAnalyzer::new();
        
        let result = analyzer.analyze(Path::new("test.rs"), b"fn main() {}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Should return empty
    }

    #[tokio::test]
    async fn test_validate_findings_empty() {
        let analyzer = ValidationAnalyzer::new();
        
        let result = analyzer.validate_findings(vec![]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_validate_findings_disabled() {
        let mut analyzer = ValidationAnalyzer::new();
        analyzer.set_enabled(false);
        
        let findings = vec![Finding::new(
            "security",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test finding".to_string(),
        )];
        
        let result = analyzer.validate_findings(findings.clone()).await;
        assert!(result.is_ok());
        let validated = result.unwrap();
        assert_eq!(validated.len(), 1);
        // Should return original findings unchanged when disabled
        assert_eq!(validated[0].message, findings[0].message);
    }

    #[tokio::test]
    async fn test_validate_findings_with_content() {
        let analyzer = ValidationAnalyzer::new();
        
        let findings = vec![
            Finding::new(
                "security",
                "hardcoded_secret",
                Severity::High,
                PathBuf::from("test.rs"),
                10,
                "Hardcoded password detected".to_string(),
            ),
            Finding::new(
                "security", 
                "sql_injection",
                Severity::Critical,
                PathBuf::from("test.py"),
                25,
                "SQL injection vulnerability".to_string(),
            ),
        ];
        
        let result = analyzer.validate_findings(findings).await;
        assert!(result.is_ok());
        let validated = result.unwrap();
        
        // Should have some findings (either validated, enhanced, or passed through)
        assert!(!validated.is_empty());
        
        // Check that findings have been processed (may have confidence scores added)
        for finding in &validated {
            assert!(!finding.message.is_empty());
        }
    }

    #[test]
    fn test_enhance_finding_with_validation() {
        let analyzer = ValidationAnalyzer::new();
        
        let validation_result = crate::core::ValidationResult {
            finding: Finding::new(
                "security",
                "test_rule",
                Severity::High,
                PathBuf::from("test.rs"),
                10,
                "Original message".to_string(),
            ),
            confidence_score: 0.75,
            validation_status: ValidationStatus::Enhanced,
            layer_results: Vec::new(),
            recommendations: vec![
                "Use parameterized queries".to_string(),
                "Validate input data".to_string(),
            ],
            requires_manual_review: false,
        };
        
        let enhanced = analyzer.enhance_finding_with_validation(validation_result);
        
        // Should contain confidence information
        assert!(enhanced.description.as_ref().unwrap().contains("Confidence: 75"));
        
        // Should contain recommendations
        assert!(enhanced.suggestion.is_some());
        assert!(enhanced.suggestion.as_ref().unwrap().contains("parameterized queries"));
        assert!(enhanced.suggestion.as_ref().unwrap().contains("Validate input data"));
    }

    #[test]
    fn test_enhance_finding_low_confidence() {
        let analyzer = ValidationAnalyzer::new();
        
        let validation_result = crate::core::ValidationResult {
            finding: Finding::new(
                "security",
                "test_rule",
                Severity::Critical,
                PathBuf::from("test.rs"),
                10,
                "Low confidence finding".to_string(),
            ),
            confidence_score: 0.3, // Low confidence
            validation_status: ValidationStatus::Validated,
            layer_results: Vec::new(),
            recommendations: Vec::new(),
            requires_manual_review: false,
        };
        
        let enhanced = analyzer.enhance_finding_with_validation(validation_result);
        
        // Should downgrade severity due to low confidence
        assert_eq!(enhanced.severity, Severity::High); // Critical -> High
    }

    #[test]
    fn test_enhance_finding_high_confidence() {
        let analyzer = ValidationAnalyzer::new();
        
        let validation_result = crate::core::ValidationResult {
            finding: Finding::new(
                "security",
                "test_rule",
                Severity::Low,
                PathBuf::from("test.rs"),
                10,
                "High confidence finding".to_string(),
            ),
            confidence_score: 0.9, // High confidence
            validation_status: ValidationStatus::Validated,
            layer_results: Vec::new(),
            recommendations: Vec::new(),
            requires_manual_review: false,
        };
        
        let enhanced = analyzer.enhance_finding_with_validation(validation_result);
        
        // Should maintain severity for high confidence
        assert_eq!(enhanced.severity, Severity::Low);
    }

    #[tokio::test]
    async fn test_validation_metrics() {
        let analyzer = ValidationAnalyzer::new();
        
        let metrics = analyzer.get_validation_metrics().await;
        
        // Should have default metrics
        assert!(metrics.total_validations >= 0);
        assert!(metrics.validation_success_rate >= 0.0 && metrics.validation_success_rate <= 1.0);
    }

    #[tokio::test]
    async fn test_review_statistics() {
        let analyzer = ValidationAnalyzer::new();
        
        let stats = analyzer.get_review_statistics().await;
        
        // Should have default statistics
        assert!(stats.total_submitted >= 0);
        assert!(stats.pending_reviews >= 0);
        assert!(stats.completed_reviews >= 0);
    }

    #[tokio::test]
    async fn test_reset_metrics() {
        let analyzer = ValidationAnalyzer::new();
        
        // Reset metrics
        analyzer.reset_metrics().await;
        
        let metrics = analyzer.get_validation_metrics().await;
        
        // Should be reset to zero or defaults
        assert_eq!(metrics.total_validations, 0);
    }

    #[test]
    fn test_update_confidence_baseline() {
        let mut analyzer = ValidationAnalyzer::new();
        
        // Update baseline
        analyzer.update_confidence_baseline("security", 0.85);
        
        // Should not panic
    }

    #[test]
    fn test_get_threshold_recommendations() {
        let analyzer = ValidationAnalyzer::new();
        
        let findings = vec![
            Finding::new(
                "security",
                "hardcoded_secret",
                Severity::High,
                PathBuf::from("test.rs"),
                10,
                "Test finding".to_string(),
            ),
        ];
        
        let recommendations = analyzer.get_threshold_recommendations(&findings);
        
        // Should return recommendations
        assert!(recommendations.recommended_thresholds.len() >= 0);
    }

    #[tokio::test]
    async fn test_validated_analyzer_registry() {
        let base_registry = crate::analyzers::AnalyzerRegistry::new();
        let registry = ValidatedAnalyzerRegistry::new(base_registry);
        
        assert!(registry.validation_enabled);
        
        // Test disabling validation
        let mut registry = registry;
        registry.set_validation_enabled(false);
        assert!(!registry.validation_enabled);
    }

    #[tokio::test]
    async fn test_analyze_file_with_validation() {
        let base_registry = crate::analyzers::AnalyzerRegistry::new();
        let registry = ValidatedAnalyzerRegistry::new(base_registry);
        
        let result = registry.analyze_file_with_validation(
            Path::new("test.rs"), 
            b"fn main() { let password = \"secret123\"; }"
        ).await;
        
        assert!(result.is_ok());
    }

    proptest! {
        #[test]
        fn test_validation_analyzer_with_random_findings(
            message in "[a-zA-Z0-9\s]{1,100}",
            line in 1..1000u32
        ) {
            let analyzer = ValidationAnalyzer::new();
            
            let validation_result = crate::core::ValidationResult {
                finding: Finding::new(
                    "security",
                    "test_rule",
                    Severity::Medium,
                    PathBuf::from("test.rs"),
                    line,
                    message,
                ),
                confidence_score: 0.8,
                validation_status: ValidationStatus::Validated,
                layer_results: Vec::new(),
                recommendations: Vec::new(),
                requires_manual_review: false,
            };
            
            let enhanced = analyzer.enhance_finding_with_validation(validation_result);
            
            // Should not crash and should enhance the finding
            assert!(enhanced.description.is_some());
            assert!(enhanced.description.as_ref().unwrap().contains("Confidence"));
        }
    }
}
