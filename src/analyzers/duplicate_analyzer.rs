use crate::analyzers::Analyzer;
use crate::cache::optimized_cache::OptimizedCache;
use crate::config::analysis::DuplicateAnalyzerConfig;
#[cfg(feature = "ml")]
#[allow(unused_imports)]
use crate::github_api::GitHubApiClient;
#[cfg(feature = "ml")]
#[allow(unused_imports)]
use crate::ml::fann_classifier::FannClassifier;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
#[cfg(feature = "ml")]
#[allow(unused_imports)]
use std::sync::Arc;
#[cfg(feature = "ml")]
#[allow(unused_imports)]
use tokio::sync::Mutex;

/// Enhanced duplicate code analyzer with ML and GitHub integration
pub struct DuplicateAnalyzer {
    config: DuplicateAnalyzerConfig,
    security_function_patterns: Vec<Regex>,
    #[cfg(feature = "ml")]
    github_client: Option<Arc<Mutex<GitHubApiClient>>>,
    #[allow(dead_code)]
    cache: OptimizedCache,
    #[allow(dead_code)]
    file_cache: HashMap<String, Vec<String>>,
}

/// Similarity score for duplicate detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore {
    pub score: f64,
    pub confidence: f64,
    pub method: String,
}

/// Duplicate detection result
#[derive(Debug, Clone)]
pub struct DuplicateResult {
    pub file1: String,
    pub file2: String,
    pub similarity: SimilarityScore,
    pub security_relevant: bool,
    pub shared_lines: Vec<String>,
}

impl Default for DuplicateAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DuplicateAnalyzer {
    pub fn new() -> Self {
        Self::with_config(DuplicateAnalyzerConfig::default())
    }

    pub fn with_config(config: DuplicateAnalyzerConfig) -> Self {
        let security_patterns = vec![
            // Authentication patterns
            Regex::new(r"(?i)(authenticate|login|signin|verify|validate)").unwrap(),
            // Authorization patterns
            Regex::new(r"(?i)(authorize|permission|access|role|privilege)").unwrap(),
            // Cryptographic patterns
            Regex::new(r"(?i)(encrypt|decrypt|hash|crypto|cipher|key|token)").unwrap(),
            // Input validation patterns
            Regex::new(r"(?i)(validate|sanitize|escape|filter|clean)").unwrap(),
            // Error handling patterns
            Regex::new(r"(?i)(error|exception|panic|fail|abort)").unwrap(),
            // Security-sensitive operations
            Regex::new(r"(?i)(password|secret|credential|session|cookie)").unwrap(),
        ];

        Self {
            config,
            security_function_patterns: security_patterns,
            #[cfg(feature = "ml")]
            github_client: None,
            cache: OptimizedCache::new(100, 50), // 100 entries, 50MB cache
            file_cache: HashMap::new(),
        }
    }

    /// Set GitHub API client for duplicate issue prevention
    #[cfg(feature = "ml")]
    pub fn with_github_client(mut self, client: Arc<Mutex<GitHubApiClient>>) -> Self {
        self.github_client = Some(client);
        self
    }

    pub fn with_min_lines(mut self, min_lines: usize) -> Self {
        self.config.min_lines = min_lines;
        self
    }

    pub fn with_security_focus(mut self, focus: bool) -> Self {
        self.config.focus_security = focus;
        self
    }

    pub fn with_test_files(mut self, include_tests: bool) -> Self {
        self.config.ignore_test_files = !include_tests;
        self
    }

