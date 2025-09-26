//! # Context Analysis Module
//!
//! This module provides context analysis capabilities for understanding
//! the environment and circumstances around security findings.

use super::{CodeContext, ContextData, EnvironmentContext, HistoricalContext, ProjectContext};
use crate::types::{AnalysisResults, Finding};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Context analyzer for enriching findings with environmental data
pub struct ContextAnalyzer {
    /// Project-level context cache
    #[allow(dead_code)]
    project_cache: Option<ProjectContext>,
}

impl ContextAnalyzer {
    /// Create a new context analyzer
    pub fn new() -> Self {
        Self {
            project_cache: None,
        }
    }
}

impl Default for ContextAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextAnalyzer {
    /// Analyze context for analysis results
    #[allow(dead_code)]
    fn analyze_context(&mut self, results: &AnalysisResults) -> Result<ContextData> {
        Ok(ContextData {
            project_context: self.analyze_project_context(results)?,
            code_context: self.analyze_code_context(results)?,
            historical_context: self.analyze_historical_context(results)?,
            environment_context: self.analyze_environment_context(results)?,
        })
    }

    /// Analyze project-level context
    fn analyze_project_context(&mut self, results: &AnalysisResults) -> Result<ProjectContext> {
        if let Some(cached) = &self.project_cache {
            return Ok(cached.clone());
        }

        let mut languages = std::collections::HashSet::new();
        let frameworks = std::collections::HashSet::new();

        // Infer languages from file extensions
        for finding in &results.findings {
            if let Some(ext) = finding.file.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                match ext_str.as_str() {
                    "rs" => {
                        languages.insert("Rust".to_string());
                    }
                    "py" => {
                        languages.insert("Python".to_string());
                    }
                    "js" | "ts" => {
                        languages.insert("JavaScript/TypeScript".to_string());
                    }
                    "java" => {
                        languages.insert("Java".to_string());
                    }
                    "cpp" | "cc" | "cxx" => {
                        languages.insert("C++".to_string());
                    }
                    "c" => {
                        languages.insert("C".to_string());
                    }
                    "go" => {
                        languages.insert("Go".to_string());
                    }
                    "rb" => {
                        languages.insert("Ruby".to_string());
                    }
                    "php" => {
                        languages.insert("PHP".to_string());
                    }
                    "cs" => {
                        languages.insert("C#".to_string());
                    }
                    _ => {}
                }
            }
        }

        // Infer project type based on file patterns
        let project_type = self.infer_project_type(&results.findings);

        let context = ProjectContext {
            project_type,
            languages: languages.into_iter().collect(),
            frameworks: frameworks.into_iter().collect(),
            maturity_level: self.assess_project_maturity(results),
            team_size: None,               // Would need additional data
            development_methodology: None, // Would need additional data
        };

