use crate::analyzers::Analyzer;
use crate::cache::memory_pool::MemoryPoolManager;
use crate::cache::regex_cache::SharedRegexCache;
use crate::config::PerformanceConfig;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

/// Performance analyzer for detecting performance bottlenecks in Rust code.
/// Focuses on common anti-patterns that can cause performance issues.
/// Updated with performance optimizations: regex caching, memory pooling, and enhanced caching.
pub struct PerformanceAnalyzer {
    regex_cache: SharedRegexCache,
    memory_pools: Arc<MemoryPoolManager>,
    nested_loop_patterns: Vec<&'static str>,
    inefficient_string_patterns: Vec<&'static str>,
    blocking_io_patterns: Vec<&'static str>,
    algorithmic_inefficiency_patterns: Vec<&'static str>,
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new().expect("Failed to create PerformanceAnalyzer with default patterns")
    }
}

impl PerformanceAnalyzer {
    /// Creates a new PerformanceAnalyzer with performance optimizations.
    pub fn new() -> Result<Self, anyhow::Error> {
        Self::with_pools(Arc::new(MemoryPoolManager::new()))
    }

    /// Create analyzer with custom memory pools
    pub fn with_pools(memory_pools: Arc<MemoryPoolManager>) -> Result<Self, anyhow::Error> {
        Ok(Self {
            regex_cache: SharedRegexCache::new(100, 3600, "lru".to_string()), // 100 patterns, 1 hour cache
            memory_pools,
            nested_loop_patterns: vec![
                // Detect nested for loops (simple heuristic)
                r"for\s+.*\{[\s\S]*?for\s+.*\{",
                // Detect nested while loops
                r"while\s+.*\{[\s\S]*?while\s+.*\{",
                // Detect for inside while or vice versa
                r"(for\s+.*\{[\s\S]*?while\s+.*\{|while\s+.*\{[\s\S]*?for\s+.*\{)",
            ],
            inefficient_string_patterns: vec![
                // String concatenation in loops using +=
                r"(for\s+.*\{[\s\S]*?(\w+)\s*\+=\s*.*|while\s+.*\{[\s\S]*?(\w+)\s*\+=\s*.*)",
                // Direct string concatenation with +=
                r"(\w+)\s*\+=\s*&?(format!|String::)",
            ],
            blocking_io_patterns: vec![
                // File I/O operations that might block
                r"std::fs::(read_to_string|read|write)",
                // Network operations
                r"reqwest::(get|post|Client::)",
                // Synchronous I/O
                r"std::io::(Read|Write)::read",
            ],
            algorithmic_inefficiency_patterns: vec![
                // O(n^2) algorithms
                r"for\s+.*\{[\s\S]*?for\s+.*\{[\s\S]*?\.contains\(|\.index_of\(|\.find\(",
                // Inefficient sorting
                r"sort\(\)\.rev\(\)\.collect\(\)",
            ],
        })
    }

    /// Create analyzer with configuration settings
    pub fn with_config(config: &PerformanceConfig) -> Result<Self, anyhow::Error> {
        let memory_pools = Arc::new(MemoryPoolManager::with_config(
            config.memory_pools.findings_pool_size,
            config.memory_pools.strings_pool_size,
            config.memory_pools.pathbuf_pool_size,
            config.memory_pools.hashmap_pool_size,
        ));

        Ok(Self {
            regex_cache: SharedRegexCache::new(
                config.regex_cache.capacity,
                config.regex_cache.expiration_seconds,
                config.regex_cache.eviction_policy.clone(),
            ),
            memory_pools,
            nested_loop_patterns: vec![
                // Detect nested for loops (simple heuristic)
                r"for\s+.*\{[\s\S]*?for\s+.*\{",
                // Detect nested while loops
                r"while\s+.*\{[\s\S]*?while\s+.*\{",
                // Detect for inside while or vice versa
                r"(for\s+.*\{[\s\S]*?while\s+.*\{|while\s+.*\{[\s\S]*?for\s+.*\{)",
            ],
            inefficient_string_patterns: vec![
                // String concatenation in loops using +=
                r"(for\s+.*\{[\s\S]*?(\w+)\s*\+=\s*.*|while\s+.*\{[\s\S]*?(\w+)\s*\+=\s*.*)",
                // Direct string concatenation with +=
                r"(\w+)\s*\+=\s*&?(format!|String::)",
            ],
            blocking_io_patterns: vec![
                // File I/O operations that might block
                r"std::fs::(read_to_string|read|write)",
                // Network operations
                r"reqwest::(get|post|Client::)",
                // Synchronous I/O
                r"std::io::(Read|Write)::read",
            ],
            algorithmic_inefficiency_patterns: vec![
                // O(n^2) algorithms
                r"for\s+.*\{[\s\S]*?for\s+.*\{[\s\S]*?\.contains\(|\.index_of\(|\.find\(",
                // Inefficient sorting
                r"sort\(\)\.rev\(\)\.collect\(\)",
            ],
        })
    }

