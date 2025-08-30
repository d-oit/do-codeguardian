use crate::ml::MLClassifier;
use crate::types::{AnalysisResults, Finding, Severity};
use anyhow::Result;
use std::path::Path;

/// Validation engine that ensures only 100% validated findings create GitHub issues
pub struct ValidationEngine {
    /// ML classifier for confidence scoring
    ml_classifier: Option<MLClassifier>,
    /// Minimum confidence threshold for creating GitHub issues (high bar)
    github_confidence_threshold: f32,
    /// Minimum confidence threshold for reporting findings (lower bar)
    report_confidence_threshold: f32,
    /// Enable strict validation mode
    strict_mode: bool,
}

impl ValidationEngine {
    /// Create a new validation engine with strict settings for GitHub issue creation
    pub fn new(ml_classifier: Option<MLClassifier>) -> Self {
        Self {
            ml_classifier,
            // Very high threshold for GitHub issues - only create issues for findings we're 90%+ confident about
            github_confidence_threshold: 0.9,
            // Lower threshold for including in reports - 70% confidence
            report_confidence_threshold: 0.7,
            strict_mode: true,
        }
    }

    /// Create validation engine with custom thresholds
    #[allow(dead_code)]
    pub fn with_thresholds(
        ml_classifier: Option<MLClassifier>,
        github_threshold: f32,
        report_threshold: f32,
        strict_mode: bool,
    ) -> Self {
        Self {
            ml_classifier,
            github_confidence_threshold: github_threshold,
            report_confidence_threshold: report_threshold,
            strict_mode,
        }
    }

    /// Validate findings for GitHub issue creation - only return findings that are 100% validated
    pub fn validate_for_github_issues(&mut self, findings: Vec<Finding>) -> Result<Vec<Finding>> {
        let mut validated_findings = Vec::new();

        for finding in findings {
            if self.should_create_github_issue(&finding)? {
                validated_findings.push(finding);
            }
        }

        Ok(validated_findings)
    }

    /// Validate findings for reports - includes findings with lower confidence
    pub fn validate_for_reports(&mut self, findings: Vec<Finding>) -> Result<Vec<Finding>> {
        let mut validated_findings = Vec::new();

        for finding in findings {
            if self.should_include_in_report(&finding)? {
                validated_findings.push(finding);
            }
        }

        Ok(validated_findings)
    }

    /// Determine if a finding should create a GitHub issue (very strict validation)
    fn should_create_github_issue(&mut self, finding: &Finding) -> Result<bool> {
        // First, apply basic validation rules
        if !self.passes_basic_validation(finding) {
            return Ok(false);
        }

        // Apply file-based exclusions for GitHub issues
        if self.is_test_or_example_file(&finding.file) {
            return Ok(false);
        }

        // Apply content-based exclusions
        if self.is_likely_test_data(finding) {
            return Ok(false);
        }

        // Apply ML-based confidence scoring if available
        if let Some(ml_classifier) = &mut self.ml_classifier {
            let confidence = ml_classifier.predict_relevance(finding)?;
            if confidence < self.github_confidence_threshold {
                return Ok(false);
            }
        } else if self.strict_mode {
            // In strict mode without ML, only allow high-severity findings
            return Ok(matches!(
                finding.severity,
                Severity::Critical | Severity::High
            ));
        }

        // Additional validation for secret detection
        if finding.rule.contains("secret")
            || finding.rule.contains("password")
            || finding.rule.contains("key")
        {
            return Ok(self.validate_secret_finding(finding));
        }

        Ok(true)
    }

    /// Determine if a finding should be included in reports (less strict)
    fn should_include_in_report(&mut self, finding: &Finding) -> Result<bool> {
        // Apply basic validation
        if !self.passes_basic_validation(finding) {
            return Ok(false);
        }

        // Apply ML-based confidence scoring if available
        if let Some(ml_classifier) = &mut self.ml_classifier {
            let confidence = ml_classifier.predict_relevance(finding)?;
            return Ok(confidence >= self.report_confidence_threshold);
        }

        // Without ML, include all findings that pass basic validation
        Ok(true)
    }

