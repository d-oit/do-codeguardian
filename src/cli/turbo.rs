use crate::config::Config;
use crate::performance::PerformanceEngine;
use crate::types::{Finding, Severity};
use crate::utils::progress::ProgressReporter;
use anyhow::Result;
use clap::Args;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Args)]
pub struct TurboArgs {
    /// Target directories or files to analyze
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Maximum number of parallel file processors
    #[arg(long, default_value = "0")]
    pub max_parallel: usize,

    /// Memory limit in MB for analysis
    #[arg(long, default_value = "1024")]
    pub memory_limit: usize,

    /// File size threshold for streaming analysis (MB)
    #[arg(long, default_value = "5")]
    pub streaming_threshold: u64,

    /// Maximum number of files to analyze (0 = unlimited)
    #[arg(long, default_value = "0")]
    pub max_files: usize,

    /// Skip files larger than this size (MB)
    #[arg(long, default_value = "100")]
    pub max_file_size: u64,

    /// Enable aggressive optimizations (may reduce accuracy slightly)
    #[arg(long)]
    pub aggressive: bool,

    /// Output format
    #[arg(long, default_value = "human")]
    pub format: String,

    /// Output file for results
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Show detailed performance metrics
    #[arg(long)]
    pub metrics: bool,
}

pub async fn run_turbo_analysis(args: TurboArgs, _config: Config) -> Result<()> {
    let start_time = Instant::now();

    // Initialize progress reporter
    let mut progress = ProgressReporter::new(true); // Always show progress for turbo mode
    progress.start_scan(1000); // Placeholder for total files

    // Determine optimal parallel settings
    let max_parallel = if args.max_parallel == 0 {
        (num_cpus::get() * 2).min(32) // Auto-detect, cap at 32
    } else {
        args.max_parallel
    };

    // Configure performance engine
    let engine = PerformanceEngine::new()
        .with_parallel_limit(max_parallel)
        .with_memory_limit(args.memory_limit)
        .with_streaming_threshold(args.streaming_threshold * 1024 * 1024);

    // Add progress callback
    let _progress_counter = Arc::new(AtomicUsize::new(0));
    let progress_total = Arc::new(AtomicUsize::new(0));
    // Note: Progress callback simplified for now

    // Progress callback simplified for initial implementation

    // Collect files efficiently
    progress.update("🔍 Discovering files...");
    let files = collect_files_optimized(&args).await?;
    let total_files = files.len();

    progress_total.store(total_files, Ordering::Relaxed);
    progress.update(&format!("📁 Found {} files to analyze", total_files));

    if total_files == 0 {
        progress.finish("No files found to analyze");
        return Ok(());
    }

    // Create optimized analyzer function
    let analyzer_fn = create_turbo_analyzer(args.aggressive);

    // Run high-performance batch analysis
    progress.update("⚡ Starting turbo analysis...");
    let file_refs: Vec<_> = files.iter().map(|p| p.as_path()).collect();

    let results = engine.analyze_batch(file_refs, analyzer_fn).await?;

    let total_duration = start_time.elapsed();

    // Display results
    display_turbo_results(&results, &args, total_duration, total_files).await?;

    progress.finish(&format!(
        "🎉 Turbo analysis complete! Processed {} files in {:.2}s",
        total_files,
        total_duration.as_secs_f64()
    ));

    Ok(())
}

async fn collect_files_optimized(args: &TurboArgs) -> Result<Vec<PathBuf>> {
    use crate::performance::LargeCodebaseIterator;

    let mut all_files = Vec::new();

    for path in &args.paths {
        if path.is_file() {
            // Check file size limit
            if let Ok(metadata) = path.metadata() {
                let size_mb = metadata.len() / (1024 * 1024);
                if size_mb <= args.max_file_size {
                    all_files.push(path.clone());
                }
            }
        } else if path.is_dir() {
            let mut iterator =
                LargeCodebaseIterator::new(path).with_size_limit(args.max_file_size * 1024 * 1024);

            if args.max_files > 0 {
                iterator = iterator.with_file_limit(args.max_files);
            }

            let mut dir_files = iterator.collect_files()?;
            all_files.append(&mut dir_files);
        }
    }

    // Filter for supported file types
    all_files.retain(|path| is_analyzable_file(path));

    Ok(all_files)
}

