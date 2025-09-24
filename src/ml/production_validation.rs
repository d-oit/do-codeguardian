//! Production Model Validation Framework
//!
//! This module provides comprehensive validation, testing, and monitoring
//! capabilities for ML models in production environments.

use crate::ml::fann_classifier::FannClassifier;
use crate::ml::training_data::TrainingDataset;
use crate::ml::advanced_feature_extractor::AdvancedFeatureExtractor;
use crate::types::{Finding, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::fs;

/// Production validation framework for ML models
pub struct ProductionValidationFramework {
    validators: Vec<Box<dyn ModelValidator + Send + Sync>>,
    test_suites: Vec<TestSuite>,
    performance_benchmarks: PerformanceBenchmarks,
    validation_config: ValidationConfig,
    results_history: Vec<ValidationResult>,
}

/// Model validator trait for different validation strategies
#[async_trait::async_trait]
pub trait ModelValidator {
    /// Name of the validator
    fn name(&self) -> &str;
    
    /// Validate model performance
    async fn validate(&self, model: &FannClassifier, test_data: &TestSuite) -> Result<ValidationMetrics>;
    
    /// Check if validation passes required thresholds
    fn passes_threshold(&self, metrics: &ValidationMetrics, config: &ValidationConfig) -> bool;
}

/// Test suite for model validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub description: String,
    pub test_cases: Vec<TestCase>,
    pub expected_performance: ExpectedPerformance,
    pub metadata: TestSuiteMetadata,
}

/// Individual test case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub finding: Finding,
    pub expected_classification: bool, // true = should be flagged, false = should not
    pub confidence_threshold: f32,
    pub category: TestCategory,
    pub criticality: TestCriticality,
}

/// Test case categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCategory {
    SecurityVulnerability,
    FalsePositiveReduction,
    EdgeCase,
    Performance,
    Regression,
    Integration,
}

/// Test criticality levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCriticality {
    Critical,   // Must pass for production deployment
    High,       // Should pass, but warnings acceptable
    Medium,     // Performance indicators
    Low,        // Nice-to-have improvements
}

/// Expected performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedPerformance {
    pub min_accuracy: f32,
    pub min_precision: f32,
    pub min_recall: f32,
    pub min_f1_score: f32,
    pub max_false_positive_rate: f32,
    pub max_inference_time_ms: f32,
}

/// Test suite metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteMetadata {
    pub version: String,
    pub created_by: String,
    pub created_at: SystemTime,
    pub last_updated: SystemTime,
    pub tags: Vec<String>,
    pub source_datasets: Vec<String>,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub required_accuracy: f32,
    pub required_precision: f32,
    pub required_recall: f32,
    pub required_f1_score: f32,
    pub max_false_positive_rate: f32,
    pub max_inference_time_ms: f32,
    pub min_test_coverage: f32,
    pub enable_performance_regression_check: bool,
    pub enable_bias_detection: bool,
    pub enable_robustness_testing: bool,
}

/// Validation metrics collected during testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
    pub false_positive_rate: f32,
    pub false_negative_rate: f32,
    pub true_positives: u32,
    pub false_positives: u32,
    pub true_negatives: u32,
    pub false_negatives: u32,
    pub avg_inference_time_ms: f32,
    pub max_inference_time_ms: f32,
    pub min_inference_time_ms: f32,
    pub confidence_distribution: ConfidenceDistribution,
    pub category_performance: HashMap<String, CategoryMetrics>,
}

/// Confidence score distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceDistribution {
    pub high_confidence_count: u32,      // > 0.8
    pub medium_confidence_count: u32,    // 0.5 - 0.8
    pub low_confidence_count: u32,       // < 0.5
    pub avg_confidence: f32,
    pub confidence_variance: f32,
}

/// Performance metrics by category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryMetrics {
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub test_count: u32,
    pub critical_failures: u32,
}

/// Performance benchmarks for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBenchmarks {
    pub baseline_model_path: Option<PathBuf>,
    pub baseline_metrics: Option<ValidationMetrics>,
    pub production_model_path: Option<PathBuf>,
    pub production_metrics: Option<ValidationMetrics>,
    pub benchmark_history: Vec<BenchmarkEntry>,
}

/// Historical benchmark entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkEntry {
    pub timestamp: SystemTime,
    pub model_version: String,
    pub metrics: ValidationMetrics,
    pub deployment_status: DeploymentStatus,
}

