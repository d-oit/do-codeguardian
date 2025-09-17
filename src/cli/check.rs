use crate::cli::CheckArgs;
use crate::config::Config;
use crate::core::GuardianEngine;
use crate::output::storage::organizer::ResultsOrganizer;
use crate::output::storage::{OrganizationStrategy, StorageConfig};
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
use std::collections::HashSet;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Instant;
use tokio::fs;

// Helper functions

async fn apply_config_overrides(
    args: &mut CheckArgs,
    config: &mut Config,
    output_dir: &str,
) -> Result<()> {
    // Override config with CLI options using consolidated utilities
    if let Some(baseline_path) = &args.baseline {
        let canonical_baseline = validate_file_path(baseline_path, "Baseline")?;
        set_baseline_file(config, &canonical_baseline);
    }

    if let Some(threshold) = args.ml_threshold {
        set_ml_threshold(config, threshold)?;
    }

    if let Some(model_path) = &args.ml_model {
        let _canonical_model = validate_file_path(model_path, "ML model")?;
        validate_file_extension(&_canonical_model, "fann", "ML model")?;
    }

    // Override parallel processing setting using consolidated utility
    set_parallel_workers(config, args.parallel);

    // Resolve output path using consolidated utility
    args.out = resolve_output_path(&args.out, "results.json", config);
    ensure_output_directory(&args.out).await?;

    // Override broken files detection settings using consolidated utilities
    if args.detect_broken_files {
        enable_broken_files_feature(config, BrokenFilesFeature::All);
    }

    if args.detect_conflicts {
        enable_broken_files_feature(config, BrokenFilesFeature::MergeConflicts);
    }

    if args.detect_placeholders {
        enable_broken_files_feature(config, BrokenFilesFeature::Placeholders);
    }

    if args.detect_duplicates {
        enable_broken_files_feature(config, BrokenFilesFeature::Duplicates);
    }

    if args.fail_on_conflicts {
        set_fail_on_conflicts(config, true);
    }

    // Use configured output directory if default output path is used
    if args.out == PathBuf::from("results.json") {
        args.out = PathBuf::from(output_dir).join("results.json");
        // Ensure output directory exists
        if let Some(parent) = args.out.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    Ok(())
}

async fn determine_files_to_scan(
    args: &CheckArgs,
    engine: &GuardianEngine,
) -> Result<Vec<PathBuf>> {
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
                    let mut baseline_files = HashSet::new();
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

    Ok(files_to_scan)
}

async fn handle_baseline_comparison(
    baseline_file: &Option<PathBuf>,
) -> Result<Option<crate::types::AnalysisResults>> {
    let mut _baseline_results = None;
    if let Some(baseline_path) = baseline_file.as_ref() {
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
    Ok(_baseline_results)
}

async fn run_analysis_and_filter(
    args: &CheckArgs,
    _config: &Config,
    engine: &mut GuardianEngine,
    files_to_scan: &[PathBuf],
    ml_threshold: Option<f64>,
) -> Result<crate::types::AnalysisResults> {
    // Run analysis
    let results = engine.analyze_files(files_to_scan, args.parallel).await?;

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

    Ok(results)
}

async fn handle_output_and_reporting(
    args: &CheckArgs,
    results: &crate::types::AnalysisResults,
    config: &Config,
    enhanced_results: Option<&crate::output::ai::EnhancedAnalysisResults>,
) -> Result<()> {
    if args.hierarchical_storage {
        // Use hierarchical storage organization
        handle_hierarchical_output(args, results, config, enhanced_results).await
    } else {
        // Use legacy flat file storage for backward compatibility
        handle_flat_file_output(args, results, config, enhanced_results).await
    }
}

async fn handle_hierarchical_output(
    args: &CheckArgs,
    results: &crate::types::AnalysisResults,
    config: &Config,
    enhanced_results: Option<&crate::output::ai::EnhancedAnalysisResults>,
) -> Result<()> {
    // Determine organization strategy
    let strategy = match args.storage_strategy.as_str() {
        "by_date" => OrganizationStrategy::ByDate,
        "by_project" => OrganizationStrategy::ByProject,
        "hybrid" => OrganizationStrategy::Hybrid,
        "hierarchical_time_based" => OrganizationStrategy::HierarchicalTimeBased,
        custom => OrganizationStrategy::Custom(custom.to_string()),
    };

    // Create storage configuration
    let storage_config = StorageConfig {
        base_directory: args.storage_dir.clone(),
        organization_strategy: strategy,
        enable_compression: args.storage_compress,
        max_results_per_directory: 1000,
        enable_indexing: true,
        retention_days: Some(365),
        enable_deduplication: true,
    };

    // Initialize results organizer
    let mut organizer = ResultsOrganizer::new(storage_config)?;

    // Determine project and repository information
    let project_name = args
        .project_name
        .clone()
        .or_else(|| std::env::var("CODEGUARDIAN_PROJECT").ok())
        .unwrap_or_else(|| "unknown_project".to_string());

    let repository_url = args.repository_url.clone().or_else(|| {
        // Try to detect repository from git
        std::process::Command::new("git")
            .args(["config", "--get", "remote.origin.url"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.trim().to_string())
    });

    // Prepare output formats
    let mut outputs = Vec::new();

    // Add JSON output
    let json_content = serde_json::to_string_pretty(results)?;
    outputs.push((
        "json".to_string(),
        crate::output::formatter::OutputResult::new(
            json_content.clone(),
            "json",
            results.config_hash.clone(),
        ),
    ));

    // Add enhanced results if available
    if let Some(enhanced) = enhanced_results {
        let enhanced_json = serde_json::to_string_pretty(enhanced)?;
        outputs.push((
            "enhanced".to_string(),
            crate::output::formatter::OutputResult::new(
                enhanced_json,
                "json",
                results.config_hash.clone(),
            ),
        ));
    }

    // Store results hierarchically
    let result_id = organizer.store_results(
        results,
        &outputs,
        &project_name,
        repository_url.as_deref(),
        vec!["analysis".to_string()], // Default tags
    )?;

    if !args.quiet {
        tracing::info!("Results stored hierarchically with ID: {}", result_id);
        tracing::info!("Storage directory: {}", args.storage_dir.display());
    }

    // For backward compatibility, also save to the specified output path if it's not the default
    if args.out != PathBuf::from("results.json") {
        fs::write(&args.out, json_content).await?;
        if !args.quiet {
            tracing::info!("Results also saved to legacy path: {}", args.out.display());
        }
    }

    // Emit markdown report if requested
    if let Some(md_path) = &args.emit_md {
        let final_md_path = resolve_output_path(md_path, "report.md", config);
        ensure_output_directory(&final_md_path).await?;

        let markdown = crate::cli::report::generate_markdown(results, enhanced_results)?;
        fs::write(&final_md_path, markdown).await?;

        if !args.quiet {
            tracing::info!("Markdown report saved to: {}", final_md_path.display());
        }
    }

    // Emit GitHub issue if requested
    if args.emit_gh {
        if let Some(repo) = &args.repo {
            crate::github::create_or_update_issue(
                results,
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
        println!("{}", generate_cli_summary(results));
    }

    Ok(())
}

async fn handle_flat_file_output(
    args: &CheckArgs,
    results: &crate::types::AnalysisResults,
    config: &Config,
    enhanced_results: Option<&crate::output::ai::EnhancedAnalysisResults>,
) -> Result<()> {
    // Save JSON results (source of truth)
    let json_output = serde_json::to_string_pretty(results)?;
    fs::write(&args.out, json_output).await?;

    if !args.quiet {
        tracing::info!("Results saved to: {}", args.out.display());
    }

    // Save enhanced results if available
    if let Some(enhanced) = enhanced_results {
        let enhanced_path = args.out.with_extension("enhanced.json");
        let enhanced_json = serde_json::to_string_pretty(enhanced)?;
        fs::write(&enhanced_path, enhanced_json).await?;

        if !args.quiet {
            tracing::info!("Enhanced results saved to: {}", enhanced_path.display());
        }
    }

    // Emit markdown report if requested
    if let Some(md_path) = &args.emit_md {
        let final_md_path = resolve_output_path(md_path, "report.md", config);
        ensure_output_directory(&final_md_path).await?;

        let markdown = crate::cli::report::generate_markdown(results, enhanced_results)?;
        fs::write(&final_md_path, markdown).await?;

        if !args.quiet {
            tracing::info!("Markdown report saved to: {}", final_md_path.display());
        }
    }

    // Emit GitHub issue if requested
    if args.emit_gh {
        if let Some(repo) = &args.repo {
            crate::github::create_or_update_issue(
                results,
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
        println!("{}", generate_cli_summary(results));
    }

    Ok(())
}

pub async fn run(mut args: CheckArgs, mut config: Config) -> Result<()> {
    let start_time = Instant::now();
    // Clone output_dir and fail_on_conflicts before moving config
    let output_dir = config.output.directory.clone();
    let fail_on_conflicts_config = config.analyzers.broken_files.conflicts.fail_on_conflicts;

    apply_config_overrides(&mut args, &mut config, &output_dir).await?;

    // Extract ml_threshold and baseline_file after overrides
    let ml_threshold = config.analysis.ml_threshold;
    let baseline_file = config.analysis.baseline_file.clone();

    // Initialize progress reporter (TTY-aware)
    let progress = ProgressReporter::new(!args.quiet && std::io::stdout().is_terminal());

    // Initialize the Guardian engine
    let mut engine =
        GuardianEngine::new_with_ai_override(config.clone(), progress, args.ai_enhance).await?;

    let files_to_scan = determine_files_to_scan(&args, &engine).await?;

    let _baseline_results = handle_baseline_comparison(&baseline_file).await?;

    if files_to_scan.is_empty() {
        tracing::info!("No files to analyze");
        return Ok(());
    }

    let mut results =
        run_analysis_and_filter(&args, &config, &mut engine, &files_to_scan, ml_threshold).await?;

    // Sort findings deterministically
    results.sort_findings();

    // Update timing
    results.summary.scan_duration_ms = start_time.elapsed().as_millis() as u64;

    // Apply AI enhancement if requested
    let enhanced_results = if args.ai_enhance {
        if !args.quiet {
            tracing::info!("🔍 Applying AI enhancement to analysis results...");
        }

        match engine.enhance_results_with_ai(&results).await {
            Ok(Some(enhanced)) => {
                if !args.quiet {
                    tracing::info!("✨ AI enhancement completed successfully");
                    tracing::info!(
                        "   - Generated {} semantic classifications",
                        enhanced.semantic_annotations.classifications.len()
                    );
                    tracing::info!(
                        "   - Detected {} relationships",
                        enhanced.relationships.len()
                    );
                    tracing::info!("   - Created {} insights", enhanced.insights.len());
                }
                Some(enhanced)
            }
            Ok(None) => {
                if !args.quiet {
                    tracing::info!(
                        "AI enhancement not available or failed, continuing with standard results"
                    );
                }
                None
            }
            Err(e) => {
                tracing::warn!(
                    "AI enhancement failed: {}. Continuing with standard results.",
                    e
                );
                None
            }
        }
    } else {
        None
    };

    handle_output_and_reporting(&args, &results, &config, enhanced_results.as_ref()).await?;

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