fn is_analyzable_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(
            ext,
            "rs" | "js"
                | "jsx"
                | "ts"
                | "tsx"
                | "py"
                | "java"
                | "cpp"
                | "cc"
                | "cxx"
                | "c"
                | "go"
                | "php"
                | "rb"
                | "json"
                | "toml"
                | "yaml"
                | "yml"
                | "md"
                | "txt"
        )
    } else {
        false
    }
}

fn create_turbo_analyzer(
    aggressive: bool,
) -> impl Fn(&std::path::Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone {
    move |path: &std::path::Path, content: &[u8]| -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);

        // High-speed pattern matching for common issues
        findings.extend(scan_security_patterns(&content_str, path, aggressive));
        findings.extend(scan_quality_patterns(&content_str, path, aggressive));
        findings.extend(scan_performance_patterns(&content_str, path, aggressive));

        Ok(findings)
    }
}

fn scan_security_patterns(content: &str, path: &std::path::Path, aggressive: bool) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Fast regex-free pattern matching for performance
    for (line_num, line) in content.lines().enumerate() {
        let line_lower = line.to_lowercase();

        // API keys and secrets (high-confidence patterns)
        if (line.contains("api_key") || line.contains("secret_key") || line.contains("password"))
            && line.contains("=")
            && (line.contains("\"") || line.contains("'"))
        {
            findings.push(Finding::new(
                "turbo-security",
                "POTENTIAL_SECRET",
                Severity::High,
                path.to_path_buf(),
                line_num as u32 + 1,
                "Potential secret or API key detected".to_string(),
            ));
        }

        // SQL injection patterns
        if line_lower.contains("select") && line_lower.contains("from") && line.contains("+") {
            findings.push(Finding::new(
                "turbo-security",
                "SQL_INJECTION_RISK",
                Severity::Medium,
                path.to_path_buf(),
                line_num as u32 + 1,
                "Potential SQL injection vulnerability".to_string(),
            ));
        }

        // Aggressive mode: additional patterns with higher false positive risk
        if aggressive && (line_lower.contains("eval(") || line_lower.contains("exec(")) {
            findings.push(Finding::new(
                "turbo-security",
                "DANGEROUS_FUNCTION",
                Severity::Medium,
                path.to_path_buf(),
                line_num as u32 + 1,
                "Use of potentially dangerous function".to_string(),
            ));
        }
    }

    findings
}

fn scan_quality_patterns(content: &str, path: &std::path::Path, aggressive: bool) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Check for high cyclomatic complexity
    findings.extend(check_cyclomatic_complexity(content, path));

    // Check for TODO/FIXME comments
    findings.extend(check_todo_comments(content, path));

    // Check for magic numbers in aggressive mode
    if aggressive {
        findings.extend(check_magic_numbers(content, path));
    }

    findings
}

/// Check for high cyclomatic complexity
fn check_cyclomatic_complexity(content: &str, path: &std::path::Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    let complexity_indicators = count_complexity_indicators(content);
    if complexity_indicators > 20 {
        findings.push(Finding::new(
            "turbo-quality",
            "HIGH_COMPLEXITY",
            Severity::Medium,
            path.to_path_buf(),
            1,
            format!(
                "High cyclomatic complexity detected ({} decision points)",
                complexity_indicators
            ),
        ));
    }

    findings
}

/// Count complexity indicators in code
fn count_complexity_indicators(content: &str) -> usize {
    content.matches("if ").count()
        + content.matches("while ").count()
        + content.matches("for ").count()
        + content.matches("switch ").count()
}

/// Check for TODO/FIXME comments
fn check_todo_comments(content: &str, path: &std::path::Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line_upper = line.to_uppercase();
        if line_upper.contains("TODO")
            || line_upper.contains("FIXME")
            || line_upper.contains("HACK")
        {
            let severity = if line_upper.contains("FIXME") || line_upper.contains("HACK") {
                Severity::Medium
            } else {
                Severity::Low
            };

            findings.push(Finding::new(
                "turbo-quality",
                "TODO_COMMENT",
                severity,
                path.to_path_buf(),
                line_num as u32 + 1,
                "TODO/FIXME comment found".to_string(),
            ));
        }
    }

    findings
}