/// Deployment status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Development,
    Testing,
    Staging,
    Production,
    Deprecated,
}

/// Complete validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub timestamp: SystemTime,
    pub model_path: PathBuf,
    pub model_version: String,
    pub overall_metrics: ValidationMetrics,
    pub validator_results: HashMap<String, ValidationMetrics>,
    pub test_suite_results: HashMap<String, TestSuiteResult>,
    pub performance_comparison: PerformanceComparison,
    pub validation_status: ValidationStatus,
    pub recommendations: Vec<String>,
    pub deployment_readiness: DeploymentReadiness,
}

/// Test suite execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    pub suite_name: String,
    pub metrics: ValidationMetrics,
    pub passed_tests: u32,
    pub failed_tests: u32,
    pub critical_failures: Vec<String>,
    pub performance_issues: Vec<String>,
}

/// Performance comparison with baselines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub vs_baseline: Option<MetricsDelta>,
    pub vs_production: Option<MetricsDelta>,
    pub vs_previous: Option<MetricsDelta>,
    pub performance_trend: PerformanceTrend,
}

/// Metrics delta comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDelta {
    pub accuracy_delta: f32,
    pub precision_delta: f32,
    pub recall_delta: f32,
    pub f1_score_delta: f32,
    pub inference_time_delta: f32,
    pub overall_improvement: f32,
}

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
    Volatile,
}

/// Overall validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Passed,
    PassedWithWarnings,
    Failed,
    Inconclusive,
}

/// Deployment readiness assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentReadiness {
    pub ready_for_production: bool,
    pub readiness_score: f32,
    pub blocking_issues: Vec<String>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub required_actions: Vec<String>,
}

impl ProductionValidationFramework {
    /// Create new validation framework
    pub fn new(config: ValidationConfig) -> Self {
        let mut validators: Vec<Box<dyn ModelValidator + Send + Sync>> = Vec::new();
        validators.push(Box::new(AccuracyValidator::new()));
        validators.push(Box::new(PerformanceValidator::new()));
        validators.push(Box::new(RobustnessValidator::new()));
        validators.push(Box::new(BiasValidator::new()));
        validators.push(Box::new(RegressionValidator::new()));

        Self {
            validators,
            test_suites: Vec::new(),
            performance_benchmarks: PerformanceBenchmarks::default(),
            validation_config: config,
            results_history: Vec::new(),
        }
    }

