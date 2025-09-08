//! Common summary generation utilities

use crate::types::AnalysisResults;
use crate::utils::formatting::{severity_emoji, summary_header};

/// Generate a standard CLI summary for analysis results
pub fn generate_cli_summary(results: &AnalysisResults) -> String {
    let mut summary = String::new();

    summary.push_str(&summary_header());
    summary.push('\n');

    // Basic stats
    summary.push_str(&format!(
        "Files scanned: {}\n",
        results.summary.total_files_scanned
    ));
    summary.push_str(&format!(
        "Total findings: {}\n",
        results.summary.total_findings
    ));
    summary.push_str(&format!(
        "Duration: {}ms\n",
        results.summary.scan_duration_ms
    ));

    // Findings by severity
    if !results.summary.findings_by_severity.is_empty() {
        summary.push_str("\nFindings by severity:\n");
        for (severity, count) in &results.summary.findings_by_severity {
            let emoji = severity_emoji(severity);
            summary.push_str(&format!("  {} {}: {}\n", emoji, severity, count));
        }
    }

    // Findings by analyzer
    if !results.summary.findings_by_analyzer.is_empty() {
        summary.push_str("\nFindings by analyzer:\n");
        for (analyzer, count) in &results.summary.findings_by_analyzer {
            summary.push_str(&format!("  {}: {}\n", analyzer, count));
        }
    }

    summary
}

/// Generate a compact summary for logging
pub fn generate_compact_summary(results: &AnalysisResults) -> String {
    format!(
        "Scanned {} files, found {} issues in {}ms",
        results.summary.total_files_scanned,
        results.summary.total_findings,
        results.summary.scan_duration_ms
    )
}

/// Check if results have any issues
pub fn has_issues(results: &AnalysisResults) -> bool {
    results.summary.total_findings > 0
}

/// Get count of critical/high severity issues
pub fn critical_high_count(results: &AnalysisResults) -> usize {
    results
        .findings
        .iter()
        .filter(|f| {
            matches!(
                f.severity,
                crate::types::Severity::Critical | crate::types::Severity::High
            )
        })
        .count()
}
