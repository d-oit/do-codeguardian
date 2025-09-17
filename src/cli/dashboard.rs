//! Dashboard CLI commands

use crate::config::Config;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct DashboardArgs {
    /// Start the dashboard server
    #[arg(long)]
    pub start: bool,

    /// Dashboard host address
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Dashboard port
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Enable real-time updates
    #[arg(long)]
    pub real_time: bool,

    /// Generate dashboard report
    #[arg(long)]
    pub generate_report: bool,

    /// View name for report generation
    #[arg(long, default_value = "Overview")]
    pub view: String,

    /// Time range for report (24h, 7d, 30d, 90d)
    #[arg(long, default_value = "7d")]
    pub time_range: String,
}

#[cfg(feature = "dashboard")]
pub async fn run(args: DashboardArgs, config: &Config) -> Result<()> {
    use crate::dashboard::{DashboardConfig, DashboardService};

    if args.start {
        let dashboard_config = DashboardConfig {
            enabled: true,
            host: args.host,
            port: args.port,
            enable_real_time: args.real_time,
            ..Default::default()
        };

        let service = DashboardService::new(dashboard_config.clone());

        tracing::info!(
            "Starting CodeGuardian Dashboard on {}:{}",
            dashboard_config.host,
            dashboard_config.port
        );

        service.start().await?;
    } else if args.generate_report {
        let dashboard_config = DashboardConfig::default();
        let service = DashboardService::new(dashboard_config);

        // Find the requested view
        let view = dashboard_config
            .custom_views
            .iter()
            .find(|v| v.name == args.view)
            .ok_or_else(|| anyhow::anyhow!("View '{}' not found", args.view))?;

        let report = service.generate_report(view)?;

        // Output report as JSON
        println!("{}", serde_json::to_string_pretty(&report)?);

        tracing::info!("Dashboard report generated for view: {}", args.view);
    } else {
        tracing::info!("Dashboard feature available. Use --start to launch the web interface.");
        tracing::info!("Available commands:");
        tracing::info!("  --start                 Start the dashboard web server");
        tracing::info!("  --generate-report       Generate a dashboard report");
        tracing::info!("  --view <name>          Specify view for report generation");
        tracing::info!("  --time-range <range>   Set time range (24h, 7d, 30d, 90d)");
    }

    Ok(())
}

#[cfg(not(feature = "dashboard"))]
pub async fn run(_args: DashboardArgs, _config: &Config) -> Result<()> {
    tracing::warn!("Dashboard feature not enabled. Rebuild with --features dashboard");
    Ok(())
}