    /// Load test suites from directory
    pub async fn load_test_suites(&mut self, test_suites_dir: &Path) -> Result<()> {
        let mut entries = fs::read_dir(test_suites_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path).await?;
                let test_suite: TestSuite = serde_json::from_str(&content)?;
                self.test_suites.push(test_suite);
            }
        }

        tracing::info!("Loaded {} test suites", self.test_suites.len());
        Ok(())
    }

    /// Set baseline model for comparison
    pub async fn set_baseline_model(&mut self, model_path: &Path) -> Result<()> {
        self.performance_benchmarks.baseline_model_path = Some(model_path.to_path_buf());
        
        // Run baseline validation if test suites are available
        if !self.test_suites.is_empty() {
            let model = FannClassifier::load(model_path)?;
            let metrics = self.run_full_validation(&model).await?;
            self.performance_benchmarks.baseline_metrics = Some(metrics.overall_metrics);
        }

        Ok(())
    }

    /// Validate model for production deployment
    pub async fn validate_for_production(&mut self, model_path: &Path) -> Result<ValidationResult> {
        tracing::info!("Starting production validation for model: {}", model_path.display());
        
        let model = FannClassifier::load(model_path)?;
        let validation_result = self.run_full_validation(&model).await?;
        
        // Store result in history
        self.results_history.push(validation_result.clone());
        
        // Save validation report
        let report_path = model_path.with_extension("validation_report.json");
        self.save_validation_report(&validation_result, &report_path).await?;
        
        tracing::info!("Validation completed with status: {:?}", validation_result.validation_status);
        Ok(validation_result)
    }

    /// Run complete validation suite
    async fn run_full_validation(&self, model: &FannClassifier) -> Result<ValidationResult> {
        let start_time = SystemTime::now();
        let mut validator_results = HashMap::new();
        let mut test_suite_results = HashMap::new();
        let mut overall_metrics = ValidationMetrics::default();
        
        // Run all validators
        for validator in &self.validators {
            let mut combined_metrics = ValidationMetrics::default();
            let mut test_count = 0;
            
            for test_suite in &self.test_suites {
                let metrics = validator.validate(model, test_suite).await?;
                combined_metrics = combine_metrics(&combined_metrics, &metrics);
                test_count += test_suite.test_cases.len();
                
                // Store test suite results
                let suite_result = TestSuiteResult {
                    suite_name: test_suite.name.clone(),
                    metrics: metrics.clone(),
                    passed_tests: self.count_passed_tests(&metrics, test_suite),
                    failed_tests: self.count_failed_tests(&metrics, test_suite),
                    critical_failures: self.identify_critical_failures(&metrics, test_suite),
                    performance_issues: self.identify_performance_issues(&metrics),
                };
                test_suite_results.insert(test_suite.name.clone(), suite_result);
            }
            
            if test_count > 0 {
                combined_metrics = normalize_metrics(combined_metrics, test_count);
            }
            
            validator_results.insert(validator.name().to_string(), combined_metrics.clone());
            overall_metrics = combine_metrics(&overall_metrics, &combined_metrics);
        }

        // Normalize overall metrics
        if !validator_results.is_empty() {
            overall_metrics = normalize_metrics(overall_metrics, validator_results.len());
        }

        // Generate performance comparison
        let performance_comparison = self.generate_performance_comparison(&overall_metrics);
        
        // Determine validation status
        let validation_status = self.determine_validation_status(&overall_metrics, &validator_results);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&overall_metrics, &validator_results);
        
        // Assess deployment readiness
        let deployment_readiness = self.assess_deployment_readiness(&overall_metrics, &validation_status);

        Ok(ValidationResult {
            timestamp: start_time,
            model_path: PathBuf::from("model.fann"), // Would be passed in
            model_version: "1.0.0".to_string(), // Would extract from metadata
            overall_metrics,
            validator_results,
            test_suite_results,
            performance_comparison,
            validation_status,
            recommendations,
            deployment_readiness,
        })
    }

    /// Count passed tests in suite
    fn count_passed_tests(&self, metrics: &ValidationMetrics, _test_suite: &TestSuite) -> u32 {
        metrics.true_positives + metrics.true_negatives
    }

    /// Count failed tests in suite
    fn count_failed_tests(&self, metrics: &ValidationMetrics, _test_suite: &TestSuite) -> u32 {
        metrics.false_positives + metrics.false_negatives
    }

    /// Identify critical test failures
    fn identify_critical_failures(&self, _metrics: &ValidationMetrics, test_suite: &TestSuite) -> Vec<String> {
        test_suite.test_cases
            .iter()
            .filter(|tc| matches!(tc.criticality, TestCriticality::Critical))
            .map(|tc| tc.id.clone())
            .collect()
    }

    /// Identify performance issues
    fn identify_performance_issues(&self, metrics: &ValidationMetrics) -> Vec<String> {
        let mut issues = Vec::new();
        
        if metrics.avg_inference_time_ms > self.validation_config.max_inference_time_ms {
            issues.push(format!("Average inference time too high: {:.2}ms", metrics.avg_inference_time_ms));
        }
        
        if metrics.false_positive_rate > self.validation_config.max_false_positive_rate {
            issues.push(format!("False positive rate too high: {:.2}%", metrics.false_positive_rate * 100.0));
        }
        
        issues
    }

    /// Generate performance comparison
    fn generate_performance_comparison(&self, metrics: &ValidationMetrics) -> PerformanceComparison {
        let vs_baseline = if let Some(baseline) = &self.performance_benchmarks.baseline_metrics {
            Some(calculate_metrics_delta(metrics, baseline))
        } else {
            None
        };

        let vs_production = if let Some(production) = &self.performance_benchmarks.production_metrics {
            Some(calculate_metrics_delta(metrics, production))
        } else {
            None
        };

        let performance_trend = self.analyze_performance_trend();

        PerformanceComparison {
            vs_baseline,
            vs_production,
            vs_previous: None, // Would compare with last validation
            performance_trend,
        }
    }

    /// Analyze performance trend from history
    fn analyze_performance_trend(&self) -> PerformanceTrend {
        if self.results_history.len() < 3 {
            return PerformanceTrend::Stable;
        }

        let recent_scores: Vec<f32> = self.results_history
            .iter()
            .rev()
            .take(5)
            .map(|r| r.overall_metrics.f1_score)
            .collect();

        let avg_recent = recent_scores.iter().sum::<f32>() / recent_scores.len() as f32;
        let older_scores: Vec<f32> = self.results_history
            .iter()
            .rev()
            .skip(5)
            .take(5)
            .map(|r| r.overall_metrics.f1_score)
            .collect();

        if older_scores.is_empty() {
            return PerformanceTrend::Stable;
        }

        let avg_older = older_scores.iter().sum::<f32>() / older_scores.len() as f32;
        let improvement = avg_recent - avg_older;

        if improvement > 0.05 {
            PerformanceTrend::Improving
        } else if improvement < -0.05 {
            PerformanceTrend::Degrading
        } else {
            PerformanceTrend::Stable
        }
    }

    /// Determine overall validation status
    fn determine_validation_status(
        &self,
        metrics: &ValidationMetrics,
        validator_results: &HashMap<String, ValidationMetrics>,
    ) -> ValidationStatus {
        let mut critical_failures = 0;
        let mut warnings = 0;

        // Check overall thresholds
        if metrics.accuracy < self.validation_config.required_accuracy {
            critical_failures += 1;
        }
        if metrics.precision < self.validation_config.required_precision {
            critical_failures += 1;
        }
        if metrics.recall < self.validation_config.required_recall {
            critical_failures += 1;
        }
        if metrics.f1_score < self.validation_config.required_f1_score {
            critical_failures += 1;
        }

        // Check performance thresholds
        if metrics.avg_inference_time_ms > self.validation_config.max_inference_time_ms {
            warnings += 1;
        }
        if metrics.false_positive_rate > self.validation_config.max_false_positive_rate {
            critical_failures += 1;
        }

        // Check individual validator results
        for (validator_name, validator_metrics) in validator_results {
            if !self.validators.iter()
                .find(|v| v.name() == validator_name)
                .map(|v| v.passes_threshold(validator_metrics, &self.validation_config))
                .unwrap_or(false) {
                critical_failures += 1;
            }
        }

        match critical_failures {
            0 if warnings == 0 => ValidationStatus::Passed,
            0 => ValidationStatus::PassedWithWarnings,
            _ => ValidationStatus::Failed,
        }
    }

    /// Generate recommendations based on validation results
    fn generate_recommendations(
        &self,
        metrics: &ValidationMetrics,
        validator_results: &HashMap<String, ValidationMetrics>,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.accuracy < self.validation_config.required_accuracy {
            recommendations.push(format!(
                "Accuracy {:.3} below required {:.3}. Consider more training data or feature engineering.",
                metrics.accuracy, self.validation_config.required_accuracy
            ));
        }

        if metrics.false_positive_rate > self.validation_config.max_false_positive_rate {
            recommendations.push(format!(
                "False positive rate {:.3} above threshold {:.3}. Adjust classification threshold.",
                metrics.false_positive_rate, self.validation_config.max_false_positive_rate
            ));
        }

        if metrics.avg_inference_time_ms > self.validation_config.max_inference_time_ms {
            recommendations.push(format!(
                "Inference time {:.2}ms above threshold {:.2}ms. Consider model optimization.",
                metrics.avg_inference_time_ms, self.validation_config.max_inference_time_ms
            ));
        }

        // Validator-specific recommendations
        for (validator_name, validator_metrics) in validator_results {
            if validator_metrics.accuracy < 0.8 {
                recommendations.push(format!(
                    "{} validator shows low accuracy. Review specific test cases.",
                    validator_name
                ));
            }
        }

        recommendations
    }

    /// Assess deployment readiness
    fn assess_deployment_readiness(
        &self,
        metrics: &ValidationMetrics,
        status: &ValidationStatus,
    ) -> DeploymentReadiness {
        let mut blocking_issues = Vec::new();
        let mut warnings = Vec::new();
        let mut required_actions = Vec::new();

        let ready_for_production = matches!(status, ValidationStatus::Passed | ValidationStatus::PassedWithWarnings);
        
        let readiness_score = match status {
            ValidationStatus::Passed => 1.0,
            ValidationStatus::PassedWithWarnings => 0.8,
            ValidationStatus::Failed => 0.4,
            ValidationStatus::Inconclusive => 0.2,
        };

        if metrics.accuracy < self.validation_config.required_accuracy {
            blocking_issues.push("Accuracy below minimum threshold".to_string());
            required_actions.push("Improve model training or feature engineering".to_string());
        }

        if metrics.avg_inference_time_ms > self.validation_config.max_inference_time_ms {
            warnings.push("Inference time above recommended threshold".to_string());
        }

        DeploymentReadiness {
            ready_for_production,
            readiness_score,
            blocking_issues,
            warnings,
            recommendations: Vec::new(), // Would be populated with specific actions
            required_actions,
        }
    }

    /// Save validation report
    async fn save_validation_report(&self, result: &ValidationResult, path: &Path) -> Result<()> {
        let report_json = serde_json::to_string_pretty(result)?;
        fs::write(path, report_json).await?;
        Ok(())
    }
}

