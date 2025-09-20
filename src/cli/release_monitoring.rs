//! Release monitoring CLI commands

use crate::release_monitoring::{ReleaseMonitoringConfig, ReleaseMonitoringService};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Release monitoring CLI arguments
#[derive(Parser)]
pub struct ReleaseMonitoringArgs {
    /// GitHub repository (owner/repo)
    #[arg(long)]
    pub repo: String,

    /// Command to execute
    #[command(subcommand)]
    pub subcommand: ReleaseMonitoringSubcommands,
}

/// Release monitoring subcommands
#[derive(Parser)]
pub enum ReleaseMonitoringSubcommands {
    /// Collect and update release metrics
    Collect,

    /// Show current release metrics
    Show,

    /// Show release trends over time
    Trends {
        /// Number of days to look back
        #[arg(long, default_value = "30")]
        days: i64,
    },

    /// Export metrics to JSON file
    Export {
        /// Output file path
        #[arg(long, default_value = "release_metrics_export.json")]
        output: PathBuf,
    },
}

/// Execute release monitoring command
pub async fn execute(args: ReleaseMonitoringArgs) -> Result<()> {
    let config = ReleaseMonitoringConfig {
        repository: args.repo,
        ..Default::default()
    };

    let service = ReleaseMonitoringService::new(config);
    service.load_metrics().await?;

    match args.subcommand {
        ReleaseMonitoringSubcommands::Collect => {
            println!("Collecting release metrics...");
            service.update_metrics().await?;
            println!("Release metrics updated successfully.");
        }
        ReleaseMonitoringSubcommands::Show => {
            if let Some(metrics) = service.get_latest_metrics().await {
                println!("Latest Release Metrics:");
                println!(
                    "  Overall Success Rate: {:.2}%",
                    metrics.overall_success_rate * 100.0
                );
                println!(
                    "  Average Time to Publish: {:.1} hours",
                    metrics.average_time_to_publish
                );
                println!(
                    "  Average Post-Release Issues: {:.1}",
                    metrics.average_post_release_issues
                );
                println!("  Total Downloads: {}", metrics.total_downloads);
                println!(
                    "  Average User Adoption: {:.2}",
                    metrics.average_user_adoption
                );
                println!("  Releases Monitored: {}", metrics.releases.len());

                println!("\nRecent Releases:");
                for release in metrics.releases.iter().take(5) {
                    println!(
                        "  {}: Success={:.0}%, Downloads={}, Issues={}",
                        release.release_tag,
                        release.success_rate * 100.0,
                        release.download_count,
                        release.post_release_issues
                    );
                }
            } else {
                println!("No release metrics available. Run 'collect' first.");
            }
        }
        ReleaseMonitoringSubcommands::Trends { days } => {
            match service.get_release_trends(days).await {
                Ok(trends) => {
                    println!("Release Trends (last {} days):", days);
                    println!("  Data points: {}", trends.timestamps.len());
                    if !trends.success_rates.is_empty() {
                        let avg_success = trends.success_rates.iter().sum::<f64>()
                            / trends.success_rates.len() as f64;
                        let avg_adoption = trends.user_adoption_scores.iter().sum::<f64>()
                            / trends.user_adoption_scores.len() as f64;
                        let avg_issues = trends.post_release_issues.iter().sum::<f64>()
                            / trends.post_release_issues.len() as f64;

                        println!("  Average Success Rate: {:.2}%", avg_success * 100.0);
                        println!("  Average User Adoption: {:.2}", avg_adoption);
                        println!("  Average Post-Release Issues: {:.2}", avg_issues);
                    }
                }
                Err(e) => {
                    println!("Error retrieving trends: {}", e);
                }
            }
        }
        ReleaseMonitoringSubcommands::Export { output } => {
            if let Some(metrics) = service.get_latest_metrics().await {
                let json = serde_json::to_string_pretty(&metrics)?;
                std::fs::write(&output, json)?;
                println!("Metrics exported to {}", output.display());
            } else {
                println!("No metrics available to export.");
            }
        }
    }

    Ok(())
}