    /// Detects nested loops that may indicate algorithmic inefficiencies.
    pub fn detect_nested_loops(&self, content: &str, file_path: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Get pooled objects
        let finding_pool = self.memory_pools.finding_pool();
        let string_pool = self.memory_pools.string_pool();
        let path_pool = self.memory_pools.path_pool();

        // Check the entire content for nested patterns, not just line by line
        for pattern_str in &self.nested_loop_patterns {
            // Use cached regex compilation
            let pattern = self.regex_cache.get_or_compile(pattern_str)?;

            if pattern.is_match(content) {
                // Find the line number where the pattern occurs
                let line_num = content
                    .lines()
                    .position(|line| line.contains("for"))
                    .unwrap_or(0);

                // Use pooled objects for finding creation
                let mut finding = finding_pool.lock().unwrap().get();

                // Use pooled strings
                let analyzer_name = string_pool.lock().unwrap().get("performance");
                let rule_name = string_pool.lock().unwrap().get("nested_loops");
                let message = string_pool
                    .lock()
                    .unwrap()
                    .get("Potential nested loop detected");
                let description = string_pool.lock().unwrap().get("Nested loops can lead to O(n^2) or higher time complexity. Consider optimizing the algorithm.");
                let suggestion = string_pool.lock().unwrap().get("Review the algorithm for potential optimizations, such as using hash maps or breaking early.");

                // Use pooled path
                let file_path_pooled = {
                    let mut path_pool = path_pool.lock().unwrap();
                    let mut pooled_path = path_pool.get();
                    pooled_path.push(file_path);
                    pooled_path
                };

                finding.id = crate::types::generate_finding_id(
                    &analyzer_name,
                    &rule_name,
                    &file_path_pooled.to_string_lossy(),
                    (line_num + 1) as u32,
                    &message,
                );
                finding.analyzer = (*analyzer_name).clone();
                finding.rule = (*rule_name).clone();
                finding.severity = Severity::Medium;
                finding.file = file_path_pooled;
                finding.line = (line_num + 1) as u32;
                finding.message = (*message).clone();
                finding.description = Some((*description).clone());
                finding.suggestion = Some((*suggestion).clone());

                findings.push(finding);
                break; // Only report once per file
            }
        }

        // Also check line by line for simpler patterns
        for (line_num, line) in content.lines().enumerate() {
            for pattern_str in &self.nested_loop_patterns {
                // Use cached regex compilation
                let pattern = self.regex_cache.get_or_compile(pattern_str)?;

                if pattern.is_match(line) {
                    // Use pooled objects for finding creation
                    let mut finding = finding_pool.lock().unwrap().get();

                    // Use pooled strings
                    let analyzer_name = string_pool.lock().unwrap().get("performance");
                    let rule_name = string_pool.lock().unwrap().get("nested_loops");
                    let message = string_pool
                        .lock()
                        .unwrap()
                        .get("Potential nested loop detected");
                    let description = string_pool.lock().unwrap().get("Nested loops can lead to O(n^2) or higher time complexity. Consider optimizing the algorithm.");
                    let suggestion = string_pool.lock().unwrap().get("Review the algorithm for potential optimizations, such as using hash maps or breaking early.");

                    // Use pooled path
                    let file_path_pooled = {
                        let mut path_pool = path_pool.lock().unwrap();
                        let mut pooled_path = path_pool.get();
                        pooled_path.push(file_path);
                        pooled_path
                    };

                    finding.id = crate::types::generate_finding_id(
                        &analyzer_name,
                        &rule_name,
                        &file_path_pooled.to_string_lossy(),
                        (line_num + 1) as u32,
                        &message,
                    );
                    finding.analyzer = (*analyzer_name).clone();
                    finding.rule = (*rule_name).clone();
                    finding.severity = Severity::Medium;
                    finding.file = file_path_pooled;
                    finding.line = (line_num + 1) as u32;
                    finding.message = (*message).clone();
                    finding.description = Some((*description).clone());
                    finding.suggestion = Some((*suggestion).clone());

                    findings.push(finding);
                }
            }
        }
        Ok(findings)
    }

