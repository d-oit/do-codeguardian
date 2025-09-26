//! Advanced Feature Engineering Module
//!
//! This module provides automated feature generation, selection, and engineering capabilities
//! for the CodeGuardian ML pipeline. It extends the existing feature extraction with:
//! - Automated feature generation from code analysis
//! - Feature selection using multiple algorithms
//! - Feature transformation and normalization
//! - Cross-validation based feature importance
//! - Dynamic feature engineering based on patterns

use crate::ml::unified_feature_extractor::UnifiedFeatureExtractor;
use crate::types::{Finding, Severity};
use anyhow::Result;
use async_trait::async_trait;
use regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Advanced feature engineering engine that automates feature generation and selection
pub struct AdvancedFeatureEngineer {
    /// Base feature extractor
    base_extractor: UnifiedFeatureExtractor,
    /// Feature generation modules
    generators: Vec<Box<dyn FeatureGenerator + Send + Sync>>,
    /// Feature selection algorithms
    selectors: Vec<Box<dyn FeatureSelector + Send + Sync>>,
    /// Configuration
    config: FeatureEngineeringConfig,
    /// Generated features cache
    feature_cache: Arc<RwLock<HashMap<String, GeneratedFeatures>>>,
    /// Performance metrics
    metrics: FeatureEngineeringMetrics,
}

/// Configuration for advanced feature engineering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureEngineeringConfig {
    /// Enable automated feature generation
    pub auto_generation: bool,
    /// Enable feature selection
    pub feature_selection: bool,
    /// Maximum number of generated features
    pub max_generated_features: usize,
    /// Feature selection threshold
    pub selection_threshold: f64,
    /// Cross-validation folds for importance scoring
    pub cv_folds: usize,
    /// Generation strategies to use
    pub generation_strategies: Vec<GenerationStrategy>,
    /// Selection methods to use
    pub selection_methods: Vec<SelectionMethod>,
}

impl Default for FeatureEngineeringConfig {
    fn default() -> Self {
        Self {
            auto_generation: true,
            feature_selection: true,
            max_generated_features: 1000,
            selection_threshold: 0.01,
            cv_folds: 5,
            generation_strategies: vec![
                GenerationStrategy::PatternBased,
                GenerationStrategy::StatisticalTransforms,
                GenerationStrategy::InteractionFeatures,
            ],
            selection_methods: vec![
                SelectionMethod::MutualInformation,
                SelectionMethod::VarianceThreshold,
                SelectionMethod::RecursiveElimination,
            ],
        }
    }
}

/// Feature generation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerationStrategy {
    /// Pattern-based feature generation from code analysis
    PatternBased,
    /// Statistical transforms (log, sqrt, polynomial)
    StatisticalTransforms,
    /// Feature interactions and combinations
    InteractionFeatures,
    /// N-gram based features from text analysis
    NGramFeatures,
    /// Frequency-based features
    FrequencyFeatures,
}

/// Feature selection methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMethod {
    /// Mutual information based selection
    MutualInformation,
    /// Variance threshold filtering
    VarianceThreshold,
    /// Recursive feature elimination
    RecursiveElimination,
    /// Correlation-based selection
    CorrelationBased,
    /// LASSO regularization
    LassoRegularization,
}

/// Generated features with metadata
#[derive(Debug, Clone)]
pub struct GeneratedFeatures {
    /// Feature vectors
    pub features: Vec<f32>,
    /// Feature names
    pub names: Vec<String>,
    /// Generation strategy used
    pub strategy: GenerationStrategy,
    /// Importance scores
    pub importance_scores: Vec<f64>,
    /// Selection mask (which features are selected)
    pub selection_mask: Vec<bool>,
}

