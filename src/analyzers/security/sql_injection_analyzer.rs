//! # SQL Injection Analyzer
//!
//! This analyzer detects potential SQL injection vulnerabilities in code files.
//! Updated with performance optimizations: regex caching, memory pooling, and enhanced caching.

use crate::analyzers::Analyzer;
use crate::cache::memory_pool::MemoryPoolManager;
use crate::cache::regex_cache::SharedRegexCache;
use crate::config::PerformanceConfig;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

/// SQL injection vulnerability analyzer with performance optimizations
pub struct SqlInjectionAnalyzer {
    regex_cache: SharedRegexCache,
    memory_pools: Arc<MemoryPoolManager>,
    pattern_strings: Vec<&'static str>,
}

impl Default for SqlInjectionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SqlInjectionAnalyzer {
    /// Create a new SQL injection analyzer with performance optimizations
    pub fn new() -> Self {
        Self::with_pools(Arc::new(MemoryPoolManager::new()))
    }

    /// Create analyzer with custom memory pools
    pub fn with_pools(memory_pools: Arc<MemoryPoolManager>) -> Self {
        let pattern_strings = vec![
            r"'?\s*OR\s+\d+\s*=\s*\d+",
            r"'?\s*AND\s+\d+\s*=\s*\d+",
            r"UNION\s+SELECT",
            r"--\s*$",
            r";\s*DROP\s+TABLE",
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
            r"'?\s*OR\s+\d+\s*=\s*\d+",
            r"'?\s*AND\s+\d+\s*=\s*\d+",
            r"UNION\s+SELECT",
            r"--\s*$",
            r";\s*DROP\s+TABLE",
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

    /// Analyze content for SQL injection patterns using cached regex
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
                        .get("sql_injection");
                    let message = string_pool
                        .lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .get("Potential SQL injection vulnerability detected");
                    let description =
                        string_pool
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .get(&format!(
                                "Line contains pattern that may indicate SQL injection: {}",
                                pattern_str
                            ));
                    let suggestion = string_pool.lock().unwrap_or_else(|e| e.into_inner()).get(
                        "Use parameterized queries or prepared statements to prevent SQL injection",
                    );

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

impl Analyzer for SqlInjectionAnalyzer {
    fn name(&self) -> &str {
        "sql_injection"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let content_str = String::from_utf8_lossy(content);
        self.analyze_content(&content_str, file_path)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "rs" | "js" | "ts" | "py" | "java" | "php" | "sql")
        } else {
            false
        }
    }
}