    pub fn with_max_files(mut self, max_files: usize) -> Self {
        self.config.max_files_to_compare = max_files;
        self
    }

    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.config.similarity_threshold = threshold;
        self
    }

    pub fn with_ml_enabled(mut self, enabled: bool) -> Self {
        self.config.enable_ml_similarity = enabled;
        self
    }

    /// Check for duplicate GitHub issues before creating new ones
    #[cfg(feature = "ml")]
    pub async fn check_github_duplicates(
        &mut self,
        title: &str,
        repo: &str,
    ) -> Result<Option<u64>> {
        if let Some(client) = &self.github_client {
            let mut client = client.lock().await;
            client.find_existing_issue(title, repo).await
        } else {
            Ok(None)
        }
    }

    /// Use ML classifier to enhance similarity detection
    fn calculate_ml_similarity(&self, _block1: &CodeBlock, _block2: &CodeBlock) -> Option<f64> {
        #[cfg(feature = "ml")]
        {
            if !self.config.enable_ml_similarity {
                return None;
            }

            // Load classifier on demand
            if let Some(model_path) = &self.config.ml_model_path {
                if let Ok(classifier) = FannClassifier::load(model_path) {
                    // Extract features for ML classification
                    if let Some(features) = self.extract_similarity_features(block1, block2) {
                        match classifier.predict(&features) {
                            Ok(score) => return Some(score as f64),
                            Err(_) => return None,
                        }
                    }
                }
            }
            None
        }
        #[cfg(not(feature = "ml"))]
        {
            None
        }
    }

    /// Extract features for ML-based similarity detection
    #[allow(dead_code)]
    fn extract_similarity_features(
        &self,
        block1: &CodeBlock,
        block2: &CodeBlock,
    ) -> Option<Vec<f32>> {
        if block1.lines.is_empty() || block2.lines.is_empty() {
            return None;
        }

        let mut features = Vec::new();

        // Basic similarity features
        let basic_similarity = self.calculate_similarity(block1, block2) as f32;
        features.push(basic_similarity);

        // Length ratio
        let len_ratio = (block1.lines.len() as f32).min(block2.lines.len() as f32)
            / (block1.lines.len() as f32).max(block2.lines.len() as f32);
        features.push(len_ratio);

        // Security relevance score
        let security_score =
            if self.is_security_relevant(block1) && self.is_security_relevant(block2) {
                1.0
            } else if self.is_security_relevant(block1) || self.is_security_relevant(block2) {
                0.5
            } else {
                0.0
            };
        features.push(security_score);

        // Function density (ratio of function-like lines)
        let func_density1 = self.calculate_function_density(block1) as f32;
        let func_density2 = self.calculate_function_density(block2) as f32;
        features.push(func_density1);
        features.push(func_density2);

        Some(features)
    }

    /// Calculate function density in a code block
    #[allow(dead_code)]
    fn calculate_function_density(&self, block: &CodeBlock) -> f64 {
        let total_lines = block.lines.len();
        if total_lines == 0 {
            return 0.0;
        }

        let function_lines = block
            .lines
            .iter()
            .filter(|line| {
                let line_lower = line.to_lowercase();
                line_lower.contains("fn ")
                    || line_lower.contains("function")
                    || line_lower.contains("def ")
                    || line_lower.contains("class ")
                    || line_lower.contains("struct ")
            })
            .count();

        function_lines as f64 / total_lines as f64
    }

    /// Normalize a line for comparison (remove whitespace, comments)
    fn normalize_line(&self, line: &str) -> String {
        let mut normalized = line.trim().to_string();

        // Remove single-line comments
        if let Some(pos) = normalized.find("//") {
            normalized = normalized[..pos].trim().to_string();
        }
        if let Some(pos) = normalized.find('#') {
            // Be careful with # in strings
            if !self.is_in_string(&normalized, pos) {
                normalized = normalized[..pos].trim().to_string();
            }
        }

        // Remove extra whitespace
        normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");

        normalized
    }

    /// Check if a position is inside a string literal
    fn is_in_string(&self, line: &str, pos: usize) -> bool {
        let before = &line[..pos];
        let single_quotes = before.matches('\'').count();
        let double_quotes = before.matches('"').count();

        // Simple heuristic: if we have an odd number of quotes before this position,
        // we're likely inside a string
        (single_quotes % 2 == 1) || (double_quotes % 2 == 1)
    }

    /// Extract meaningful code blocks from content
    fn extract_code_blocks(&self, content: &str) -> Vec<CodeBlock> {
        let lines: Vec<String> = content
            .lines()
            .map(|line| self.normalize_line(line))
            .collect();

        let mut blocks = Vec::new();
        let mut current_block = Vec::new();
        let mut start_line = 0;

        for (line_num, line) in lines.iter().enumerate() {
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                if current_block.len() >= self.config.min_lines {
                    blocks.push(CodeBlock {
                        lines: current_block.clone(),
                        start_line: start_line + 1, // 1-indexed
                        end_line: line_num,
                    });
                }
                current_block.clear();
                start_line = line_num + 1;
            } else {
                current_block.push(line.clone());
            }
        }

        // Don't forget the last block
        if current_block.len() >= self.config.min_lines {
            blocks.push(CodeBlock {
                lines: current_block,
                start_line: start_line + 1,
                end_line: lines.len(),
            });
        }

        blocks
    }

    /// Check if a code block contains security-relevant code
    fn is_security_relevant(&self, block: &CodeBlock) -> bool {
        if !self.config.focus_security {
            return true; // If not focusing on security, all blocks are relevant
        }

        let block_text = block.lines.join(" ").to_lowercase();

        for pattern in &self.security_function_patterns {
            if pattern.is_match(&block_text) {
                return true;
            }
        }

        false
    }

    /// Calculate similarity between two code blocks
    fn calculate_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        if block1.lines.is_empty() || block2.lines.is_empty() {
            return 0.0;
        }

        let mut matching_lines = 0;
        let max_lines = block1.lines.len().max(block2.lines.len());
        let min_lines = block1.lines.len().min(block2.lines.len());

        // Compare line by line
        for i in 0..min_lines {
            if block1.lines[i] == block2.lines[i] {
                matching_lines += 1;
            }
        }

        // Calculate similarity as percentage of matching lines
        matching_lines as f64 / max_lines as f64
    }

    /// Find duplicates within a single file
    fn find_internal_duplicates(&self, file_path: &Path, content: &str) -> Vec<Finding> {
        let mut findings = Vec::new();
        let blocks = self.extract_code_blocks(content);

        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                let basic_similarity = self.calculate_similarity(&blocks[i], &blocks[j]);

                // Use ML-enhanced similarity if available
                let similarity =
                    if let Some(ml_score) = self.calculate_ml_similarity(&blocks[i], &blocks[j]) {
                        // Combine basic and ML similarity (weighted average)
                        (basic_similarity * 0.7) + (ml_score * 0.3)
                    } else {
                        basic_similarity
                    };

                if similarity >= self.config.similarity_threshold
                    && (self.is_security_relevant(&blocks[i]) || !self.config.focus_security)
                {
                    let severity = self.calculate_severity(similarity, &blocks[i], &blocks[j]);

                    findings.push(
                        Finding::new(
                            "duplicate",
                            "internal_duplication",
                            severity,
                            file_path.to_path_buf(),
                            blocks[i].start_line as u32,
                            format!("Duplicate code block detected ({}% similar)", (similarity * 100.0) as u32),
                        )
                        .with_description(format!(
                            "Code block at lines {}-{} is {:.1}% similar to block at lines {}-{}",
                            blocks[i].start_line, blocks[i].end_line,
                            similarity * 100.0,
                            blocks[j].start_line, blocks[j].end_line
                        ))
                        .with_suggestion("Consider extracting common code into a shared function to reduce duplication and maintenance burden".to_string()),
                    );
                }
            }
        }

        findings
    }

    /// Find enhanced duplicates using ML and advanced techniques
    fn find_enhanced_duplicates(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let blocks = self.extract_code_blocks(content);

        // Look for patterns that might indicate security issues in duplicates
        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                if let Some(ml_score) = self.calculate_ml_similarity(&blocks[i], &blocks[j]) {
                    // Check for security patterns in duplicated code
                    let security_patterns =
                        self.detect_security_patterns_in_duplicate(&blocks[i], &blocks[j]);

                    if !security_patterns.is_empty() && ml_score > 0.7 {
                        let severity = if ml_score > 0.9 {
                            Severity::High
                        } else if ml_score > 0.8 {
                            Severity::Medium
                        } else {
                            Severity::Low
                        };

                        findings.push(
                            Finding::new(
                                "duplicate",
                                "enhanced_security_duplication",
                                severity,
                                file_path.to_path_buf(),
                                blocks[i].start_line as u32,
                                format!("Security-sensitive duplicate code detected (ML confidence: {:.1}%)", ml_score * 100.0),
                            )
                            .with_description(format!(
                                "ML-enhanced analysis detected security-relevant code duplication between lines {}-{} and {}-{} with patterns: {}",
                                blocks[i].start_line, blocks[i].end_line,
                                blocks[j].start_line, blocks[j].end_line,
                                security_patterns.join(", ")
                            ))
                            .with_suggestion("Security-sensitive code should not be duplicated. Extract to a shared, well-tested function.".to_string()),
                        );
                    }
                }
            }
        }

        Ok(findings)
    }

    /// Detect security patterns in duplicated code blocks
    fn detect_security_patterns_in_duplicate(
        &self,
        block1: &CodeBlock,
        block2: &CodeBlock,
    ) -> Vec<String> {
        let mut patterns = Vec::new();
        let combined_text = format!("{} {}", block1.lines.join(" "), block2.lines.join(" "));

        for pattern in &self.security_function_patterns {
            if pattern.is_match(&combined_text) {
                // Extract the matched pattern name for reporting
                let pattern_str = pattern.as_str();
                if let Some(name_start) = pattern_str.find('(') {
                    if let Some(name_end) = pattern_str[name_start..].find(')') {
                        let pattern_name = &pattern_str[name_start + 1..name_start + name_end];
                        patterns.push(pattern_name.to_string());
                    }
                }
            }
        }

        patterns
    }

    /// Calculate severity based on similarity and security relevance
    fn calculate_severity(
        &self,
        similarity: f64,
        block1: &CodeBlock,
        block2: &CodeBlock,
    ) -> Severity {
        let base_severity = if similarity >= 0.95 {
            Severity::High
        } else if similarity >= 0.9 {
            Severity::Medium
        } else {
            Severity::Low
        };

        // Increase severity if both blocks contain security-relevant code
        if self.is_security_relevant(block1) && self.is_security_relevant(block2) {
            match base_severity {
                Severity::Low => Severity::Medium,
                Severity::Medium => Severity::High,
                Severity::High => Severity::High,
                _ => base_severity,
            }
        } else {
            base_severity
        }
    }

    /// Check if file should be ignored
    fn should_ignore_file(&self, file_path: &Path) -> bool {
        if self.config.ignore_test_files && self.is_test_file(file_path) {
            return true;
        }

        // Ignore generated files
        let path_str = file_path.to_string_lossy().to_lowercase();
        if path_str.contains("generated")
            || path_str.contains("target/")
            || path_str.contains("build/")
            || path_str.contains("dist/")
            || path_str.contains("node_modules/")
        {
            return true;
        }

        false
    }

    /// Check if file is a test file
    fn is_test_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy().to_lowercase();

        path_str.contains("/test")
            || path_str.contains("\\test")
            || path_str.contains("/tests")
            || path_str.contains("\\tests")
            || path_str.ends_with("_test.rs")
            || path_str.ends_with(".test.js")
            || path_str.ends_with("_test.py")
            || path_str.ends_with("test.go")
    }

    #[allow(dead_code)]
    /// Get security risk level for duplicate code
    fn get_security_risk_level(&self, block: &CodeBlock) -> Severity {
        let block_text = block.lines.join(" ").to_lowercase();

        // High risk patterns
        if block_text.contains("password")
            || block_text.contains("secret")
            || block_text.contains("encrypt")
            || block_text.contains("decrypt")
            || block_text.contains("authenticate")
        {
            return Severity::High;
        }

        // Medium risk patterns
        if block_text.contains("validate")
            || block_text.contains("authorize")
            || block_text.contains("permission")
            || block_text.contains("session")
        {
            return Severity::Medium;
        }

        Severity::Low
    }
}

