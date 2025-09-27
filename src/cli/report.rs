use crate::cli::ReportArgs;
use crate::config::Config;
use crate::output::ai::EnhancedAnalysisResults;
use crate::output::storage::organizer::ResultsOrganizer;
use crate::output::storage::{OrganizationStrategy, QueryCriteria, StorageConfig};
use crate::types::AnalysisResults;
use crate::utils::formatting::{format_finding_id, severity_emoji};
use crate::utils::path_utils::{ensure_output_directory, resolve_input_path, resolve_output_path};
use crate::utils::report_utils::{
    generate_severity_table_row, get_severity_order, group_findings_by_severity,
};
use anyhow::Result;
use regex::Regex;
use tokio::fs;

pub async fn run(args: ReportArgs, config: &Config) -> Result<()> {
    let (results, enhanced_results) = if args.hierarchical_storage {
        // Use hierarchical storage retrieval
        load_results_from_hierarchical_storage(&args).await?
    } else {
        // Use legacy flat file loading for backward compatibility
        load_results_from_flat_files(&args).await?
    };

    // Generate report based on format
    let report_content = match args.format {
        crate::cli::ReportFormat::Markdown => {
            generate_markdown(&results, enhanced_results.as_ref())?
        }
        crate::cli::ReportFormat::Html => generate_html(&results, enhanced_results.as_ref())?,
        crate::cli::ReportFormat::Text => generate_text(&results, enhanced_results.as_ref())?,
        crate::cli::ReportFormat::Json => {
            if let Some(enhanced) = &enhanced_results {
                serde_json::to_string_pretty(enhanced)?
            } else {
                serde_json::to_string_pretty(&results)?
            }
        }
        crate::cli::ReportFormat::Yaml => {
            if let Some(enhanced) = &enhanced_results {
                serde_yaml::to_string(enhanced)?
            } else {
                serde_yaml::to_string(&results)?
            }
        }
    };

    // Output to file or stdout
    if let Some(output_path) = &args.md {
        let final_output_path =
            resolve_output_path(output_path, "report.md", config, Some("report"), None);
        ensure_output_directory(&final_output_path).await?;
        fs::write(&final_output_path, report_content).await?;
        tracing::info!("Report saved to: {}", final_output_path.display());
    } else {
        print!("{}", report_content);
    }

    Ok(())
}

async fn load_results_from_hierarchical_storage(
    args: &ReportArgs,
) -> Result<(AnalysisResults, Option<EnhancedAnalysisResults>)> {
    // Create storage configuration
    let storage_config = StorageConfig {
        base_directory: args.storage_dir.clone(),
        organization_strategy: OrganizationStrategy::HierarchicalTimeBased, // Default for retrieval
        enable_compression: true, // Assume compression for retrieval
        max_results_per_directory: 1000,
        enable_indexing: true,
        retention_days: Some(365),
        enable_deduplication: true,
    };

    // Initialize results organizer
    let organizer = ResultsOrganizer::new(storage_config)?;

    let results;
    let enhanced_results;

    if let Some(result_id) = &args.result_id {
        // Retrieve specific result by ID
        if let Some((retrieved_results, retrieved_outputs)) =
            organizer.retrieve_results(result_id)?
        {
            results = retrieved_results;

            // Find enhanced results in outputs
            enhanced_results = retrieved_outputs
                .iter()
                .find(|(format, _)| format == "enhanced")
                .and_then(|(_, content)| serde_json::from_str(content).ok());

            if enhanced_results.is_some() {
                tracing::info!("Loaded AI-enhanced results from hierarchical storage");
            }
        } else {
            return Err(anyhow::anyhow!(
                "Result with ID '{}' not found in hierarchical storage",
                result_id
            ));
        }
    } else {
        // Query results based on criteria
        let mut criteria = QueryCriteria::default();

        if let Some(project) = &args.query_project {
            criteria.project = Some(project.clone());
        }

        if let Some(repository) = &args.query_repository {
            criteria.repository = Some(repository.clone());
        }

        if let Some(date_range) = &args.query_date_range {
            if let Some((start, end)) = parse_date_range(date_range) {
                criteria.date_range = Some((start, end));
            }
        }

        if let Some(tags_str) = &args.query_tags {
            criteria.tags = tags_str.split(',').map(|s| s.trim().to_string()).collect();
        }

        criteria.limit = args.query_limit;

        let query_results = organizer.query_results(&criteria);

        if query_results.is_empty() {
            return Err(anyhow::anyhow!(
                "No results found matching the query criteria"
            ));
        }

        // Use the most recent result
        let metadata = &query_results[0];
        if let Some((retrieved_results, retrieved_outputs)) =
            organizer.retrieve_results(&metadata.id)?
        {
            results = retrieved_results;

            // Find enhanced results in outputs
            enhanced_results = retrieved_outputs
                .iter()
                .find(|(format, _)| format == "enhanced")
                .and_then(|(_, content)| serde_json::from_str(content).ok());

            tracing::info!(
                "Loaded results from hierarchical storage (ID: {})",
                metadata.id
            );
        } else {
            return Err(anyhow::anyhow!(
                "Failed to retrieve result with ID '{}'",
                metadata.id
            ));
        }
    }

    Ok((results, enhanced_results))
}

