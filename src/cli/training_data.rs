//! Training Data Collection Pipeline
//!
//! This module provides comprehensive tools for collecting, labeling, and managing
//! training data for machine learning models in CodeGuardian.

use crate::cli::interactive_labeling::InteractiveLabelingSession;
use crate::cli::TrainingDataArgs;
use crate::ml::feature_extractor::FeatureExtractor;
use crate::ml::training_data::{FeedbackSource, TrainingDataset, TrainingExample};
use crate::types::{AnalysisResults, Finding, Severity};
use crate::Config;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{info, warn};

/// Training data collection pipeline
#[derive(Debug)]
pub struct TrainingDataCollectionPipeline {
    feature_extractor: FeatureExtractor,
    labeling_strategies: Vec<Box<dyn LabelingStrategy + Send + Sync>>,
    export_formats: Vec<Box<dyn TrainingDataExporter + Send + Sync>>,
    quality_validator: QualityValidator,
}

/// Strategy for automatically labeling findings
#[async_trait::async_trait]
pub trait LabelingStrategy {
    /// Name of the labeling strategy
    fn name(&self) -> &str;

    /// Confidence level of this strategy (0.0-1.0)
    fn confidence(&self) -> f64;

    /// Label a finding as true/false positive
    async fn label_finding(&self, finding: &Finding) -> Option<bool>;

    /// Get explanation for the labeling decision
    fn get_explanation(&self, finding: &Finding, label: bool) -> String;
}

/// Export training data in different formats
#[async_trait::async_trait]
pub trait TrainingDataExporter {
    /// Export format name
    fn format_name(&self) -> &str;

    /// Export training dataset
    async fn export(&self, dataset: &TrainingDataset, output_path: &Path) -> Result<()>;
}

/// Validate training data quality
#[derive(Debug)]
pub struct QualityValidator {
    min_examples_per_class: usize,
    max_imbalance_ratio: f64,
    min_confidence: f64,
}

/// Training data collection configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// Minimum number of examples to collect
    pub min_examples: usize,
    /// Target balance ratio (true_positives / false_positives)
    pub target_balance_ratio: f64,
    /// Include low-confidence labels
    pub include_low_confidence: bool,
    /// Require manual review for uncertain cases
    pub require_manual_review: bool,
    /// Export formats to generate
    pub export_formats: Vec<String>,
    /// Labeling strategies to use
    pub labeling_strategies: Vec<String>,
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            min_examples: 100,
            target_balance_ratio: 1.0, // Balanced dataset
            include_low_confidence: false,
            require_manual_review: true,
            export_formats: vec!["json".to_string(), "csv".to_string()],
            labeling_strategies: vec![
                "heuristic".to_string(),
                "severity_based".to_string(),
                "file_type_based".to_string(),
            ],
        }
    }
}

/// Collection statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionStats {
    pub total_findings: usize,
    pub labeled_findings: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub uncertain_cases: usize,
    pub balance_ratio: f64,
    pub quality_score: f64,
    pub strategy_performance: HashMap<String, StrategyStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyStats {
    pub labels_generated: usize,
    pub average_confidence: f64,
    pub agreement_rate: f64, // How often this strategy agrees with others
}

impl TrainingDataCollectionPipeline {
    /// Create new training data collection pipeline
    pub fn new() -> Self {
        let mut labeling_strategies: Vec<Box<dyn LabelingStrategy + Send + Sync>> = Vec::new();
        labeling_strategies.push(Box::new(HeuristicLabelingStrategy::new()));
        labeling_strategies.push(Box::new(SeverityBasedLabelingStrategy::new()));
        labeling_strategies.push(Box::new(FileTypeBasedLabelingStrategy::new()));
        labeling_strategies.push(Box::new(AnalyzerBasedLabelingStrategy::new()));

        let mut export_formats: Vec<Box<dyn TrainingDataExporter + Send + Sync>> = Vec::new();
        export_formats.push(Box::new(JsonExporter::new()));
        export_formats.push(Box::new(CsvExporter::new()));
        export_formats.push(Box::new(TensorFlowExporter::new()));

        Self {
            feature_extractor: FeatureExtractor::new(),
            labeling_strategies,
            export_formats,
            quality_validator: QualityValidator::new(),
        }
    }

