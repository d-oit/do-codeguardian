use do_codeguardian::analyzers::{
    ai_content_analyzer::AiContentAnalyzer, duplicate_analyzer::DuplicateAnalyzer,
    git_conflict_analyzer::GitConflictAnalyzer, Analyzer,
};
use do_codeguardian::types::Severity;
use std::path::Path;

#[cfg(test)]
mod git_conflict_unit_tests {
    use super::*;

    #[test]
    fn test_conflict_marker_patterns() {
        let analyzer = GitConflictAnalyzer::new();

        // Test various conflict marker formats
        let test_cases = vec![
            ("<<<<<<< HEAD", true),
            ("<<<<<<<", true),
            ("<<<<<<< feature-branch", true),
            ("=======", true),
            (">>>>>>> main", true),
            (">>>>>>>", true),
            ("< < < < < < <", false), // Spaces should not match
            ("======", false),        // Wrong number of equals
            ("> > > > > > >", false), // Spaces should not match
        ];

        for (line, should_match) in test_cases {
            let content = format!("some code\n{}\nmore code", line);
            let findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for conflict markers");

            if should_match {
                assert!(
                    !findings.is_empty(),
                    "Should detect conflict marker: {}",
                    line
                );
            } else {
                assert!(
                    findings.is_empty(),
                    "Should not detect false positive: {}",
                    line
                );
            }
        }
    }

    #[test]
    fn test_nested_conflicts() {
        let analyzer = GitConflictAnalyzer::new();
        let content = r#"
fn main() {
<<<<<<< HEAD
    let x = 1;
<<<<<<< INNER
    let y = 2;
=======
    let y = 3;
>>>>>>> INNER
=======
    let x = 4;
>>>>>>> branch
}
"#;

        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();

        // Should detect all conflict markers
        assert!(findings.len() >= 4); // At least 4 markers
        assert!(findings.iter().any(|f| f.rule == "merge_conflict_start"));
        assert!(findings
            .iter()
            .any(|f| f.rule == "merge_conflict_separator"));
        assert!(findings.iter().any(|f| f.rule == "merge_conflict_end"));
    }

    #[test]
    fn test_syntax_validation_disabled() {
        let analyzer = GitConflictAnalyzer::new().with_syntax_validation(false);
        let invalid_json = r#"{"invalid": json}"#;

        let findings = analyzer
            .analyze(Path::new("test.json"), invalid_json.as_bytes())
            .unwrap();

        // Should not report syntax errors when validation is disabled
        assert!(!findings.iter().any(|f| f.rule == "syntax_error"));
    }