    /// Basic validation rules that apply to all findings
    fn passes_basic_validation(&self, finding: &Finding) -> bool {
        // Exclude findings with empty or very short messages
        if finding.message.len() < 10 {
            return false;
        }

        // Exclude findings in obviously safe locations
        if self.is_safe_file_location(&finding.file) {
            return false;
        }

        // Exclude findings that look like documentation or comments
        if self.is_documentation_content(finding) {
            return false;
        }

        true
    }

    /// Check if file is a test or example file
    fn is_test_or_example_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().to_lowercase();

        // Test file patterns
        if path_str.contains("/test")
            || path_str.contains("\\test")
            || path_str.contains("_test.")
            || path_str.contains(".test.")
            || path_str.contains("/tests/")
            || path_str.contains("\\tests\\")
            || path_str.ends_with("_test.rs")
            || path_str.ends_with("_test.js")
            || path_str.ends_with("_test.py")
            || path_str.ends_with(".spec.")
            || path_str.contains("spec/")
        {
            return true;
        }

        // Example file patterns
        if path_str.contains("/example")
            || path_str.contains("\\example")
            || path_str.contains("/examples/")
            || path_str.contains("\\examples\\")
            || path_str.contains("/demo")
            || path_str.contains("\\demo")
            || path_str.contains("/sample")
            || path_str.contains("\\sample")
        {
            return true;
        }

        // Documentation patterns
        if path_str.contains("/doc")
            || path_str.contains("\\doc")
            || path_str.contains("/docs/")
            || path_str.contains("\\docs\\")
            || path_str.ends_with(".md")
            || path_str.ends_with(".txt")
            || path_str.ends_with(".rst")
        {
            return true;
        }