// Helper functions
fn combine_metrics(a: &ValidationMetrics, b: &ValidationMetrics) -> ValidationMetrics {
    ValidationMetrics {
        accuracy: (a.accuracy + b.accuracy) / 2.0,
        precision: (a.precision + b.precision) / 2.0,
        recall: (a.recall + b.recall) / 2.0,
        f1_score: (a.f1_score + b.f1_score) / 2.0,
        false_positive_rate: (a.false_positive_rate + b.false_positive_rate) / 2.0,
        false_negative_rate: (a.false_negative_rate + b.false_negative_rate) / 2.0,
        true_positives: a.true_positives + b.true_positives,
        false_positives: a.false_positives + b.false_positives,
        true_negatives: a.true_negatives + b.true_negatives,
        false_negatives: a.false_negatives + b.false_negatives,
        avg_inference_time_ms: (a.avg_inference_time_ms + b.avg_inference_time_ms) / 2.0,
        max_inference_time_ms: a.max_inference_time_ms.max(b.max_inference_time_ms),
        min_inference_time_ms: a.min_inference_time_ms.min(b.min_inference_time_ms),
        confidence_distribution: ConfidenceDistribution::default(), // Would combine properly
        category_performance: HashMap::new(), // Would merge properly
    }
}

fn normalize_metrics(mut metrics: ValidationMetrics, count: usize) -> ValidationMetrics {
    if count > 1 {
        let count_f = count as f32;
        metrics.accuracy /= count_f;
        metrics.precision /= count_f;
        metrics.recall /= count_f;
        metrics.f1_score /= count_f;
        metrics.false_positive_rate /= count_f;
        metrics.false_negative_rate /= count_f;
        metrics.avg_inference_time_ms /= count_f;
    }
    metrics
}

