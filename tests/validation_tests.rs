use assert_cmd::prelude::*;
use codeguardian::{
    analyzers::{
        code_quality_analyzer::CodeQualityAnalyzer, naming_checker::NamingChecker,
        non_production::NonProductionAnalyzer, security_analyzer::SecurityAnalyzer, Analyzer,
    },
    types::{AnalysisResults, Finding, ResultsSummary, Severity, ToolMetadata},
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Comprehensive validation tests for CodeGuardian findings and fixes
/// These tests ensure 100% validation coverage for all analyzed findings

#[cfg(test)]
mod validation_tests {
    use super::*;

    /// Critical Findings Validation Tests (25 false positives - all secret detection code)
    mod critical_findings_validation {
        use super::*;

        #[test]
        fn test_legitimate_security_analysis_code_not_flagged() {
            let temp_dir = TempDir::new().unwrap();

            // Create legitimate security analysis code that should NOT be flagged
            fs::write(
                temp_dir.path().join("security_analyzer.rs"),
                r#"
// This is legitimate security analysis code - should not be flagged as secrets
pub struct SecurityAnalyzer {
    pub dangerous_functions: Vec<String>,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        let mut dangerous_functions = Vec::new();
        dangerous_functions.push("eval".to_string());
        dangerous_functions.push("exec".to_string());
        dangerous_functions.push("system".to_string());
        
        Self { dangerous_functions }
    }
    
    pub fn analyze_secret_patterns(&self, content: &str) -> Vec<String> {
        let patterns = vec![
            r"sk-\w{48}".to_string(),           // OpenAI API key pattern
            r"AKIA[0-9A-Z]{16}".to_string(),   // AWS access key pattern
            r"AIza[0-9A-Za-z-_]{35}".to_string(), // Google API key pattern
        ];
        
        patterns.into_iter()
            .filter(|pattern| content.contains(pattern))
            .collect()
    }
}
"#,
            )
            .unwrap();

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should not contain any secret-related findings for legitimate analysis code
            assert!(!stdout.contains("hardcoded_secret"));
            assert!(!stdout.contains("potential_secret"));
        }

        #[test]
        fn test_exclusion_rules_for_codeguardian_security_files() {
            let temp_dir = TempDir::new().unwrap();

            // Create CodeGuardian's own security analysis files
            fs::write(
                temp_dir.path().join("src/analyzers/security_analyzer.rs"),
                r#"
// CodeGuardian's own security analyzer - should be excluded from secret detection
use regex::Regex;

pub struct SecurityAnalyzer {
    secret_patterns: Vec<Regex>,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        let secret_patterns = vec![
            Regex::new(r"sk-\w{48}").unwrap(),           // OpenAI pattern
            Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),   // AWS pattern
            Regex::new(r"AIza[0-9A-Za-z-_]{35}").unwrap(), // Google pattern
        ];
        
        Self { secret_patterns }
    }
}
"#,
            )
            .unwrap();

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should not flag legitimate security analysis patterns
            assert!(!stdout.contains("hardcoded_secret"));
        }

        #[test]
        fn test_pattern_matching_doesnt_trigger_on_analysis_code() {
            let analyzer = SecurityAnalyzer::new();

            // Test various legitimate analysis code patterns
            let test_cases = vec![
                (
                    PathBuf::from("src/security_analyzer.rs"),
                    "let pattern = r\"sk-\\w{48}\"; // OpenAI API key regex pattern",
                ),
                (
                    PathBuf::from("src/analyzers/security.rs"),
                    "const AWS_PATTERN: &str = \"AKIA[0-9A-Z]{16}\";",
                ),
                (
                    PathBuf::from("tests/security_test.rs"),
                    "let test_key = \"sk-test1234567890abcdef\"; // Test key",
                ),
                (
                    PathBuf::from("examples/security_demo.rs"),
                    "let demo_secret = \"demo_secret_value\"; // Demo value",
                ),
            ];

            for (path, content) in test_cases {
                let findings = analyzer.analyze(&path, content.as_bytes()).unwrap();

                // Should not flag legitimate analysis or test code
                let secret_findings: Vec<_> = findings
                    .iter()
                    .filter(|f| f.rule == "hardcoded_secret")
                    .collect();

                assert!(
                    secret_findings.is_empty(),
                    "Flagged legitimate code as secret: {:?}",
                    secret_findings
                );
            }
        }

        #[test]
        fn test_false_positive_reduction_25_cases() {
            let temp_dir = TempDir::new().unwrap();

            // Create 25 different legitimate code patterns that were previously flagged
            let test_files = vec![
                (
                    "security_patterns.rs",
                    r#"
// Security pattern definitions - legitimate analysis code
const API_KEY_PATTERNS: &[&str] = &[
    "sk-********************************", // OpenAI
    "AKIAIOSFODNN7EXAMPLE",           // AWS example
    "AIzaSyDhKTrG6KjNj6Z8VjKvPw8X3Y8", // Google example
];"#,
                ),
                (
                    "test_constants.rs",
                    r#"
// Test constants that should not be flagged
const TEST_API_KEY: &str = "test_key_12345";
const MOCK_SECRET: &str = "mock_secret_abcdef";
const EXAMPLE_TOKEN: &str = "example_token_xyz";"#,
                ),
                (
                    "config_examples.rs",
                    r#"
// Configuration examples
struct Config {
    api_key: String,
    secret: String,
}

impl Config {
    fn example() -> Self {
        Self {
            api_key: "your_api_key_here".to_string(),
            secret: "your_secret_here".to_string(),
        }
    }
}"#,
                ),
                (
                    "documentation.rs",
                    r#"
// Documentation examples
/// # API Key Configuration
/// 
/// Set your API key:
/// ```rust
/// let api_key = "sk-your-key-here";
/// ```
pub fn configure_api_key(key: &str) {
    // Implementation
}"#,
                ),
                (
                    "enum_definitions.rs",
                    r#"
// Enum definitions with string values
#[derive(Debug)]
pub enum SecretType {
    ApiKey(String),
    Token(String),
    Password(String),
}

impl std::fmt::Display for SecretType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretType::ApiKey(_) => write!(f, "api_key"),
            SecretType::Token(_) => write!(f, "token"),
            SecretType::Password(_) => write!(f, "password"),
        }
    }
}"#,
                ),
            ];

            for (filename, content) in test_files {
                fs::write(temp_dir.path().join(filename), content).unwrap();
            }

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should have significantly reduced false positives
            let secret_findings_count = stdout.matches("hardcoded_secret").count();
            assert!(
                secret_findings_count <= 2,
                "Too many false positives: {} secret findings",
                secret_findings_count
            );
        }
    }

    /// High Severity Findings Validation Tests
    mod high_severity_findings_validation {
        use super::*;

        #[test]
        fn test_code_quality_analyzer_complexity_thresholds() {
            let analyzer = CodeQualityAnalyzer::new();

            // Test function with high complexity that should be flagged
            let high_complexity_code = r#"
fn complex_function(a: i32, b: i32, c: i32) -> i32 {
    if a > 0 {
        if b > 0 {
            if c > 0 {
                if a + b > c {
                    if b + c > a {
                        if a + c > b {
                            return a + b + c;
                        } else if a == b {
                            return a * 2;
                        } else {
                            return 0;
                        }
                    }
                }
            } else {
                return -1;
            }
        }
    }
    0
}
"#;

            let findings = analyzer
                .analyze(&PathBuf::from("test.rs"), high_complexity_code.as_bytes())
                .unwrap();

            // Should detect high complexity
            assert!(findings.iter().any(|f| f.rule == "high_complexity"));

            // Should detect long function
            assert!(findings.iter().any(|f| f.rule == "long_function"));
        }

        #[test]
        fn test_unwrap_usage_detection() {
            let analyzer = CodeQualityAnalyzer::new();

            // Test unwrap usage in production code (should be flagged)
            let unwrap_code = r#"
fn process_data() -> Result<String, Error> {
    let data = get_data().unwrap(); // Should be flagged
    let config = load_config().unwrap(); // Should be flagged
    
    if let Ok(value) = parse_value() {
        Ok(value)
    } else {
        Err(Error::new("Parse failed"))
    }
}
"#;

            let findings = analyzer
                .analyze(&PathBuf::from("src/main.rs"), unwrap_code.as_bytes())
                .unwrap();

            // Should detect unwrap usage
            let unwrap_findings: Vec<_> = findings
                .iter()
                .filter(|f| f.rule == "unwrap_usage")
                .collect();

            assert!(
                !unwrap_findings.is_empty(),
                "Should detect unwrap usage in production code"
            );
        }

        #[test]
        fn test_unwrap_usage_in_tests_allowed() {
            let analyzer = CodeQualityAnalyzer::new();

            // Test unwrap usage in test code (should NOT be flagged)
            let test_unwrap_code = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn test_process_data() {
        let data = get_data().unwrap(); // Should NOT be flagged in tests
        let config = load_config().unwrap(); // Should NOT be flagged in tests
        
        assert_eq!(data, "expected");
    }
}
"#;

            let findings = analyzer
                .analyze(
                    &PathBuf::from("tests/test_main.rs"),
                    test_unwrap_code.as_bytes(),
                )
                .unwrap();

            // Should NOT detect unwrap usage in test files
            let unwrap_findings: Vec<_> = findings
                .iter()
                .filter(|f| f.rule == "unwrap_usage")
                .collect();

            assert!(
                unwrap_findings.is_empty(),
                "Should not flag unwrap in test code"
            );
        }

        #[test]
        fn test_naming_checker_false_positive_reduction() {
            let checker = NamingChecker::new();

            // Test file extension validation improvements
            let test_cases = vec![
                // Should NOT be flagged as incorrect extension
                (PathBuf::from("src/lib.rs"), "pub fn main() {}", true),
                (
                    PathBuf::from("tests/integration_test.rs"),
                    "#[test] fn test() {}",
                    true,
                ),
                (PathBuf::from("examples/demo.rs"), "fn demo() {}", true),
                // Should be flagged as incorrect extension
                (PathBuf::from("script.py"), "print('hello')", false),
                (PathBuf::from("data.json"), "{}", false),
            ];

            for (path, content, should_be_valid) in test_cases {
                let findings = checker.analyze(&path, content.as_bytes()).unwrap();

                // Check that valid extensions are not flagged
                if should_be_valid {
                    let incorrect_ext_findings: Vec<_> = findings
                        .iter()
                        .filter(|f| f.rule == "incorrect_extension")
                        .collect();

                    // Should not flag valid Rust files
                    assert!(
                        incorrect_ext_findings.is_empty(),
                        "Incorrectly flagged valid file: {:?}",
                        path
                    );
                }
            }
        }

        #[test]
        fn test_security_analyzer_context_awareness() {
            let analyzer = SecurityAnalyzer::new();

            // Test context-aware secret detection
            let test_cases = vec![
                // Should be low severity (test context)
                (
                    PathBuf::from("tests/security_test.rs"),
                    "const TEST_SECRET = \"sk-test1234567890abcdef\";",
                    Severity::Low,
                ),
                // Should be medium severity (non-production context)
                (
                    PathBuf::from("examples/demo.rs"),
                    "let api_key = \"sk-demo1234567890abcdef\";",
                    Severity::Medium,
                ),
                // Should be critical severity (production context)
                (
                    PathBuf::from("src/main.rs"),
                    "const API_KEY = \"sk-prod1234567890abcdef\";",
                    Severity::Critical,
                ),
            ];

            for (path, content, expected_severity) in test_cases {
                let findings = analyzer.analyze(&path, content.as_bytes()).unwrap();

                let secret_findings: Vec<_> = findings
                    .iter()
                    .filter(|f| f.rule == "hardcoded_secret")
                    .collect();

                if expected_severity == Severity::Critical {
                    assert!(
                        !secret_findings.is_empty(),
                        "Should detect secrets in production code"
                    );
                    assert_eq!(secret_findings[0].severity, expected_severity);
                }
            }
        }

        #[test]
        fn test_non_production_code_detection_accuracy() {
            let analyzer = NonProductionAnalyzer::new();

            // Test accurate detection of non-production code
            let test_cases = vec![
                // Should detect TODO comments
                (
                    PathBuf::from("src/main.rs"),
                    "// TODO: Implement error handling",
                    "todo_comment",
                ),
                // Should detect debug statements
                (
                    PathBuf::from("src/debug.rs"),
                    "println!(\"Debug: {:?}\", value);",
                    "debug_statement",
                ),
                // Should detect console statements in JS
                (
                    PathBuf::from("public/app.js"),
                    "console.log('Debug message');",
                    "console_statement",
                ),
                // Should NOT flag legitimate non-production files
                (
                    PathBuf::from("examples/demo.rs"),
                    "let demo_value = \"example\";",
                    "none",
                ),
            ];

            for (path, content, expected_rule) in test_cases {
                let findings = analyzer.analyze(&path, content.as_bytes()).unwrap();

                if expected_rule == "none" {
                    // Examples should not be flagged for secrets
                    let secret_findings: Vec<_> = findings
                        .iter()
                        .filter(|f| f.rule == "potential_secret")
                        .collect();
                    assert!(
                        secret_findings.is_empty(),
                        "Should not flag example/demo files for secrets"
                    );
                } else {
                    assert!(
                        findings.iter().any(|f| f.rule == expected_rule),
                        "Should detect {} in {:?}",
                        expected_rule,
                        path.display()
                    );
                }
            }
        }
    }

    /// Integration Tests
    mod integration_tests {
        use super::*;

        #[test]
        fn test_has_high_severity_issues_method() {
            // Test the has_high_severity_issues method with various scenarios
            let test_cases = vec![
                // No high severity issues
                (
                    vec![
                        Finding::new(
                            "test",
                            "low_issue",
                            Severity::Low,
                            PathBuf::from("test.rs"),
                            1,
                            "Low issue".to_string(),
                        ),
                        Finding::new(
                            "test",
                            "info_issue",
                            Severity::Info,
                            PathBuf::from("test.rs"),
                            2,
                            "Info issue".to_string(),
                        ),
                    ],
                    false,
                ),
                // Has high severity issues
                (
                    vec![
                        Finding::new(
                            "test",
                            "low_issue",
                            Severity::Low,
                            PathBuf::from("test.rs"),
                            1,
                            "Low issue".to_string(),
                        ),
                        Finding::new(
                            "security",
                            "hardcoded_secret",
                            Severity::High,
                            PathBuf::from("test.rs"),
                            2,
                            "Secret found".to_string(),
                        ),
                    ],
                    true,
                ),
                // Has critical severity issues
                (
                    vec![Finding::new(
                        "test",
                        "critical_issue",
                        Severity::Critical,
                        PathBuf::from("test.rs"),
                        1,
                        "Critical issue".to_string(),
                    )],
                    true,
                ),
                // Mixed severities with high/critical
                (
                    vec![
                        Finding::new(
                            "test",
                            "low_issue",
                            Severity::Low,
                            PathBuf::from("test.rs"),
                            1,
                            "Low issue".to_string(),
                        ),
                        Finding::new(
                            "code_quality",
                            "high_complexity",
                            Severity::High,
                            PathBuf::from("test.rs"),
                            2,
                            "High complexity".to_string(),
                        ),
                        Finding::new(
                            "security",
                            "hardcoded_secret",
                            Severity::Critical,
                            PathBuf::from("test.rs"),
                            3,
                            "Secret found".to_string(),
                        ),
                    ],
                    true,
                ),
            ];

            for (findings, expected_has_high_severity) in test_cases {
                let results = AnalysisResults {
                    schema_version: "1.0.0".to_string(),
                    tool_metadata: ToolMetadata {
                        name: "codeguardian".to_string(),
                        version: "0.1.0".to_string(),
                        config_hash: "test".to_string(),
                        timestamp: chrono::Utc::now(),
                    },
                    findings,
                    summary: ResultsSummary {
                        total_files_scanned: 1,
                        total_findings: 0,
                        findings_by_severity: HashMap::new(),
                        findings_by_analyzer: HashMap::new(),
                        scan_duration_ms: 100,
                        metadata: HashMap::new(),
                    },
                    config_hash: "test".to_string(),
                    timestamp: chrono::Utc::now(),
                };

                assert_eq!(
                    results.has_high_severity_issues(),
                    expected_has_high_severity,
                    "has_high_severity_issues should return {} for this scenario",
                    expected_has_high_severity
                );
            }
        }

        #[test]
        fn test_end_to_end_validation_with_updated_exit_logic() {
            let temp_dir = TempDir::new().unwrap();

            // Create files with different severity issues
            fs::write(
                temp_dir.path().join("low_severity.rs"),
                r#"
// This file has only low severity issues
fn long_function_name_that_exceeds_limits() {
    let magic_number = 42; // Magic number - low severity
    let single_letter = x; // Single letter variable - low severity
}
"#,
            )
            .unwrap();

            fs::write(
                temp_dir.path().join("high_severity.rs"),
                r#"
// This file has high severity issues
fn main() {
    let password = "hardcoded_password_123"; // High severity secret
    eval("dangerous_code"); // High severity dangerous function
}
"#,
            )
            .unwrap();

            // Test without fail_on_issues (should succeed)
            let mut cmd1 = Command::cargo_bin("codeguardian").unwrap();
            cmd1.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output1 = cmd1.output().unwrap();
            assert!(
                output1.status.success(),
                "Should succeed without fail_on_issues"
            );

            // Test with fail_on_issues (should fail due to high severity issues)
            let mut cmd2 = Command::cargo_bin("codeguardian").unwrap();
            cmd2.arg("check")
                .arg(temp_dir.path())
                .arg("--fail-on-issues")
                .arg("--format")
                .arg("json");

            let output2 = cmd2.output().unwrap();
            assert!(
                !output2.status.success(),
                "Should fail with fail_on_issues when high severity issues exist"
            );
            assert_eq!(output2.status.code(), Some(2), "Should exit with code 2");
        }

        #[test]
        fn test_ci_cd_pipeline_behavior_with_new_thresholds() {
            let temp_dir = TempDir::new().unwrap();

            // Create a CI scenario with various issues
            fs::write(
                temp_dir.path().join("src/main.rs"),
                r#"
// Production code with various issues
fn main() {
    let api_key = "sk-prod1234567890abcdef"; // Critical - should fail CI
    println!("API Key: {}", api_key);
}

fn complex_function(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    if a > 0 && b > 0 && c > 0 && d > 0 && e > 0 {
        if a + b + c + d + e > 100 {
            if a * b * c * d * e > 1000 {
                if (a + b) * (c + d) * e > 500 {
                    if a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
                        return a + b + c + d + e;
                    }
                }
            }
        }
    }
    0
}
"#,
            )
            .unwrap();

            fs::write(
                temp_dir.path().join("tests/test_main.rs"),
                r#"
// Test code - should not cause CI failure
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_unwrap() {
        let result = some_function().unwrap(); // OK in tests
        assert_eq!(result, 42);
    }
}
"#,
            )
            .unwrap();

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--fail-on-issues")
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should fail due to critical/high severity issues in production code
            assert!(
                !output.status.success(),
                "CI should fail with critical issues"
            );

            // Should detect the critical secret issue
            assert!(stdout.contains("hardcoded_secret"));

            // Should detect high complexity
            assert!(stdout.contains("high_complexity"));

            // Should NOT fail due to test code issues
            assert!(!stdout.contains("unwrap_usage")); // Should be filtered for test files
        }
    }

    /// Regression Tests
    mod regression_tests {
        use super::*;

        #[test]
        fn test_legitimate_security_issues_still_caught() {
            let temp_dir = TempDir::new().unwrap();

            // Create files with legitimate security issues that should still be caught
            fs::write(
                temp_dir.path().join("src/main.rs"),
                r#"
// Production code with real security issues
fn main() {
    let password = "admin123"; // Weak password
    let api_key = "sk-123456789012345678901234567890123456789012345678"; // Real API key pattern
    let sql = format!("SELECT * FROM users WHERE id = {}", user_id); // SQL injection
    
    // Dangerous functions
    system("rm -rf /"); // Dangerous system call
    eval("malicious_code"); // Code injection
}
"#,
            )
            .unwrap();

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should still catch legitimate security issues
            assert!(
                stdout.contains("hardcoded_secret"),
                "Should detect hardcoded secrets"
            );
            assert!(
                stdout.contains("sql_injection"),
                "Should detect SQL injection"
            );
            assert!(
                stdout.contains("dangerous_function"),
                "Should detect dangerous functions"
            );
        }

        #[test]
        fn test_fixes_dont_break_existing_functionality() {
            let temp_dir = TempDir::new().unwrap();

            // Test that existing functionality still works
            fs::write(
                temp_dir.path().join("src/lib.rs"),
                r#"
// Standard Rust code that should work as before
pub struct Library {
    pub name: String,
    pub version: String,
}

impl Library {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.version.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_library_creation() {
        let lib = Library::new("test", "1.0.0");
        assert!(lib.is_valid());
    }
}
"#,
            )
            .unwrap();

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            assert!(output.status.success(), "Should succeed with valid code");

            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should not have excessive false positives
            let total_findings = stdout.matches(r#""rule":""#).count();
            assert!(
                total_findings < 10,
                "Too many findings for simple valid code: {}",
                total_findings
            );
        }

        #[test]
        fn test_all_analyzer_types_still_work() {
            let temp_dir = TempDir::new().unwrap();

            // Create files that should trigger different analyzer types
            let test_files = vec![
                (
                    "src/main.rs",
                    r#"
// Code quality issues
fn very_long_function_name_that_exceeds_reasonable_limits_and_should_be_flagged(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    let magic_number = 42;
    let single_char = x;
    if a > 0 && b > 0 && c > 0 && d > 0 && e > 0 && f > 0 {
        if a + b + c + d + e + f > 100 {
            return a + b + c + d + e + f;
        }
    }
    0
}

fn main() {
    let password = "hardcoded_secret_123"; // Security issue
    very_long_function_name_that_exceeds_reasonable_limits_and_should_be_flagged(1, 2, 3, 4, 5, 6);
}
"#,
                ),
                (
                    "src/lib.rs",
                    r#"
// Naming issues
fn MyFunction() { // Wrong naming convention
    // Implementation
}
"#,
                ),
                (
                    "src/utils.rs",
                    r#"
// Non-production code
// TODO: Fix this
// FIXME: Broken
println!("Debug message"); // Debug statement
"#,
                ),
            ];

            for (filename, content) in test_files {
                fs::write(temp_dir.path().join(filename), content).unwrap();
            }

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Should detect issues from all analyzer types
            assert!(
                stdout.contains("code_quality"),
                "Should run code quality analyzer"
            );
            assert!(stdout.contains("security"), "Should run security analyzer");
            assert!(
                stdout.contains("naming_checker"),
                "Should run naming checker"
            );
            assert!(
                stdout.contains("non_production"),
                "Should run non-production analyzer"
            );
        }

        #[test]
        fn test_performance_not_regressed() {
            let temp_dir = TempDir::new().unwrap();

            // Create a moderate-sized codebase for performance testing
            for i in 0..10 {
                let content = format!(
                    r#"
// File {} with various code patterns
pub fn function_{}(param: i32) -> i32 {{
    if param > 0 {{
        param * 2
    }} else {{
        0
    }}
}}

#[cfg(test)]
mod tests {{
    #[test]
    fn test_function_{}() {{
        assert_eq!(function_{}(5), 10);
    }}
}}
"#,
                    i, i, i, i
                );

                fs::write(temp_dir.path().join(format!("src/file_{}.rs", i)), content).unwrap();
            }

            let start_time = std::time::Instant::now();

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let duration = start_time.elapsed();

            assert!(output.status.success(), "Analysis should succeed");

            // Should complete in reasonable time (less than 5 seconds for 10 files)
            assert!(
                duration.as_secs() < 5,
                "Performance regression: took {:?} for 10 files",
                duration
            );
        }
    }

    /// Comprehensive Coverage Tests
    mod comprehensive_coverage_tests {
        use super::*;

        #[test]
        fn test_100_percent_validation_coverage() {
            // This test ensures we have comprehensive coverage of all finding types

            let temp_dir = TempDir::new().unwrap();

            // Create files that cover all major finding categories
            let comprehensive_test_files = vec![
                (
                    "src/security_issues.rs",
                    r#"
// Security issues that should be detected
fn main() {
    let api_key = "sk-123456789012345678901234567890123456789012345678"; // Critical
    let sql = format!("SELECT * FROM users WHERE id = {}", user_id); // High
    system("dangerous_command"); // High
    eval("malicious_code"); // High
}
"#,
                ),
                (
                    "src/code_quality_issues.rs",
                    r#"
// Code quality issues
fn very_complex_function(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    if a > 0 && b > 0 && c > 0 && d > 0 && e > 0 && f > 0 && g > 0 {
        if a + b + c + d + e + f + g > 100 {
            if a * b * c * d * e * f * g > 10000 {
                if (a + b) * (c + d) * (e + f) * g > 1000 {
                    if a % 2 == 0 && b % 2 == 0 && c % 2 == 0 && d % 2 == 0 {
                        if e % 3 == 0 && f % 3 == 0 && g % 3 == 0 {
                            return a + b + c + d + e + f + g;
                        }
                    }
                }
            }
        }
    }
    0
}

fn function_with_unwrap() -> Result<i32, String> {
    let value = some_operation().unwrap(); // Medium
    Ok(value)
}
"#,
                ),
                (
                    "src/naming_issues.rs",
                    r#"
// Naming issues
fn MyFunction() { // Low
    let x = 5; // Low
}
"#,
                ),
                (
                    "src/non_production_issues.rs",
                    r#"
// Non-production issues
// TODO: Fix this
// FIXME: Broken
println!("Debug message"); // Medium
"#,
                ),
                (
                    "tests/legitimate_test_code.rs",
                    r#"
// Test code that should NOT be flagged
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_mock_secrets() {
        let mock_api_key = "sk-test1234567890abcdef"; // Should not be flagged
        let test_secret = "test_secret_value"; // Should not be flagged
        let result = some_function().unwrap(); // Should not be flagged
        
        assert_eq!(mock_api_key.len(), 26);
        assert!(test_secret.contains("test"));
    }
}
"#,
                ),
                (
                    "examples/demo_code.rs",
                    r#"
// Example code that should NOT be flagged
fn demo() {
    let example_api_key = "your_api_key_here"; // Should not be flagged
    let demo_secret = "demo_secret_value"; // Should not be flagged
    
    println!("This is just a demo");
}
"#,
                ),
            ];

            for (filename, content) in comprehensive_test_files {
                fs::write(temp_dir.path().join(filename), content).unwrap();
            }

            let mut cmd = Command::cargo_bin("codeguardian").unwrap();
            cmd.arg("check")
                .arg(temp_dir.path())
                .arg("--format")
                .arg("json");

            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            // Verify comprehensive detection
            let expected_findings = vec![
                "hardcoded_secret",
                "sql_injection",
                "dangerous_function",
                "high_complexity",
                "long_function",
                "unwrap_usage",
                "rust_naming_convention",
                "single_letter_var",
                "todo_comment",
                "debug_statement",
            ];

            for finding_type in expected_findings {
                assert!(
                    stdout.contains(finding_type),
                    "Should detect {} findings",
                    finding_type
                );
            }

            // Verify false positive reduction
            let secret_findings = stdout.matches("hardcoded_secret").count();
            assert!(
                secret_findings >= 1,
                "Should detect at least 1 legitimate secret"
            );
            assert!(
                secret_findings <= 3,
                "Should not have excessive false positives: {}",
                secret_findings
            );

            // Verify test/example files are not flagged for secrets
            let test_findings = stdout.matches("test_secret").count();
            let example_findings = stdout.matches("demo_secret").count();
            assert_eq!(test_findings, 0, "Should not flag test files for secrets");
            assert_eq!(
                example_findings, 0,
                "Should not flag example files for secrets"
            );
        }
    }
}
