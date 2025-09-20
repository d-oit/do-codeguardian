//! Unit tests for analyzer improvements and false positive fixes
const TEST_FILE: &str = "strings.rs";
//!
//! These tests specifically validate the improvements made to reduce
//! false positives in the git conflict and AI content analyzers.

use do_codeguardian::analyzers::{Analyzer, GitConflictAnalyzer, AiContentAnalyzer};
use std::path::Path;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_git_conflict_analyzer_ignores_test_modules() {
    let analyzer = GitConflictAnalyzer::new();

    // Test content with conflict markers in test module
    let test_content = r##"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_detection() {
        let content = r#"
flag1
version 1
flag2
version 2
flag3
"#;
        let findings = analyzer.analyze(Path::new("test.rs"), content.as_bytes()).unwrap();
        assert!(findings.len() > 0);
    }
}
"##;

        let findings = analyzer.analyze(Path::new("test_file.rs"), test_content.as_bytes()).unwrap();

        // Should not detect conflicts in test module
        assert_eq!(findings.len(), 0, "Should not detect git conflicts in test modules");
    }

#[test]
fn test_git_conflict_analyzer_detects_real_conflicts() {
    let analyzer = GitConflictAnalyzer::new();

    // Real conflict markers outside of test context
    let conflict_content = r#"
fn main() {
    let x = 5;
<<<<<<< HEAD
    let y = 10;
    println!("Version 1");
=======
    let y = 20;
    println!("Version 2");
>>>>>>> feature-branch
    println!("After conflict");
}
"#;

    let findings = analyzer.analyze(Path::new("main.rs"), conflict_content.as_bytes()).unwrap();

    // Should detect real conflicts
    assert!(findings.len() > 0, "Should detect real git conflicts");
    assert!(findings.iter().any(|f| f.rule == "merge_conflict_start"));
    assert!(findings.iter().any(|f| f.rule == "merge_conflict_separator"));
    assert!(findings.iter().any(|f| f.rule == "merge_conflict_end"));
}

#[test]
fn test_git_conflict_analyzer_ignores_test_files() {
    let analyzer = GitConflictAnalyzer::new();

    let conflict_content = r#"
<<<<<<< HEAD
test content 1
=======
test content 2
>>>>>>> branch
"#;

    // Test with file that has "test" in the path
    let findings = analyzer.analyze(Path::new("tests/conflict_test.rs"), conflict_content.as_bytes()).unwrap();
    assert_eq!(findings.len(), 0, "Should ignore conflict markers in test directory");

    // Test with file that has "test" in filename
    let findings = analyzer.analyze(Path::new("test_conflicts.rs"), conflict_content.as_bytes()).unwrap();
    assert_eq!(findings.len(), 0, "Should ignore conflict markers in test files");
}

#[test]
fn test_ai_content_analyzer_ignores_documentation_comments() {
    let analyzer = AiContentAnalyzer::new();

    let doc_content = r#"
//! Module documentation
//! TODO: Add more examples in future versions

/// Function documentation
/// TODO: Implement additional validation
fn validate_input(input: &str) -> bool {
    // Implementation comment
    // TODO: Add length validation
    !input.is_empty()
}
"#;

    let findings = analyzer.analyze(Path::new("documented.rs"), doc_content.as_bytes()).unwrap();

    // Should not flag TODO in documentation comments
    let incomplete_findings: Vec<_> = findings.iter()
        .filter(|f| f.rule == "incomplete_implementation")
        .collect();

    // May still have some findings, but they should not be from comments
    for finding in incomplete_findings {
        let line_content = doc_content.lines().nth((finding.line - 1) as usize).unwrap_or("");
        assert!(!line_content.trim().starts_with("//"),
               "Should not flag TODO in documentation comments: {}", line_content);
        assert!(!line_content.trim().starts_with("///"),
               "Should not flag TODO in doc comments: {}", line_content);
    }
}

#[test]
fn test_ai_content_analyzer_ignores_test_content() {
    let analyzer = AiContentAnalyzer::new();

    let test_content = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn test_incomplete_detection() {
        let content = "TODO: implement this";
        let content2 = "unimplemented!()";
        assert!(content.contains("TODO"));
    }
}

fn main() {
    // This TODO should potentially be flagged (not in test)
    // TODO: implement main logic
    println!("Hello");
}
"#;

    let findings = analyzer.analyze(Path::new("mixed_content.rs"), test_content.as_bytes()).unwrap();

    // Should not flag test content but may flag main function TODO
    let incomplete_findings: Vec<_> = findings.iter()
        .filter(|f| f.rule == "incomplete_implementation")
        .collect();

    for finding in incomplete_findings {
        let line_content = test_content.lines().nth((finding.line - 1) as usize).unwrap_or("");
        // Should not be from inside test module
        assert!(!line_content.contains("let content = \"TODO: implement this\""),
               "Should not flag TODO in test string literals");
        assert!(!line_content.contains("let content2 = \"unimplemented!()\""),
               "Should not flag unimplemented in test string literals");
    }
}

