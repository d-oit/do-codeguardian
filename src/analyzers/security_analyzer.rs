use crate::analyzers::security::{
    CommandInjectionAnalyzer, SecretAnalyzer, SqlInjectionAnalyzer, VulnerabilityAnalyzer,
    XssAnalyzer,
};
use crate::analyzers::Analyzer;
use crate::config::PerformanceConfig;
use crate::types::Finding;
use anyhow::Result;
use std::path::Path;

/// Composite security analyzer that delegates to specialized analyzers
pub struct SecurityAnalyzer {
    sql_analyzer: SqlInjectionAnalyzer,
    xss_analyzer: XssAnalyzer,
    command_analyzer: CommandInjectionAnalyzer,
    secret_analyzer: SecretAnalyzer,
    vulnerability_analyzer: VulnerabilityAnalyzer,
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityAnalyzer {
    /// Creates a new security analyzer with default configuration
    ///
    /// Initializes all specialized analyzers (SQL injection, XSS, command injection,
    /// secrets, and general vulnerabilities) with their default settings.
    pub fn new() -> Self {
        Self {
            sql_analyzer: SqlInjectionAnalyzer::new(),
            xss_analyzer: XssAnalyzer::new(),
            command_analyzer: CommandInjectionAnalyzer::new(),
            secret_analyzer: SecretAnalyzer::new(),
            vulnerability_analyzer: VulnerabilityAnalyzer::new(),
        }
    }

    /// Creates a new security analyzer with custom performance configuration
    ///
    /// This method allows configuring memory pools and performance settings
    /// for optimized analysis of large codebases.
    ///
    /// # Arguments
    ///
    /// * `config` - Performance configuration including memory pool sizes
    pub fn with_config(config: &PerformanceConfig) -> Self {
        use crate::cache::memory_pool::MemoryPoolManager;
        use std::sync::Arc;

        let memory_pools = Arc::new(MemoryPoolManager::with_config(
            config.memory_pools.findings_pool_size,
            config.memory_pools.strings_pool_size,
            config.memory_pools.pathbuf_pool_size,
            config.memory_pools.hashmap_pool_size,
        ));

        Self {
            sql_analyzer: SqlInjectionAnalyzer::with_pools(Arc::clone(&memory_pools)),
            xss_analyzer: XssAnalyzer::with_pools(Arc::clone(&memory_pools)),
            command_analyzer: CommandInjectionAnalyzer::with_pools(Arc::clone(&memory_pools)),
            secret_analyzer: SecretAnalyzer::with_pools(Arc::clone(&memory_pools)),
            vulnerability_analyzer: VulnerabilityAnalyzer::with_pools(Arc::clone(&memory_pools)),
        }
    }

    /// Check if a file should be skipped from security analysis
    fn should_skip_file(&self, file_path: &Path) -> bool {
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            // Skip analyzer files as they contain security patterns by design
            if file_name.contains("analyzer") || file_name.contains("security") {
                return true;
            }
        }

        // Skip test files
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            if file_name.ends_with("_test.rs")
                || file_name == "tests.rs"
                || file_name.contains("test")
            {
                return true;
            }
        }

        // Skip files in tests directory
        if file_path.to_string_lossy().contains("/tests/") {
            return true;
        }

        false
    }

    /// Performs comprehensive security analysis on file content
    ///
    /// Delegates to all specialized analyzers (SQL injection, XSS, command injection,
    /// secrets, and general vulnerabilities) and aggregates their findings.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file being analyzed
    /// * `content` - File content as bytes
    ///
    /// # Returns
    ///
    /// Returns a vector of security findings from all analyzers
    fn perform_security_checks(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();

        // Skip security analysis for analyzer files as they contain security patterns by design
        if self.should_skip_file(file_path) {
            return Ok(all_findings);
        }

        // Delegate to individual analyzers
        if self.sql_analyzer.supports_file(file_path) {
            all_findings.extend(self.sql_analyzer.analyze(file_path, content)?);
        }
        if self.xss_analyzer.supports_file(file_path) {
            all_findings.extend(self.xss_analyzer.analyze(file_path, content)?);
        }
        if self.command_analyzer.supports_file(file_path) {
            all_findings.extend(self.command_analyzer.analyze(file_path, content)?);
        }
        if self.secret_analyzer.supports_file(file_path) {
            all_findings.extend(self.secret_analyzer.analyze(file_path, content)?);
        }
        if self.vulnerability_analyzer.supports_file(file_path) {
            all_findings.extend(self.vulnerability_analyzer.analyze(file_path, content)?);
        }

        Ok(all_findings)
    }
}

impl Analyzer for SecurityAnalyzer {
    fn name(&self) -> &str {
        "security"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.perform_security_checks(file_path, content)
    }

    fn supports_file(&self, _file_path: &Path) -> bool {
        // Support all file types - individual analyzers will filter
        true
    }
}
