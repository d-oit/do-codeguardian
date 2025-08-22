use anyhow::Result;
use clap::Parser;

mod analyzers;
mod cache;
mod cli;
mod config;
mod core;
mod error;
mod github;
mod github_api;
mod ml;
mod performance;
mod report;
mod streaming;
mod types;
mod utils;

use cli::turbo::TurboArgs;
use cli::*;

#[derive(Parser)]
#[command(name = "codeguardian")]
#[command(about = "A security-first code analysis CLI with GitHub integration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run code analysis (primary command)
    Check(CheckArgs),
    /// Convert results to different formats
    Report(ReportArgs),
    /// Create or update GitHub issues
    GhIssue(GhIssueArgs),
    /// Initialize configuration
    Init(InitArgs),
    /// Train ML model for false positive reduction
    Train(TrainArgs),
    /// View ML model performance metrics
    Metrics(MetricsArgs),
    /// High-performance analysis for large codebases
    Turbo(TurboArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => cli::check::run(args).await,
        Commands::Report(args) => cli::report::run(args).await,
        Commands::GhIssue(args) => cli::gh_issue::run(args).await,
        Commands::Init(args) => cli::init::run(args).await,
        Commands::Train(args) => cli::train::run(args).await,
        Commands::Metrics(args) => cli::metrics::run(args).await,
        Commands::Turbo(args) => {
            let config = config::Config::load(&std::path::PathBuf::from("codeguardian.toml"))
                .unwrap_or_else(|_| config::Config::minimal());
            cli::turbo::run_turbo_analysis(args, config).await
        }
    }
}
