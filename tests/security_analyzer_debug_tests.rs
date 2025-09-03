use do_codeguardian::analyzers::security_analyzer::SecurityAnalyzer;
use do_codeguardian::analyzers::Analyzer;
use tempfile::tempdir;

#[cfg(test)]
mod security_analyzer_debug_tests {
    use super::*;

    #[test]
    fn test_hardcoded_secret_detection() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = tempdir().unwrap();

        // Test with a simple hardcoded secret
        let test_content = r#"let api_key = "sk-1234567890abcdef";"#;
        let file_path = temp_dir.path().join("example.rs");
        std::fs::write(&file_path, test_content).unwrap();

        let findings = analyzer
            .analyze(&file_path, test_content.as_bytes())
            .unwrap();

        // Verify that at least one finding is detected
        assert!(!findings.is_empty(), "Should detect hardcoded API key");

        // Print findings for debugging (optional)
        println!("Found {} findings", findings.len());
        for finding in &findings {
            println!("  - {}: {}", finding.rule, finding.message);
        }
    }

    #[test]
    fn test_no_findings_with_safe_content() {
        let analyzer = SecurityAnalyzer::new();
        let temp_dir = tempdir().unwrap();

        // Test with safe content that should not trigger any findings
        let test_content = r#"fn main() { println!("Hello, World!"); }"#;
        let file_path = temp_dir.path().join("safe_example.rs");
        std::fs::write(&file_path, test_content).unwrap();

        let findings = analyzer
            .analyze(&file_path, test_content.as_bytes())
            .unwrap();

        // Should not detect any security issues in safe code
        assert!(
            findings.is_empty(),
            "Safe code should not trigger security findings"
        );
    }
}