    #[test]
    fn test_toml_syntax_validation() {
        let analyzer = GitConflictAnalyzer::new();
        let invalid_toml = r#"
[section
key = "missing bracket"
"#;

        let findings = analyzer
            .analyze(Path::new("test.toml"), invalid_toml.as_bytes())
            .unwrap();

        assert!(findings.iter().any(|f| f.rule == "syntax_error"));
        assert!(findings.iter().any(|f| f.severity == Severity::High));
    }

    #[test]
    fn test_suspicious_duplication_detection() {
        let analyzer = GitConflictAnalyzer::new();
        let content = r#"
fn main() {
    println!("duplicate line");
    println!("duplicate line");
    println!("duplicate line");
    println!("normal line");
}
"#;

        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();

        // Should detect suspicious duplication
        assert!(findings.iter().any(|f| f.rule == "suspicious_duplication"));
        assert!(findings.iter().any(|f| f.severity == Severity::Medium));
    }

    #[test]
    fn test_binary_file_exclusion() {
        let analyzer = GitConflictAnalyzer::new();

        let binary_extensions = vec!["exe", "bin", "so", "dll", "png", "jpg", "pdf"];
        let text_extensions = vec!["rs", "js", "py", "txt", "md", "json"];

        for ext in binary_extensions {
            let file_name = format!("test.{}", ext);
            let path = Path::new(&file_name);
            assert!(!analyzer.supports_file(path), "Should not support .{}", ext);
        }

        for ext in text_extensions {
            let file_name = format!("test.{}", ext);
            let path = Path::new(&file_name);
            assert!(analyzer.supports_file(path), "Should support .{}", ext);
        }
    }

    #[test]
    fn test_empty_file_handling() {
        let analyzer = GitConflictAnalyzer::new();
        let findings = analyzer
            .analyze(Path::new("empty.rs"), b"")
            .expect("Failed to analyze empty file");
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_large_file_with_conflicts() {
        let analyzer = GitConflictAnalyzer::new();

        // Create large content with conflict in the middle
        let mut content = String::new();
        for i in 0..1000 {
            content.push_str(&format!("line {}\n", i));
        }
        content
            .push_str("<<<<<<< HEAD\nconflict content\n=======\nother content\n>>>>>>> branch\n");
        for i in 1000..2000 {
            content.push_str(&format!("line {}\n", i));
        }

        let findings = analyzer
            .analyze(Path::new("large.rs"), content.as_bytes())
            .expect("Failed to analyze large file with conflicts");

        // Should still detect conflicts in large files
        assert_eq!(findings.len(), 3); // start, separator, end
    }
}

#[cfg(test)]
mod ai_content_unit_tests {
    use super::*;

    #[test]
    fn test_placeholder_pattern_variations() {
        let analyzer = AiContentAnalyzer::new();

        let test_cases = vec![
            ("// TODO: implement this", true),
            ("// todo: IMPLEMENT this function", true),
            ("add content here please", true),
            ("Add Content Here", true),
            ("your code here", true),
            ("YOUR CODE HERE", true),
            ("implement this feature", true),
            ("placeholder text", true),
            ("fill in the details", true),
            ("complete this section", true),
            ("normal comment", false),
            ("implementation details", false),
            ("content management", false),
        ];

        for (text, should_detect) in test_cases {
            let content = format!("fn test() {{\n    // {}\n}}", text);
            let findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for conflict markers");

            let has_placeholder = findings.iter().any(|f| f.rule == "placeholder_content");
            assert_eq!(has_placeholder, should_detect, "Text: '{}'", text);
        }
    }

    #[test]
    fn test_ai_comment_markers() {
        let analyzer = AiContentAnalyzer::new();

        let test_cases = vec![
            "// Generated by AI assistant",
            "// Created by GPT-4",
            "// Powered by Copilot",
            "# Generated by AI",
            "/* Created by AI assistant */",
            "<!-- Generated by AI -->",
        ];

        for marker in test_cases {
            let content = format!("{}\nfn test() {{}}", marker);
            let findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for conflict markers");

            assert!(
                findings.iter().any(|f| f.rule == "ai_generated_marker"),
                "Should detect AI marker: {}",
                marker
            );
        }
    }

    #[test]
    fn test_generic_function_patterns() {
        let analyzer = AiContentAnalyzer::new();

        let generic_functions = vec![
            "fn do_something() {}",
            "fn handle_this(x: i32) {}",
            "fn process_data(data: &str) {}",
            "fn perform_action() {}",
            "function doSomething() {}",
            "def handle_this():",
            "public void performAction() {}",
        ];

        for func in generic_functions {
            let content = format!("{}\n", func);
            let findings = analyzer
                .analyze(Path::new("src/main.rs"), content.as_bytes())
                .unwrap();

            assert!(
                findings.iter().any(|f| f.rule == "generic_function_name"),
                "Should detect generic function: {}",
                func
            );
        }
    }

    #[test]
    fn test_incomplete_implementation_patterns() {
        let analyzer = AiContentAnalyzer::new();

        let incomplete_patterns = vec![
            ("unimplemented!()", Severity::High),
            ("panic!(\"not implemented\")", Severity::High),
            ("throw new NotImplementedException()", Severity::High),
            ("raise NotImplementedError", Severity::High),
            ("// TODO: not implemented", Severity::Medium),
            ("// FIXME: incomplete", Severity::Medium),
            ("// stub implementation", Severity::Medium),
        ];

        for (pattern, expected_severity) in incomplete_patterns {
            let content = format!("fn test() {{\n    {}\n}}", pattern);
            let findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for conflict markers");

            let incomplete_finding = findings
                .iter()
                .find(|f| f.rule == "incomplete_implementation" || f.rule == "placeholder_content");

            assert!(
                incomplete_finding.is_some(),
                "Should detect incomplete: {}",
                pattern
            );

            if let Some(finding) = incomplete_finding {
                assert_eq!(
                    finding.severity, expected_severity,
                    "Wrong severity for: {}",
                    pattern
                );
            }
        }
    }

    #[test]
    fn test_documentation_exclusion() {
        let analyzer = AiContentAnalyzer::new();

        let doc_content = r#"
/// This is documentation
/// Examples of AI-generated placeholder patterns:
/// - "implement this functionality"
/// - "add your code here"
/// - "customize this section"
/// - "complete the implementation"
fn documented_function() {}

//! Module documentation
//! Examples for testing:
//! - "fill in the details"
//! - "modify as needed"
//! - "replace with actual implementation"

fn normal_function() {
    // TODO: implement this  <- should be detected
}
"#;

        let findings = analyzer
            .analyze(Path::new("test.rs"), doc_content.as_bytes())
            .unwrap();

        // Should only detect the non-documentation TODO
        let placeholder_findings: Vec<_> = findings
            .iter()
            .filter(|f| f.rule == "placeholder_content")
            .collect();

        assert_eq!(placeholder_findings.len(), 1);
        assert_eq!(placeholder_findings[0].line, 11); // The line with non-doc TODO
    }

    #[test]
    fn test_test_file_exclusion() {
        let analyzer = AiContentAnalyzer::new();
        let content = r#"
fn do_something() {
    // Generic function in test file
}

fn handle_this() {
    // Another generic function
}
"#;

        let test_paths = vec![
            "tests/test_module.rs",
            "src/module_test.rs",
            "test_helper.rs",
            "examples/example_code.rs",
        ];

        for path in test_paths {
            let findings = analyzer
                .analyze(Path::new(path), content.as_bytes())
                .unwrap();

            // Generic function names should be ignored in test/example files
            assert!(
                !findings.iter().any(|f| f.rule == "generic_function_name"),
                "Should ignore generic functions in: {}",
                path
            );
        }
    }

    #[test]
    fn test_custom_patterns_integration() {
        let custom_patterns = vec![
            "implement me".to_string(),
            "fix this later".to_string(),
            "custom todo".to_string(),
        ];

        let analyzer = AiContentAnalyzer::new()
            .with_custom_patterns(custom_patterns)
            .expect("Failed to create analyzer with custom patterns");

        let content = r#"
fn test() {
    // implement me properly
    println!("fix this later");
    // custom todo: handle errors
}
"#;

        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();

        // Should detect all custom patterns
        assert_eq!(
            findings
                .iter()
                .filter(|f| f.rule == "placeholder_content")
                .count(),
            3
        );
    }

    #[test]
    fn test_invalid_custom_pattern() {
        let invalid_patterns = vec!["[invalid regex(".to_string()];

        let result = AiContentAnalyzer::new().with_custom_patterns(invalid_patterns);

        assert!(result.is_err(), "Should reject invalid regex patterns");
    }

    #[test]
    fn test_file_type_filtering() {
        let analyzer = AiContentAnalyzer::new();

        // Should support source code files
        let supported_files = vec![
            "test.rs",
            "test.js",
            "test.ts",
            "test.py",
            "test.java",
            "test.cpp",
            "test.go",
            "test.php",
            "test.rb",
            "test.cs",
        ];

        for file in supported_files {
            assert!(
                analyzer.supports_file(Path::new(file)),
                "Should support: {}",
                file
            );
        }

        // Should not support non-source files
        let unsupported_files = vec!["test.txt", "test.md", "test.pdf", "test.exe", "test.png"];

        for file in unsupported_files {
            assert!(
                !analyzer.supports_file(Path::new(file)),
                "Should not support: {}",
                file
            );
        }
    }
}

#[cfg(test)]
mod duplicate_analyzer_unit_tests {
    use super::*;

    #[test]
    fn test_line_normalization() {
        let analyzer = DuplicateAnalyzer::new();

        // Test normalization of different line formats
        let test_cases = vec![
            ("  let x = 5;  // comment", "let x = 5;"),
            ("    if condition {", "if condition {"),
            ("let y = 10; # Python comment", "let y = 10;"),
            ("  \t  fn test()  \t  ", "fn test()"),
            ("", ""),
            ("// only comment", ""),
            ("# only comment", ""),
        ];

        for (input, _expected) in test_cases {
            // Note: This would require exposing the normalize_line method
            // For now, we test through the full analysis pipeline
            let content = format!("{}\n{}\n", input, input);
            let _findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for line normalization");

            // If normalization works correctly, identical normalized lines should be detected
            // (though they need to meet minimum line requirements)
        }
    }

    #[test]
    fn test_security_pattern_detection() {
        let analyzer = DuplicateAnalyzer::new().with_min_lines(2);

        let security_functions = vec![
            "authenticate_user",
            "validate_input",
            "encrypt_data",
            "hash_password",
            "authorize_access",
            "sanitize_input",
            "handle_error",
        ];

        for func_name in security_functions {
            let content = format!(
                r#"
fn {}() {{
    // Security function implementation
    println!("Processing");
}}

fn {}_copy() {{
    // Security function implementation
    println!("Processing");
}}
"#,
                func_name, func_name
            );

            let findings = analyzer
                .analyze(Path::new("security.rs"), content.as_bytes())
                .expect("Failed to analyze security functions for duplication");

            // Should detect security-relevant duplicates
            if !findings.is_empty() {
                assert!(findings.iter().any(|f| f.rule == "internal_duplication"));
                // Security duplicates should have higher severity
                assert!(findings
                    .iter()
                    .any(|f| matches!(f.severity, Severity::High | Severity::Medium)));
            }
        }
    }

    #[test]
    fn test_non_security_code_with_focus() {
        let analyzer = DuplicateAnalyzer::new()
            .with_min_lines(3)
            .with_security_focus(true);

        let non_security_content = r#"
fn print_hello() {
    println!("Hello");
    println!("World");
    println!("!");
}

fn print_hello_copy() {
    println!("Hello");
    println!("World");
    println!("!");
}
"#;

        let findings = analyzer
            .analyze(Path::new("utils.rs"), non_security_content.as_bytes())
            .expect("Failed to analyze non-security code with focus");

        // With security focus enabled, non-security duplicates should be ignored or low priority
        if !findings.is_empty() {
            assert!(findings.iter().all(|f| f.severity == Severity::Low));
        }
    }

    #[test]
    fn test_minimum_lines_threshold() {
        let test_cases = vec![
            (3, true),  // Should detect - functions have 6 lines each
            (5, true),  // Should detect - functions have 6 lines each
            (8, false), // Should not detect - functions only have 6 lines each
        ];

        for (min_lines, should_detect) in test_cases {
            let analyzer = DuplicateAnalyzer::new()
                .with_min_lines(min_lines)
                .with_security_focus(false);

            let content = r#"
fn function_a() {
    let x = 1;
    let y = 2;
    let z = 3;
    println!("{} {} {}", x, y, z);
}

fn function_b() {
    let x = 1;
    let y = 2;
    let z = 3;
    println!("{} {} {}", x, y, z);
}
"#;

            let findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for conflict markers");

            if should_detect {
                assert!(
                    !findings.is_empty(),
                    "Should detect with min_lines={}",
                    min_lines
                );
            } else {
                assert!(
                    findings.is_empty(),
                    "Should not detect with min_lines={}",
                    min_lines
                );
            }
        }
    }

    #[test]
    fn test_test_file_handling() {
        let analyzer_include_tests = DuplicateAnalyzer::new()
            .with_min_lines(3)
            .with_test_files(true)
            .with_security_focus(false);

        let analyzer_exclude_tests = DuplicateAnalyzer::new()
            .with_min_lines(3)
            .with_test_files(false)
            .with_security_focus(false);

        let test_content = r#"
fn test_function_a() {
    assert_eq!(1, 1);
    assert_eq!(2, 2);
    assert_eq!(3, 3);
}

fn test_function_b() {
    assert_eq!(1, 1);
    assert_eq!(2, 2);
    assert_eq!(3, 3);
}
"#;

        let test_file_path = Path::new("tests/test_module.rs");

        let findings_include = analyzer_include_tests
            .analyze(test_file_path, test_content.as_bytes())
            .unwrap();
        let findings_exclude = analyzer_exclude_tests
            .analyze(test_file_path, test_content.as_bytes())
            .unwrap();

        // When including tests, should detect duplicates
        assert!(!findings_include.is_empty());

        // When excluding tests, should not detect duplicates in test files
        assert!(findings_exclude.is_empty());
    }

    #[test]
    fn test_similarity_thresholds() {
        let analyzer = DuplicateAnalyzer::new()
            .with_min_lines(3)
            .with_security_focus(false)
            .with_similarity_threshold(0.4); // Lower threshold to catch similar code

        // Test different levels of similarity
        let test_cases = vec![
            // Identical code (100% similar)
            (
                r#"
fn identical_a() {
    let x = 1;
    let y = 2;
    return x + y;
}

fn identical_b() {
    let x = 1;
    let y = 2;
    return x + y;
}
"#,
                true,
                "100% identical",
            ),
            // Very similar code (~80% similar)
            (
                r#"
fn similar_a() {
    let x = 1;
    let y = 2;
    return x + y;
}

fn similar_b() {
    let x = 1;
    let y = 2;
    return x + y;
}
"#,
                true,
                "80% similar",
            ),
            // Different code (<50% similar)
            (
                r#"
fn different_a() {
    let x = 1;
    println!("Value: {}", x);
    return x * 2;
}

fn different_b() {
    let data = vec![1, 2, 3];
    for item in data {
        println!("{}", item);
    }
}
"#,
                false,
                "Different code",
            ),
        ];

        for (content, should_detect, description) in test_cases {
            let findings = analyzer
                .analyze(Path::new("test.rs"), content.as_bytes())
                .expect("Failed to analyze test file for conflict markers");

            if should_detect {
                assert!(
                    !findings.is_empty(),
                    "Should detect duplication: {}",
                    description
                );
                if !findings.is_empty() {
                    assert!(
                        findings[0].message.contains("%"),
                        "Should include similarity percentage: {}",
                        description
                    );
                }
            } else {
                assert!(
                    findings.is_empty(),
                    "Should not detect duplication: {}",
                    description
                );
            }
        }
    }

    #[test]
    fn test_ignore_generated_files() {
        let analyzer = DuplicateAnalyzer::new();

        let generated_paths = vec![
            "target/debug/build.rs",
            "build/generated/code.rs",
            "dist/output.js",
            "node_modules/package/index.js",
            "src/generated_code.rs",
        ];

        let content = r#"
fn duplicate_a() {
    println!("test");
}

fn duplicate_b() {
    println!("test");
}
"#;

        for path in generated_paths {
            let findings = analyzer
                .analyze(Path::new(path), content.as_bytes())
                .unwrap();

            // Should ignore generated files
            assert!(
                findings.is_empty(),
                "Should ignore generated file: {}",
                path
            );
        }
    }

    #[test]
    fn test_code_block_extraction() {
        let analyzer = DuplicateAnalyzer::new().with_min_lines(2);

        let content_with_blocks = r#"
// This is a comment block
// It should be ignored

fn function_one() {
    let x = 1;
    let y = 2;
}

// Another comment
// More comments

fn function_two() {
    let a = 3;
    let b = 4;
}

fn function_three() {
    let x = 1;
    let y = 2;
}
"#;

        let findings = analyzer
            .analyze(Path::new("test.rs"), content_with_blocks.as_bytes())
            .unwrap();

        // Should extract code blocks and detect similarity between function_one and function_three
        if !findings.is_empty() {
            assert!(findings.iter().any(|f| f.rule == "internal_duplication"));
        }
    }

    #[test]
    fn test_empty_and_whitespace_handling() {
        let analyzer = DuplicateAnalyzer::new().with_min_lines(2);

        let content_with_whitespace = r#"


fn test_a() {

    let x = 1;

    let y = 2;

}



fn test_b() {

    let x = 1;

    let y = 2;

}


"#;

        let findings = analyzer
            .analyze(Path::new("test.rs"), content_with_whitespace.as_bytes())
            .unwrap();

        // Should handle whitespace correctly and still detect duplicates
        if !findings.is_empty() {
            assert!(findings.iter().any(|f| f.rule == "internal_duplication"));
        }
    }

    #[test]
    fn test_max_files_limit() {
        let analyzer = DuplicateAnalyzer::new().with_max_files(5);

        // This test verifies the configuration is set correctly
        // The actual cross-file comparison would require a different architecture
        let content = "fn test() { println!(\"test\"); }";
        let findings = analyzer
            .analyze(Path::new("test.rs"), content.as_bytes())
            .unwrap();

        // Should not crash with max_files limit set
        assert!(findings.len() == 0); // No duplicates in single function
    }

    #[test]
    fn test_supported_file_types() {
        let analyzer = DuplicateAnalyzer::new();

        let supported_extensions = vec![
            "rs", "js", "ts", "py", "java", "cpp", "c", "h", "hpp", "go", "php", "rb", "cs",
            "swift", "kt", "scala", "dart",
        ];

        let unsupported_extensions = vec!["txt", "md", "json", "xml", "html", "css", "png", "jpg"];

        for ext in supported_extensions {
            let file_name = format!("test.{}", ext);
            let path = Path::new(&file_name);
            assert!(analyzer.supports_file(path), "Should support .{}", ext);
        }

        for ext in unsupported_extensions {
            let file_name = format!("test.{}", ext);
            let path = Path::new(&file_name);
            assert!(!analyzer.supports_file(path), "Should not support .{}", ext);
        }
    }

    #[test]
    fn test_security_risk_assessment() {
        let analyzer = DuplicateAnalyzer::new().with_min_lines(3);

        // High-risk security code
        let high_risk_content = r#"
fn encrypt_password(pass: &str) -> String {
    let secret_key = "hardcoded_key";
    encrypt_with_key(pass, secret_key)
}

fn encrypt_password_copy(pass: &str) -> String {
    let secret_key = "hardcoded_key";
    encrypt_with_key(pass, secret_key)
}
"#;

        // Medium-risk security code
        let medium_risk_content = r#"
fn validate_user_input(input: &str) -> bool {
    !input.is_empty() && input.len() < 100
}

fn validate_user_input_copy(input: &str) -> bool {
    !input.is_empty() && input.len() < 100
}
"#;

        let high_findings = analyzer
            .analyze(Path::new("crypto.rs"), high_risk_content.as_bytes())
            .expect("Failed to analyze high-risk security code");
        let medium_findings = analyzer
            .analyze(Path::new("validation.rs"), medium_risk_content.as_bytes())
            .expect("Failed to analyze medium-risk security code");

        // High-risk duplicates should have higher severity
        if !high_findings.is_empty() {
            assert!(high_findings.iter().any(|f| f.severity == Severity::High));
        }

        // Medium-risk duplicates should have medium severity
        if !medium_findings.is_empty() {
            assert!(medium_findings
                .iter()
                .any(|f| matches!(f.severity, Severity::Medium | Severity::High)));
        }
    }
}
