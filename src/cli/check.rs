use crate::cli::CheckArgs;
use crate::config::Config;
use crate::core::GuardianEngine;
use crate::utils::config_utils::{
    enable_broken_files_feature, set_baseline_file, set_fail_on_conflicts, set_ml_threshold,
    set_parallel_workers, BrokenFilesFeature,
};
use crate::utils::path_utils::{
    ensure_output_directory, resolve_output_path, validate_file_extension, validate_file_path,
};
use crate::utils::progress::ProgressReporter;
use crate::utils::summary_utils::{generate_cli_summary, has_issues};
use anyhow::Result;
use serde_json;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Instant;
use tokio::fs;

pub async fn run(mut args: CheckArgs, mut config: Config) -> Result<()> {
    let start_time = Instant::now();

    // Clone output_dir and fail_on_conflicts before moving config
    let output_dir = config.output.directory.clone();
    let fail_on_conflicts_config = config.analyzers.broken_files.conflicts.fail_on_conflicts;

    // Override config with CLI options using consolidated utilities
    if let Some(baseline_path) = &args.baseline {
        let canonical_baseline = validate_file_path(baseline_path, "Baseline")?;
        set_baseline_file(&mut config, &canonical_baseline);
    }

    if let Some(threshold) = args.ml_threshold {
        set_ml_threshold(&mut config, threshold)?;
    }

    if let Some(model_path) = &args.ml_model {
        let _canonical_model = validate_file_path(model_path, "ML model")?;
        validate_file_extension(&_canonical_model, "fann", "ML model")?;
    }

    // Extract ml_threshold and baseline_file after overrides
    let ml_threshold = config.analysis.ml_threshold;
    let baseline_file = config.analysis.baseline_file.clone();

    // Override parallel processing setting using consolidated utility
    set_parallel_workers(&mut config, args.parallel);

    // Resolve output path using consolidated utility
    args.out = resolve_output_path(&args.out, "results.json", &config);
    ensure_output_directory(&args.out).await?;

    // Override broken files detection settings using consolidated utilities
    if args.detect_broken_files {
        enable_broken_files_feature(&mut config, BrokenFilesFeature::All);
    }

    if args.detect_conflicts {
        enable_broken_files_feature(&mut config, BrokenFilesFeature::MergeConflicts);
    }

    if args.detect_placeholders {
        enable_broken_files_feature(&mut config, BrokenFilesFeature::Placeholders);
    }

    if args.detect_duplicates {
        enable_broken_files_feature(&mut config, BrokenFilesFeature::Duplicates);
    }

    if args.fail_on_conflicts {
        set_fail_on_conflicts(&mut config, true);
    }

    // Use configured output directory if default output path is used
    if args.out == PathBuf::from("results.json") {
        args.out = PathBuf::from(&output_dir).join("results.json");
        // Ensure output directory exists
        if let Some(parent) = args.out.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    // Initialize progress reporter (TTY-aware)
    let progress = ProgressReporter::new(!args.quiet && std::io::stdout().is_terminal());

    // Initialize the Guardian engine
    let mut engine = GuardianEngine::new(config.clone(), progress).await?;

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
                    let mut baseline_files = std::collections::HashSet::new();
                    for finding in &baseline_results.findings {
                        if let Ok(canonical) = finding.file.canonicalize() {
                            baseline_files.insert(canonical);
                        }
                    }
                    files_to_scan.retain(|f| {
                        if let Ok(canonical) = f.canonicalize() {
                            !baseline_files.contains(&canonical)
                        } else {
                            true // Keep files that can't be canonicalized
                        }
                    });
                }
            }
        }
    }

    // Handle baseline comparison for drift detection
    let mut _baseline_results = None;
    if let Some(baseline_path) = &baseline_file {
        if let Ok(baseline_content) = tokio::fs::read_to_string(baseline_path).await {
            if let Ok(results) =
                serde_json::from_str::<crate::types::AnalysisResults>(&baseline_content)
            {
                _baseline_results = Some(results);
                tracing::info!("Loaded baseline from: {}", baseline_path.display());
            } else {
                tracing::warn!("Failed to parse baseline file: {}", baseline_path.display());
            }
        } else {
            tracing::warn!("Failed to read baseline file: {}", baseline_path.display());
        }
    }

    if files_to_scan.is_empty() {
        tracing::info!("No files to analyze");
        return Ok(());
    }

    // Run analysis
    let mut results = engine.analyze_files(&files_to_scan, args.parallel).await?;

    // Apply ML-based false positive filtering if threshold is configured
    #[allow(unused_variables)]
    if let Some(threshold) = ml_threshold {
        #[cfg(feature = "ml")]
        {
            let model_path_str = args
                .ml_model
                .as_ref()
                .map(|p| p.to_string_lossy().to_string());
            let mut ml_classifier = crate::ml::MLClassifier::new(model_path_str.as_deref());
            results.findings = ml_classifier
                .filter_findings(results.findings, threshold as f32)
                .await?;
            if args.ml_model.is_some() {
                tracing::info!(
                    "Applied ML filtering with custom model and threshold: {}",
                    threshold
                );
            } else {
                tracing::info!(
                    "Applied ML filtering with default model and threshold: {}",
                    threshold
                );
            }
        }

        #[cfg(not(feature = "ml"))]
        {
            tracing::warn!("ML threshold configured but ML feature is not enabled. Use --features ml to enable ML-based filtering");
        }
    }

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
        let final_md_path = resolve_output_path(md_path, "report.md", &config);
        ensure_output_directory(&final_md_path).await?;

        let markdown = crate::report::generate_markdown(&results)?;
        fs::write(&final_md_path, markdown).await?;

        if !args.quiet {
            tracing::info!("Markdown report saved to: {}", final_md_path.display());
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
        println!("{}", generate_cli_summary(&results));
    }

    // Check for conflicts if fail_on_conflicts is enabled
    if args.fail_on_conflicts || fail_on_conflicts_config {
        let has_conflicts = results.findings.iter().any(|f| {
            f.analyzer == "git_conflict" && matches!(f.severity, crate::types::Severity::Critical)
        });

        if has_conflicts {
            tracing::error!("🚨 Git merge conflicts detected! Failing as requested.");
            std::process::exit(3); // Different exit code for conflicts
        }
    }

    // Determine exit code
    if args.fail_on_issues && has_issues(&results) {
        std::process::exit(2);
    }

    Ok(())
}