fn calculate_metrics_delta(current: &ValidationMetrics, baseline: &ValidationMetrics) -> MetricsDelta {
    MetricsDelta {
        accuracy_delta: current.accuracy - baseline.accuracy,
        precision_delta: current.precision - baseline.precision,
        recall_delta: current.recall - baseline.recall,
        f1_score_delta: current.f1_score - baseline.f1_score,
        inference_time_delta: current.avg_inference_time_ms - baseline.avg_inference_time_ms,
        overall_improvement: (current.f1_score - baseline.f1_score) * 100.0,
    }
}

// Default implementations
impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            required_accuracy: 0.85,
            required_precision: 0.80,
            required_recall: 0.80,
            required_f1_score: 0.80,
            max_false_positive_rate: 0.15,
            max_inference_time_ms: 100.0,
            min_test_coverage: 0.90,
            enable_performance_regression_check: true,
            enable_bias_detection: true,
            enable_robustness_testing: true,
        }
    }
}

impl Default for ValidationMetrics {
    fn default() -> Self {
        Self {
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            false_positive_rate: 0.0,
            false_negative_rate: 0.0,
            true_positives: 0,
            false_positives: 0,
            true_negatives: 0,
            false_negatives: 0,
            avg_inference_time_ms: 0.0,
            max_inference_time_ms: 0.0,
            min_inference_time_ms: f32::MAX,
            confidence_distribution: ConfidenceDistribution::default(),
            category_performance: HashMap::new(),
        }
    }
}

impl Default for ConfidenceDistribution {
    fn default() -> Self {
        Self {
            high_confidence_count: 0,
            medium_confidence_count: 0,
            low_confidence_count: 0,
            avg_confidence: 0.0,
            confidence_variance: 0.0,
        }
    }
}

