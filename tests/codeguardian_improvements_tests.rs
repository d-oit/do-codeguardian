use std::fs;
use std::path::Path;
use tempfile::TempDir;

use do_codeguardian::analyzers::security::SecretAnalyzer;
use do_codeguardian::analyzers::{
    non_production::NonProductionAnalyzer, security_analyzer::SecurityAnalyzer, Analyzer,
};

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    /// Test edge cases for file exclusion patterns
    #[test]
    fn test_file_exclusion_edge_cases() {
        let security_analyzer = SecurityAnalyzer::new();
        let non_prod_analyzer = NonProductionAnalyzer::new();

        // Edge case 1: Files with "test" in middle of name (should NOT be excluded)
        let legitimate_files = [
            "src/contest.rs",       // Contains "test" but not a test file
            "src/protest.rs",       // Contains "test" but not a test file
            "src/latest.rs",        // Contains "test" but not a test file
            "src/fastest_algo.rs",  // Contains "test" but not a test file
            "src/testing_utils.rs", // Contains "test" but is utility, not test
        ];

        for file_path in &legitimate_files {
            let path = Path::new(file_path);
            let content = b"let api_key = \"sk-proj1234567890abcdefghijklmnopqrstuvwxyz\";";

            // These should be analyzed (not skipped)
            let security_findings = security_analyzer.analyze(path, content).unwrap();
            let non_prod_findings = non_prod_analyzer.analyze(path, content).unwrap();

            assert!(
                !security_findings.is_empty() || !non_prod_findings.is_empty(),
                "File {} should be analyzed and detect secrets",
                file_path
            );
        }

        // Edge case 2: Files that should definitely be excluded
        let test_files = [
            "tests/auth_test.rs",
            "benches/performance_benchmark.rs",
            "src/utils_test.rs",
            "tests/integration_tests.rs",
            "benches/crypto_bench.rs",
            "examples/usage_example.rs",
            "fixtures/test_data.rs",
            "mocks/mock_service.rs",
            "test_helpers/common.rs",
            "bench_utils/setup.rs",
        ];

        for file_path in &test_files {
            let path = Path::new(file_path);
            let content = b"let api_key = \"sk-proj1234567890abcdefghijklmnopqrstuvwxyz\";";

            // These should be skipped (no findings)
            let security_findings = security_analyzer.analyze(path, content).unwrap();
            let non_prod_findings = non_prod_analyzer.analyze(path, content).unwrap();

            assert!(
                security_findings.is_empty() && non_prod_findings.is_empty(),
                "File {} should be excluded from analysis",
                file_path
            );
        }
    }

    /// Test edge cases for secret pattern detection
    #[test]
    fn test_secret_pattern_edge_cases() {
        let analyzer = SecretAnalyzer::new();

        // Edge case 1: Secrets in different contexts that should be ignored
        let false_positive_cases = [
            // Test function contexts
            "#[test]\nfn test_auth() {\n    let key = \"sk-1234567890abcdef\";\n}",
            "#[cfg(test)]\nmod tests {\n    const API_KEY: &str = \"sk-test123456789\";\n}",

            // Benchmark contexts
            "#[bench]\nfn bench_crypto(b: &mut Bencher) {\n    let secret = \"dummy_secret_123\";\n}",

            // Documentation and comments
            "/// Example usage:\n/// let api_key = \"sk-your-key-here\";",
            "// TODO: Replace with real key: sk-placeholder123",
            "/* Example API key: sk-1234567890abcdef */",

            // Pattern definitions
            "Regex::new(r\"sk-[a-zA-Z0-9]+\").unwrap()",
            "let pattern = \"api_key = \\\"sk-\\\"\";",
            "const SECRET_PATTERN: &str = \"password = \";",

            // Environment variable access
            "let key = env!(\"API_KEY\");",
            "let secret = std::env::var(\"SECRET_KEY\").unwrap();",

            // Mock and test data
            "let mock_password = \"fake_password_123\";",
            "const DUMMY_TOKEN: &str = \"dummy_token_456\";",
            "let example_key = \"example_api_key_789\";",
            "let placeholder_secret = \"placeholder_secret_000\";",

            // Obviously fake patterns
            "let key = \"sk-1234567890abcdef\";",  // Repetitive pattern
            "let token = \"xxxxxxxxxxxxxxxxxx\";",  // All x's
            "let secret = \"0123456789abcdefghij\";", // Sequential
        ];

        for (i, test_case) in false_positive_cases.iter().enumerate() {
            let findings = analyzer
                .analyze(Path::new("src/test.rs"), test_case.as_bytes())
                .unwrap();
            assert!(
                findings.is_empty(),
                "Case {}: Should not detect secret in: {}",
                i,
                test_case.lines().next().unwrap_or(test_case)
            );
        }

        // Edge case 2: Real secrets that should still be detected
        let real_secret_cases = [
            "let api_key = \"sk-proj-RealSecretKeyThatShouldBeDetected123456\";",
            "const PASSWORD: &str = \"MyRealSecretPassword123!\";",
            "let auth_token = \"ghp_RealGitHubTokenThatShouldBeDetected123456789\";",
            "api_key = \"AKIA1234567890REALKEY\";",
            "let secret = \"real-production-secret-value-here\";",
        ];

        for (i, test_case) in real_secret_cases.iter().enumerate() {
            let findings = analyzer
                .analyze(Path::new("src/config.rs"), test_case.as_bytes())
                .unwrap();
            assert!(
                !findings.is_empty(),
                "Case {}: Should detect real secret in: {}",
                i,
                test_case
            );
        }
    }

    /// Test edge cases for mixed content scenarios
    #[test]
    fn test_mixed_content_scenarios() {
        let security_analyzer = SecurityAnalyzer::new();

        // Edge case 1: File with both test patterns and real secrets
        let mixed_content = r#"
// This is a test file with mixed content
#[cfg(test)]
mod tests {
    // This should be ignored - test pattern
    const TEST_KEY: &str = "sk-1234567890abcdef";

    #[test]
    fn test_auth() {
        let dummy_password = "fake_password_123";
        assert!(authenticate(dummy_password));
    }
}

// This is production code - should be detected
pub struct Config {
    // This should be detected as a real secret
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key: "sk-proj-RealProductionKeyThatShouldBeDetected123".to_string(),
        }
    }
}
"#;

        // Test in a source file (should detect the real secret)
        let findings = security_analyzer
            .analyze(Path::new("src/config.rs"), mixed_content.as_bytes())
            .unwrap();
        assert!(
            !findings.is_empty(),
            "Should detect the real secret in production code"
        );

        // Verify it only detects the real secret, not the test patterns
        let secret_findings: Vec<_> = findings
            .iter()
            .filter(|f| f.rule == "hardcoded_secret")
            .collect();

        // Should find the real secret but not the test patterns
        assert!(
            secret_findings.len() >= 1,
            "Should detect at least one real secret"
        );

        // Check that the detected secret is from the production code section
        let production_findings: Vec<_> = secret_findings
            .iter()
            .filter(|f| f.line > 15) // Production code starts after line 15
            .collect();

        assert!(
            !production_findings.is_empty(),
            "Should detect secret in production code section"
        );
    }

    /// Test edge cases for file path variations
    #[test]
    fn test_file_path_variations() {
        let analyzer = SecurityAnalyzer::new();
        let secret_content = b"let api_key = \"sk-proj1234567890abcdefghijklmnopqrstuvwxyz\";";

        // Edge case 1: Different path separators and structures
        let path_variations = [
            // Windows-style paths
            "tests\\auth_test.rs",
            "benches\\performance_benchmark.rs",
            "src\\utils_test.rs",
            // Nested test directories
            "tests/unit/auth_test.rs",
            "tests/integration/api_test.rs",
            "benches/micro/crypto_bench.rs",
            // Different test naming conventions
            "test/auth.rs",
            "testing/utils.rs",
            "benchmark/performance.rs",
            // Edge cases with similar names
            "src/test_utils.rs",    // Should be excluded (starts with test_)
            "src/utils_test.rs",    // Should be excluded (ends with _test)
            "src/bench_helpers.rs", // Should be excluded (starts with bench_)
            "src/helpers_bench.rs", // Should be excluded (ends with _bench)
        ];

        for file_path in &path_variations {
            let path = Path::new(file_path);
            let findings = analyzer.analyze(path, secret_content).unwrap();

            assert!(
                findings.is_empty(),
                "Path {} should be excluded from analysis",
                file_path
            );
        }

        // Edge case 2: Legitimate files that should NOT be excluded
        let legitimate_paths = [
            "src/contest_manager.rs",   // Contains "test" but not a test file
            "src/protest_handler.rs",   // Contains "test" but not a test file
            "src/fastest_algorithm.rs", // Contains "test" but not a test file
            "src/benchmark_results.rs", // Contains "benchmark" but not in test dir
            "src/testing_framework.rs", // Contains "testing" but not a test file
        ];

        for file_path in &legitimate_paths {
            let path = Path::new(file_path);
            let findings = analyzer.analyze(path, secret_content).unwrap();

            assert!(
                !findings.is_empty(),
                "Path {} should be analyzed and detect secrets",
                file_path
            );
        }
    }

    /// Test edge cases for content with tricky patterns
    #[test]
    fn test_tricky_content_patterns() {
        let analyzer = SecretAnalyzer::new();

        // Edge case 1: Multiline scenarios
        let multiline_cases = [
            // Test function spanning multiple lines
            r#"#[test]
fn test_complex_auth() {
    let api_key = "sk-1234567890abcdef";
    let result = authenticate(api_key);
    assert!(result.is_ok());
}"#,
            // Comment blocks
            r#"/*
 * Example configuration:
 * api_key = "sk-your-key-here"
 * password = "your-password"
 */"#,
            // Mixed legitimate and test code
            r#"
// Production configuration
let real_key = "sk-proj-RealKeyThatShouldBeDetected123456";

#[cfg(test)]
mod tests {
    // Test key - should be ignored
    let test_key = "sk-1234567890abcdef";
}
"#,
        ];

        for (i, content) in multiline_cases.iter().enumerate() {
            let findings = analyzer
                .analyze(Path::new("src/test.rs"), content.as_bytes())
                .unwrap();

            // For the mixed case, should only detect the real key
            if i == 2 {
                assert!(
                    !findings.is_empty(),
                    "Case {}: Should detect the real key",
                    i
                );
                // Should detect exactly one secret (the real one)
                assert_eq!(
                    findings.len(),
                    1,
                    "Case {}: Should detect exactly one real secret",
                    i
                );
            } else {
                assert!(
                    findings.is_empty(),
                    "Case {}: Should not detect secrets in test/comment context",
                    i
                );
            }
        }
    }

    /// Test edge cases for analyzer file exclusions
    #[test]
    fn test_analyzer_file_exclusions() {
        let analyzer = SecurityAnalyzer::new();

        // Edge case: Analyzer files themselves should be excluded
        let analyzer_files = [
            "src/analyzers/security_analyzer.rs",
            "src/analyzers/secret_analyzer.rs",
            "src/security/vulnerability_scanner.rs",
            "src/analyzers/mod.rs",
            "src/security_utils.rs",
        ];

        // Content that would normally trigger findings
        let content_with_patterns = br#"
// This file contains security patterns by design
let api_key_pattern = "sk-[a-zA-Z0-9]+";
let password_regex = Regex::new(r"password\s*=\s*[\"'][^\"']+[\"']").unwrap();
let secret_example = "sk-1234567890abcdef";
"#;

        for file_path in &analyzer_files {
            let path = Path::new(file_path);
            let findings = analyzer.analyze(path, content_with_patterns).unwrap();

            assert!(
                findings.is_empty(),
                "Analyzer file {} should be excluded from security analysis",
                file_path
            );
        }
    }

    /// Integration test with temporary files
    #[test]
    fn test_integration_with_temp_files() {
        let temp_dir = TempDir::new().unwrap();
        let analyzer = SecurityAnalyzer::new();

        // Create test directory structure
        let test_dirs = ["tests", "benches", "src", "examples"];
        for dir in &test_dirs {
            fs::create_dir_all(temp_dir.path().join(dir)).unwrap();
        }

        // Create files with secrets
        let test_cases = [
            (
                "tests/auth_test.rs",
                "let test_key = \"sk-1234567890abcdef\";",
                true,
            ), // Should be ignored
            (
                "benches/perf_bench.rs",
                "let bench_secret = \"dummy_secret_123\";",
                true,
            ), // Should be ignored
            (
                "src/config.rs",
                "let api_key = \"sk-proj-RealSecret123456789\";",
                false,
            ), // Should be detected
            (
                "examples/demo.rs",
                "let example_key = \"example_key_123\";",
                true,
            ), // Should be ignored
        ];

        for (file_path, content, should_be_ignored) in &test_cases {
            let full_path = temp_dir.path().join(file_path);
            fs::write(&full_path, content).unwrap();

            let findings = analyzer.analyze(&full_path, content.as_bytes()).unwrap();

            if *should_be_ignored {
                assert!(findings.is_empty(), "File {} should be ignored", file_path);
            } else {
                assert!(
                    !findings.is_empty(),
                    "File {} should detect secrets",
                    file_path
                );
            }
        }
    }
}
