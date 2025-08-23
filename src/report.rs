use crate::types::AnalysisResults;
use anyhow::Result;

pub fn generate_markdown(results: &AnalysisResults) -> Result<String> {
    crate::cli::report::generate_markdown(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnalysisResults, Finding, Severity};
    use std::path::PathBuf;

    fn create_test_results() -> AnalysisResults {
        let mut results = AnalysisResults::new("test_config_hash".to_string());
        
        // Add some test findings
        results.add_finding(Finding::new(
            "security",
            "hardcoded_secret",
            Severity::Critical,
            PathBuf::from("src/main.rs"),
            42,
            "Hardcoded API key detected".to_string(),
        ).with_description("This appears to be a hardcoded API key".to_string())
         .with_suggestion("Move the API key to environment variables".to_string()));

        results.add_finding(Finding::new(
            "quality",
            "complex_function",
            Severity::Medium,
            PathBuf::from("src/utils.rs"),
            15,
            "Function has high complexity".to_string(),
        ).with_description("Function complexity exceeds recommended threshold".to_string()));

        results.add_finding(Finding::new(
            "performance",
            "inefficient_loop",
            Severity::Low,
            PathBuf::from("src/parser.rs"),
            88,
            "Inefficient loop detected".to_string(),
        ));

        results.summary.total_files_scanned = 25;
        results.summary.scan_duration_ms = 1500;
        
        results
    }

    #[test]
    fn test_generate_markdown_basic() {
        let results = create_test_results();
        let markdown = generate_markdown(&results);
        
        // Should generate markdown without errors
        assert!(markdown.is_ok());
        let content = markdown.unwrap();
        
        // Should contain basic structure
        assert!(content.contains("# CodeGuardian Analysis Report"));
        assert!(content.contains("## Summary"));
        assert!(content.contains("## Findings"));
    }

    #[test]
    fn test_generate_markdown_contains_findings() {
        let results = create_test_results();
        let markdown = generate_markdown(&results).unwrap();
        
        // Should contain finding details
        assert!(markdown.contains("hardcoded_secret"));
        assert!(markdown.contains("Hardcoded API key detected"));
        assert!(markdown.contains("src/main.rs"));
        assert!(markdown.contains("line 42"));
    }

    #[test]
    fn test_generate_markdown_contains_summary() {
        let results = create_test_results();
        let markdown = generate_markdown(&results).unwrap();
        
        // Should contain summary information
        assert!(markdown.contains("25")); // total files scanned
        assert!(markdown.contains("3")); // total findings
        assert!(markdown.contains("1500")); // scan duration
    }

    #[test]
    fn test_generate_markdown_severity_grouping() {
        let results = create_test_results();
        let markdown = generate_markdown(&results).unwrap();
        
        // Should group by severity
        assert!(markdown.contains("Critical"));
        assert!(markdown.contains("Medium"));
        assert!(markdown.contains("Low"));
    }

    #[test]
    fn test_generate_markdown_empty_results() {
        let results = AnalysisResults::new("test_hash".to_string());
        let markdown = generate_markdown(&results);
        
        // Should handle empty results gracefully
        assert!(markdown.is_ok());
        let content = markdown.unwrap();
        assert!(content.contains("No issues found"));
    }

    #[test]
    fn test_generate_markdown_with_metadata() {
        let mut results = create_test_results();
        
        // Add finding with metadata
        let finding_with_metadata = Finding::new(
            "security",
            "sql_injection",
            Severity::High,
            PathBuf::from("src/db.rs"),
            123,
            "Potential SQL injection".to_string(),
        ).with_metadata("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from(95)));
        
        results.add_finding(finding_with_metadata);
        
        let markdown = generate_markdown(&results).unwrap();
        
        // Should include metadata information
        assert!(markdown.contains("sql_injection"));
        assert!(markdown.contains("src/db.rs"));
    }

    #[test]
    fn test_generate_markdown_performance() {
        // Test with a larger number of findings to ensure performance
        let mut results = AnalysisResults::new("test_hash".to_string());
        
        for i in 0..100 {
            results.add_finding(Finding::new(
                "test",
                "test_rule",
                Severity::Info,
                PathBuf::from(format!("src/file_{}.rs", i)),
                i as u32 + 1,
                format!("Test finding {}", i),
            ));
        }
        
        let start = std::time::Instant::now();
        let markdown = generate_markdown(&results);
        let duration = start.elapsed();
        
        assert!(markdown.is_ok());
        // Should complete within reasonable time (1 second)
        assert!(duration.as_secs() < 1);
    }
}
