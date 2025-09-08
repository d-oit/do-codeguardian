//! Consolidated report generation utilities

use crate::types::Severity;
use crate::utils::formatting::{
    format_file_location, format_finding_id, severity_css_class, severity_emoji,
};
use std::collections::HashMap;

/// Generate severity order for consistent sorting
pub fn get_severity_order() -> Vec<Severity> {
    vec![
        Severity::Critical,
        Severity::High,
        Severity::Medium,
        Severity::Low,
        Severity::Info,
    ]
}

/// Group findings by severity
pub fn group_findings_by_severity(
    findings: &[crate::types::Finding],
) -> HashMap<Severity, Vec<&crate::types::Finding>> {
    let mut grouped = HashMap::new();
    for finding in findings {
        grouped
            .entry(finding.severity.clone())
            .or_insert_with(Vec::new)
            .push(finding);
    }
    grouped
}

/// Generate markdown table row for severity summary
pub fn generate_severity_table_row(severity: &Severity, count: usize) -> String {
    let emoji = severity_emoji(severity);
    format!("| {} | {} | {} |\n", severity, count, emoji)
}

/// Generate HTML finding entry
pub fn generate_html_finding(finding: &crate::types::Finding) -> String {
    let severity_class = severity_css_class(&finding.severity);
    let severity_emoji = severity_emoji(&finding.severity);

    let mut html = format!(
        r#"<div class="finding {}">
<h4>{} {}</h4>
<div class="finding-meta">
<div class="meta-item"><strong>ID:</strong> {}</div>
<div class="meta-item"><strong>File:</strong> {}</div>
<div class="meta-item"><strong>Line:</strong> {}</div>
<div class="meta-item"><strong>Analyzer:</strong> {}</div>
<div class="meta-item"><strong>Rule:</strong> {}</div>
</div>"#,
        severity_class,
        severity_emoji,
        finding.message,
        format_finding_id(&finding.id),
        finding.file.display(),
        finding.line,
        finding.analyzer,
        finding.rule
    );

    if let Some(description) = &finding.description {
        html.push_str(&format!(
            r#"<div class="finding-description">{}</div>"#,
            description
        ));
    }

    if let Some(suggestion) = &finding.suggestion {
        html.push_str(&format!(
            r#"<div class="finding-suggestion"><strong>Suggestion:</strong> {}</div>"#,
            suggestion
        ));
    }

    html.push_str("</div>\n");
    html
}

/// Generate checklist item for GitHub issues
pub fn generate_checklist_item(finding: &crate::types::Finding) -> String {
    let emoji = severity_emoji(&finding.severity);
    format!(
        "- [ ] {} **{}** - `{}` ({})\n",
        emoji,
        finding.message,
        format_file_location(
            &finding.file,
            finding.line as usize,
            finding.column.map(|c| c as usize)
        ),
        format_finding_id(&finding.id)
    )
}

/// Generate simple text finding entry
pub fn generate_text_finding(finding: &crate::types::Finding, index: usize) -> String {
    format!(
        "{}. [{}] {} - {}\n   File: {}\n   Line: {}\n   Analyzer: {} ({})\n   ID: {}\n\n",
        index + 1,
        finding.severity.to_string().to_uppercase(),
        finding.message,
        finding.rule,
        finding.file.display(),
        finding.line,
        finding.analyzer,
        finding.rule,
        format_finding_id(&finding.id)
    )
}

/// Truncate findings list to specified maximum
pub fn truncate_findings<'a>(
    findings: &[&'a crate::types::Finding],
    max_items: usize,
) -> Vec<&'a crate::types::Finding> {
    findings.iter().take(max_items).cloned().collect()
}

/// Generate findings summary by analyzer
pub fn generate_analyzer_summary(findings: &[crate::types::Finding]) -> HashMap<String, usize> {
    let mut summary = HashMap::new();
    for finding in findings {
        *summary.entry(finding.analyzer.clone()).or_insert(0) += 1;
    }
    summary
}
