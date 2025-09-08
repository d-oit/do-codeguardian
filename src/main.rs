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
    let config = Config::from_file(config_path).unwrap_or_else(|e| {
        eprintln!("DEBUG: Config loading error: {}", e);
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
        Commands::GitCommitPush(args) => {
            git_commit_push::execute_git_commit_push(args, &config).await?;
            if !cli.quiet {
                tracing::info!("Commit and push successful");
            }
        }
        Commands::Turbo(args) => {
            turbo::execute_turbo(args, config).await?;
        }
        Commands::UpdateDocs(_args) => {
            update_docs::execute_update_docs(&config).await?;
            if !cli.quiet {
                tracing::info!("Documentation update successful");
            }
        }
        #[cfg(feature = "ml")]
        Commands::Train(args) => {
            do_codeguardian::cli::train::run(args, &config).await?;
        }
    }

    info!("CodeGuardian CLI completed");
    Ok(())
}