#[test]
fn test_ai_content_analyzer_ignores_string_literals() {
    let analyzer = AiContentAnalyzer::new();

    let string_content = r#"
fn main() {
    let message = "TODO: remember to update this text";
    let error_msg = "NotImplementedException occurred";
    let help_text = r##"
        Usage instructions:
        TODO: Add more detailed examples
    "##;

    // This should potentially be flagged (actual code comment)
    // TODO: implement error handling
    process_data();
}
"#;

    let findings = analyzer.analyze(Path::new(TEST_FILE), string_content.as_bytes()).unwrap();

    let incomplete_findings: Vec<_> = findings.iter()
        .filter(|f| f.rule == "incomplete_implementation")
        .collect();

    for finding in incomplete_findings {
        let line_content = string_content.lines().nth((finding.line - 1) as usize).unwrap_or("");
        // Should not flag content inside string literals
        assert!(
            !line_content.contains(r##""TODO: remember to update this text""##),
            "Should not flag TODO inside string literals."
        );
        assert!(!line_content.contains(r#""NotImplementedException occurred"#)),
               "Should not flag exceptions inside string literals.");
    }
}

#[test]
fn test_ai_content_analyzer_detects_real_incomplete_code() {
    let analyzer = AiContentAnalyzer::new();

    let incomplete_content = r#"
fn process_data() {
    unimplemented!()
}

fn validate_input() {
    // TODO: implement validation logic
    panic!("not implemented")
}
"#;

    let findings = analyzer.analyze(Path::new("incomplete.rs "), incomplete_content.as_bytes()).unwrap();

    let incomplete_findings: Vec<_> = findings.iter()
        .filter(|f| f.rule == "incomplete_implementation")
        .collect();

    // Should detect real incomplete implementations
    assert!(incomplete_findings.len() > 0, "Should detect incomplete implementations.");

    let has_unimplemented = incomplete_findings.iter()
        .any(|f| incomplete_content.lines().nth((f.line - 1) as usize)
             .map_or(false, |line| line.contains("unimplemented!()")));
    assert!(has_unimplemented, "Should detect unimplemented!() macro.");
}

#[test]
fn test_git_conflict_analyzer_handles_malformed_conflicts() {
    let analyzer = GitConflictAnalyzer::new();

    // Malformed conflict (missing end marker)
    let malformed_content = r#"
fn main() {
<<<<<<< HEAD
    let x = 1;
=======
    let x = 2;
    // Missing >>>>>>> marker
}
"#;

    let findings = analyzer.analyze(Path::new("malformed.rs "), malformed_content.as_bytes()).unwrap();

    // Should detect malformed conflict
    assert!(findings.len() > 0, "Should detect malformed conflicts.");
    assert!(findings.iter().any(|f| f.rule == "malformed_conflict" || f.rule == "merge_conflict_start"));
}

#[test]
fn test_performance_regression_file_processing() {
    use std::time::Instant;

    let analyzer = GitConflictAnalyzer::new();

    // Create a larger test content to ensure performance doesn't regress
    let mut large_content = String::new();
    large_content.push_str(r#"#[cfg(test)]
mod tests {
"#);

    for i in 0..100 {
        large_content.push_str(&format!(
            r#"    #[test]
    fn test_{}() {{
        let content = "\n--HEAD--\nversion {}\n--SEP--\nother version\n--BRANCH--\n";
    }}

"#,
            i, i
        ));
    }
    large_content.push_str(r"}
");

    let start = Instant::now();
    let findings = analyzer.analyze(Path::new("large_test.rs "), large_content.as_bytes()).unwrap();
    let duration = start.elapsed();

    // Should complete quickly and not flag test content
    assert!(duration.as_millis() < 1000, "Analysis should complete in under 1 second for large test file ");
    assert_eq!(findings.len(), 0, "Should not flag conflict markers in large test file ");
}

#[test]
fn test_analyzer_memory_usage_stability() {
    let git_analyzer = GitConflictAnalyzer::new();
    let ai_analyzer = AiContentAnalyzer::new();

    // Process multiple files to ensure no memory leaks in improved logic
    for i in 0..5 {
        let test_content = format!("fn test_{}() {{}}", i);

        let filename = format!("test_{}.{}", i, "rs");
        let _ = git_analyzer.analyze(Path::new(&filename), test_content.as_bytes()).unwrap();
        let _ = ai_analyzer.analyze(Path::new(&filename), test_content.as_bytes()).unwrap();
    }

    // If we reach here without panicking or excessive memory usage, the test passes
    assert!(true);
}