    /// Run the complete training data collection pipeline
    pub async fn collect_training_data(
        &mut self,
        config: CollectionConfig,
        input_findings: Vec<Finding>,
        output_dir: &Path,
    ) -> Result<CollectionStats> {
        info!("Starting training data collection pipeline");
        info!("Processing {} findings", input_findings.len());

        // Create output directory
        fs::create_dir_all(output_dir).await?;

        // Initialize dataset
        let mut dataset = TrainingDataset::new();
        let mut stats = CollectionStats::default();
        stats.total_findings = input_findings.len();

        // Process each finding
        for finding in &input_findings {
            if let Some(training_example) = self.process_finding(finding, &config).await? {
                dataset.add_example(training_example);
                stats.labeled_findings += 1;
            }
        }

        // Validate dataset quality
        let quality_issues = self.quality_validator.validate(&dataset)?;
        if !quality_issues.is_empty() {
            warn!("Quality issues detected: {:?}", quality_issues);
        }

        // Calculate statistics
        stats = self.calculate_stats(&dataset, stats);

        // Export in requested formats
        for format_name in &config.export_formats {
            if let Some(exporter) = self
                .export_formats
                .iter()
                .find(|e| e.format_name() == format_name)
            {
                let output_file = output_dir.join(format!("training_data.{}", format_name));
                exporter.export(&dataset, &output_file).await?;
                info!("Exported training data to: {}", output_file.display());
            }
        }

        // Save collection report
        let report_path = output_dir.join("collection_report.json");
        self.save_collection_report(&stats, &report_path).await?;

        info!("Training data collection completed");
        info!("Collected {} labeled examples", stats.labeled_findings);
        info!("Balance ratio: {:.2}", stats.balance_ratio);
        info!("Quality score: {:.2}", stats.quality_score);

        Ok(stats)
    }

    /// Process a single finding through the labeling pipeline
    async fn process_finding(
        &self,
        finding: &Finding,
        config: &CollectionConfig,
    ) -> Result<Option<TrainingExample>> {
        // Extract features
        let features = self.feature_extractor.extract_features(finding)?;

        // Apply labeling strategies
        let mut labels = Vec::new();
        let mut explanations = Vec::new();

        for strategy in &self.labeling_strategies {
            if config
                .labeling_strategies
                .contains(&strategy.name().to_string())
            {
                if let Some(label) = strategy.label_finding(finding).await {
                    labels.push((label, strategy.confidence()));
                    explanations.push(strategy.get_explanation(finding, label));
                }
            }
        }

        // Resolve conflicting labels
        if let Some((final_label, confidence)) = self.resolve_labels(&labels) {
            // Check if we should include this example
            if !config.include_low_confidence && confidence < 0.7 {
                return Ok(None);
            }

            let feedback_source = if confidence > 0.9 {
                FeedbackSource::AutomaticHeuristic
            } else if config.require_manual_review && confidence < 0.8 {
                FeedbackSource::UserFeedback // Requires manual review
            } else {
                FeedbackSource::AutomaticHeuristic
            };

            let example = TrainingExample {
                finding_id: finding.id.clone(),
                features,
                is_true_positive: final_label,
                feedback_source,
                timestamp: chrono::Utc::now(),
            };

            Ok(Some(example))
        } else {
            // No consensus reached
            Ok(None)
        }
    }

    /// Resolve conflicting labels from multiple strategies
    fn resolve_labels(&self, labels: &[(bool, f64)]) -> Option<(bool, f64)> {
        if labels.is_empty() {
            return None;
        }

        // Weighted voting based on confidence
        let mut true_weight = 0.0;
        let mut false_weight = 0.0;
        let mut total_weight = 0.0;

        for &(label, confidence) in labels {
            if label {
                true_weight += confidence;
            } else {
                false_weight += confidence;
            }
            total_weight += confidence;
        }

        let final_label = true_weight > false_weight;
        let confidence = if final_label {
            true_weight / total_weight
        } else {
            false_weight / total_weight
        };

        Some((final_label, confidence))
    }

