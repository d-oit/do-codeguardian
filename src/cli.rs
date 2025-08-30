use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

pub mod check;
pub mod gh_issue;
pub mod init;
pub mod report;

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

    /// Perform enhanced git commit with security analysis
    GitCommit(GitCommitArgs),

    /// Run high-performance parallel analysis (turbo mode)
    Turbo(TurboArgs),
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
    #[arg(long)]
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

    /// Baseline file for drift analysis
    #[arg(short, long)]
    pub baseline: Option<PathBuf>,

    /// Suppress all output except errors
    #[arg(long)]
    pub quiet: bool,

    /// Only analyze files that are new compared to baseline
    #[arg(long)]
    pub only_new: bool,

    /// Enable streaming analysis for large files
    #[arg(long)]
    pub streaming: bool,

    /// ML threshold for anomaly detection (0.0-1.0)
    #[arg(long, value_name = "THRESHOLD")]
    pub ml_threshold: Option<f64>,
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
    #[arg(long)]
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
pub struct GitCommitArgs {
    /// Custom commit message
    #[arg(short, long)]
    pub message: Option<String>,
}

#[derive(Parser)]
pub struct TurboArgs {
    /// Paths to analyze (files or directories)
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// Maximum number of parallel workers
    #[arg(long, default_value = "0")]
    pub max_parallel: usize,

    /// Memory limit in MB (0 = no limit)
    #[arg(long, default_value = "0")]
    pub memory_limit: usize,

    /// Output format (json, human, sarif)
    #[arg(long, default_value = "json")]
    pub format: OutputFormat,

    /// Output file for results
    #[arg(long, default_value = "turbo-results.json")]
    pub output: PathBuf,

    /// Enable metrics output
    #[arg(long)]
    pub metrics: bool,

    /// Aggressive analysis mode (more thorough but slower)
    #[arg(long)]
    pub aggressive: bool,

    /// Only analyze changed files (git diff)
    #[arg(long)]
    pub diff: Option<String>,

    /// Only analyze staged files
    #[arg(long)]
    pub only_staged: bool,

    /// Exit with non-zero code if issues are found
    #[arg(long)]
    pub fail_on_issues: bool,

    /// Baseline file for drift analysis
    #[arg(long)]
    pub baseline: Option<PathBuf>,
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
