use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Cross-file duplicate detection analyzer for identifying duplicate code patterns
/// across multiple files in the codebase
pub struct CrossFileDuplicateAnalyzer {
    min_lines: usize,
    similarity_threshold: f64,
    max_files_to_compare: usize,
    ignore_test_files: bool,
    focus_security: bool,
    security_patterns: Vec<Regex>,
    file_cache: HashMap<String, FileAnalysis>,
}

/// Analysis result for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub code_blocks: Vec<CodeBlock>,
    pub security_relevant: bool,
    pub file_hash: String,
}

/// Represents a code block that can be compared across files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub lines: Vec<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub normalized_content: String,
    pub security_score: f64,
    pub function_signature: Option<String>,
    pub complexity_score: f64,
}

/// Result of cross-file duplicate detection
#[derive(Debug, Clone)]
pub struct CrossFileDuplicate {
    pub file1: String,
    pub file2: String,
    pub block1: CodeBlock,
    pub block2: CodeBlock,
    pub similarity_score: f64,
    pub security_impact: Severity,
    pub refactoring_suggestion: String,
}

impl Default for CrossFileDuplicateAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossFileDuplicateAnalyzer {
    pub fn new() -> Self {
        let security_patterns = vec![
            // Authentication and authorization patterns
            Regex::new(r"(?i)(authenticate|login|signin|verify|validate|authorize|permission)")
                .unwrap(),
            // Cryptographic operations
            Regex::new(r"(?i)(encrypt|decrypt|hash|crypto|cipher|key|token|secret)").unwrap(),
            // Input validation and sanitization
            Regex::new(r"(?i)(validate|sanitize|escape|filter|clean|normalize)").unwrap(),
            // Error handling and security checks
            Regex::new(r"(?i)(error|exception|panic|fail|abort|security|vulnerability)").unwrap(),
            // Sensitive data handling
            Regex::new(r"(?i)(password|credential|session|cookie|jwt|oauth)").unwrap(),
        ];

        Self {
            min_lines: 10,
            similarity_threshold: 0.8,
            max_files_to_compare: 1000,
            ignore_test_files: true,
            focus_security: true,
            security_patterns,
            file_cache: HashMap::new(),
        }
    }

