//! # Output Module
//!
//! This module provides unified output formatting capabilities for CodeGuardian analysis results.
//! It implements the standardized output format interface as specified in the implementation roadmap.

pub mod ai;
pub mod continuous_improvement;
pub mod documentation;
pub mod enterprise;
pub mod formats;
pub mod formatter;
pub mod metrics;
pub mod parallel;
pub mod security;
pub mod storage;
pub mod testing;
pub mod validation;

pub use ai::{create_enhancement_engine, AIEnhancementConfig, EnhancedAnalysisResults};
pub use continuous_improvement::{
    ABTestInsight, ContinuousImprovementManager, ImprovementRecommendation,
};
pub use documentation::{DocumentationConfig, DocumentationGenerator, DocumentationSuite};
pub use enterprise::{
    EnterpriseConfig, EnterpriseContext, EnterpriseManager, SubscriptionTier, Tenant,
};
pub use formats::{HtmlFormatter, JsonFormatter, MarkdownFormatter, SarifFormatter, YamlFormatter};
pub use formatter::{
    BaseFormatter, FormatMetadata, FormatterConfig, OutputFormatter, OutputMetadata, OutputResult,
    SecurityConfig, ToolMetadata, ValidationStatus,
};
pub use metrics::{Alert, AlertSeverity, MetricsReport, OutputMetricsService, SystemHealth};
pub use parallel::{
    ChunkParallelProcessor, ConcurrentPipelineResult, ParallelOutputConfig,
    ParallelOutputProcessor, ParallelPerformanceMetrics, ProcessedChunk, ValidationResult,
};
pub use security::sanitize_html;
pub use storage::{OrganizationStrategy, ResultMetadata, StorageConfig, StorageIndex};
pub use testing::{OutputTestSuite, TestConfig, TestResult, TestStatus};
pub use validation::validate_output;

use crate::performance::GlobalMemoryPools;
use crate::types::AnalysisResults;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Html,
    Markdown,
    Sarif,
    Yaml,
    Text,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Html => write!(f, "html"),
            OutputFormat::Markdown => write!(f, "markdown"),
            OutputFormat::Sarif => write!(f, "sarif"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Text => write!(f, "text"),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "html" => Ok(OutputFormat::Html),
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "sarif" => Ok(OutputFormat::Sarif),
            "yaml" | "yml" => Ok(OutputFormat::Yaml),
            "text" | "txt" => Ok(OutputFormat::Text),
            _ => Err(anyhow::anyhow!("Unsupported output format: {}", s)),
        }
    }
}

/// Create a formatter for the specified output format
pub fn create_formatter(format: OutputFormat) -> Box<dyn OutputFormatter> {
    match format {
        OutputFormat::Json => Box::new(JsonFormatter::new()),
        OutputFormat::Html => Box::new(HtmlFormatter::new()),
        OutputFormat::Markdown => Box::new(MarkdownFormatter::new()),
        OutputFormat::Sarif => Box::new(SarifFormatter::new()),
        OutputFormat::Yaml => Box::new(YamlFormatter::new()),
        OutputFormat::Text => Box::new(MarkdownFormatter::new()), // Text uses markdown for now
    }
}

/// Format analysis results using the specified format
pub fn format_results(results: &AnalysisResults, format: OutputFormat) -> Result<OutputResult> {
    let formatter = create_formatter(format);
    formatter.format(results)
}

/// Create a formatter with memory pool optimization
pub fn create_formatter_with_memory_manager(
    format: OutputFormat,
    memory_manager: std::sync::Arc<GlobalMemoryPools>,
) -> Box<dyn OutputFormatter> {
    match format {
        OutputFormat::Json => Box::new(JsonFormatter::with_memory_manager(memory_manager)),
        OutputFormat::Html => Box::new(HtmlFormatter::new()),
        OutputFormat::Markdown => Box::new(MarkdownFormatter::new()),
        OutputFormat::Sarif => Box::new(SarifFormatter::new()),
        OutputFormat::Yaml => Box::new(YamlFormatter::new()),
        OutputFormat::Text => Box::new(MarkdownFormatter::new()),
    }
}

/// Format analysis results using the specified format with memory optimization
pub fn format_results_with_memory_manager(
    results: &AnalysisResults,
    format: OutputFormat,
    memory_manager: std::sync::Arc<GlobalMemoryPools>,
) -> Result<OutputResult> {
    let formatter = create_formatter_with_memory_manager(format, memory_manager);
    formatter.format(results)
}