async fn load_results_from_flat_files(
    args: &ReportArgs,
) -> Result<(AnalysisResults, Option<EnhancedAnalysisResults>)> {
    // Resolve input path using consolidated utility
    let input_path = resolve_input_path(&args.from, "results/results.json", &Config::default());

    // Load results from JSON file
    let json_content = fs::read_to_string(&input_path).await?;
    let results: AnalysisResults = serde_json::from_str(&json_content)?;

    // Load enhanced results if available and requested
    let enhanced_results = if args.ai_enhance {
        let enhanced_path = input_path.with_extension("enhanced.json");
        if enhanced_path.exists() {
            match fs::read_to_string(&enhanced_path).await {
                Ok(content) => match serde_json::from_str::<EnhancedAnalysisResults>(&content) {
                    Ok(enhanced) => {
                        tracing::info!(
                            "Loaded AI-enhanced results from: {}",
                            enhanced_path.display()
                        );
                        Some(enhanced)
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse enhanced results: {}. Using standard results.",
                            e
                        );
                        None
                    }
                },
                Err(_) => {
                    tracing::warn!(
                        "Enhanced results file not found: {}. Using standard results.",
                        enhanced_path.display()
                    );
                    None
                }
            }
        } else {
            tracing::info!(
                "AI enhancement requested but no enhanced results found. Using standard results."
            );
            None
        }
    } else {
        None
    };

    Ok((results, enhanced_results))
}