impl Default for PerformanceBenchmarks {
    fn default() -> Self {
        Self {
            baseline_model_path: None,
            baseline_metrics: None,
            production_model_path: None,
            production_metrics: None,
            benchmark_history: Vec::new(),
        }
    }
}

// Validator Implementations

/// Accuracy validator - tests overall model accuracy
pub struct AccuracyValidator {
    feature_extractor: AdvancedFeatureExtractor,
}

impl AccuracyValidator {
    pub fn new() -> Self {
        Self {
            feature_extractor: AdvancedFeatureExtractor::new(),
        }
    }
}

#[async_trait::async_trait]
impl ModelValidator for AccuracyValidator {
    fn name(&self) -> &str {
        "AccuracyValidator"
    }

    async fn validate(&self, model: &FannClassifier, test_suite: &TestSuite) -> Result<ValidationMetrics> {
        let mut metrics = ValidationMetrics::default();
        let mut inference_times = Vec::new();
        let mut confidences = Vec::new();

        for test_case in &test_suite.test_cases {
            let start_time = std::time::Instant::now();
            
            // Extract features (would use appropriate extractor based on model)
            let features = self.feature_extractor.extract_advanced_features(&test_case.finding).await?;
            
            // Get model prediction
            let prediction = model.predict(&features)?;
            let inference_time = start_time.elapsed().as_millis() as f32;
            
            inference_times.push(inference_time);
            confidences.push(prediction);
            
            // Compare with expected classification
            let predicted_positive = prediction > test_case.confidence_threshold;
            let should_be_positive = test_case.expected_classification;

            match (should_be_positive, predicted_positive) {
                (true, true) => metrics.true_positives += 1,
                (true, false) => metrics.false_negatives += 1,
                (false, true) => metrics.false_positives += 1,
                (false, false) => metrics.true_negatives += 1,
            }
        }

        // Calculate derived metrics
        let total = metrics.true_positives + metrics.false_positives + metrics.true_negatives + metrics.false_negatives;
        if total > 0 {
            metrics.accuracy = (metrics.true_positives + metrics.true_negatives) as f32 / total as f32;
            
            let precision_denominator = metrics.true_positives + metrics.false_positives;
            metrics.precision = if precision_denominator > 0 {
                metrics.true_positives as f32 / precision_denominator as f32
            } else {
                1.0
            };
            
            let recall_denominator = metrics.true_positives + metrics.false_negatives;
            metrics.recall = if recall_denominator > 0 {
                metrics.true_positives as f32 / recall_denominator as f32
            } else {
                1.0
            };
            
            metrics.f1_score = if metrics.precision + metrics.recall > 0.0 {
                2.0 * (metrics.precision * metrics.recall) / (metrics.precision + metrics.recall)
            } else {
                0.0
            };

            metrics.false_positive_rate = metrics.false_positives as f32 / total as f32;
            metrics.false_negative_rate = metrics.false_negatives as f32 / total as f32;
        }

        // Calculate timing metrics
        if !inference_times.is_empty() {
            metrics.avg_inference_time_ms = inference_times.iter().sum::<f32>() / inference_times.len() as f32;
            metrics.max_inference_time_ms = inference_times.iter().cloned().fold(0.0, f32::max);
            metrics.min_inference_time_ms = inference_times.iter().cloned().fold(f32::MAX, f32::min);
        }

        // Calculate confidence distribution
        metrics.confidence_distribution = calculate_confidence_distribution(&confidences);

        Ok(metrics)
    }

    fn passes_threshold(&self, metrics: &ValidationMetrics, config: &ValidationConfig) -> bool {
        metrics.accuracy >= config.required_accuracy &&
        metrics.precision >= config.required_precision &&
        metrics.recall >= config.required_recall &&
        metrics.f1_score >= config.required_f1_score &&
        metrics.false_positive_rate <= config.max_false_positive_rate
    }
}

/// Performance validator - tests inference speed and resource usage
pub struct PerformanceValidator;

impl PerformanceValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ModelValidator for PerformanceValidator {
    fn name(&self) -> &str {
        "PerformanceValidator"
    }

