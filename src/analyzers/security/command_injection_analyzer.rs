//! # Command Injection Analyzer
//!
//! This analyzer detects potential command injection vulnerabilities in code files.
//! Updated with performance optimizations: regex caching, memory pooling, and enhanced caching.

use crate::analyzers::Analyzer;
use crate::cache::memory_pool::MemoryPoolManager;
use crate::cache::regex_cache::SharedRegexCache;
use crate::config::PerformanceConfig;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

/// Command injection vulnerability analyzer with performance optimizations
pub struct CommandInjectionAnalyzer {
    regex_cache: SharedRegexCache,
    memory_pools: Arc<MemoryPoolManager>,
    pattern_strings: Vec<&'static str>,
}

impl Default for CommandInjectionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandInjectionAnalyzer {
    /// Create a new command injection analyzer with performance optimizations
    pub fn new() -> Self {
        Self::with_pools(Arc::new(MemoryPoolManager::new()))
    }

    /// Create analyzer with custom memory pools
    pub fn with_pools(memory_pools: Arc<MemoryPoolManager>) -> Self {
        let pattern_strings = vec![
            r";\s*(rm|del|format|shutdown)",
            r"\|\s*(cat|ls|dir)",
            r"`[^`]*`",     // More precise backtick pattern
            r"\$\([^)]*\)", // More precise dollar-parentheses
            r"system\s*\(",
        ];

        Self {
            regex_cache: SharedRegexCache::new(100, 3600, "lru".to_string()), // 100 patterns, 1 hour cache
            memory_pools,
            pattern_strings,
        }
    }

    /// Create analyzer with configuration settings
    pub fn with_config(config: &PerformanceConfig) -> Self {
        let memory_pools = Arc::new(MemoryPoolManager::with_config(
            config.memory_pools.findings_pool_size,
            config.memory_pools.strings_pool_size,
            config.memory_pools.pathbuf_pool_size,
            config.memory_pools.hashmap_pool_size,
        ));

        let pattern_strings = vec![
            r";\s*(rm|del|format|shutdown)",
            r"\|\s*(cat|ls|dir)",
            r"`[^`]*`",     // More precise backtick pattern
            r"\$\([^)]*\)", // More precise dollar-parentheses
            r"system\s*\(",
        ];

        Self {
            regex_cache: SharedRegexCache::new(
                config.regex_cache.capacity,
                config.regex_cache.expiration_seconds,
                config.regex_cache.eviction_policy.clone(),
            ),
            memory_pools,
            pattern_strings,
        }
    }

    /// Check if the file is a Rust source file
    fn is_rust_file(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|e| e.to_str()) == Some("rs")
    }

    /// Check if a pattern match is likely a false positive
    fn is_likely_false_positive(&self, line: &str, pattern_str: &str, is_rust: bool) -> bool {
        if is_rust {
            // In Rust, backticks in strings are typically for documentation or raw strings
            if pattern_str.contains("`")
                && (line.contains("///") || line.contains("//!") || line.contains("r#\""))
            {
                return true;
            }
            // Dollar-parentheses in Rust are typically macro syntax or documentation
            if pattern_str.contains(r#"\$\("#)
                && (line.contains("macro_rules!") || line.contains("///") || line.contains("//!"))
            {
                return true;
            }
        }

        // Skip patterns in comments
        if line.trim().starts_with("//")
            || line.trim().starts_with("#")
            || line.trim().starts_with("/*")
        {
            return true;
        }

        // Skip patterns in test functions
        if line.contains("#[test]") || line.contains("fn test_") {
            return true;
        }

        false
    }

    /// Check if a command pattern is high risk
    fn is_high_risk_command_pattern(&self, pattern_str: &str) -> bool {
        // High risk patterns that are more likely to be real vulnerabilities
        pattern_str.contains("rm")
            || pattern_str.contains("format")
            || pattern_str.contains("shutdown")
            || pattern_str.contains("DROP TABLE")
            || pattern_str.contains("DELETE FROM")
    }

    /// Analyze content for command injection patterns using cached regex
    fn analyze_content(&self, content: &str, file_path: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let is_rust = self.is_rust_file(file_path);

        // Get pooled objects
        let finding_pool = self.memory_pools.finding_pool();
        let string_pool = self.memory_pools.string_pool();
        let path_pool = self.memory_pools.path_pool();

        for (line_num, line) in content.lines().enumerate() {
            for pattern_str in &self.pattern_strings {
                // Skip backtick pattern for Rust files as backticks are not used for command execution in Rust
                if is_rust && pattern_str.contains("`") {
                    continue;
                }
                // Skip dollar-parentheses pattern for Rust files as $(...) is macro syntax
                if is_rust && pattern_str.contains(r#"\$\("#) {
                    continue;
                }

                // Use cached regex compilation
                let pattern = self.regex_cache.get_or_compile(pattern_str)?;

                if pattern.is_match(line) {
                    // Additional context checks for false positives
                    if self.is_likely_false_positive(line, pattern_str, is_rust) {
                        continue;
                    }

                    let severity = if self.is_high_risk_command_pattern(pattern_str) {
                        Severity::Critical
                    } else {
                        Severity::High
                    };

                    // Use pooled objects for finding creation
                    let mut finding = finding_pool.lock().unwrap_or_else(|e| e.into_inner()).get();

                    // Use pooled strings
                    let analyzer_name = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("security");
                    let rule_name = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("command_injection");
                    let message = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("Potential command injection vulnerability detected");
                    let description =
                        string_pool
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .get(&format!(
                                "Line contains pattern that may indicate command injection: {}",
                                pattern_str
                            ));
                    let suggestion = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("Validate and sanitize user input before passing to system commands");

                    // Use pooled path
                    let file_path_pooled = {
                        let mut path_pool = path_pool.lock().unwrap_or_else(|e| e.into_inner());
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
                    finding.severity = severity;
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
}

impl Analyzer for CommandInjectionAnalyzer {
    fn name(&self) -> &str {
        "command_injection"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        self.analyze_content(&content_str, file_path)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "js" | "ts" | "py" | "java" | "php" | "sh" | "bash"
            )
        } else {
            false
        }
    }
}
