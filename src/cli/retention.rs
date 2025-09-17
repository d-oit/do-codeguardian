use crate::config::Config;
use crate::core::RetentionManager;
use crate::error::Result;
use clap::Parser;

/// Retention policy management command
#[derive(Parser)]
pub struct RetentionArgs {
    /// Run cleanup operation
    #[arg(long)]
    pub cleanup: bool,

    /// Check data integrity only
    #[arg(long)]
    pub check_integrity: bool,

    /// Generate integrity report
    #[arg(long)]
    pub report_integrity: bool,

    /// Force cleanup (ignore minimum keep limits)
    #[arg(long)]
    pub force: bool,

    /// Dry run - show what would be cleaned without actually doing it
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn run_retention(args: RetentionArgs, config: &Config) -> Result<()> {
    let manager = RetentionManager::new(config.retention.clone());

    if args.check_integrity || args.report_integrity {
        println!("🔍 Checking data integrity...");
        let integrity_report = manager.check_integrity().await?;

        println!("📊 Integrity Report:");
        println!("   Total files: {}", integrity_report.total_files);
        println!(
            "   Corrupted files: {}",
            integrity_report.corrupted_files.len()
        );

        if !integrity_report.corrupted_files.is_empty() {
            println!("   Corrupted files:");
            for path in &integrity_report.corrupted_files {
                println!("     - {}", path.display());
            }
        }

        if args.report_integrity {
            manager.generate_integrity_report(&integrity_report).await?;
            println!(
                "✅ Integrity report generated at: {}",
                config.retention.integrity_report_path
            );
        }

        return Ok(());
    }

    if args.cleanup {
        if args.dry_run {
            println!("🔍 Dry run - analyzing what would be cleaned...");
            // For dry run, we would need to modify the manager to not actually delete
            // For now, we'll just show the current state
            let files = manager.collect_result_files()?;
            let total_size: u64 = files.iter().map(|f| f.size).sum();

            println!("📊 Current state:");
            println!("   Total files: {}", files.len());
            println!(
                "   Total size: {:.2} MB",
                total_size as f64 / (1024.0 * 1024.0)
            );

            if let Some(oldest) = files.first() {
                println!(
                    "   Oldest file: {} ({})",
                    oldest.path.display(),
                    oldest.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }

            if let Some(newest) = files.last() {
                println!(
                    "   Newest file: {} ({})",
                    newest.path.display(),
                    newest.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                );
            }

            return Ok(());
        }

        println!("🧹 Running retention policy cleanup...");
        let report = manager.cleanup().await?;

        println!("✅ Cleanup completed:");
        println!("   Files removed by age: {}", report.files_removed_by_age);
        println!("   Files removed by size: {}", report.files_removed_by_size);
        println!(
            "   Total size freed: {:.2} MB",
            report.total_size_freed as f64 / (1024.0 * 1024.0)
        );
        println!("   Integrity issues found: {}", report.integrity_issues);
        println!("   Files repaired: {}", report.files_repaired);
        println!("   Files backed up: {}", report.files_backed_up);

        return Ok(());
    }

    // Default action: show retention status
    println!("📋 Retention Policy Status");
    println!("   Enabled: {}", config.retention.enabled);
    println!("   Results directory: {}", config.retention.results_dir);
    println!("   Max age: {} days", config.retention.max_age_days);
    println!("   Max size: {} MB", config.retention.max_size_mb);
    println!(
        "   Min results to keep: {}",
        config.retention.min_results_to_keep
    );
    println!(
        "   Integrity checks: {}",
        config.retention.enable_integrity_check
    );
    println!("   Auto repair: {}", config.retention.enable_auto_repair);

    let files = manager.collect_result_files()?;
    let total_size: u64 = files.iter().map(|f| f.size).sum();

    println!("\n📊 Current Results:");
    println!("   Total files: {}", files.len());
    println!(
        "   Total size: {:.2} MB",
        total_size as f64 / (1024.0 * 1024.0)
    );

    if !files.is_empty() {
        if let Some(oldest) = files.first() {
            let age_duration = chrono::Utc::now() - oldest.timestamp;
            let age_days = age_duration.num_days();
            println!("   Oldest file age: {} days", age_days);
        }
    }

    Ok(())
}
