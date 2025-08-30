use crate::cli::CheckArgs;
use crate::config::Config;
use crate::core::GuardianEngine;
use crate::types::AnalysisResults;
use crate::utils::progress::ProgressReporter;
use anyhow::Result;
use serde_json;
use std::time::Instant;
use tokio::fs;

pub async fn run(args: CheckArgs, mut config: Config) -> Result<()> {
    let start_time = Instant::now();

    // Override config with CLI options
    if let Some(baseline_path) = &args.baseline {
        config.analysis.baseline_path = Some(baseline_path.clone());
    }
    if let Some(threshold) = args.ml_threshold {
        config.analysis.ml_threshold = Some(threshold);
    }
    config.analysis.streaming = args.streaming;

    // Initialize progress reporter (TTY-aware)
    let progress = ProgressReporter::new(!args.quiet && atty::is(atty::Stream::Stdout));

    // Initialize the Guardian engine
    let mut engine = GuardianEngine::new(config, progress).await?;

    // Determine files to scan
    let mut files_to_scan = if let Some(diff_spec) = &args.diff {
        engine.get_diff_files(diff_spec).await?
    } else if args.only_changed {
        engine.get_staged_files().await?
    } else {
        engine.get_all_files(&args.paths).await?
    };

    // Filter for only new files if requested
    if args.only_new && args.baseline.is_some() {
        if let Some(baseline_path) = &args.baseline {
            if let Ok(baseline_content) = tokio::fs::read_to_string(baseline_path).await {
                if let Ok(baseline_results) =
                    serde_json::from_str::<crate::types::AnalysisResults>(&baseline_content)
                {
                    let baseline_files: std::collections::HashSet<_> = baseline_results
                        .findings
                        .iter()
                        .map(|f| f.file.canonicalize().ok())
                        .collect();
                    files_to_scan.retain(|f| !baseline_files.contains(&f.canonicalize().ok()));
                }
            }
        }
    }

    if files_to_scan.is_empty() {
        tracing::info!("No files to analyze");
        return Ok(());
    }

    // Run analysis
    let mut results = engine.analyze_files(&files_to_scan, args.parallel).await?;

    // Sort findings deterministically
    results.sort_findings();

    // Update timing
    results.summary.scan_duration_ms = start_time.elapsed().as_millis() as u64;

    // Save JSON results (source of truth)
    let json_output = serde_json::to_string_pretty(&results)?;
    fs::write(&args.out, json_output).await?;

    if !args.quiet {
        tracing::info!("Results saved to: {}", args.out.display());
    }

    // Emit markdown report if requested
    if let Some(md_path) = &args.emit_md {
        let markdown = crate::report::generate_markdown(&results)?;
        fs::write(md_path, markdown).await?;

        if !args.quiet {
            tracing::info!("Markdown report saved to: {}", md_path.display());
        }
    }

    // Emit GitHub issue if requested
    if args.emit_gh {
        if let Some(repo) = &args.repo {
            crate::github::create_or_update_issue(
                &results,
                repo,
                &args.gh_mode,
                &args.labels,
                false, // dry_run
            )
            .await?;
        } else {
            tracing::warn!("--emit-gh specified but no --repo provided");
        }
    }

    // Print summary to stdout if not quiet
    if !args.quiet {
        print_summary(&results);
    }

    // Determine exit code
    if args.fail_on_issues && results.has_issues() {
        std::process::exit(2);
    }

    Ok(())
}

fn print_summary(results: &AnalysisResults) {
    tracing::info!("📊 Analysis Summary");
    tracing::info!("==================");
    tracing::info!("Files scanned: {}", results.summary.total_files_scanned);
    tracing::info!("Total findings: {}", results.summary.total_findings);
    tracing::info!("Duration: {}ms", results.summary.scan_duration_ms);

    if !results.summary.findings_by_severity.is_empty() {
        tracing::info!("Findings by severity:");
        for (severity, count) in &results.summary.findings_by_severity {
            let emoji = match severity {
                crate::types::Severity::Critical => "🔴",
                crate::types::Severity::High => "🟠",
                crate::types::Severity::Medium => "🟡",
                crate::types::Severity::Low => "🔵",
                crate::types::Severity::Info => "ℹ️",
            };
            tracing::info!("  {} {}: {}", emoji, severity, count);
        }
    }

    if !results.summary.findings_by_analyzer.is_empty() {
        tracing::info!("Findings by analyzer:");
        for (analyzer, count) in &results.summary.findings_by_analyzer {
            tracing::info!("  {}: {}", analyzer, count);
        }
    }
}
