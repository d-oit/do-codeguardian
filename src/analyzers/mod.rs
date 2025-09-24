pub mod ai_content_analyzer;
pub mod build_artifact_analyzer;
pub mod cross_file_duplicate_analyzer;
pub mod dependency_analyzer;
pub mod duplicate_analyzer;
pub mod git_conflict_analyzer;
pub mod integrity;
pub mod lint_drift;
pub mod non_production;
pub mod performance_analyzer;
pub mod security;
pub mod security_analyzer;
pub mod validation_analyzer;

use crate::config::Config;
use crate::types::Finding;
use anyhow::Result;
use std::path::Path;

pub trait Analyzer {
    fn name(&self) -> &str;
    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>>;
    fn supports_file(&self, file_path: &Path) -> bool;
}

pub struct AnalyzerRegistry {
    analyzers: Vec<Box<dyn Analyzer + Send + Sync>>,
}

impl Default for AnalyzerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AnalyzerRegistry {
    fn clone(&self) -> Self {
        // Create a new registry with the same configuration
        // This is a simplified approach - in practice you might want to store config
        Self::new()
    }
}

impl AnalyzerRegistry {
    pub fn new() -> Self {
        Self::with_config(&Config::default()).expect("Failed to create default analyzer registry")
    }

    pub fn with_config(config: &Config) -> Result<Self> {
        let mut registry = Self {
            analyzers: Vec::new(),
        };

        // Register default analyzers
        registry.register(Box::new(integrity::IntegrityAnalyzer::new()));
        registry.register(Box::new(lint_drift::LintDriftAnalyzer::new()));
        registry.register(Box::new(non_production::NonProductionAnalyzer::new(
            &config.analyzers.non_production,
        )));
        registry.register(Box::new(
            performance_analyzer::PerformanceAnalyzer::with_config(&config.performance)?,
        ));
        registry.register(Box::new(security_analyzer::SecurityAnalyzer::with_config(
            &config.performance,
        )));
        registry.register(Box::new(dependency_analyzer::DependencyAnalyzer::new(
            std::env::current_dir()
                .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?,
        )));
        registry.register(Box::new(
            build_artifact_analyzer::BuildArtifactAnalyzer::new(),
        ));

        // Register broken files detection analyzers based on configuration
        if config.analyzers.broken_files.enabled {
            if config.analyzers.broken_files.detect_merge_conflicts {
                let conflict_analyzer = git_conflict_analyzer::GitConflictAnalyzer::new()
                    .with_syntax_validation(
                        config.analyzers.broken_files.conflicts.validate_syntax,
                    );
                registry.register(Box::new(conflict_analyzer));
            }

            if config.analyzers.broken_files.detect_ai_placeholders {
                let ai_analyzer = if !config
                    .analyzers
                    .broken_files
                    .placeholders
                    .custom_patterns
                    .is_empty()
                {
                    ai_content_analyzer::AiContentAnalyzer::new()?.with_custom_patterns(
                        config
                            .analyzers
                            .broken_files
                            .placeholders
                            .custom_patterns
                            .clone(),
                    )?
                } else {
                    ai_content_analyzer::AiContentAnalyzer::new()?
                };
                registry.register(Box::new(ai_analyzer));
            }

            if config.analyzers.broken_files.detect_duplicates {
                let duplicate_analyzer = duplicate_analyzer::DuplicateAnalyzer::new()?
                    .with_min_lines(config.analyzers.broken_files.duplicates.min_lines)
                    .with_security_focus(config.analyzers.broken_files.duplicates.focus_security)
                    .with_test_files(!config.analyzers.broken_files.duplicates.ignore_test_files)
                    .with_max_files(
                        config
                            .analyzers
                            .broken_files
                            .duplicates
                            .max_files_to_compare,
                    );
                registry.register(Box::new(duplicate_analyzer));
            }
        }

        Ok(registry)
    }

    pub fn register(&mut self, analyzer: Box<dyn Analyzer + Send + Sync>) {
        self.analyzers.push(analyzer);
    }

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