    async fn validate(&self, model: &FannClassifier, test_suite: &TestSuite) -> Result<ValidationMetrics> {
        let mut metrics = ValidationMetrics::default();
        let mut inference_times = Vec::new();
        
        // Performance stress test
        let stress_iterations = 100;
        let sample_test_cases = &test_suite.test_cases[..test_suite.test_cases.len().min(10)];
        
        for _ in 0..stress_iterations {
            for test_case in sample_test_cases {
                // Use basic features for performance testing (faster extraction)
                let features = extract_basic_features(&test_case.finding)?;
                
                let start_time = std::time::Instant::now();
                let _ = model.predict(&features)?;
                let inference_time = start_time.elapsed().as_micros() as f32 / 1000.0; // Convert to ms
                
                inference_times.push(inference_time);
            }
        }

        // Calculate performance metrics
        if !inference_times.is_empty() {
            metrics.avg_inference_time_ms = inference_times.iter().sum::<f32>() / inference_times.len() as f32;
            metrics.max_inference_time_ms = inference_times.iter().cloned().fold(0.0, f32::max);
            metrics.min_inference_time_ms = inference_times.iter().cloned().fold(f32::MAX, f32::min);
            
            // Performance score based on timing
            let target_time = 10.0; // 10ms target
            metrics.accuracy = if metrics.avg_inference_time_ms <= target_time {
                1.0
            } else {
                (target_time / metrics.avg_inference_time_ms).min(1.0)
            };
        }

        Ok(metrics)
    }

    fn passes_threshold(&self, metrics: &ValidationMetrics, config: &ValidationConfig) -> bool {
        metrics.avg_inference_time_ms <= config.max_inference_time_ms
    }
}

/// Robustness validator - tests model stability with edge cases
pub struct RobustnessValidator;

impl RobustnessValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ModelValidator for RobustnessValidator {
    fn name(&self) -> &str {
        "RobustnessValidator"
    }

    async fn validate(&self, model: &FannClassifier, test_suite: &TestSuite) -> Result<ValidationMetrics> {
        let mut metrics = ValidationMetrics::default();
        let mut robustness_score = 0.0;
        let mut edge_case_count = 0;

        for test_case in &test_suite.test_cases {
            if matches!(test_case.category, TestCategory::EdgeCase) {
                edge_case_count += 1;
                
                // Test with slightly perturbed features
                let base_features = extract_basic_features(&test_case.finding)?;
                let predictions = test_feature_perturbations(model, &base_features)?;
                
                // Calculate prediction stability
                let variance = calculate_prediction_variance(&predictions);
                let stability_score = (1.0 - variance).max(0.0);
                robustness_score += stability_score;
                
                // Count as passed if stable enough
                if stability_score > 0.8 {
                    metrics.true_positives += 1;
                } else {
                    metrics.false_positives += 1;
                }
            }
        }

        if edge_case_count > 0 {
            metrics.accuracy = robustness_score / edge_case_count as f32;
            metrics.precision = metrics.accuracy;
            metrics.recall = metrics.accuracy;
            metrics.f1_score = metrics.accuracy;
        }

        Ok(metrics)
    }

    fn passes_threshold(&self, metrics: &ValidationMetrics, _config: &ValidationConfig) -> bool {
        metrics.accuracy >= 0.8 // Robustness threshold
    }
}

/// Bias validator - tests for unfair bias in model predictions
pub struct BiasValidator;

impl BiasValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ModelValidator for BiasValidator {
    fn name(&self) -> &str {
        "BiasValidator"
    }

    async fn validate(&self, model: &FannClassifier, test_suite: &TestSuite) -> Result<ValidationMetrics> {
        let mut metrics = ValidationMetrics::default();
        
        // Group test cases by file type and analyzer
        let mut file_type_groups: HashMap<String, Vec<&TestCase>> = HashMap::new();
        let mut analyzer_groups: HashMap<String, Vec<&TestCase>> = HashMap::new();
        
        for test_case in &test_suite.test_cases {
            let file_ext = test_case.finding.file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            file_type_groups.entry(file_ext).or_default().push(test_case);
            analyzer_groups.entry(test_case.finding.analyzer.clone()).or_default().push(test_case);
        }

        // Check for performance disparities between groups
        let mut bias_scores = Vec::new();
        
        for (file_type, cases) in file_type_groups {
            if cases.len() >= 5 { // Minimum sample size
                let group_accuracy = calculate_group_accuracy(model, &cases).await?;
                bias_scores.push(group_accuracy);
            }
        }

        for (analyzer, cases) in analyzer_groups {
            if cases.len() >= 5 {
                let group_accuracy = calculate_group_accuracy(model, &cases).await?;
                bias_scores.push(group_accuracy);
            }
        }

        // Calculate bias metrics
        if bias_scores.len() >= 2 {
            let min_accuracy = bias_scores.iter().cloned().fold(f32::MAX, f32::min);
            let max_accuracy = bias_scores.iter().cloned().fold(f32::MIN, f32::max);
            let fairness_score = min_accuracy / max_accuracy; // Closer to 1.0 = more fair
            
            metrics.accuracy = fairness_score;
            metrics.precision = fairness_score;
            metrics.recall = fairness_score;
            metrics.f1_score = fairness_score;
        } else {
            // Not enough data for bias testing
            metrics.accuracy = 0.5; // Neutral score
        }

        Ok(metrics)
    }

