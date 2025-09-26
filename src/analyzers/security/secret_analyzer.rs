//! # Secret Detection Analyzer
//!
//! This analyzer detects potential hardcoded secrets and credentials in code files.
//! Updated with performance optimizations: regex caching, memory pooling, and enhanced caching.

use crate::analyzers::Analyzer;
use crate::cache::memory_pool::MemoryPoolManager;
use crate::cache::regex_cache::SharedRegexCache;
use crate::config::PerformanceConfig;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

/// Secret detection analyzer with performance optimizations
pub struct SecretAnalyzer {
    regex_cache: SharedRegexCache,
    memory_pools: Arc<MemoryPoolManager>,
    pattern_strings: Vec<&'static str>,
}

impl Default for SecretAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretAnalyzer {
    /// Create a new secret analyzer with performance optimizations
    pub fn new() -> Self {
        Self::with_pools(Arc::new(MemoryPoolManager::new()))
    }

    /// Create analyzer with custom memory pools
    pub fn with_pools(memory_pools: Arc<MemoryPoolManager>) -> Self {
        let pattern_strings = vec![
            // Case-insensitive patterns for real-world variable names
            r#"(?i)(api_key|apikey)\s*=\s*["'][^"']{8,}["']"#,
            r#"(?i)(password|passwd|pwd)\s*=\s*["'][^"']{6,}["']"#,
            r#"(?i)(secret|secret_key)\s*=\s*["'][^"']{8,}["']"#,
            r#"(?i)(token|access_token|auth_token)\s*=\s*["'][^"']{10,}["']"#,
            // Common API key patterns
            r#"["']sk-[a-zA-Z0-9]{20,}["']"#,
            r#"["']pk_[a-zA-Z0-9]{20,}["']"#,
            r#"["']AIza[a-zA-Z0-9]{35}["']"#,
            // AWS patterns
            r#"["']AKIA[A-Z0-9]{16}["']"#,
            // GitHub tokens
            r#"["']ghp_[a-zA-Z0-9]{36}["']"#,
            r#"aws_access_key_id\s*=\s*["'][^"']*["']"#,
            r#"aws_secret_access_key\s*=\s*["'][^"']*["']"#,
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
            // Case-insensitive patterns for real-world variable names
            r#"(?i)(api_key|apikey)\s*=\s*["'][^"']{8,}["']"#,
            r#"(?i)(password|passwd|pwd)\s*=\s*["'][^"']{6,}["']"#,
            r#"(?i)(secret|secret_key)\s*=\s*["'][^"']{8,}["']"#,
            r#"(?i)(token|access_token|auth_token)\s*=\s*["'][^"']{10,}["']"#,
            // Common API key patterns
            r#"["']sk-[a-zA-Z0-9]{20,}["']"#,
            r#"["']pk_[a-zA-Z0-9]{20,}["']"#,
            r#"["']AIza[a-zA-Z0-9]{35}["']"#,
            // AWS patterns
            r#"["']AKIA[A-Z0-9]{16}["']"#,
            // GitHub tokens
            r#"["']ghp_[a-zA-Z0-9]{36}["']"#,
            r#"aws_access_key_id\s*=\s*["'][^"']*["']"#,
            r#"aws_secret_access_key\s*=\s*["'][^"']*["']"#,
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

    /// Check if a secret pattern match is likely a false positive
    fn is_likely_false_positive(&self, line: &str, _pattern_str: &str) -> bool {
        let trimmed = line.trim();

        // Skip only obvious documentation/comments (be more selective)
        if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            return true;
        }

        // Skip only if it's clearly a pattern definition or documentation
        if trimmed.contains("example") && trimmed.starts_with("//") {
            return true;
        }

        // Skip only obvious test patterns (be more specific)
        if line.contains("#[test]") || line.contains("fn test_") {
            return true;
        }

        // Skip pattern definitions (common in security analyzers)
        if line.contains("Regex::new") || line.contains("Pattern::new") || line.contains("pattern")
        {
            return true;
        }

        // Skip documentation or example code
        if line.contains("///")
            || line.contains("//!")
            || line.contains("example")
            || line.contains("Example")
        {
            return true;
        }

        // Skip if the line contains quotes around the pattern (indicating it's a string literal for pattern matching)
        if line.contains("\"API_KEY\"")
            || line.contains("\"PASSWORD\"")
            || line.contains("\"SECRET\"")
            || line.contains("\"TOKEN\"")
            || line.contains("\"aws_access_key")
        {
            return true;
        }

        false
    }

    /// Analyze content for hardcoded secrets using cached regex
    fn analyze_content(&self, content: &str, file_path: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Get pooled objects
        let finding_pool = self.memory_pools.finding_pool();
        let string_pool = self.memory_pools.string_pool();
        let path_pool = self.memory_pools.path_pool();

        for (line_num, line) in content.lines().enumerate() {
            for pattern_str in &self.pattern_strings {
                // Use cached regex compilation
                let pattern = self.regex_cache.get_or_compile(pattern_str)?;

                if pattern.is_match(line) {
                    // Skip if this is likely a false positive (pattern definition, test, or documentation)
                    if self.is_likely_false_positive(line, pattern_str) {
                        continue;
                    }

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
                        .get("hardcoded_secret");
                    let message = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("Hardcoded secret detected");
                    let description = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("Line contains what appears to be a hardcoded secret or credential");
                    let suggestion = string_pool.lock().unwrap_or_else(|e| e.into_inner()).get("Use environment variables or secure credential storage instead of hardcoding secrets");

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
}

impl Analyzer for SecretAnalyzer {
    fn name(&self) -> &str {
        "secret"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        self.analyze_content(&content_str, file_path)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext,
                "rs" | "js" | "ts" | "py" | "java" | "php" | "yaml" | "yml" | "json" | "toml"
            )
        } else {
            false
        }
    }
}