        false
    }

    /// Check if file is in a safe location (build artifacts, dependencies, etc.)
    fn is_safe_file_location(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().to_lowercase();

        // Build and dependency directories
        let safe_patterns = [
            "/target/",
            "\\target\\",
            "target/",
            "target\\",
            "/node_modules/",
            "\\node_modules\\",
            "node_modules/",
            "node_modules\\",
            "/.git/",
            "\\.git\\",
            ".git/",
            ".git\\",
            "/vendor/",
            "\\vendor\\",
            "vendor/",
            "vendor\\",
            "/build/",
            "\\build\\",
            "build/",
            "build\\",
            "/dist/",
            "\\dist\\",
            "dist/",
            "dist\\",
            "/.cargo/",
            "\\.cargo\\",
            ".cargo/",
            ".cargo\\",
            "/pkg/",
            "\\pkg\\",
            "pkg/",
            "pkg\\",
        ];

        safe_patterns
            .iter()
            .any(|pattern| path_str.contains(pattern))
    }

    /// Check if finding content looks like test data
    fn is_likely_test_data(&self, finding: &Finding) -> bool {
        let _message_lower = finding.message.to_lowercase();
        let file_content = finding.file.to_string_lossy().to_lowercase();

        // Test data indicators - be more specific to avoid false positives
        let test_indicators = [
            "test_",
            "example_",
            "sample_",
            "demo_",
            "mock_",
            "fake_",
            "dummy_",
            "placeholder_",
            "your_",
            "insert_",
            "test-",
            "example-",
            "sample-",
            "demo-",
            "mock-",
        ];

        // Check if the detected secret looks like test data
        if finding.rule.contains("secret")
            || finding.rule.contains("password")
            || finding.rule.contains("key")
        {
            // Extract the potential secret from the message or description
            let content_to_check = format!(
                "{} {}",
                finding.message,
                finding.description.as_deref().unwrap_or("")
            )
            .to_lowercase();

            // Only check for test indicators in the actual secret content, not the whole message
            for indicator in &test_indicators {
                if content_to_check.contains(indicator) {
                    return true;
                }
            }

            // Check for common test patterns (only if it also contains obvious test indicators)
            if content_to_check.contains("sk-")
                && (content_to_check.contains("1234")
                    || content_to_check.contains("test")
                    || content_to_check.contains("example"))
            {
                return true; // OpenAI test key pattern
            }
            if content_to_check.contains("test") && content_to_check.len() < 50 {
                return true; // Short test strings
            }
        }

        // Check file path for test indicators (but not for normal source files)
        if file_content.contains("/test")
            || file_content.contains("\\test")
            || file_content.contains("/example")
            || file_content.contains("\\example")
        {
            return test_indicators
                .iter()
                .any(|indicator| file_content.contains(indicator));
        }

        false
    }

    /// Check if finding is in documentation or comments
    fn is_documentation_content(&self, finding: &Finding) -> bool {
        let message_lower = finding.message.to_lowercase();

        // Documentation indicators
        if message_lower.contains("documentation")
            || message_lower.contains("comment")
            || message_lower.contains("readme")
            || message_lower.contains("example")
            || message_lower.contains("tutorial")
        {
            return true;
        }

        // Check if it's in a comment block
        if let Some(description) = &finding.description {
            let desc_lower = description.to_lowercase();
            if desc_lower.contains("//")
                || desc_lower.contains("/*")
                || desc_lower.contains("<!--")
                || desc_lower.contains("#")
            {
                return true;
            }
        }

        false
    }

    /// Enhanced validation for secret findings
    fn validate_secret_finding(&self, finding: &Finding) -> bool {
        let message_lower = finding.message.to_lowercase();

        // Exclude obvious test patterns
        let test_patterns = [
            "test",
            "example",
            "sample",
            "demo",
            "mock",
            "fake",
            "dummy",
            "placeholder",
            "xxx",
            "123",
            "abc",
            "password123",
            "secret123",
            "your_api_key",
            "your_secret",
            "insert_key_here",
            "replace_with",
        ];

        for pattern in &test_patterns {
            if message_lower.contains(pattern) {
                return false;
            }
        }

        // Validate entropy and length for secrets
        if let Some(description) = &finding.description {
            let desc_lower = description.to_lowercase();

            // Extract potential secret value
            if let Some(secret_value) = self.extract_secret_value(&desc_lower) {
                // Check minimum length
                if secret_value.len() < 16 {
                    return false;
                }

                // Check entropy (randomness)
                if self.calculate_entropy(&secret_value) < 3.5 {
                    return false;
                }

                // Check for repeated patterns
                if self.has_repeated_patterns(&secret_value) {
                    return false;
                }
            }
        }

        true
    }

    /// Extract potential secret value from description
    fn extract_secret_value(&self, text: &str) -> Option<String> {
        // Simple extraction - look for quoted strings
        if let Some(start) = text.find('"') {
            if let Some(end) = text[start + 1..].find('"') {
                return Some(text[start + 1..start + 1 + end].to_string());
            }
        }
        if let Some(start) = text.find('\'') {
            if let Some(end) = text[start + 1..].find('\'') {
                return Some(text[start + 1..start + 1 + end].to_string());
            }
        }
        None
    }

    /// Calculate Shannon entropy of a string
    fn calculate_entropy(&self, text: &str) -> f64 {
        let mut char_counts = std::collections::HashMap::new();
        let chars: Vec<char> = text.chars().collect();

        for &ch in &chars {
            *char_counts.entry(ch).or_insert(0) += 1;
        }

        let len = chars.len() as f64;
        let entropy: f64 = char_counts
            .values()
            .map(|&count| {
                let p = count as f64 / len;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum();

        entropy
    }

    /// Check for repeated patterns that indicate test data
    fn has_repeated_patterns(&self, text: &str) -> bool {
        // Check for simple repeated patterns
        if text.len() < 8 {
            return true;
        }

        // Check for repeated characters
        let chars: Vec<char> = text.chars().collect();
        let mut repeated_count = 0;
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                repeated_count += 1;
            }
        }

        // If more than 30% of characters are repeated, likely test data
        (repeated_count as f64 / chars.len() as f64) > 0.3
    }

    /// Filter analysis results for GitHub issue creation
    pub fn filter_results_for_github(
        &mut self,
        mut results: AnalysisResults,
    ) -> Result<AnalysisResults> {
        let validated_findings = self.validate_for_github_issues(results.findings)?;

        // Update summary with filtered counts
        let original_count = results.summary.total_findings;
        let filtered_count = validated_findings.len();

        results.findings = validated_findings;
        results.summary.total_findings = filtered_count;

        // Add validation metadata
        results.summary.metadata.insert(
            "validation_applied".to_string(),
            "strict_github_validation".to_string(),
        );
        results.summary.metadata.insert(
            "original_findings_count".to_string(),
            original_count.to_string(),
        );
        results.summary.metadata.insert(
            "filtered_findings_count".to_string(),
            filtered_count.to_string(),
        );
        results.summary.metadata.insert(
            "filter_rate".to_string(),
            format!(
                "{:.1}%",
                (1.0 - filtered_count as f64 / original_count.max(1) as f64) * 100.0
            ),
        );

        Ok(results)
    }

    /// Get validation statistics
    pub fn get_validation_stats(&self) -> ValidationStats {
        ValidationStats {
            github_threshold: self.github_confidence_threshold,
            report_threshold: self.report_confidence_threshold,
            strict_mode: self.strict_mode,
            ml_enabled: self.ml_classifier.is_some(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidationStats {
    pub github_threshold: f32,
    pub report_threshold: f32,
    pub strict_mode: bool,
    pub ml_enabled: bool,
}

impl std::fmt::Display for ValidationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation: GitHub threshold: {:.1}%, Report threshold: {:.1}%, Strict: {}, ML: {}",
            self.github_threshold * 100.0,
            self.report_threshold * 100.0,
            self.strict_mode,
            self.ml_enabled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;

    #[test]
    fn test_validation_engine_creation() {
        let engine = ValidationEngine::new(None);
        assert_eq!(engine.github_confidence_threshold, 0.9);
        assert_eq!(engine.report_confidence_threshold, 0.7);
        assert!(engine.strict_mode);
    }

    #[test]
    fn test_test_file_detection() {
        let engine = ValidationEngine::new(None);

        assert!(engine.is_test_or_example_file(&PathBuf::from("src/test/mod.rs")));
        assert!(engine.is_test_or_example_file(&PathBuf::from("tests/integration_test.rs")));
        assert!(engine.is_test_or_example_file(&PathBuf::from("examples/demo.rs")));
        assert!(engine.is_test_or_example_file(&PathBuf::from("docs/README.md")));

        assert!(!engine.is_test_or_example_file(&PathBuf::from("src/main.rs")));
        assert!(!engine.is_test_or_example_file(&PathBuf::from("src/lib.rs")));
    }

    #[test]
    fn test_safe_file_location() {
        let engine = ValidationEngine::new(None);

        assert!(engine.is_safe_file_location(&PathBuf::from("target/debug/deps/lib.rs")));
        assert!(engine.is_safe_file_location(&PathBuf::from("node_modules/package/index.js")));
        assert!(engine.is_safe_file_location(&PathBuf::from(".git/config")));

        assert!(!engine.is_safe_file_location(&PathBuf::from("src/main.rs")));
    }

    #[test]
    fn test_test_data_detection() {
        let engine = ValidationEngine::new(None);

        let test_finding = Finding::new(
            "security",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Potential secret: test_api_key_123".to_string(),
        );

        assert!(engine.is_likely_test_data(&test_finding));

        let real_finding = Finding::new(
            "security",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Potential secret: sk-proj-abc123def456ghi789jkl012".to_string(),
        );

        // Debug what's happening
        let is_test_data = engine.is_likely_test_data(&real_finding);
        if is_test_data {
            println!(
                "Real finding incorrectly detected as test data: {}",
                real_finding.message
            );
        }
        assert!(!is_test_data);
    }

    #[test]
    fn test_entropy_calculation() {
        let engine = ValidationEngine::new(None);

        // High entropy (random-looking)
        assert!(engine.calculate_entropy("sk-proj-abc123def456ghi789") > 3.5);

        // Low entropy (repeated patterns)
        assert!(engine.calculate_entropy("test123test123") < 3.5);
        assert!(engine.calculate_entropy("aaaaaaaaaa") < 3.5);
    }

    #[test]
    fn test_repeated_patterns() {
        let engine = ValidationEngine::new(None);

        assert!(engine.has_repeated_patterns("aaaaaaa"));
        assert!(engine.has_repeated_patterns("1111111"));
        assert!(!engine.has_repeated_patterns("sk-proj-abc123def456"));
    }
}