    pub fn with_min_lines(mut self, min_lines: usize) -> Self {
        self.min_lines = min_lines;
        self
    }

    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold;
        self
    }

    pub fn with_max_files(mut self, max_files: usize) -> Self {
        self.max_files_to_compare = max_files;
        self
    }

    pub fn with_security_focus(mut self, focus: bool) -> Self {
        self.focus_security = focus;
        self
    }

    pub fn with_test_files(mut self, include_tests: bool) -> Self {
        self.ignore_test_files = !include_tests;
        self
    }

    /// Analyze a file and cache its code blocks for cross-file comparison
    pub fn analyze_file(&mut self, file_path: &Path, content: &str) -> Result<FileAnalysis> {
        if self.should_ignore_file(file_path) {
            return Ok(FileAnalysis {
                path: file_path.to_string_lossy().to_string(),
                code_blocks: Vec::new(),
                security_relevant: false,
                file_hash: self.calculate_file_hash(content),
            });
        }

        let code_blocks = self.extract_code_blocks(content);
        let security_relevant = self.is_file_security_relevant(&code_blocks);
        let file_hash = self.calculate_file_hash(content);

        let analysis = FileAnalysis {
            path: file_path.to_string_lossy().to_string(),
            code_blocks,
            security_relevant,
            file_hash,
        };

        // Cache the analysis for cross-file comparison
        self.file_cache
            .insert(file_path.to_string_lossy().to_string(), analysis.clone());

        Ok(analysis)
    }

    /// Find duplicates across all cached files
    pub fn find_cross_file_duplicates(&self) -> Result<Vec<CrossFileDuplicate>> {
        let mut duplicates = Vec::new();
        let files: Vec<_> = self.file_cache.values().collect();

        for i in 0..files.len() {
            for j in (i + 1)..files.len() {
                let file1 = &files[i];
                let file2 = &files[j];

                // Skip if both files are not security relevant and we're focusing on security
                if self.focus_security && !file1.security_relevant && !file2.security_relevant {
                    continue;
                }

                let file_duplicates = self.compare_files(file1, file2)?;
                duplicates.extend(file_duplicates);
            }
        }

        // Sort by security impact and similarity score
        duplicates.sort_by(|a, b| {
            b.security_impact
                .partial_cmp(&a.security_impact)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    b.similarity_score
                        .partial_cmp(&a.similarity_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        Ok(duplicates)
    }

    /// Compare two files for duplicate code blocks
    fn compare_files(
        &self,
        file1: &FileAnalysis,
        file2: &FileAnalysis,
    ) -> Result<Vec<CrossFileDuplicate>> {
        let mut duplicates = Vec::new();

        for block1 in &file1.code_blocks {
            // Early filtering optimization: skip obviously different blocks
            if block1.lines.is_empty() || block2.lines.is_empty() {
                continue;
            }

            // Length-based filtering to reduce O(n²) complexity
            let len_ratio = block1.lines.len() as f64 / block2.lines.len() as f64;
            if len_ratio < 0.5 || len_ratio > 2.0 {
                continue;
            }

            // Skip very short blocks that are unlikely to be meaningful duplicates
            if block1.lines.len() < 3 && block2.lines.len() < 3 {
                continue;
            }
            for block2 in &file2.code_blocks {
                let similarity = self.calculate_block_similarity(block1, block2);

                if similarity >= self.similarity_threshold {
                    let security_impact =
                        self.calculate_security_impact(block1, block2, similarity);
                    let refactoring_suggestion =
                        self.generate_refactoring_suggestion(block1, block2);

                    duplicates.push(CrossFileDuplicate {
                        file1: file1.path.clone(),
                        file2: file2.path.clone(),
                        block1: block1.clone(),
                        block2: block2.clone(),
                        similarity_score: similarity,
                        security_impact,
                        refactoring_suggestion,
                    });
                }
            }
        }

        Ok(duplicates)
    }

    /// Extract meaningful code blocks from file content
    fn extract_code_blocks(&self, content: &str) -> Vec<CodeBlock> {
        let lines: Vec<String> = content
            .lines()
            .map(|line| self.normalize_line(line))
            .filter(|line| !line.is_empty())
            .collect();

        let mut blocks = Vec::new();
        let mut current_block = Vec::new();
        let mut start_line = 0;
        let mut brace_depth = 0;
        let mut in_function = false;

        for (line_num, line) in lines.iter().enumerate() {
            // Skip comment-only lines
            if line.trim_start().starts_with("//") || line.trim_start().starts_with('#') {
                continue;
            }

            // Detect function/method boundaries
            if self.is_function_start(line) {
                // Finish current block if it meets minimum requirements
                if !current_block.is_empty() && current_block.len() >= self.min_lines {
                    let block = self.create_code_block(current_block.clone(), start_line, line_num);
                    blocks.push(block);
                }
                current_block.clear();
                start_line = line_num;
                in_function = true;
                brace_depth = 0;
            }

            // Track brace depth for function boundaries
            brace_depth += line.matches('{').count() as i32;
            brace_depth -= line.matches('}').count() as i32;

            current_block.push(line.clone());

            // End of function when brace depth returns to 0
            if in_function && brace_depth == 0 && line.contains('}') {
                if current_block.len() >= self.min_lines {
                    let block =
                        self.create_code_block(current_block.clone(), start_line, line_num + 1);
                    blocks.push(block);
                }
                current_block.clear();
                in_function = false;
            }
        }

        // Handle any remaining block
        if !current_block.is_empty() && current_block.len() >= self.min_lines {
            let block = self.create_code_block(current_block, start_line, lines.len());
            blocks.push(block);
        }

        blocks
    }

    /// Create a code block with analysis metadata
    fn create_code_block(
        &self,
        lines: Vec<String>,
        start_line: usize,
        end_line: usize,
    ) -> CodeBlock {
        let normalized_content = lines.join("\n");
        let security_score = self.calculate_security_score(&lines);
        let function_signature = self.extract_function_signature(&lines);
        let complexity_score = self.calculate_complexity_score(&lines);

        CodeBlock {
            lines,
            start_line: start_line + 1,
            end_line,
            normalized_content,
            security_score,
            function_signature,
            complexity_score,
        }
    }

    /// Check if a line indicates the start of a function
    fn is_function_start(&self, line: &str) -> bool {
        let line_lower = line.to_lowercase();
        line_lower.contains("fn ")
            || line_lower.contains("function ")
            || line_lower.contains("def ")
            || line_lower.contains("class ")
            || line_lower.contains("struct ")
            || line_lower.contains("impl ")
    }

    /// Calculate similarity between two code blocks using multiple metrics
    fn calculate_block_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        // Structural similarity (line-by-line comparison)
        let structural = self.calculate_structural_similarity(block1, block2);

        // Semantic similarity (function signatures, patterns)
        let semantic = self.calculate_semantic_similarity(block1, block2);

        // Token-based similarity
        let token_based = self.calculate_token_similarity(block1, block2);

        // Weighted combination
        (structural * 0.4) + (semantic * 0.3) + (token_based * 0.3)
    }

    /// Calculate structural similarity between code blocks
    fn calculate_structural_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        if block1.lines.is_empty() || block2.lines.is_empty() {
            return 0.0;
        }

        let mut matching_lines = 0;
        let max_lines = block1.lines.len().max(block2.lines.len());
        let min_lines = block1.lines.len().min(block2.lines.len());

        for i in 0..min_lines {
            if block1.lines[i] == block2.lines[i] {
                matching_lines += 1;
            }
        }

        matching_lines as f64 / max_lines as f64
    }

    /// Calculate semantic similarity based on function signatures and patterns
    fn calculate_semantic_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        let mut similarity = 0.0;

        // Function signature similarity
        if let (Some(sig1), Some(sig2)) = (&block1.function_signature, &block2.function_signature) {
            similarity += self.calculate_signature_similarity(sig1, sig2) * 0.5;
        }

        // Security pattern similarity
        if block1.security_score > 0.0 && block2.security_score > 0.0 {
            let security_sim = 1.0 - (block1.security_score - block2.security_score).abs();
            similarity += security_sim * 0.3;
        }

        // Complexity similarity
        let complexity_sim = 1.0 - (block1.complexity_score - block2.complexity_score).abs() / 10.0;
        similarity += complexity_sim.max(0.0) * 0.2;

        similarity.min(1.0)
    }

    /// Calculate token-based similarity
    fn calculate_token_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        let tokens1: std::collections::HashSet<&str> =
            block1.normalized_content.split_whitespace().collect();
        let tokens2: std::collections::HashSet<&str> =
            block2.normalized_content.split_whitespace().collect();

        let intersection = tokens1.intersection(&tokens2).count();
        let union = tokens1.union(&tokens2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Calculate signature similarity
    fn calculate_signature_similarity(&self, sig1: &str, sig2: &str) -> f64 {
        // Simple token-based comparison for function signatures
        let tokens1: std::collections::HashSet<&str> = sig1.split_whitespace().collect();
        let tokens2: std::collections::HashSet<&str> = sig2.split_whitespace().collect();

        let intersection = tokens1.intersection(&tokens2).count();
        let union = tokens1.union(&tokens2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Calculate security impact of duplicate code
    fn calculate_security_impact(
        &self,
        block1: &CodeBlock,
        block2: &CodeBlock,
        similarity: f64,
    ) -> Severity {
        let avg_security_score = (block1.security_score + block2.security_score) / 2.0;

        if avg_security_score > 0.8 && similarity > 0.9 {
            Severity::High
        } else if avg_security_score > 0.5 && similarity > 0.8 {
            Severity::Medium
        } else if avg_security_score > 0.0 || similarity > 0.9 {
            Severity::Low
        } else {
            Severity::Info
        }
    }

    /// Generate refactoring suggestion for duplicate code
    fn generate_refactoring_suggestion(&self, block1: &CodeBlock, block2: &CodeBlock) -> String {
        if block1.security_score > 0.5 || block2.security_score > 0.5 {
            "Extract security-sensitive duplicate code into a shared, well-tested utility function. \
             Ensure proper input validation and error handling in the extracted function.".to_string()
        } else if let (Some(sig1), Some(sig2)) =
            (&block1.function_signature, &block2.function_signature)
        {
            format!(
                "Consider extracting common logic from '{}' and '{}' into a shared function. \
                 This will reduce maintenance overhead and improve code consistency.",
                sig1, sig2
            )
        } else {
            "Extract duplicate code into a shared function or module to improve maintainability \
             and reduce the risk of inconsistent changes."
                .to_string()
        }
    }

    /// Calculate security relevance score for a code block
    fn calculate_security_score(&self, lines: &[String]) -> f64 {
        let content = lines.join(" ").to_lowercase();
        let mut score: f64 = 0.0;

        for pattern in &self.security_patterns {
            if pattern.is_match(&content) {
                score += 0.2; // Each pattern match adds to security score
            }
        }

        score.min(1.0)
    }

    /// Extract function signature from code block
    fn extract_function_signature(&self, lines: &[String]) -> Option<String> {
        for line in lines {
            let line_lower = line.to_lowercase();
            if line_lower.contains("fn ")
                || line_lower.contains("function ")
                || line_lower.contains("def ")
            {
                // Extract just the signature part (before opening brace)
                if let Some(pos) = line.find('{') {
                    return Some(line[..pos].trim().to_string());
                } else {
                    return Some(line.trim().to_string());
                }
            }
        }
        None
    }

    /// Calculate complexity score for a code block
    fn calculate_complexity_score(&self, lines: &[String]) -> f64 {
        let mut complexity = 1.0; // Base complexity

        for line in lines {
            let line_lower = line.to_lowercase();

            // Control flow statements increase complexity
            if line_lower.contains("if ") || line_lower.contains("else") {
                complexity += 1.0;
            }
            if line_lower.contains("for ") || line_lower.contains("while ") {
                complexity += 2.0;
            }
            if line_lower.contains("match ") || line_lower.contains("switch ") {
                complexity += 1.0;
            }
            if line_lower.contains("try ") || line_lower.contains("catch ") {
                complexity += 1.0;
            }
        }

        complexity
    }

    /// Normalize a line for comparison
    fn normalize_line(&self, line: &str) -> String {
        line.trim().split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Check if file should be ignored
    fn should_ignore_file(&self, file_path: &Path) -> bool {
        if self.ignore_test_files && self.is_test_file(file_path) {
            return true;
        }

        let path_str = file_path.to_string_lossy().to_lowercase();
        path_str.contains("generated")
            || path_str.contains("target/")
            || path_str.contains("build/")
            || path_str.contains("dist/")
            || path_str.contains("node_modules/")
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

    /// Check if file contains security-relevant code
    fn is_file_security_relevant(&self, blocks: &[CodeBlock]) -> bool {
        blocks.iter().any(|block| block.security_score > 0.0)
    }

    /// Calculate file hash for change detection
    fn calculate_file_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Clear the file cache
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        let total_files = self.file_cache.len();
        let total_blocks: usize = self
            .file_cache
            .values()
            .map(|analysis| analysis.code_blocks.len())
            .sum();
        (total_files, total_blocks)
    }
}

impl Analyzer for CrossFileDuplicateAnalyzer {
    fn name(&self) -> &str {
        "cross_file_duplicate"
    }

    fn analyze(&self, _file_path: &Path, _content: &[u8]) -> Result<Vec<Finding>> {
        // This analyzer works differently - it needs to collect all files first
        // then perform cross-file analysis. Individual file analysis just caches data.

        // For now, return empty findings as cross-file analysis happens separately
        // The actual cross-file analysis would be triggered by a separate process
        Ok(Vec::new())
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

    #[test]
    fn test_cross_file_duplicate_detection() {
        let mut analyzer = CrossFileDuplicateAnalyzer::new()
            .with_min_lines(3)
            .with_similarity_threshold(0.8);

        let content1 = r#"
fn authenticate_user(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}
"#;

        let content2 = r#"
fn authenticate_admin(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}
"#;

        let _ = analyzer
            .analyze_file(Path::new("auth1.rs"), content1)
            .unwrap();
        let _ = analyzer
            .analyze_file(Path::new("auth2.rs"), content2)
            .unwrap();

        let duplicates = analyzer.find_cross_file_duplicates().unwrap();
        assert!(!duplicates.is_empty());
        assert!(duplicates[0].similarity_score > 0.8);
    }

    #[test]
    fn test_security_score_calculation() {
        let analyzer = CrossFileDuplicateAnalyzer::new();
        let security_lines = vec![
            "authenticate_user".to_string(),
            "hash_password".to_string(),
            "validate_input".to_string(),
        ];
        let normal_lines = vec!["println!".to_string(), "format!".to_string()];

        let security_score = analyzer.calculate_security_score(&security_lines);
        let normal_score = analyzer.calculate_security_score(&normal_lines);

        assert!(security_score > normal_score);
        assert!(security_score > 0.0);
    }

    #[test]
    fn test_function_signature_extraction() {
        let analyzer = CrossFileDuplicateAnalyzer::new();
        let lines = vec![
            "fn authenticate_user(username: &str, password: &str) -> bool {".to_string(),
            "    let hashed = hash_password(password);".to_string(),
        ];

        let signature = analyzer.extract_function_signature(&lines);
        assert!(signature.is_some());
        assert!(signature.unwrap().contains("authenticate_user"));
    }
}