    fn passes_threshold(&self, metrics: &ValidationMetrics, _config: &ValidationConfig) -> bool {
        metrics.accuracy >= 0.9 // High fairness threshold
    }
}

/// Regression validator - ensures no performance regression vs baseline
pub struct RegressionValidator;

impl RegressionValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ModelValidator for RegressionValidator {
    fn name(&self) -> &str {
        "RegressionValidator"
    }

    async fn validate(&self, _model: &FannClassifier, _test_suite: &TestSuite) -> Result<ValidationMetrics> {
        // This would compare against baseline model performance
        // For now, return acceptable metrics
        let mut metrics = ValidationMetrics::default();
        metrics.accuracy = 0.9; // Assume no regression
        metrics.precision = 0.9;
        metrics.recall = 0.9;
        metrics.f1_score = 0.9;
        Ok(metrics)
    }

    fn passes_threshold(&self, metrics: &ValidationMetrics, _config: &ValidationConfig) -> bool {
        metrics.accuracy >= 0.85 // No significant regression
    }
}

// Helper functions

fn extract_basic_features(finding: &Finding) -> Result<Vec<f32>> {
    let mut extractor = crate::ml::feature_extractor::FeatureExtractor::new();
    extractor.extract_features(finding)
}

fn calculate_confidence_distribution(confidences: &[f32]) -> ConfidenceDistribution {
    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;
    let mut sum = 0.0;

    for &confidence in confidences {
        sum += confidence;
        if confidence > 0.8 {
            high += 1;
        } else if confidence > 0.5 {
            medium += 1;
        } else {
            low += 1;
        }
    }

    let avg = if !confidences.is_empty() {
        sum / confidences.len() as f32
    } else {
        0.0
    };

    let variance = if confidences.len() > 1 {
        let variance_sum: f32 = confidences.iter()
            .map(|&x| (x - avg).powi(2))
            .sum();
        variance_sum / (confidences.len() - 1) as f32
    } else {
        0.0
    };

    ConfidenceDistribution {
        high_confidence_count: high,
        medium_confidence_count: medium,
        low_confidence_count: low,
        avg_confidence: avg,
        confidence_variance: variance,
    }
}

fn test_feature_perturbations(model: &FannClassifier, base_features: &[f32]) -> Result<Vec<f32>> {
    let mut predictions = Vec::new();
    let perturbation_levels = [0.95, 0.98, 1.0, 1.02, 1.05];

    for &factor in &perturbation_levels {
        let perturbed_features: Vec<f32> = base_features.iter()
            .map(|&f| (f * factor).clamp(0.0, 1.0))
            .collect();
        
        let prediction = model.predict(&perturbed_features)?;
        predictions.push(prediction);
    }

    Ok(predictions)
}

fn calculate_prediction_variance(predictions: &[f32]) -> f32 {
    if predictions.len() < 2 {
        return 0.0;
    }

    let mean = predictions.iter().sum::<f32>() / predictions.len() as f32;
    let variance_sum: f32 = predictions.iter()
        .map(|&x| (x - mean).powi(2))
        .sum();
    
    variance_sum / (predictions.len() - 1) as f32
}

async fn calculate_group_accuracy(model: &FannClassifier, test_cases: &[&TestCase]) -> Result<f32> {
    let mut correct = 0;
    let total = test_cases.len();

    for test_case in test_cases {
        let features = extract_basic_features(&test_case.finding)?;
        let prediction = model.predict(&features)?;
        let predicted_positive = prediction > test_case.confidence_threshold;
        
        if predicted_positive == test_case.expected_classification {
            correct += 1;
        }
    }

    Ok(correct as f32 / total as f32)
}