#[derive(Debug, Clone)]
struct CodeBlock {
    lines: Vec<String>,
    start_line: usize,
    end_line: usize,
}

impl Analyzer for DuplicateAnalyzer {
    fn name(&self) -> &str {
        "duplicate"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        if self.should_ignore_file(file_path) {
            return Ok(Vec::new());
        }

        let content_str = String::from_utf8_lossy(content);

        // Analyze internal duplicates within the same file
        let mut findings = self.find_internal_duplicates(file_path, &content_str);

        // Add enhanced analysis with ML if available
        if self.config.enable_ml_similarity {
            let enhanced_findings = self.find_enhanced_duplicates(file_path, &content_str)?;
            findings.extend(enhanced_findings);
        }

        Ok(findings)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext.to_lowercase().as_str(),
                "rs" | "js"
                    | "ts"
                    | "py"
                    | "java"
                    | "cpp"
                    | "c"
                    | "h"
                    | "hpp"
                    | "go"
                    | "php"
                    | "rb"
                    | "cs"
                    | "swift"
                    | "kt"
                    | "scala"
                    | "dart"
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::analysis::DuplicateAnalyzerConfig;

    #[test]
    fn test_detect_internal_duplicates() {
        let config = DuplicateAnalyzerConfig {
            enabled: true,
            min_lines: 3,
            focus_security: true,
            ignore_test_files: true,
            max_files_to_compare: 1000,
            enable_ml_similarity: false,
            ml_model_path: None,
            similarity_threshold: 0.8,
            enable_github_prevention: false,
            cache: Default::default(),
        };
        let analyzer = DuplicateAnalyzer::with_config(config);
        let content = r#"
fn authenticate_user(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}

fn authenticate_admin(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}
"#;
        let findings = analyzer
            .analyze(Path::new("auth.rs"), content.as_bytes())
            .unwrap();
        assert!(findings.iter().any(|f| f.rule == "internal_duplication"));
    }

    #[test]
    fn test_normalize_line() {
        let analyzer = DuplicateAnalyzer::new();
        assert_eq!(
            analyzer.normalize_line("  let x = 5;  // comment"),
            "let x = 5;"
        );
        assert_eq!(
            analyzer.normalize_line("    if condition {"),
            "if condition {"
        );
    }

    #[test]
    fn test_security_relevance() {
        let analyzer = DuplicateAnalyzer::new();
        let security_block = CodeBlock {
            lines: vec!["authenticate_user".to_string(), "hash_password".to_string()],
            start_line: 1,
            end_line: 2,
        };
        let normal_block = CodeBlock {
            lines: vec!["println!".to_string(), "format!".to_string()],
            start_line: 1,
            end_line: 2,
        };

        assert!(analyzer.is_security_relevant(&security_block));
        assert!(!analyzer.is_security_relevant(&normal_block));
    }

    #[test]
    fn test_ignore_test_files() {
        let analyzer = DuplicateAnalyzer::new();
        assert!(analyzer.should_ignore_file(Path::new("tests/test_auth.rs")));
        assert!(analyzer.should_ignore_file(Path::new("src/auth_test.rs")));
        assert!(!analyzer.should_ignore_file(Path::new("src/auth.rs")));
    }

    #[test]
    fn test_calculate_similarity() {
        let analyzer = DuplicateAnalyzer::new();
        let block1 = CodeBlock {
            lines: vec![
                "line1".to_string(),
                "line2".to_string(),
                "line3".to_string(),
            ],
            start_line: 1,
            end_line: 3,
        };
        let block2 = CodeBlock {
            lines: vec![
                "line1".to_string(),
                "line2".to_string(),
                "line3".to_string(),
            ],
            start_line: 5,
            end_line: 7,
        };
        let block3 = CodeBlock {
            lines: vec![
                "line1".to_string(),
                "different".to_string(),
                "line3".to_string(),
            ],
            start_line: 9,
            end_line: 11,
        };

        assert_eq!(analyzer.calculate_similarity(&block1, &block2), 1.0);
        assert!((analyzer.calculate_similarity(&block1, &block3) - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_supports_file() {
        let analyzer = DuplicateAnalyzer::new();
        assert!(analyzer.supports_file(Path::new("test.rs")));
        assert!(analyzer.supports_file(Path::new("test.js")));
        assert!(analyzer.supports_file(Path::new("test.py")));
        assert!(!analyzer.supports_file(Path::new("test.txt")));
        assert!(!analyzer.supports_file(Path::new("test.png")));
    }

    #[test]
    fn test_ml_similarity_calculation() {
        let analyzer = DuplicateAnalyzer::new();
        let block1 = CodeBlock {
            lines: vec![
                "fn authenticate()".to_string(),
                "hash_password()".to_string(),
            ],
            start_line: 1,
            end_line: 2,
        };
        let block2 = CodeBlock {
            lines: vec![
                "fn authenticate()".to_string(),
                "hash_password()".to_string(),
            ],
            start_line: 5,
            end_line: 6,
        };

        // Without ML classifier, should return None
        assert!(analyzer.calculate_ml_similarity(&block1, &block2).is_none());
    }

    #[test]
    fn test_security_patterns_detection() {
        let analyzer = DuplicateAnalyzer::new();
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
        assert!(
            patterns.contains(&"authenticate".to_string())
                || patterns.contains(&"validate".to_string())
        );
    }
}