    /// Detects inefficient string operations, particularly concatenation in loops.
    pub fn detect_inefficient_strings(
        &self,
        content: &str,
        file_path: &Path,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Get pooled objects
        let finding_pool = self.memory_pools.finding_pool();
        let string_pool = self.memory_pools.string_pool();
        let path_pool = self.memory_pools.path_pool();

        for (line_num, line) in content.lines().enumerate() {
            for pattern_str in &self.inefficient_string_patterns {
                // Use cached regex compilation
                let pattern = self.regex_cache.get_or_compile(pattern_str)?;

                if pattern.is_match(line) {
                    // Use pooled objects for finding creation
                    let mut finding = finding_pool.lock().unwrap().get();

                    // Use pooled strings
                    let analyzer_name = string_pool.lock().unwrap().get("performance");
                    let rule_name = string_pool.lock().unwrap().get("inefficient_string_ops");
                    let message = string_pool
                        .lock()
                        .unwrap()
                        .get("Inefficient string operation detected");
                    let description = string_pool.lock().unwrap().get(
                        "String concatenation in loops can be inefficient due to reallocations.",
                    );
                    let suggestion = string_pool.lock().unwrap().get("Use String::with_capacity or collect into a Vec and join for better performance.");

                    // Use pooled path
                    let file_path_pooled = {
                        let mut path_pool = path_pool.lock().unwrap();
                        let mut pooled_path = path_pool.get();
                        pooled_path.push(file_path);
                        pooled_path
                    };

                    finding.id = crate::types::generate_finding_id(
                        &analyzer_name,
                        &rule_name,
                        &file_path_pooled.to_string_lossy(),
                        (line_num + 1) as u32,
                        &message,
                    );
                    finding.analyzer = (*analyzer_name).clone();
                    finding.rule = (*rule_name).clone();
                    finding.severity = Severity::Medium;
                    finding.file = file_path_pooled;
                    finding.line = (line_num + 1) as u32;
                    finding.message = (*message).clone();
                    finding.description = Some((*description).clone());
                    finding.suggestion = Some((*suggestion).clone());

                    findings.push(finding);
                }
            }
        }
        Ok(findings)
    }

    /// Detects blocking I/O operations that should be asynchronous.
    pub fn detect_blocking_io(&self, content: &str, file_path: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Get pooled objects
        let finding_pool = self.memory_pools.finding_pool();
        let string_pool = self.memory_pools.string_pool();
        let path_pool = self.memory_pools.path_pool();

        for (line_num, line) in content.lines().enumerate() {
            for pattern_str in &self.blocking_io_patterns {
                // Use cached regex compilation
                let pattern = self.regex_cache.get_or_compile(pattern_str)?;

                if pattern.is_match(line) {
                    // Use pooled objects for finding creation
                    let mut finding = finding_pool.lock().unwrap().get();

                    // Use pooled strings
                    let analyzer_name = string_pool.lock().unwrap().get("performance");
                    let rule_name = string_pool.lock().unwrap().get("blocking_io");
                    let message = string_pool
                        .lock()
                        .unwrap()
                        .get("Blocking I/O operation detected");
                    let description = string_pool
                        .lock()
                        .unwrap()
                        .get("Blocking I/O can cause performance issues in async contexts.");
                    let suggestion = string_pool
                        .lock()
                        .unwrap()
                        .get("Use async equivalents like tokio::fs or spawn blocking operations.");

                    // Use pooled path
                    let file_path_pooled = {
                        let mut path_pool = path_pool.lock().unwrap();
                        let mut pooled_path = path_pool.get();
                        pooled_path.push(file_path);
                        pooled_path
                    };

                    finding.id = crate::types::generate_finding_id(
                        &analyzer_name,
                        &rule_name,
                        &file_path_pooled.to_string_lossy(),
                        (line_num + 1) as u32,
                        &message,
                    );
                    finding.analyzer = (*analyzer_name).clone();
                    finding.rule = (*rule_name).clone();
                    finding.severity = Severity::High;
                    finding.file = file_path_pooled;
                    finding.line = (line_num + 1) as u32;
                    finding.message = (*message).clone();
                    finding.description = Some((*description).clone());
                    finding.suggestion = Some((*suggestion).clone());

                    findings.push(finding);
                }
            }
        }
        Ok(findings)
    }

