pub mod code_quality_analyzer;
pub mod dependency_analyzer;
pub mod integrity;
pub mod lint_drift;
pub mod naming_checker;
pub mod non_production;
pub mod optimized_analyzer;
pub mod optimized_patterns;
pub mod performance_analyzer;
pub mod security_analyzer;
pub mod security_checks;

use crate::types::Finding;
use anyhow::Result;
use std::path::Path;

/// Trait for code analysis components.
///
/// Analyzers implement this trait to provide specific types of code analysis
/// such as security checks, performance analysis, or code quality metrics.
/// Each analyzer can examine files and produce findings based on its rules.
pub trait Analyzer {
    /// Returns the name of this analyzer.
    #[allow(dead_code)]
    fn name(&self) -> &str;

    /// Analyzes the given file content and returns any findings.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file being analyzed
    /// * `content` - Raw content of the file as bytes
    ///
    /// # Returns
    /// A vector of findings discovered during analysis
    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>>;

    /// Determines if this analyzer can process the given file type.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to check
    ///
    /// # Returns
    /// `true` if this analyzer supports the file type, `false` otherwise
    fn supports_file(&self, file_path: &Path) -> bool;
}

/// Registry that manages and coordinates multiple analyzers.
///
/// The AnalyzerRegistry holds all registered analyzers and provides
/// a unified interface for analyzing files. It automatically determines
/// which analyzers are applicable to each file type and coordinates
/// their execution.
pub struct AnalyzerRegistry {
    /// Collection of registered analyzers
    analyzers: Vec<Box<dyn Analyzer + Send + Sync>>,
}

impl Default for AnalyzerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerRegistry {
    /// Creates a new AnalyzerRegistry with all default analyzers registered.
    ///
    /// This includes analyzers for integrity checking, lint drift detection,
    /// non-production code identification, dependency analysis, performance
    /// analysis, security analysis, code quality, and optimized analysis.
    pub fn new() -> Self {
        let mut registry = Self {
            analyzers: Vec::new(),
        };

        // Register default analyzers
        registry.register(Box::new(integrity::IntegrityAnalyzer::new()));
        registry.register(Box::new(lint_drift::LintDriftAnalyzer::new()));
        registry.register(Box::new(non_production::NonProductionAnalyzer::new()));

        // Register enhanced analyzers
        registry.register(Box::new(dependency_analyzer::DependencyAnalyzer::new()));
        registry.register(Box::new(performance_analyzer::PerformanceAnalyzer::new()));
        registry.register(Box::new(security_analyzer::SecurityAnalyzer::new()));
        registry.register(Box::new(code_quality_analyzer::CodeQualityAnalyzer::new()));
        registry.register(Box::new(naming_checker::NamingChecker::new()));

        // Register optimized analyzer for high-performance analysis
        registry.register(Box::new(optimized_analyzer::OptimizedAnalyzer::new()));

        registry
    }

    /// Registers a new analyzer with the registry.
    ///
    /// # Arguments
    /// * `analyzer` - The analyzer to register (boxed for dynamic dispatch)
    pub fn register(&mut self, analyzer: Box<dyn Analyzer + Send + Sync>) {
        self.analyzers.push(analyzer);
    }

    /// Analyzes a file using all applicable registered analyzers.
    ///
    /// Iterates through all registered analyzers, checks if each supports
    /// the file type, and collects findings from all applicable analyzers.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file being analyzed
    /// * `content` - Raw content of the file as bytes
    ///
    /// # Returns
    /// A vector containing all findings from all applicable analyzers
    pub fn analyze_file(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();

        for analyzer in &self.analyzers {
            if analyzer.supports_file(file_path) {
                let findings = analyzer.analyze(file_path, content)?;
                all_findings.extend(findings);
            }
        }

        Ok(all_findings)
    }
}