        self.project_cache = Some(context.clone());
        Ok(context)
    }

    /// Analyze code-level context for findings
    fn analyze_code_context(
        &self,
        results: &AnalysisResults,
    ) -> Result<HashMap<String, CodeContext>> {
        let mut contexts = HashMap::new();

        for finding in &results.findings {
            let context = CodeContext {
                function_context: self.extract_function_context(finding),
                class_context: self.extract_class_context(finding),
                file_role: self.determine_file_role(&finding.file),
                complexity_metrics: self.calculate_complexity_metrics(finding),
                dependencies: self.extract_dependencies(finding),
            };
            contexts.insert(finding.id.clone(), context);
        }

        Ok(contexts)
    }

    /// Analyze historical context (simplified implementation)
    fn analyze_historical_context(&self, _results: &AnalysisResults) -> Result<HistoricalContext> {
        // This would typically involve loading previous analysis results
        // For now, return empty context
        Ok(HistoricalContext {
            previous_results: vec![],
            trends: vec![],
            recurring_issues: vec![],
        })
    }

    /// Analyze environment context
    fn analyze_environment_context(&self, results: &AnalysisResults) -> Result<EnvironmentContext> {
        let security_requirements = self.infer_security_requirements(results);
        let performance_requirements = self.infer_performance_requirements(results);

        Ok(EnvironmentContext {
            deployment_environment: None, // Would need configuration
            performance_requirements,
            security_requirements,
            compliance_requirements: vec![], // Would need additional analysis
        })
    }

    /// Infer project type from file patterns
    fn infer_project_type(&self, findings: &[Finding]) -> Option<String> {
        let mut file_patterns = HashMap::new();

        for finding in findings {
            let path_str = finding.file.to_string_lossy().to_lowercase();

            // Web application patterns
            if path_str.contains("controller")
                || path_str.contains("router")
                || path_str.contains("middleware")
                || path_str.contains("handler")
            {
                *file_patterns.entry("web").or_insert(0) += 1;
            }

            // Mobile application patterns
            if path_str.contains("activity")
                || path_str.contains("fragment")
                || path_str.contains("viewcontroller")
                || path_str.contains("storyboard")
            {
                *file_patterns.entry("mobile").or_insert(0) += 1;
            }

            // Desktop application patterns
            if path_str.contains("window")
                || path_str.contains("dialog")
                || path_str.contains("form")
                || path_str.contains("ui")
            {
                *file_patterns.entry("desktop").or_insert(0) += 1;
            }

            // Library/framework patterns
            if path_str.contains("lib")
                || path_str.contains("api")
                || path_str.contains("core")
                || path_str.contains("util")
            {
                *file_patterns.entry("library").or_insert(0) += 1;
            }
        }

        // Return the most common pattern
        file_patterns
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(pattern, _)| pattern.to_string())
    }

    /// Assess project maturity based on code quality indicators
    fn assess_project_maturity(&self, results: &AnalysisResults) -> Option<String> {
        let total_findings = results.findings.len();
        let high_severity_findings = results
            .findings
            .iter()
            .filter(|f| {
                matches!(
                    f.severity,
                    crate::types::Severity::Critical | crate::types::Severity::High
                )
            })
            .count();

        let high_severity_ratio = if total_findings > 0 {
            high_severity_findings as f64 / total_findings as f64
        } else {
            0.0
        };

        match high_severity_ratio {
            r if r > 0.3 => Some("Early".to_string()),
            r if r > 0.1 => Some("Developing".to_string()),
            r if r > 0.05 => Some("Stable".to_string()),
            _ => Some("Mature".to_string()),
        }
    }

    /// Extract function context from finding (simplified)
    fn extract_function_context(&self, _finding: &Finding) -> Option<String> {
        // Would need AST parsing or pattern matching
        None
    }

    /// Extract class context from finding (simplified)
    fn extract_class_context(&self, _finding: &Finding) -> Option<String> {
        // Would need AST parsing or pattern matching
        None
    }

    /// Determine file role based on path and name
    fn determine_file_role(&self, file_path: &Path) -> Option<String> {
        let path_str = file_path.to_string_lossy().to_lowercase();

        if path_str.contains("test") {
            Some("Test".to_string())
        } else if path_str.contains("config") {
            Some("Configuration".to_string())
        } else if path_str.contains("util") || path_str.contains("helper") {
            Some("Utility".to_string())
        } else if path_str.contains("model") || path_str.contains("entity") {
            Some("Data Model".to_string())
        } else if path_str.contains("controller") || path_str.contains("handler") {
            Some("Business Logic".to_string())
        } else if path_str.contains("view") || path_str.contains("ui") {
            Some("User Interface".to_string())
        } else {
            Some("Application Logic".to_string())
        }
    }

    /// Calculate complexity metrics (simplified)
    fn calculate_complexity_metrics(&self, _finding: &Finding) -> HashMap<String, f32> {
        // Would calculate actual complexity metrics
        HashMap::new()
    }

    /// Extract dependencies (simplified)
    fn extract_dependencies(&self, _finding: &Finding) -> Vec<String> {
        // Would parse import statements
        vec![]
    }

    /// Infer security requirements from findings
    fn infer_security_requirements(&self, results: &AnalysisResults) -> Vec<String> {
        let mut requirements = std::collections::HashSet::new();

        for finding in &results.findings {
            let text = format!("{} {}", finding.message, finding.rule).to_lowercase();

            if text.contains("authentication") {
                requirements.insert("Strong Authentication".to_string());
            }
            if text.contains("authorization") {
                requirements.insert("Access Control".to_string());
            }
            if text.contains("encryption") {
                requirements.insert("Data Encryption".to_string());
            }
            if text.contains("injection") {
                requirements.insert("Input Validation".to_string());
            }
            if text.contains("xss") {
                requirements.insert("Output Sanitization".to_string());
            }
        }

        requirements.into_iter().collect()
    }

    /// Infer performance requirements from findings
    fn infer_performance_requirements(&self, results: &AnalysisResults) -> Vec<String> {
        let mut requirements = std::collections::HashSet::new();

        for finding in &results.findings {
            let text = format!("{} {}", finding.message, finding.rule).to_lowercase();

            if text.contains("performance") || text.contains("slow") {
                requirements.insert("Performance Optimization".to_string());
            }
            if text.contains("memory") {
                requirements.insert("Memory Management".to_string());
            }
            if text.contains("timeout") {
                requirements.insert("Response Time Limits".to_string());
            }
        }

        requirements.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;

    fn create_test_finding(file: &str) -> Finding {
        Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::Medium,
            PathBuf::from(file),
            10,
            "Test finding".to_string(),
        )
    }

    #[test]
    fn test_context_analyzer() -> Result<(), Box<dyn std::error::Error>> {
        let mut analyzer = ContextAnalyzer::new();
        let mut results = crate::types::AnalysisResults::new("test_config".to_string());

        results.add_finding(create_test_finding("src/main.rs"));
        results.add_finding(create_test_finding("src/controller.py"));

        let context = analyzer.analyze_context(&results)?;

        assert!(!context.project_context.languages.is_empty());
        assert!(context.project_context.project_type.is_some());
        Ok(())
    }

    #[test]
    fn test_project_type_inference() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = ContextAnalyzer::new();
        let findings = vec![
            create_test_finding("src/controller/user.rs"),
            create_test_finding("src/router/api.rs"),
        ];

        let project_type = analyzer.infer_project_type(&findings);
        assert_eq!(project_type, Some("web".to_string()));
        Ok(())
    }
}