fn parse_date_range(date_range: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = date_range.split(':').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

pub fn generate_markdown(
    results: &AnalysisResults,
    enhanced_results: Option<&EnhancedAnalysisResults>,
) -> Result<String> {
    let mut md = String::new();
    md.push_str(&generate_header(results));
    md.push_str(&generate_summary(results));
    md.push_str(&generate_detailed_findings(results));

    // Add AI insights if available
    if let Some(enhanced) = enhanced_results {
        md.push_str(&generate_ai_insights(enhanced));
    }

    md.push_str(&generate_footer());
    Ok(md)
}

fn generate_header(results: &AnalysisResults) -> String {
    let mut md = String::new();
    md.push_str("# CodeGuardian Analysis Report\n\n");
    md.push_str(&format!(
        "**Generated:** {}\n",
        results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    md.push_str(&format!(
        "**Tool Version:** {} v{}\n",
        results.tool_metadata.name, results.tool_metadata.version
    ));
    md.push_str(&format!("**Schema Version:** {}\n", results.schema_version));
    md.push_str(&format!(
        "**Config Hash:** `{}`\n\n",
        &results.config_hash[..8]
    ));
    md
}

fn generate_summary(results: &AnalysisResults) -> String {
    let mut md = String::new();
    md.push_str("## 📊 Summary\n\n");
    md.push_str(&format!(
        "- **Files Scanned:** {}\n",
        results.summary.total_files_scanned
    ));
    md.push_str(&format!(
        "- **Total Findings:** {}\n",
        results.summary.total_findings
    ));
    md.push_str(&format!(
        "- **Scan Duration:** {}ms\n\n",
        results.summary.scan_duration_ms
    ));

    // Findings by severity
    if !results.summary.findings_by_severity.is_empty() {
        md.push_str("### Findings by Severity\n\n");
        md.push_str("| Severity | Count | Emoji |\n");
        md.push_str("|----------|-------|-------|\n");

        let severity_order = get_severity_order();
        for severity in &severity_order {
            if let Some(count) = results.summary.findings_by_severity.get(severity) {
                md.push_str(&generate_severity_table_row(severity, *count));
            }
        }
        md.push('\n');
    }

    // Findings by analyzer
    if !results.summary.findings_by_analyzer.is_empty() {
        md.push_str("### Findings by Analyzer\n\n");
        md.push_str("| Analyzer | Count |\n");
        md.push_str("|----------|-------|\n");

        let mut analyzers: Vec<_> = results.summary.findings_by_analyzer.iter().collect();
        analyzers.sort_by_key(|(name, _)| *name);

        for (analyzer, count) in analyzers {
            md.push_str(&format!("| {} | {} |\n", analyzer, count));
        }
        md.push('\n');
    }
    md
}

fn generate_detailed_findings(results: &AnalysisResults) -> String {
    let mut md = String::new();
    if !results.findings.is_empty() {
        md.push_str("## 🔍 Detailed Findings\n\n");

        let findings_by_severity = group_findings_by_severity(&results.findings);
        let severity_order = get_severity_order();

        for severity in &severity_order {
            if let Some(findings) = findings_by_severity.get(severity) {
                let emoji = severity_emoji(severity);
                md.push_str(&format!("### {} {} Issues\n\n", emoji, severity));

                for finding in findings {
                    md.push_str(&format!("#### {}\n\n", finding.message));
                    md.push_str(&format!("- **ID:** {}\n", format_finding_id(&finding.id)));
                    md.push_str(&format!("- **File:** `{}`\n", finding.file.display()));
                    md.push_str(&format!("- **Line:** {}\n", finding.line));
                    if let Some(column) = finding.column {
                        md.push_str(&format!("- **Column:** {}\n", column));
                    }
                    md.push_str(&format!("- **Analyzer:** {}\n", finding.analyzer));
                    md.push_str(&format!("- **Rule:** {}\n", finding.rule));

                    if let Some(description) = &finding.description {
                        md.push_str(&format!("- **Description:** {}\n", description));
                    }

                    if let Some(suggestion) = &finding.suggestion {
                        md.push_str(&format!("- **Suggestion:** {}\n", suggestion));
                    }

                    md.push('\n');
                }
            }
        }
    } else {
        md.push_str(&crate::utils::formatting::no_issues_message());
    }
    md
}

fn generate_ai_insights(enhanced: &EnhancedAnalysisResults) -> String {
    let mut md = String::new();

    if !enhanced.insights.is_empty() {
        md.push_str("## 🤖 AI-Generated Insights\n\n");

        for insight in &enhanced.insights {
            md.push_str(&format!("### {}\n\n", insight.title));
            md.push_str(&format!(
                "**Priority:** {} | **Confidence:** {:.1}%\n\n",
                insight.priority,
                insight.confidence * 100.0
            ));
            md.push_str(&format!("{}\n\n", insight.description));

            if !insight.recommendations.is_empty() {
                md.push_str("**Recommendations:**\n\n");
                for rec in &insight.recommendations {
                    md.push_str(&format!(
                        "- **{}** (Priority: {}): {}\n",
                        rec.action, rec.priority, rec.expected_benefit
                    ));
                    if let Some(details) = &rec.implementation_details {
                        md.push_str(&format!("  - *Details:* {}\n", details));
                    }
                    md.push('\n');
                }
            }

            md.push_str(&format!(
                "**Affected Findings:** {}\n\n",
                insight.affected_findings.len()
            ));
        }
    }

    if !enhanced.relationships.is_empty() {
        md.push_str("## 🔗 Finding Relationships\n\n");

        for rel in &enhanced.relationships {
            md.push_str(&format!(
                "- **{}** ↔ **{}**: {} (Strength: {:.1}%)\n",
                rel.source_id,
                rel.target_id,
                rel.description,
                rel.strength * 100.0
            ));
        }
        md.push('\n');
    }

    md
}

fn generate_footer() -> String {
    let mut md = String::new();
    md.push_str("---\n");
    md.push_str("*Generated by CodeGuardian - Security-first code analysis*\n");
    md
}

pub fn generate_html(
    results: &AnalysisResults,
    enhanced_results: Option<&EnhancedAnalysisResults>,
) -> Result<String> {
    // Generate markdown content
    let markdown_content = generate_markdown(results, enhanced_results)?;

    Ok(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CodeGuardian Analysis Report</title>
    <style>
        * {{
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #24292f;
            background-color: #ffffff;
            margin: 0;
            padding: 20px;
            max-width: 1200px;
            margin: 0 auto;
        }}

        .header {{
            text-align: center;
            margin-bottom: 40px;
            padding: 30px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border-radius: 12px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }}

        .header h1 {{
            margin: 0;
            font-size: 2.5em;
            font-weight: 700;
        }}

        .header p {{
            margin: 10px 0 0 0;
            opacity: 0.9;
            font-size: 1.1em;
        }}

        .summary {{
            background: #f8f9fa;
            padding: 25px;
            border-radius: 8px;
            margin: 30px 0;
            border-left: 5px solid #0366d6;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        }}

        .summary h2 {{
            margin-top: 0;
            color: #0366d6;
            font-size: 1.5em;
        }}

        .summary-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }}

        .metric {{
            text-align: center;
            padding: 15px;
            background: white;
            border-radius: 6px;
            border: 1px solid #e1e4e8;
        }}

        .metric .value {{
            font-size: 2em;
            font-weight: bold;
            color: #0366d6;
        }}

        .metric .label {{
            color: #586069;
            font-size: 0.9em;
            margin-top: 5px;
        }}

        .finding {{
            border-left: 5px solid #0366d6;
            padding: 20px;
            margin: 20px 0;
            background: #f8f9fa;
            border-radius: 6px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        }}

        .finding.critical {{ border-left-color: #d73a49; }}
        .finding.high {{ border-left-color: #f66a0a; }}
        .finding.medium {{ border-left-color: #ffd33d; }}
        .finding.low {{ border-left-color: #0366d6; }}
        .finding.info {{ border-left-color: #6f42c1; }}

        .finding h3 {{
            margin-top: 0;
            color: #24292f;
            font-size: 1.2em;
        }}

        .finding-meta {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 10px;
            margin: 15px 0;
            font-size: 0.9em;
        }}

        .meta-item {{
            background: white;
            padding: 8px 12px;
            border-radius: 4px;
            border: 1px solid #e1e4e8;
        }}

        .meta-item strong {{
            color: #586069;
        }}

        .finding-description {{
            margin: 15px 0;
            line-height: 1.5;
        }}

        .finding-suggestion {{
            background: #fff3cd;
            border: 1px solid #ffeaa7;
            border-radius: 4px;
            padding: 12px;
            margin: 15px 0;
        }}

        .finding-suggestion strong {{
            color: #856404;
        }}

        h1, h2, h3, h4 {{
            color: #24292f;
            margin-top: 40px;
            margin-bottom: 20px;
        }}

        h1 {{ font-size: 2em; border-bottom: 2px solid #e1e4e8; padding-bottom: 10px; }}
        h2 {{ font-size: 1.5em; border-bottom: 1px solid #e1e4e8; padding-bottom: 8px; }}
        h3 {{ font-size: 1.2em; }}

        p {{
            margin: 15px 0;
        }}

        code {{
            background: #f6f8fa;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
            font-size: 0.9em;
            border: 1px solid #e1e4e8;
        }}

        pre {{
            background: #f6f8fa;
            padding: 16px;
            border-radius: 6px;
            overflow-x: auto;
            border: 1px solid #e1e4e8;
        }}

        pre code {{
            background: none;
            padding: 0;
            border: none;
        }}

        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 20px 0;
            background: white;
            border-radius: 6px;
            overflow: hidden;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        }}

        th, td {{
            border: 1px solid #e1e4e8;
            padding: 12px 16px;
            text-align: left;
        }}

        th {{
            background: #f6f8fa;
            font-weight: 600;
            color: #24292f;
        }}

        tr:nth-child(even) {{
            background: #f8f9fa;
        }}

        ul {{
            padding-left: 20px;
        }}

        li {{
            margin: 8px 0;
        }}

        .no-issues {{
            text-align: center;
            padding: 40px;
            background: #f0f8ff;
            border: 2px solid #0366d6;
            border-radius: 8px;
            margin: 30px 0;
        }}

        .no-issues h2 {{
            color: #0366d6;
            margin-bottom: 10px;
        }}

        .footer {{
            text-align: center;
            margin-top: 50px;
            padding: 20px;
            color: #586069;
            border-top: 1px solid #e1e4e8;
        }}

        @media (max-width: 768px) {{
            body {{
                padding: 10px;
            }}

            .header {{
                padding: 20px;
            }}

            .header h1 {{
                font-size: 2em;
            }}

            .summary-grid {{
                grid-template-columns: 1fr;
            }}

            .finding-meta {{
                grid-template-columns: 1fr;
            }}

            table {{
                font-size: 0.9em;
            }}

            th, td {{
                padding: 8px 12px;
            }}
        }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🔒 CodeGuardian Analysis Report</h1>
        <p>Security-first code analysis results</p>
    </div>
    {}
    <div class="footer">
        <p>Generated by <strong>CodeGuardian</strong> - Advanced security and code quality analysis tool</p>
        <p>Report generated on {} | Tool Version: {} v{}</p>
    </div>
</body>
</html>"#,
        markdown_to_html(&markdown_content)?,
        results.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        results.tool_metadata.name,
        results.tool_metadata.version
    ))
}

pub fn generate_text(
    results: &AnalysisResults,
    enhanced_results: Option<&EnhancedAnalysisResults>,
) -> Result<String> {
    let mut text = String::new();

    text.push_str("CODEGUARDIAN ANALYSIS REPORT\n");
    text.push_str("============================\n\n");

    text.push_str(&format!(
        "Generated: {}\n",
        results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
    ));
    text.push_str(&format!(
        "Tool: {} v{}\n",
        results.tool_metadata.name, results.tool_metadata.version
    ));
    text.push_str(&format!("Schema: {}\n", results.schema_version));
    text.push_str(&format!("Config Hash: {}\n\n", &results.config_hash[..8]));

    text.push_str("SUMMARY\n");
    text.push_str("-------\n");
    text.push_str(&format!(
        "Files Scanned: {}\n",
        results.summary.total_files_scanned
    ));
    text.push_str(&format!(
        "Total Findings: {}\n",
        results.summary.total_findings
    ));
    text.push_str(&format!(
        "Scan Duration: {}ms\n\n",
        results.summary.scan_duration_ms
    ));

    if !results.findings.is_empty() {
        text.push_str("FINDINGS\n");
        text.push_str("--------\n\n");

        for (i, finding) in results.findings.iter().enumerate() {
            text.push_str(&format!(
                "{}. [{}] {}\n",
                i + 1,
                finding.severity.to_string().to_uppercase(),
                finding.message
            ));
            text.push_str(&format!("   File: {}\n", finding.file.display()));
            text.push_str(&format!("   Line: {}\n", finding.line));
            text.push_str(&format!(
                "   Analyzer: {} ({})\n",
                finding.analyzer, finding.rule
            ));
            text.push_str(&format!("   ID: {}\n\n", finding.id));
        }
    } else {
        text.push_str("No issues found.\n\n");
    }

    // Add AI insights if available
    if let Some(enhanced) = enhanced_results {
        if !enhanced.insights.is_empty() {
            text.push_str("AI-GENERATED INSIGHTS\n");
            text.push_str("---------------------\n\n");

            for insight in &enhanced.insights {
                text.push_str(&format!("INSIGHT: {}\n", insight.title));
                text.push_str(&format!(
                    "Priority: {} | Confidence: {:.1}%\n",
                    insight.priority,
                    insight.confidence * 100.0
                ));
                text.push_str(&format!("Description: {}\n", insight.description));

                if !insight.recommendations.is_empty() {
                    text.push_str("Recommendations:\n");
                    for rec in &insight.recommendations {
                        text.push_str(&format!(
                            "  - {} (Priority: {})\n",
                            rec.action, rec.priority
                        ));
                        text.push_str(&format!("    Expected Benefit: {}\n", rec.expected_benefit));
                    }
                }

                text.push_str(&format!(
                    "Affected Findings: {}\n\n",
                    insight.affected_findings.len()
                ));
            }
        }

        if !enhanced.relationships.is_empty() {
            text.push_str("FINDING RELATIONSHIPS\n");
            text.push_str("---------------------\n\n");

            for rel in &enhanced.relationships {
                text.push_str(&format!(
                    "- {} <-> {}: {} (Strength: {:.1}%)\n",
                    rel.source_id,
                    rel.target_id,
                    rel.description,
                    rel.strength * 100.0
                ));
            }
            text.push('\n');
        }
    }

    Ok(text)
}

/// Converts basic markdown to HTML with proper tag structure
fn markdown_to_html(markdown: &str) -> anyhow::Result<String> {
    use regex::Regex;

    let mut html = String::new();
    let lines: Vec<&str> = markdown.lines().collect();

    // Regex patterns for markdown elements
    let header_regex =
        Regex::new(r"^(#{1,6})\s+(.+)$").map_err(|e| anyhow::anyhow!("Failed to compile header regex: {}", e))?;
    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").map_err(|e| anyhow::anyhow!("Failed to compile bold regex: {}", e))?;
    let italic_regex = Regex::new(r"\*(.*?)\*").map_err(|e| anyhow::anyhow!("Failed to compile italic regex: {}", e))?;
    let code_regex = Regex::new(r"`([^`]+)`").map_err(|e| anyhow::anyhow!("Failed to compile code regex: {}", e))?;
    let list_regex = Regex::new(r"^\s*-\s+(.+)$").map_err(|e| anyhow::anyhow!("Failed to compile list regex: {}", e))?;
    let table_row_regex =
        Regex::new(r"^\|(.+)\|$").map_err(|e| anyhow::anyhow!("Failed to compile table regex: {}", e))?;
    let severity_header_regex =
        Regex::new(r"###\s+(\u{1F534}|\u{1F7E0}|\u{1F7E1}|\u{1F535}|\u{2139})\s+(\w+)\s+Issues")
            .map_err(|e| anyhow::anyhow!("Failed to compile severity regex: {}", e))?;

    let mut in_list = false;
    let mut in_table = false;
    let mut current_severity_class = String::new();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            if in_table {
                html.push_str("</table>\n");
                in_table = false;
            }
            html.push_str("<br>\n");
            continue;
        }

        // Handle severity headers
        if let Some(caps) = severity_header_regex.captures(trimmed) {
            let emoji = &caps[1];
            let _severity = &caps[2];
            let severity_class = match emoji {
                "🔴" => "critical",
                "🟠" => "high",
                "🟡" => "medium",
                "🔵" => "low",
                "ℹ️" => "info",
                _ => "info",
            };
            current_severity_class = severity_class.to_string();
            let processed_content =
                process_inline_elements(&caps[0], &bold_regex, &italic_regex, &code_regex);
            html.push_str(&format!("<h3>{}</h3>\n", processed_content));
            continue;
        }

        // Handle headers
        if let Some(caps) = header_regex.captures(trimmed) {
            let level = caps[1].len();
            let content = &caps[2];
            let processed_content =
                process_inline_elements(content, &bold_regex, &italic_regex, &code_regex);

            // Special handling for summary section
            if content.contains("Summary") {
                html.push_str(&format!(
                    "<div class=\"summary\"><h{}>{}</h{}>\n",
                    level, processed_content, level
                ));
                continue;
            }

            html.push_str(&format!("<h{}>{}</h{}>\n", level, processed_content, level));
            continue;
        }

        // Handle table rows
        if let Some(caps) = table_row_regex.captures(trimmed) {
            if !in_table {
                html.push_str("<table>\n");
                in_table = true;
            }
            let cells: Vec<&str> = caps[1].split('|').map(|s| s.trim()).collect();
            html.push_str("<tr>\n");
            for cell in cells {
                let processed_cell =
                    process_inline_elements(cell, &bold_regex, &italic_regex, &code_regex);
                html.push_str(&format!("<td>{}</td>\n", processed_cell));
            }
            html.push_str("</tr>\n");
            continue;
        }

        // Handle list items
        if let Some(caps) = list_regex.captures(trimmed) {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            let content = &caps[1];
            let processed_content =
                process_inline_elements(content, &bold_regex, &italic_regex, &code_regex);
            html.push_str(&format!("<li>{}</li>\n", processed_content));
            continue;
        }

        // Handle finding sections (detect by ID pattern)
        if trimmed.starts_with("#### ") && trimmed.contains("**ID:**") {
            let processed_line =
                process_inline_elements(trimmed, &bold_regex, &italic_regex, &code_regex);
            html.push_str(&format!(
                "<div class=\"finding {}\">\n<h4>{}</h4>\n",
                current_severity_class, processed_line
            ));
            continue;
        }

        // Handle finding metadata
        if trimmed.starts_with("- **")
            && (trimmed.contains("File:")
                || trimmed.contains("Line:")
                || trimmed.contains("Analyzer:")
                || trimmed.contains("Rule:"))
        {
            let processed_line =
                process_inline_elements(trimmed, &bold_regex, &italic_regex, &code_regex);
            html.push_str(&format!(
                "<div class=\"finding-meta\">{}</div>\n",
                processed_line
            ));
            continue;
        }

        // Handle description and suggestion
        if trimmed.starts_with("- **Description:**") || trimmed.starts_with("- **Suggestion:**") {
            let processed_line =
                process_inline_elements(trimmed, &bold_regex, &italic_regex, &code_regex);
            if trimmed.contains("Suggestion") {
                html.push_str(&format!(
                    "<div class=\"finding-suggestion\">{}</div>\n",
                    processed_line
                ));
            } else {
                html.push_str(&format!(
                    "<div class=\"finding-description\">{}</div>\n",
                    processed_line
                ));
            }
            continue;
        }

        // Handle paragraphs
        if !in_list && !in_table {
            let processed_line =
                process_inline_elements(trimmed, &bold_regex, &italic_regex, &code_regex);

            // Check for summary content
            if trimmed.contains("Files Scanned:")
                || trimmed.contains("Total Findings:")
                || trimmed.contains("Scan Duration:")
            {
                if !html.contains("<div class=\"summary-grid\">") {
                    html.push_str("<div class=\"summary-grid\">\n");
                }
                let parts: Vec<&str> = trimmed.split(':').collect();
                if parts.len() == 2 {
                    let label = parts[0].trim();
                    let value = parts[1].trim();
                    html.push_str(&format!("<div class=\"metric\"><div class=\"value\">{}</div><div class=\"label\">{}</div></div>\n", value, label));
                }
                continue;
            }

            // Check for "No Issues Found"
            if trimmed.contains("No Issues Found") {
                html.push_str("</div>\n<div class=\"no-issues\">\n");
                html.push_str(&format!("<h2>{}</h2>\n", processed_line));
                continue;
            }

            html.push_str(&format!("<p>{}</p>\n", processed_line));
        }
    }

    // Close any open tags
    if in_list {
        html.push_str("</ul>\n");
    }
    if in_table {
        html.push_str("</table>\n");
    }

    // Close summary div if open
    if html.contains("<div class=\"summary\">") && !html.contains("</div>") {
        html.push_str("</div>\n");
    }

    Ok(html)
}

/// Processes inline markdown elements like bold, italic, and code
fn process_inline_elements(
    text: &str,
    bold_regex: &Regex,
    italic_regex: &Regex,
    code_regex: &Regex,
) -> String {
    let mut result = text.to_string();

    // Replace code first (to avoid conflicts)
    result = code_regex
        .replace_all(&result, "<code>$1</code>")
        .to_string();

    // Replace bold
    result = bold_regex
        .replace_all(&result, "<strong>$1</strong>")
        .to_string();

    // Replace italic
    result = italic_regex.replace_all(&result, "<em>$1</em>").to_string();

    result
}