    /// Calculate collection statistics
    fn calculate_stats(
        &self,
        dataset: &TrainingDataset,
        mut stats: CollectionStats,
    ) -> CollectionStats {
        let dataset_stats = dataset.get_stats();

        stats.true_positives = dataset_stats.true_positives;
        stats.false_positives = dataset_stats.false_positives;
        stats.balance_ratio = dataset_stats.balance_ratio;

        // Calculate quality score based on balance and quantity
        let balance_score =
            if dataset_stats.balance_ratio > 0.5 && dataset_stats.balance_ratio < 2.0 {
                1.0 // Well balanced
            } else {
                0.5 // Imbalanced
            };

        let quantity_score = (dataset_stats.total_examples as f64 / 100.0).min(1.0);
        stats.quality_score = (balance_score + quantity_score) / 2.0;

        stats
    }

    /// Save collection report
    async fn save_collection_report(&self, stats: &CollectionStats, path: &Path) -> Result<()> {
        let report_json = serde_json::to_string_pretty(stats)?;
        fs::write(path, report_json).await?;
        Ok(())
    }
}

impl Default for CollectionStats {
    fn default() -> Self {
        Self {
            total_findings: 0,
            labeled_findings: 0,
            true_positives: 0,
            false_positives: 0,
            uncertain_cases: 0,
            balance_ratio: 0.0,
            quality_score: 0.0,
            strategy_performance: HashMap::new(),
        }
    }
}

impl QualityValidator {
    pub fn new() -> Self {
        Self {
            min_examples_per_class: 10,
            max_imbalance_ratio: 10.0,
            min_confidence: 0.6,
        }
    }

    pub fn validate(&self, dataset: &TrainingDataset) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        let stats = dataset.get_stats();

        // Check minimum examples
        if stats.true_positives < self.min_examples_per_class {
            issues.push(format!(
                "Insufficient true positives: {} (minimum: {})",
                stats.true_positives, self.min_examples_per_class
            ));
        }

        if stats.false_positives < self.min_examples_per_class {
            issues.push(format!(
                "Insufficient false positives: {} (minimum: {})",
                stats.false_positives, self.min_examples_per_class
            ));
        }

        // Check balance
        if stats.balance_ratio > self.max_imbalance_ratio
            || stats.balance_ratio < (1.0 / self.max_imbalance_ratio)
        {
            issues.push(format!(
                "Dataset is severely imbalanced: ratio {:.2}",
                stats.balance_ratio
            ));
        }

        Ok(issues)
    }
}

// Labeling Strategy Implementations

/// Heuristic-based labeling using domain knowledge
pub struct HeuristicLabelingStrategy;

impl HeuristicLabelingStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LabelingStrategy for HeuristicLabelingStrategy {
    fn name(&self) -> &str {
        "heuristic"
    }

    fn confidence(&self) -> f64 {
        0.8
    }

