use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

pub mod check;
pub mod gh_issue;
pub mod init;
pub mod metrics;
pub mod report;
pub mod train;
pub mod turbo;

#[derive(Parser)]
#[command(
    name = "codeguardian",
    version,
    about = "A security-first code analysis CLI with GitHub integration",
    long_about = "CodeGuardian performs comprehensive code analysis with best-practice defaults:\n\
                  • Deterministic findings with stable IDs\n\
                  • Security-by-default configuration\n\
                  • CI-first UX with GitHub integration\n\
                  • Minimal developer friction"
)]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "codeguardian.toml")]
    pub config: PathBuf,

    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run code analysis (primary command)
    Check(CheckArgs),

    /// Convert results to different formats
    Report(ReportArgs),

    /// Create or update GitHub issues
    GhIssue(GhIssueArgs),

    /// Initialize configuration
    Init(InitArgs),

    /// High-performance analysis for large codebases
    Turbo(turbo::TurboArgs),
}

#[derive(Parser)]
pub struct CheckArgs {
    /// Paths to analyze (files or directories)
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// Output format (json is source of truth)
    #[arg(long, default_value = "json")]
    pub format: OutputFormat,

    /// Output file for results
    #[arg(long, default_value = "results.json")]
    pub out: PathBuf,

    /// Emit markdown report
    #[arg(long)]
    pub emit_md: Option<PathBuf>,

    /// Emit GitHub issue
    #[arg(long)]
    pub emit_gh: bool,

    /// GitHub repository (owner/repo)
    #[arg(long, env = "GITHUB_REPOSITORY")]
    pub repo: Option<String>,

    /// GitHub issue mode
    #[arg(long, default_value = "checklist")]
    pub gh_mode: GhMode,

    /// GitHub issue labels
    #[arg(long, default_value = "codeguardian,automated")]
    pub labels: String,

    /// Only analyze changed files (git diff)
    #[arg(long)]
    pub diff: Option<String>,

    /// Only analyze staged files
    #[arg(long)]
    pub only_changed: bool,

    /// Exit with non-zero code if issues are found
    #[arg(long)]
    pub fail_on_issues: bool,

    /// Number of parallel workers (0 = auto)
    #[arg(short, long, default_value = "0")]
    pub parallel: usize,

    /// Suppress all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    /// Baseline file for drift analysis
    #[arg(short, long)]
    pub baseline: Option<PathBuf>,

    /// Path to ML model for false positive reduction
    #[arg(long)]
    pub ml_model: Option<String>,
}

#[derive(Parser)]
pub struct ReportArgs {
    /// Input results file
    #[arg(long, default_value = "results.json")]
    pub from: PathBuf,

    /// Output markdown file
    #[arg(long)]
    pub md: Option<PathBuf>,

    /// Output format
    #[arg(long, default_value = "markdown")]
    pub format: ReportFormat,
}

#[derive(Parser)]
pub struct GhIssueArgs {
    /// Input results file
    #[arg(long, default_value = "results.json")]
    pub from: PathBuf,

    /// GitHub repository (owner/repo)
    #[arg(long, env = "GITHUB_REPOSITORY")]
    pub repo: String,

    /// GitHub issue mode
    #[arg(long, default_value = "checklist")]
    pub mode: GhMode,

    /// Issue title prefix
    #[arg(long, default_value = "CodeGuardian: ")]
    pub title: String,

    /// Issue labels
    #[arg(long, default_value = "codeguardian,automated")]
    pub labels: String,

    /// Manual summary file
    #[arg(long)]
    pub summary_from: Option<PathBuf>,

    /// Auto-generate summary
    #[arg(long)]
    pub summary_auto: Option<String>,

    /// Maximum characters in summary
    #[arg(long, default_value = "800")]
    pub summary_max_chars: usize,

    /// Maximum issues to include in summary
    #[arg(long, default_value = "10")]
    pub summary_max_issues: usize,

    /// Dry run mode (print commands without executing)
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Parser)]
pub struct InitArgs {
    /// Initialize with default configuration
    #[arg(long)]
    pub default: bool,

    /// Template to use
    #[arg(long)]
    pub template: Option<String>,
}

#[derive(Parser)]
pub struct TrainArgs {
    /// Paths to analyze for training data (files or directories)
    #[arg(default_value = ".")]
    pub paths: Vec<std::path::PathBuf>,

    /// Path to save/load the trained model
    #[arg(long, short = 'm', default_value = "codeguardian-model.fann")]
    pub model_path: Option<String>,

    /// Path to save/load training dataset
    #[arg(long, short = 'd')]
    pub dataset: Option<String>,

    /// Number of training epochs
    #[arg(long, default_value = "100")]
    pub epochs: u32,

    /// Generate bootstrap training data from codebase analysis
    #[arg(long)]
    pub bootstrap: bool,

    /// Number of synthetic training samples to generate
    #[arg(long, default_value = "0")]
    pub synthetic_samples: usize,

    /// Use balanced training data (equal true/false positives)
    #[arg(long)]
    pub balanced: bool,

    /// Suppress all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    /// Show detailed training progress
    #[arg(long)]
    pub verbose: bool,
}

#[derive(Parser)]
pub struct MetricsArgs {
    /// Path to the ML model file
    #[arg(long, short = 'm', default_value = "codeguardian-model.fann")]
    pub model_path: String,

    /// Suppress all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: MetricsCommand,
}

#[derive(clap::Subcommand)]
pub enum MetricsCommand {
    /// Show detailed metrics report
    Show,
    /// Export metrics to JSON file
    Export {
        /// Output file path
        #[arg(short, long, default_value = "model-metrics.json")]
        output: String,
    },
    /// Show condensed metrics summary
    Summary,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// JSON output for programmatic use (source of truth)
    Json,
    /// Human-readable output with colors
    Human,
    /// SARIF format for security tools
    Sarif,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ReportFormat {
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// Plain text format
    Text,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum GhMode {
    /// Checklist format with checkboxes
    Checklist,
    /// Simple issue format
    Simple,
    /// Children mode for large reports
    Children,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Human => write!(f, "human"),
            OutputFormat::Sarif => write!(f, "sarif"),
        }
    }
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportFormat::Markdown => write!(f, "markdown"),
            ReportFormat::Html => write!(f, "html"),
            ReportFormat::Text => write!(f, "text"),
        }
    }
}

impl std::fmt::Display for GhMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GhMode::Checklist => write!(f, "checklist"),
            GhMode::Simple => write!(f, "simple"),
            GhMode::Children => write!(f, "children"),
        }
    }
}
