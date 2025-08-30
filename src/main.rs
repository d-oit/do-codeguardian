//! # CodeGuardian CLI
//!
//! The main entry point for the CodeGuardian command-line tool.
//! This tool provides security analysis and enhanced git operations.
//!
//! ## Usage
//!
//! ```bash
//! codeguardian analyze <files...>
//! codeguardian git-commit [--message <msg>]
//! ```

use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use anyhow::Result;
use do_codeguardian::cli::check;
use do_codeguardian::cli::gh_issue;
use do_codeguardian::cli::init;
use do_codeguardian::cli::report;
use do_codeguardian::cli::{Cli, Commands};
use do_codeguardian::commands::git_commit;
use do_codeguardian::commands::turbo;
use do_codeguardian::{CodeGuardianError, Config};

// CLI structure is now imported from cli.rs

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine log level based on verbosity
    let log_level = if cli.verbose > 0 {
        Level::DEBUG
    } else {
        Level::INFO
    };

    // Initialize logging
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| CodeGuardianError::LoggingSetup)?;

    info!("CodeGuardian CLI starting");

    // Load configuration
    let config_path = &cli.config;
    let config = Config::from_file(config_path).unwrap_or_else(|_| {
        if !cli.quiet {
            tracing::warn!(
                "No configuration file found at {}, using defaults",
                config_path.display()
            );
        }
        Config::default()
    });

    // Execute command
    match cli.command {
        Commands::Check(args) => {
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
        Commands::Turbo(args) => {
            turbo::execute_turbo(args, config).await?;
        }
    }

    info!("CodeGuardian CLI completed");
    Ok(())
}