    /// Detects potential algorithmic inefficiencies.
    pub fn detect_algorithmic_inefficiencies(
        &self,
        content: &str,
        file_path: &Path,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Get pooled objects
        let finding_pool = self.memory_pools.finding_pool();
        let string_pool = self.memory_pools.string_pool();
        let path_pool = self.memory_pools.path_pool();

        for (line_num, line) in content.lines().enumerate() {
            for pattern_str in &self.algorithmic_inefficiency_patterns {
                // Use cached regex compilation
                let pattern = self.regex_cache.get_or_compile(pattern_str)?;

                if pattern.is_match(line) {
                    // Use pooled objects for finding creation
                    let mut finding = finding_pool.lock().unwrap().get();

                    // Use pooled strings
                    let analyzer_name = string_pool.lock().unwrap().get("performance");
                    let rule_name = string_pool.lock().unwrap().get("algorithmic_inefficiency");
                    let message = string_pool
                        .lock()
                        .unwrap()
                        .get("Potential algorithmic inefficiency detected");
                    let description = string_pool.lock().unwrap().get("Code pattern suggests potential inefficiency, such as unnecessary collections.");
                    let suggestion = string_pool.lock().unwrap().get("Review the algorithm for optimizations, e.g., avoid collect().iter() chains.");

                    // Use pooled path
                    let file_path_pooled = {
                        let mut path_pool = path_pool.lock().unwrap();
                        let mut pooled_path = path_pool.get();
                        pooled_path.push(file_path);
                        pooled_path
                    };

                    finding.id = crate::types::generate_finding_id(
                        &analyzer_name,
                        &rule_name,
                        &file_path_pooled.to_string_lossy(),
                        (line_num + 1) as u32,
                        &message,
                    );
                    finding.analyzer = (*analyzer_name).clone();
                    finding.rule = (*rule_name).clone();
                    finding.severity = Severity::Medium;
                    finding.file = file_path_pooled;
                    finding.line = (line_num + 1) as u32;
                    finding.message = (*message).clone();
                    finding.description = Some((*description).clone());
                    finding.suggestion = Some((*suggestion).clone());

                    findings.push(finding);
                }
            }
        }
        Ok(findings)
    }

    /// Performs all performance checks on the given content.
    fn perform_performance_checks(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        let mut all_findings = Vec::new();

        all_findings.extend(self.detect_nested_loops(&content_str, file_path)?);
        all_findings.extend(self.detect_inefficient_strings(&content_str, file_path)?);
        all_findings.extend(self.detect_blocking_io(&content_str, file_path)?);
        all_findings.extend(self.detect_algorithmic_inefficiencies(&content_str, file_path)?);

        Ok(all_findings)
    }
}

impl Analyzer for PerformanceAnalyzer {
    fn name(&self) -> &str {
        "performance"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.perform_performance_checks(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        // Only analyze Rust files for performance issues
        file_path.extension().and_then(|e| e.to_str()) == Some("rs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_detect_nested_loops() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        let content = r#"
        fn example() {
            for i in 0..10 {
                for j in 0..10 {
                    println!("{} {}", i, j);
                }
            }
        }
        "#;
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, content).expect("Failed to write test file");

        let findings = analyzer.detect_nested_loops(content, &file_path).unwrap();
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule, "nested_loops");
    }

    #[test]
    fn test_detect_inefficient_strings() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        let content = r#"
        fn example() {
            let mut s = String::new();
            for i in 0..10 {
                s += &format!(" {}", i);
            }
        }
        "#;
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let file_path = temp_dir.path().join("test.rs");
        std::fs::write(&file_path, content).expect("Failed to write test file");

        let findings = analyzer
            .detect_inefficient_strings(content, &file_path)
            .unwrap();
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule, "inefficient_string_ops");
    }

    #[test]
    fn test_supports_file() {
        let analyzer = PerformanceAnalyzer::new().unwrap();
        assert!(analyzer.supports_file(&PathBuf::from("test.rs")));
        assert!(!analyzer.supports_file(&PathBuf::from("test.js")));
    }
}