/// Check for magic numbers
fn check_magic_numbers(content: &str, path: &std::path::Path) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if line.chars().any(|c| c.is_ascii_digit()) {
            let numbers: Vec<&str> = line
                .split_whitespace()
                .filter(|word| word.chars().all(|c| c.is_ascii_digit()))
                .filter(|&num| num != "0" && num != "1" && num.len() > 2)
                .collect();

            if !numbers.is_empty() {
                findings.push(Finding::new(
                    "turbo-quality",
                    "MAGIC_NUMBER",
                    Severity::Low,
                    path.to_path_buf(),
                    line_num as u32 + 1,
                    format!("Magic number(s) detected: {}", numbers.join(", ")),
                ));
            }
        }
    }

    findings
}

fn scan_performance_patterns(
    content: &str,
    path: &std::path::Path,
    _aggressive: bool,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Nested loops detection
    let mut in_loop = false;
    let mut loop_depth: i32 = 0;

    for (line_num, line) in content.lines().enumerate() {
        let line_trimmed = line.trim();

        // Simple nested loop detection
        if line_trimmed.contains("for ") || line_trimmed.contains("while ") {
            if in_loop {
                loop_depth += 1;
                if loop_depth >= 2 {
                    findings.push(Finding::new(
                        "turbo-performance",
                        "NESTED_LOOPS",
                        Severity::Medium,
                        path.to_path_buf(),
                        line_num as u32 + 1,
                        "Nested loops detected - potential O(n²) complexity".to_string(),
                    ));
                }
            } else {
                in_loop = true;
                loop_depth = 1;
            }
        }

        // String concatenation in loops
        if in_loop
            && (line.contains(" + ") || line.contains("+="))
            && (line.contains("String") || line.contains("str"))
        {
            findings.push(Finding::new(
                "turbo-performance",
                "STRING_CONCAT_LOOP",
                Severity::Low,
                path.to_path_buf(),
                line_num as u32 + 1,
                "String concatenation in loop - consider using StringBuilder".to_string(),
            ));
        }

        // Reset loop tracking on closing braces
        if line_trimmed.contains("}") && in_loop {
            loop_depth = loop_depth.saturating_sub(1);
            if loop_depth == 0 {
                in_loop = false;
            }
        }
    }

    findings
}

async fn display_turbo_results(
    results: &crate::types::AnalysisResults,
    args: &TurboArgs,
    duration: std::time::Duration,
    total_files: usize,
) -> Result<()> {
    // Report generation simplified

    let files_per_second = total_files as f64 / duration.as_secs_f64();

    if args.metrics {
        println!("\n🚀 Turbo Analysis Metrics:");
        println!("  📁 Files analyzed: {}", total_files);
        println!("  🔍 Total findings: {}", results.findings.len());
        println!("  ⏱️  Duration: {:.2}s", duration.as_secs_f64());
        println!("  ⚡ Speed: {:.1} files/second", files_per_second);
        println!("  🧠 Memory limit: {} MB", args.memory_limit);
        println!("  🔄 Max parallel: {}", args.max_parallel);

        // Findings by severity
        let mut severity_counts = std::collections::HashMap::new();
        for finding in &results.findings {
            *severity_counts.entry(&finding.severity).or_insert(0) += 1;
        }

        println!("\n📊 Findings by severity:");
        for (severity, count) in severity_counts {
            println!("  {:?}: {}", severity, count);
        }
    }

    // Generate report
    match args.format.as_str() {
        "json" => {
            let json_output = serde_json::to_string_pretty(results)?;
            if let Some(output_path) = &args.output {
                tokio::fs::write(output_path, json_output).await?;
                println!("📄 Report saved to: {}", output_path.display());
            } else {
                println!("{}", json_output);
            }
        }
        _ => {
            let human_output = format!("Turbo Analysis Results\n======================\nFiles: {}\nFindings: {}\nDuration: {:.2}s", 
                total_files, results.findings.len(), duration.as_secs_f64());
            if let Some(output_path) = &args.output {
                tokio::fs::write(output_path, human_output).await?;
                println!("📄 Report saved to: {}", output_path.display());
            } else {
                println!("{}", human_output);
            }
        }
    }

    Ok(())
}