/// Performance metrics for feature engineering
#[derive(Debug, Clone)]
pub struct FeatureEngineeringMetrics {
    pub total_features_generated: usize,
    pub features_selected: usize,
    pub generation_time_ms: u64,
    pub selection_time_ms: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Default for FeatureEngineeringMetrics {
    fn default() -> Self {
        Self {
            total_features_generated: 0,
            features_selected: 0,
            generation_time_ms: 0,
            selection_time_ms: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

impl AdvancedFeatureEngineer {
    /// Create a new advanced feature engineer
    pub fn new() -> Self {
        Self::with_config(FeatureEngineeringConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: FeatureEngineeringConfig) -> Self {
        let mut engineer = Self {
            base_extractor: UnifiedFeatureExtractor::new(),
            generators: Vec::new(),
            selectors: Vec::new(),
            config,
            feature_cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: FeatureEngineeringMetrics::default(),
        };

        engineer.initialize_generators();
        engineer.initialize_selectors();
        engineer
    }

    /// Initialize feature generators based on configuration
    fn initialize_generators(&mut self) {
        for strategy in &self.config.generation_strategies {
            match strategy {
                GenerationStrategy::PatternBased => {
                    self.generators.push(Box::new(PatternBasedGenerator::new()));
                }
                GenerationStrategy::StatisticalTransforms => {
                    self.generators
                        .push(Box::new(StatisticalTransformGenerator::new()));
                }
                GenerationStrategy::InteractionFeatures => {
                    self.generators
                        .push(Box::new(InteractionFeatureGenerator::new()));
                }
                GenerationStrategy::NGramFeatures => {
                    self.generators.push(Box::new(NGramFeatureGenerator::new()));
                }
                GenerationStrategy::FrequencyFeatures => {
                    self.generators
                        .push(Box::new(FrequencyFeatureGenerator::new()));
                }
            }
        }
    }

    /// Initialize feature selectors based on configuration
    fn initialize_selectors(&mut self) {
        for method in &self.config.selection_methods {
            match method {
                SelectionMethod::MutualInformation => {
                    self.selectors
                        .push(Box::new(MutualInformationSelector::new()));
                }
                SelectionMethod::VarianceThreshold => {
                    self.selectors
                        .push(Box::new(VarianceThresholdSelector::new()));
                }
                SelectionMethod::RecursiveElimination => {
                    self.selectors
                        .push(Box::new(RecursiveEliminationSelector::new()));
                }
                SelectionMethod::CorrelationBased => {
                    self.selectors
                        .push(Box::new(CorrelationBasedSelector::new()));
                }
                SelectionMethod::LassoRegularization => {
                    self.selectors
                        .push(Box::new(LassoRegularizationSelector::new()));
                }
            }
        }
    }

    /// Extract enhanced features with automated generation and selection
    pub async fn extract_enhanced_features(&mut self, finding: &Finding) -> Result<Vec<f32>> {
        let start_time = std::time::Instant::now();

        // Get base features from unified extractor
        let base_features = self.base_extractor.extract_features(finding).await?;

        if !self.config.auto_generation && !self.config.feature_selection {
            return Ok(base_features);
        }

        let cache_key = self.generate_cache_key(finding);

        // Check cache first
        {
            let cache = self.feature_cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                self.metrics.cache_hits += 1;
                return Ok(self.apply_selection(&cached.features, &cached.selection_mask));
            }
        }

        self.metrics.cache_misses += 1;

        // Generate additional features
        let mut all_features = base_features.clone();
        let mut all_names = self.base_extractor.get_feature_names();

        if self.config.auto_generation {
            let generated = self.generate_features(finding, &base_features).await?;
            all_features.extend(generated.features.clone());
            all_names.extend(generated.names.clone());
        }

        // Apply feature selection
        let selection_mask = if self.config.feature_selection {
            self.select_features(&all_features, &all_names, finding)
                .await?
        } else {
            vec![true; all_features.len()]
        };

        let selected_features = self.apply_selection(&all_features, &selection_mask);

        // Cache the result
        let total_features = all_features.len();
        let selected_count = selected_features.len();
        let generated_features = GeneratedFeatures {
            features: all_features,
            names: all_names,
            strategy: GenerationStrategy::PatternBased, // Default for mixed
            importance_scores: vec![1.0; selected_count],
            selection_mask,
        };

        {
            let mut cache = self.feature_cache.write().await;
            cache.insert(cache_key, generated_features);
        }

        let duration = start_time.elapsed();
        self.metrics.generation_time_ms += duration.as_millis() as u64;
        self.metrics.total_features_generated = total_features;
        self.metrics.features_selected = selected_count;

        Ok(selected_features)
    }

    /// Generate additional features using configured strategies
    async fn generate_features(
        &mut self,
        finding: &Finding,
        base_features: &[f32],
    ) -> Result<GeneratedFeatures> {
        let mut generated_features = Vec::new();
        let mut generated_names = Vec::new();

        for generator in &self.generators {
            let features = generator.generate(finding, base_features).await?;
            generated_features.extend(features.features);
            generated_names.extend(features.names);
        }

        // Limit number of generated features
        if generated_features.len() > self.config.max_generated_features {
            generated_features.truncate(self.config.max_generated_features);
            generated_names.truncate(self.config.max_generated_features);
        }

        let feature_count = generated_features.len();
        Ok(GeneratedFeatures {
            features: generated_features,
            names: generated_names,
            strategy: GenerationStrategy::PatternBased, // Mixed strategies
            importance_scores: vec![1.0; feature_count],
            selection_mask: vec![true; feature_count],
        })
    }

    /// Select features using configured methods
    async fn select_features(
        &mut self,
        features: &[f32],
        names: &[String],
        finding: &Finding,
    ) -> Result<Vec<bool>> {
        if self.selectors.is_empty() {
            return Ok(vec![true; features.len()]);
        }

        let mut selection_scores = vec![0.0; features.len()];

        // Apply each selector and aggregate scores
        for selector in &self.selectors {
            let scores = selector.score_features(features, names, finding).await?;
            for (i, score) in scores.iter().enumerate() {
                if i < selection_scores.len() {
                    selection_scores[i] += score;
                }
            }
        }

        // Normalize scores by number of selectors
        let num_selectors = self.selectors.len() as f64;
        for score in &mut selection_scores {
            *score /= num_selectors;
        }

        // Apply threshold
        Ok(selection_scores
            .iter()
            .map(|&score| score >= self.config.selection_threshold)
            .collect())
    }

    /// Apply selection mask to features
    fn apply_selection(&self, features: &[f32], selection_mask: &[bool]) -> Vec<f32> {
        features
            .iter()
            .zip(selection_mask.iter())
            .filter(|(_, &selected)| selected)
            .map(|(&feature, _)| feature)
            .collect()
    }

    /// Generate cache key for finding
    fn generate_cache_key(&self, finding: &Finding) -> String {
        format!(
            "{}_{}_{}",
            finding.file.to_string_lossy(),
            finding.line,
            finding.rule
        )
    }

    /// Get feature engineering metrics
    pub fn get_metrics(&self) -> &FeatureEngineeringMetrics {
        &self.metrics
    }

    /// Clear feature cache
    pub async fn clear_cache(&mut self) {
        let mut cache = self.feature_cache.write().await;
        cache.clear();
    }

    /// Update configuration
    pub fn update_config(&mut self, config: FeatureEngineeringConfig) {
        self.config = config;
        self.generators.clear();
        self.selectors.clear();
        self.initialize_generators();
        self.initialize_selectors();
    }
}

/// Trait for feature generators
#[async_trait::async_trait]
pub trait FeatureGenerator {
    async fn generate(&self, finding: &Finding, base_features: &[f32])
        -> Result<GeneratedFeatures>;
    fn name(&self) -> &str;
}

/// Trait for feature selectors
#[async_trait::async_trait]
pub trait FeatureSelector {
    async fn score_features(
        &self,
        features: &[f32],
        names: &[String],
        finding: &Finding,
    ) -> Result<Vec<f64>>;
    fn name(&self) -> &str;
}

/// Pattern-based feature generator
pub struct PatternBasedGenerator {
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
struct Pattern {
    name: String,
    regex: regex::Regex,
    weight: f64,
}

impl PatternBasedGenerator {
    pub fn new() -> Self {
        let patterns = vec![
            Pattern {
                name: "security_keywords".to_string(),
                regex: regex::Regex::new(r"(?i)(password|secret|key|token|auth|login|admin)")?,
                weight: 2.0,
            },
            Pattern {
                name: "sql_patterns".to_string(),
                regex: regex::Regex::new(r"(?i)(select|insert|update|delete|drop|union)")?,
                weight: 1.5,
            },
            Pattern {
                name: "file_operations".to_string(),
                regex: regex::Regex::new(r"(?i)(file|read|write|open|close|delete)")?,
                weight: 1.2,
            },
            Pattern {
                name: "network_operations".to_string(),
                regex: regex::Regex::new(r"(?i)(http|url|socket|connect|request|response)")?,
                weight: 1.3,
            },
            Pattern {
                name: "crypto_operations".to_string(),
                regex: regex::Regex::new(r"(?i)(encrypt|decrypt|hash|cipher|crypto|ssl|tls)")?,
                weight: 1.8,
            },
        ];

        Self { patterns }
    }
}

#[async_trait::async_trait]
impl FeatureGenerator for PatternBasedGenerator {
    async fn generate(
        &self,
        finding: &Finding,
        _base_features: &[f32],
    ) -> Result<GeneratedFeatures> {
        let content = format!(
            "{} {} {}",
            finding.rule,
            finding.message,
            finding.description.as_deref().unwrap_or("")
        );

        let mut features = Vec::new();
        let mut names = Vec::new();

        for pattern in &self.patterns {
            let matches = pattern.regex.find_iter(&content).count();
            let normalized_score = (matches as f64 * pattern.weight).min(10.0) / 10.0;

            features.push(normalized_score as f32);
            names.push(format!("pattern_{}", pattern.name));
        }

        // Add complexity features
        features.push((content.len() as f32).ln().max(0.0) / 10.0); // Log length
        names.push("content_log_length".to_string());

        features.push((content.split_whitespace().count() as f32) / 100.0); // Word count
        names.push("content_word_count".to_string());

        let feature_count = features.len();
        Ok(GeneratedFeatures {
            features,
            names,
            strategy: GenerationStrategy::PatternBased,
            importance_scores: vec![1.0; feature_count],
            selection_mask: vec![true; feature_count],
        })
    }

    fn name(&self) -> &str {
        "PatternBasedGenerator"
    }
}

/// Statistical transform generator
pub struct StatisticalTransformGenerator;

impl StatisticalTransformGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FeatureGenerator for StatisticalTransformGenerator {
    async fn generate(
        &self,
        finding: &Finding,
        base_features: &[f32],
    ) -> Result<GeneratedFeatures> {
        let mut features = Vec::new();
        let mut names = Vec::new();

        // Generate transformed features
        for (i, &value) in base_features.iter().enumerate() {
            // Log transform (with small epsilon to avoid log(0))
            let log_value = (value + 1e-8).ln();
            features.push(log_value);
            names.push(format!("log_feature_{}", i));

            // Square root transform
            let sqrt_value = value.sqrt();
            features.push(sqrt_value);
            names.push(format!("sqrt_feature_{}", i));

            // Squared transform
            let squared_value = value * value;
            features.push(squared_value);
            names.push(format!("squared_feature_{}", i));
        }

        // Add statistical aggregations
        let mean = base_features.iter().sum::<f32>() / base_features.len() as f32;
        let variance = base_features
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>()
            / base_features.len() as f32;

        features.push(mean);
        names.push("features_mean".to_string());

        features.push(variance.sqrt());
        names.push("features_std".to_string());

        // Add severity-based transforms
        let severity_score = match finding.severity {
            Severity::Critical => 4.0,
            Severity::High => 3.0,
            Severity::Medium => 2.0,
            Severity::Low => 1.0,
            Severity::Info => 0.5,
        };

        features.push(severity_score * mean);
        names.push("severity_weighted_mean".to_string());

        let feature_count = features.len();
        Ok(GeneratedFeatures {
            features,
            names,
            strategy: GenerationStrategy::StatisticalTransforms,
            importance_scores: vec![1.0; feature_count],
            selection_mask: vec![true; feature_count],
        })
    }

    fn name(&self) -> &str {
        "StatisticalTransformGenerator"
    }
}

/// Interaction feature generator
pub struct InteractionFeatureGenerator;

impl InteractionFeatureGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FeatureGenerator for InteractionFeatureGenerator {
    async fn generate(
        &self,
        _finding: &Finding,
        base_features: &[f32],
    ) -> Result<GeneratedFeatures> {
        let mut features = Vec::new();
        let mut names = Vec::new();

        // Generate pairwise interactions (limited to avoid explosion)
        let max_interactions = 20;
        let mut interaction_count = 0;

        for i in 0..base_features.len() {
            for j in (i + 1)..base_features.len() {
                if interaction_count >= max_interactions {
                    break;
                }

                let interaction = base_features[i] * base_features[j];
                features.push(interaction);
                names.push(format!("interaction_{}_{}", i, j));
                interaction_count += 1;
            }
            if interaction_count >= max_interactions {
                break;
            }
        }

        let feature_count = features.len();
        Ok(GeneratedFeatures {
            features,
            names,
            strategy: GenerationStrategy::InteractionFeatures,
            importance_scores: vec![1.0; feature_count],
            selection_mask: vec![true; feature_count],
        })
    }

    fn name(&self) -> &str {
        "InteractionFeatureGenerator"
    }
}

/// N-gram feature generator
pub struct NGramFeatureGenerator;

impl NGramFeatureGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FeatureGenerator for NGramFeatureGenerator {
    async fn generate(
        &self,
        finding: &Finding,
        _base_features: &[f32],
    ) -> Result<GeneratedFeatures> {
        let text = format!("{} {}", finding.rule, finding.message);
        let words: Vec<&str> = text.split_whitespace().collect();

        let mut features = Vec::new();
        let mut names = Vec::new();

        // Generate bigrams and trigrams
        for n in 2..=3 {
            let mut ngram_counts = HashMap::new();

            for window in words.windows(n) {
                let ngram = window.join("_");
                *ngram_counts.entry(ngram).or_insert(0) += 1;
            }

            // Take top frequent n-grams
            let mut sorted_ngrams: Vec<_> = ngram_counts.into_iter().collect();
            sorted_ngrams.sort_by(|a, b| b.1.cmp(&a.1));

            for (ngram, count) in sorted_ngrams.into_iter().take(5) {
                features.push((count as f32).ln().max(0.0));
                names.push(format!("ngram_{}_{}", n, ngram));
            }
        }

        let feature_count = features.len();
        Ok(GeneratedFeatures {
            features,
            names,
            strategy: GenerationStrategy::NGramFeatures,
            importance_scores: vec![1.0; feature_count],
            selection_mask: vec![true; feature_count],
        })
    }

    fn name(&self) -> &str {
        "NGramFeatureGenerator"
    }
}

/// Frequency feature generator
pub struct FrequencyFeatureGenerator;

impl FrequencyFeatureGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FeatureGenerator for FrequencyFeatureGenerator {
    async fn generate(
        &self,
        finding: &Finding,
        _base_features: &[f32],
    ) -> Result<GeneratedFeatures> {
        let text = format!(
            "{} {} {}",
            finding.rule,
            finding.message,
            finding.description.as_deref().unwrap_or("")
        );

        let mut features = Vec::new();
        let mut names = Vec::new();

        // Character frequency features
        let char_counts = text.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        // Key character frequencies
        let important_chars = [
            '(', ')', '{', '}', '[', ']', ';', ':', '=', '+', '-', '*', '/',
        ];
        for &ch in &important_chars {
            let count = char_counts.get(&ch).unwrap_or(&0);
            let frequency = *count as f32 / text.len() as f32;
            features.push(frequency);
            names.push(format!("char_freq_{}", ch as u8));
        }

        // Word length statistics
        let words: Vec<&str> = text.split_whitespace().collect();
        if !words.is_empty() {
            let total_length: usize = words.iter().map(|w| w.len()).sum();
            let avg_word_length = total_length as f32 / words.len() as f32;
            features.push(avg_word_length / 20.0); // Normalize
            names.push("avg_word_length".to_string());

            // Unique word ratio
            let unique_words: HashSet<&str> = words.iter().cloned().collect();
            let unique_ratio = unique_words.len() as f32 / words.len() as f32;
            features.push(unique_ratio);
            names.push("unique_word_ratio".to_string());
        }

        let feature_count = features.len();
        Ok(GeneratedFeatures {
            features,
            names,
            strategy: GenerationStrategy::FrequencyFeatures,
            importance_scores: vec![1.0; feature_count],
            selection_mask: vec![true; feature_count],
        })
    }

    fn name(&self) -> &str {
        "FrequencyFeatureGenerator"
    }
}

// Feature Selectors Implementation

/// Mutual Information based feature selector
pub struct MutualInformationSelector;

impl MutualInformationSelector {
    pub fn new() -> Self {
        Self
    }

    fn calculate_mutual_information(&self, feature_values: &[f32], target_severity: f32) -> f64 {
        // Simplified mutual information calculation
        // In a real implementation, you'd use proper MI calculation
        let correlation = self.calculate_correlation(feature_values, target_severity);
        correlation.abs() as f64
    }

    fn calculate_correlation(&self, x: &[f32], y: f32) -> f32 {
        if x.is_empty() {
            return 0.0;
        }

        let mean_x = x.iter().sum::<f32>() / x.len() as f32;
        let mean_y = y;

        let mut numerator = 0.0;
        let mut sum_sq_x = 0.0;

        for &val in x {
            let diff_x = val - mean_x;
            let diff_y = val - mean_y; // Simplified correlation with constant y
            numerator += diff_x * diff_y;
            sum_sq_x += diff_x * diff_x;
        }

        if sum_sq_x == 0.0 {
            return 0.0;
        }

        numerator / sum_sq_x.sqrt()
    }
}

#[async_trait::async_trait]
impl FeatureSelector for MutualInformationSelector {
    async fn score_features(
        &self,
        features: &[f32],
        _names: &[String],
        finding: &Finding,
    ) -> Result<Vec<f64>> {
        let target_severity = match finding.severity {
            Severity::Critical => 4.0,
            Severity::High => 3.0,
            Severity::Medium => 2.0,
            Severity::Low => 1.0,
            Severity::Info => 0.5,
        };

        // Group features and calculate MI for each
        let chunk_size = if features.len() > 100 {
            features.len() / 100
        } else {
            1
        };
        let mut scores = Vec::new();

        for chunk in features.chunks(chunk_size.max(1)) {
            let mi_score = self.calculate_mutual_information(chunk, target_severity);
            scores.extend(vec![mi_score; chunk.len()]);
        }

        // Ensure we have the right number of scores
        scores.truncate(features.len());
        while scores.len() < features.len() {
            scores.push(0.5); // Default score
        }

        Ok(scores)
    }

    fn name(&self) -> &str {
        "MutualInformationSelector"
    }
}

/// Variance threshold selector
pub struct VarianceThresholdSelector {
    threshold: f64,
}

impl VarianceThresholdSelector {
    pub fn new() -> Self {
        Self { threshold: 0.01 }
    }

    fn calculate_variance(&self, values: &[f32]) -> f64 {
        if values.len() <= 1 {
            return 0.0;
        }

        let mean = values.iter().sum::<f32>() / values.len() as f32;
        let variance =
            values.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / values.len() as f32;

        variance as f64
    }
}

#[async_trait::async_trait]
impl FeatureSelector for VarianceThresholdSelector {
    async fn score_features(
        &self,
        features: &[f32],
        _names: &[String],
        _finding: &Finding,
    ) -> Result<Vec<f64>> {
        let chunk_size = if features.len() > 50 {
            features.len() / 50
        } else {
            1
        };
        let mut scores = Vec::new();

        for chunk in features.chunks(chunk_size.max(1)) {
            let variance = self.calculate_variance(chunk);
            let score = if variance > self.threshold { 1.0 } else { 0.0 };
            scores.extend(vec![score; chunk.len()]);
        }

        scores.truncate(features.len());
        while scores.len() < features.len() {
            scores.push(0.5);
        }

        Ok(scores)
    }

    fn name(&self) -> &str {
        "VarianceThresholdSelector"
    }
}

/// Recursive feature elimination selector
pub struct RecursiveEliminationSelector;

impl RecursiveEliminationSelector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FeatureSelector for RecursiveEliminationSelector {
    async fn score_features(
        &self,
        features: &[f32],
        _names: &[String],
        finding: &Finding,
    ) -> Result<Vec<f64>> {
        // Simplified RFE - rank features by their individual predictive power
        let target_severity = match finding.severity {
            Severity::Critical => 4.0,
            Severity::High => 3.0,
            Severity::Medium => 2.0,
            Severity::Low => 1.0,
            Severity::Info => 0.5,
        };

        let mut scores = Vec::new();

        for &feature_val in features {
            // Simple scoring based on feature magnitude and target alignment
            let score = (feature_val * target_severity).abs() as f64;
            scores.push(score.min(1.0));
        }

        Ok(scores)
    }

    fn name(&self) -> &str {
        "RecursiveEliminationSelector"
    }
}

/// Correlation-based selector
pub struct CorrelationBasedSelector;

impl CorrelationBasedSelector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FeatureSelector for CorrelationBasedSelector {
    async fn score_features(
        &self,
        features: &[f32],
        _names: &[String],
        _finding: &Finding,
    ) -> Result<Vec<f64>> {
        let mut scores = Vec::new();

        // Calculate correlation with neighboring features
        for i in 0..features.len() {
            let mut correlation_sum = 0.0;
            let mut count = 0;

            // Check correlation with nearby features
            for j in 0..features.len() {
                if i != j {
                    let corr = self.simple_correlation(features[i], features[j]);
                    correlation_sum += corr.abs();
                    count += 1;
                }
            }

            let avg_correlation = if count > 0 {
                correlation_sum / count as f64
            } else {
                0.0
            };
            // Lower correlation with others = higher importance
            scores.push((1.0 - avg_correlation).max(0.0));
        }

        Ok(scores)
    }

    fn name(&self) -> &str {
        "CorrelationBasedSelector"
    }
}

impl CorrelationBasedSelector {
    fn simple_correlation(&self, x: f32, y: f32) -> f64 {
        // Simplified correlation for two values
        if x == 0.0 && y == 0.0 {
            return 1.0;
        }
        let diff = (x - y).abs();
        let max_val = x.abs().max(y.abs());
        if max_val == 0.0 {
            1.0
        } else {
            1.0 - (diff / max_val) as f64
        }
    }
}

/// LASSO regularization selector
pub struct LassoRegularizationSelector {
    lambda: f64,
}

impl LassoRegularizationSelector {
    pub fn new() -> Self {
        Self { lambda: 0.1 }
    }
}

#[async_trait::async_trait]
impl FeatureSelector for LassoRegularizationSelector {
    async fn score_features(
        &self,
        features: &[f32],
        _names: &[String],
        finding: &Finding,
    ) -> Result<Vec<f64>> {
        let target = match finding.severity {
            Severity::Critical => 4.0,
            Severity::High => 3.0,
            Severity::Medium => 2.0,
            Severity::Low => 1.0,
            Severity::Info => 0.5,
        };

        let mut scores = Vec::new();

        for &feature_val in features {
            // Simplified LASSO-like scoring
            let coefficient = if feature_val.abs() > self.lambda as f32 {
                feature_val.abs() - self.lambda as f32
            } else {
                0.0
            };

            let score = (coefficient * target).abs() as f64;
            scores.push(score.min(1.0));
        }

        Ok(scores)
    }

    fn name(&self) -> &str {
        "LassoRegularizationSelector"
    }
}
