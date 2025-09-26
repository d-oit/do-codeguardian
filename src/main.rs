//! # CodeGuardian CLI
//!
//! The main entry point for the CodeGuardian command-line tool.
//! This tool provides security analysis and enhanced git operations.
//!
//! ## Usage
//!
//! ```bash
//! codeguardian check <files...>
//! codeguardian git-commit [--message <msg>]
//! ```

use clap::Parser;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use anyhow::Result;
use do_codeguardian::cli::check;
use do_codeguardian::cli::gh_issue;
use do_codeguardian::cli::init;
#[cfg(feature = "ml")]
use do_codeguardian::cli::metrics;
#[cfg(feature = "release-monitoring")]
use do_codeguardian::cli::release_monitoring;
use do_codeguardian::cli::report;
use do_codeguardian::cli::retention;
use do_codeguardian::cli::{Cli, Commands};
use do_codeguardian::commands::git_commit;
use do_codeguardian::commands::git_commit_push;
use do_codeguardian::commands::turbo;
use do_codeguardian::commands::update_docs;
use do_codeguardian::{CodeGuardianError, Config};

// CLI structure is now imported from cli.rs

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine log level based on verbosity
    let log_level = if cli.verbose > 0 {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    // Initialize logging
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| CodeGuardianError::LoggingSetup)?;

    info!("CodeGuardian CLI starting");

    // Get project root directory
    let project_root = std::env::current_dir().map_err(CodeGuardianError::Io)?;

    // Load configuration
    let config_path = &cli.config;
    let config = Config::from_file(config_path).unwrap_or_else(|e| {
        if !cli.quiet {
            tracing::warn!(
                "Configuration file error at {}: {}. Using defaults",
                config_path.display(),
                e
            );
        }
        Config::default()
    });

    // Execute command
    match cli.command {
        Commands::Check(mut args) => {
            // Override AI enhancement from global flag
            if cli.ai_enhance {
                args.ai_enhance = true;
            }
            check::run(args, config).await?;
        }
        Commands::Report(args) => {
            report::run(args, &config).await?;
        }
        Commands::GhIssue(args) => {
            gh_issue::run(args, &config).await?;
        }
        Commands::Init(args) => {
            init::run(args, &config).await?;
        }
        Commands::GitCommit(args) => {
            git_commit::execute_git_commit(args.message.as_deref(), &config).await?;
            if !cli.quiet {
                tracing::info!("Commit successful");
            }
        }
        Commands::GitCommitPush(args) => {
            git_commit_push::execute_git_commit_push(args, &config).await?;
            if !cli.quiet {
                tracing::info!("Commit and push successful");
            }
        }
        Commands::Turbo(args) => {
            turbo::execute_turbo(args, config).await?;
        }
        Commands::UpdateDocs(args) => {
            update_docs::execute_update_docs(&config, &args, &project_root).await?;
            if !cli.quiet {
                tracing::info!("Documentation update successful");
            }
        }
        #[cfg(feature = "ml")]
        Commands::Train(args) => {
            do_codeguardian::cli::train::run(args, &config).await?;
        }
        #[cfg(feature = "ml")]
        Commands::TrainingData(args) => {
            do_codeguardian::cli::training_data::run(args, &config).await?;
        }

        #[cfg(feature = "ml")]
        Commands::Metrics(args) => {
            metrics::run(args)?;
        }
        #[cfg(feature = "dashboard")]
        Commands::Dashboard(args) => {
            do_codeguardian::cli::dashboard::run(args, &config).await?;
        }
        Commands::Remediation(args) => {
            do_codeguardian::cli::remediation::run(args, &config).await?;
        }
        Commands::Integrations(args) => {
            do_codeguardian::cli::integrations::run(args, &config).await?;
        }
        Commands::Bulk(args) => {
            do_codeguardian::cli::bulk::run(args, &config).await?;
        }
        Commands::Retention(args) => {
            retention::run_retention(args, &config).await?;
        }
        Commands::TuneThresholds(args) => {
            do_codeguardian::cli::threshold_tuning::execute_threshold_tuning(args, &config).await?;
        }
        #[cfg(feature = "release-monitoring")]
        Commands::ReleaseMonitoring(args) => {
            do_codeguardian::cli::release_monitoring::execute(args).await?;
        }
        #[cfg(feature = "ml")]
        Commands::FeatureEngineering(args) => {
            do_codeguardian::cli::feature_engineering::run_feature_engineering(args).await?;
        }
        Commands::MLEnhancements(args) => {
            do_codeguardian::cli::ml_enhancements::run_ml_enhancements(args).await?;
        }
    }

    info!("CodeGuardian CLI completed");
    Ok(())
}