    async fn label_finding(&self, finding: &Finding) -> Option<bool> {
        // High confidence true positives
        if matches!(finding.severity, Severity::Critical | Severity::High) {
            if finding.analyzer == "integrity"
                || (finding.analyzer == "security" && finding.message.contains("injection"))
                || finding.message.to_lowercase().contains("secret")
                || finding.message.to_lowercase().contains("vulnerability")
            {
                return Some(true);
            }
        }

        // High confidence false positives
        if finding.message.to_lowercase().contains("todo")
            && finding.file.to_string_lossy().contains("test")
        {
            return Some(false);
        }

        if finding.message.to_lowercase().contains("debug")
            && matches!(finding.severity, Severity::Low | Severity::Info)
        {
            return Some(false);
        }

        // Medium confidence based on patterns
        if matches!(finding.severity, Severity::Medium | Severity::High) {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn get_explanation(&self, finding: &Finding, label: bool) -> String {
        if label {
            format!(
                "Classified as true positive based on severity ({:?}) and analyzer ({})",
                finding.severity, finding.analyzer
            )
        } else {
            format!(
                "Classified as false positive based on content patterns in {} message",
                finding.message
            )
        }
    }
}

/// Severity-based labeling strategy
pub struct SeverityBasedLabelingStrategy;

impl SeverityBasedLabelingStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LabelingStrategy for SeverityBasedLabelingStrategy {
    fn name(&self) -> &str {
        "severity_based"
    }

    fn confidence(&self) -> f64 {
        0.7
    }

    async fn label_finding(&self, finding: &Finding) -> Option<bool> {
        match finding.severity {
            Severity::Critical | Severity::High => Some(true),
            Severity::Medium => None, // Uncertain
            Severity::Low | Severity::Info => Some(false),
        }
    }

    fn get_explanation(&self, finding: &Finding, label: bool) -> String {
        format!(
            "Labeled based on severity: {:?} -> {}",
            finding.severity, label
        )
    }
}

/// File type-based labeling strategy
pub struct FileTypeBasedLabelingStrategy;

impl FileTypeBasedLabelingStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LabelingStrategy for FileTypeBasedLabelingStrategy {
    fn name(&self) -> &str {
        "file_type_based"
    }

    fn confidence(&self) -> f64 {
        0.6
    }

    async fn label_finding(&self, finding: &Finding) -> Option<bool> {
        let file_str = finding.file.to_string_lossy();

        // Test files often have false positives
        if file_str.contains("test") || file_str.contains("spec") {
            return Some(false);
        }

        // Documentation files
        if file_str.ends_with(".md") || file_str.ends_with(".txt") {
            return Some(false);
        }

        // Source code files are more likely to have real issues
        if file_str.ends_with(".rs") || file_str.ends_with(".js") || file_str.ends_with(".py") {
            return Some(true);
        }

        None
    }

    fn get_explanation(&self, finding: &Finding, label: bool) -> String {
        format!(
            "Labeled based on file type: {} -> {}",
            finding.file.display(),
            label
        )
    }
}

/// Analyzer-based labeling strategy
pub struct AnalyzerBasedLabelingStrategy;

impl AnalyzerBasedLabelingStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LabelingStrategy for AnalyzerBasedLabelingStrategy {
    fn name(&self) -> &str {
        "analyzer_based"
    }

    fn confidence(&self) -> f64 {
        0.9
    }

    async fn label_finding(&self, finding: &Finding) -> Option<bool> {
        match finding.analyzer.as_str() {
            "integrity" => Some(true),   // High confidence analyzer
            "security" => Some(true),    // Security issues are usually real
            "performance" => Some(true), // Performance issues matter
            "duplicate" => Some(false),  // Often false positives
            "style" => Some(false),      // Style issues are subjective
            _ => None,
        }
    }

    fn get_explanation(&self, finding: &Finding, label: bool) -> String {
        format!(
            "Labeled based on analyzer reliability: {} -> {}",
            finding.analyzer, label
        )
    }
}

// Export Format Implementations

/// JSON export format
pub struct JsonExporter;

impl JsonExporter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl TrainingDataExporter for JsonExporter {
    fn format_name(&self) -> &str {
        "json"
    }

    async fn export(&self, dataset: &TrainingDataset, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(dataset)?;
        fs::write(output_path, json).await?;
        Ok(())
    }
}

/// CSV export format
pub struct CsvExporter;

impl CsvExporter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl TrainingDataExporter for CsvExporter {
    fn format_name(&self) -> &str {
        "csv"
    }

    async fn export(&self, dataset: &TrainingDataset, output_path: &Path) -> Result<()> {
        let mut csv_content = String::new();

        // Header
        csv_content.push_str("finding_id,");
        for i in 0..8 {
            csv_content.push_str(&format!("feature_{},", i));
        }
        csv_content.push_str("is_true_positive,feedback_source,timestamp\n");

        // Data rows
        for example in &dataset.examples {
            csv_content.push_str(&format!("{},", example.finding_id));
            for feature in &example.features {
                csv_content.push_str(&format!("{},", feature));
            }
            csv_content.push_str(&format!(
                "{},{:?},{}\n",
                example.is_true_positive,
                example.feedback_source,
                example.timestamp.to_rfc3339()
            ));
        }

        fs::write(output_path, csv_content).await?;
        Ok(())
    }
}

/// TensorFlow-compatible export format
pub struct TensorFlowExporter;

impl TensorFlowExporter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl TrainingDataExporter for TensorFlowExporter {
    fn format_name(&self) -> &str {
        "tfrecord"
    }

    async fn export(&self, dataset: &TrainingDataset, output_path: &Path) -> Result<()> {
        // For now, export as JSON with TensorFlow-friendly structure
        let tf_data: Vec<_> = dataset
            .examples
            .iter()
            .map(|example| {
                serde_json::json!({
                    "features": example.features,
                    "label": if example.is_true_positive { 1 } else { 0 },
                    "metadata": {
                        "finding_id": example.finding_id,
                        "timestamp": example.timestamp.to_rfc3339()
                    }
                })
            })
            .collect();

        let json = serde_json::to_string_pretty(&tf_data)?;
        fs::write(output_path, json).await?;
        Ok(())
    }
}

/// Run training data collection command
pub async fn run(args: TrainingDataArgs, _config: &Config) -> Result<()> {
    // Handle validation-only mode
    if args.validate_only {
        return validate_existing_training_data(&args.output_dir).await;
    }

    // Load findings from input file or run analysis
    let findings = if let Some(input_file) = &args.input_file {
        load_findings_from_file(input_file).await?
    } else if let Some(source_path) = &args.source_path {
        run_analysis_and_collect_findings(source_path).await?
    } else {
        return Err(anyhow!(
            "Either --input-file or --source-path must be specified"
        ));
    };

    if findings.is_empty() {
        warn!("No findings to process");
        return Ok(());
    }

    info!("Loaded {} findings for processing", findings.len());

    // Handle interactive mode
    if args.interactive {
        return run_interactive_collection(findings, &args).await;
    }

    // Load or create collection configuration
    let config = create_collection_config_from_args(&args)?;

    // Run automated collection pipeline
    let mut pipeline = TrainingDataCollectionPipeline::new();
    let stats = pipeline
        .collect_training_data(config, findings, &args.output_dir)
        .await?;

    // Print summary
    print_collection_summary(&stats, &args.output_dir);

    Ok(())
}

/// Run interactive collection with manual labeling
async fn run_interactive_collection(findings: Vec<Finding>, args: &TrainingDataArgs) -> Result<()> {
    info!("Starting interactive labeling session");

    let mut session = InteractiveLabelingSession::new(findings);
    let dataset = session.run().await?;

    // Save the manually labeled dataset
    fs::create_dir_all(&args.output_dir).await?;
    let output_file = args.output_dir.join("interactive_training_data.json");
    dataset
        .save_to_file_async(&output_file.to_string_lossy())
        .await?;

    info!("Interactive dataset saved to: {}", output_file.display());

    // Export in additional formats if requested
    export_interactive_dataset(&dataset, args).await?;

    Ok(())
}

/// Export interactive dataset in requested formats
async fn export_interactive_dataset(
    dataset: &TrainingDataset,
    args: &TrainingDataArgs,
) -> Result<()> {
    let formats: Vec<&str> = args.export_formats.split(',').collect();

    for format in formats {
        match format.trim() {
            "csv" => {
                let csv_path = args.output_dir.join("interactive_training_data.csv");
                export_to_csv(dataset, &csv_path).await?;
                info!("CSV export saved to: {}", csv_path.display());
            }
            "tfrecord" => {
                let tf_path = args.output_dir.join("interactive_training_data.tfrecord");
                export_to_tensorflow(dataset, &tf_path).await?;
                info!("TensorFlow export saved to: {}", tf_path.display());
            }
            "json" => {
                // Already saved above
            }
            _ => {
                warn!("Unknown export format: {}", format);
            }
        }
    }

    Ok(())
}

/// Create collection config from command line arguments
fn create_collection_config_from_args(args: &TrainingDataArgs) -> Result<CollectionConfig> {
    let config = if let Some(config_file) = &args.config_file {
        load_collection_config_sync(config_file)?
    } else {
        CollectionConfig {
            min_examples: args.min_examples,
            target_balance_ratio: args.target_balance,
            include_low_confidence: args.include_low_confidence,
            require_manual_review: !args.skip_manual_review,
            export_formats: args
                .export_formats
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            labeling_strategies: args
                .labeling_strategies
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    };

    Ok(config)
}

/// Print collection summary
fn print_collection_summary(stats: &CollectionStats, output_dir: &Path) {
    println!("\n🎉 Training Data Collection Complete!");
    println!("════════════════════════════════════════");
    println!();
    println!("📊 Collection Statistics:");
    println!("  • Total findings processed: {}", stats.total_findings);
    println!("  • Successfully labeled: {}", stats.labeled_findings);
    println!("  • True positives: {}", stats.true_positives);
    println!("  • False positives: {}", stats.false_positives);
    println!("  • Balance ratio: {:.2}", stats.balance_ratio);
    println!("  • Quality score: {:.2}/1.0", stats.quality_score);

    // Quality assessment
    if stats.quality_score >= 0.8 {
        println!("  • ✅ High quality dataset");
    } else if stats.quality_score >= 0.6 {
        println!("  • ⚠️  Good quality dataset");
    } else {
        println!("  • ❌ Low quality dataset - consider more examples");
    }

    println!();
    println!("📁 Output Files:");
    println!(
        "  • Training data: {}/training_data.json",
        output_dir.display()
    );
    println!(
        "  • Collection report: {}/collection_report.json",
        output_dir.display()
    );

    println!();
    println!("🚀 Next Steps:");
    println!("  1. Review the collection report for detailed metrics");
    println!("  2. Train a model with the collected data:");
    println!(
        "     codeguardian train --training-data {}/training_data.json",
        output_dir.display()
    );
    println!("  3. Collect more data from different projects if needed");
}

/// Load findings from JSON file
async fn load_findings_from_file(file_path: &Path) -> Result<Vec<Finding>> {
    let content = fs::read_to_string(file_path).await?;
    let analysis_results: AnalysisResults = serde_json::from_str(&content)?;
    Ok(analysis_results.findings)
}

/// Run analysis on source path and collect findings
async fn run_analysis_and_collect_findings(source_path: &Path) -> Result<Vec<Finding>> {
    // This would integrate with the main analysis engine
    // For now, return empty vector - to be implemented
    warn!("Direct source analysis not yet implemented, please provide --input-file");
    Ok(Vec::new())
}

/// Load collection configuration from file (async)
async fn load_collection_config(config_file: &Path) -> Result<CollectionConfig> {
    let content = fs::read_to_string(config_file).await?;
    let config: CollectionConfig = serde_json::from_str(&content)?;
    Ok(config)
}

/// Load collection configuration from file (sync)
fn load_collection_config_sync(config_file: &Path) -> Result<CollectionConfig> {
    let content = std::fs::read_to_string(config_file)?;
    let config: CollectionConfig = serde_json::from_str(&content)?;
    Ok(config)
}

/// Validate existing training data
async fn validate_existing_training_data(output_dir: &Path) -> Result<()> {
    info!(
        "Validating existing training data in: {}",
        output_dir.display()
    );

    let training_file = output_dir.join("training_data.json");
    if !training_file.exists() {
        return Err(anyhow!(
            "No training data found at: {}",
            training_file.display()
        ));
    }

    let dataset = TrainingDataset::load_from_file_async(&training_file.to_string_lossy()).await?;
    let stats = dataset.get_stats();

    let validator = QualityValidator::new();
    let issues = validator.validate(&dataset)?;

    println!("Training Data Validation Report:");
    println!("════════════════════════════════");
    println!("{}", stats);

    if issues.is_empty() {
        println!("✅ No quality issues detected");
    } else {
        println!("⚠️  Quality Issues Found:");
        for issue in &issues {
            println!("  • {}", issue);
        }
    }

    Ok(())
}

/// Export dataset to CSV format
async fn export_to_csv(dataset: &TrainingDataset, output_path: &Path) -> Result<()> {
    let exporter = CsvExporter::new();
    exporter.export(dataset, output_path).await
}

/// Export dataset to TensorFlow format
async fn export_to_tensorflow(dataset: &TrainingDataset, output_path: &Path) -> Result<()> {
    let exporter = TensorFlowExporter::new();
    exporter.export(dataset, output_path).await
}

impl Default for TrainingDataCollectionPipeline {
    fn default() -> Self {
        Self::new()
    }
}
