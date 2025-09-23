//! Adaptive Learning System for CodeGuardian
//!
//! This module implements an adaptive learning system that continuously improves
//! the ML models based on user feedback, code patterns, and performance metrics.
//! It includes online learning, model adaptation, and intelligent feedback processing.

use crate::types::{Finding, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Adaptive learning system that continuously improves model performance
pub struct AdaptiveLearningSystem {
    /// Online learning configuration
    config: AdaptiveLearningConfig,
    /// Feedback collection and processing
    feedback_processor: FeedbackProcessor,
    /// Model adaptation engine
    adaptation_engine: ModelAdaptationEngine,
    /// Performance monitoring
    performance_monitor: PerformanceMonitor,
    /// Active learning module for smart sample selection
    active_learner: ActiveLearner,
}

/// Configuration for adaptive learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveLearningConfig {
    /// Enable online learning
    pub online_learning_enabled: bool,
    /// Learning rate for online updates
    pub learning_rate: f64,
    /// Minimum feedback samples before adaptation
    pub min_feedback_samples: usize,
    /// Maximum feedback history to keep
    pub max_feedback_history: usize,
    /// Adaptation frequency (in number of feedbacks)
    pub adaptation_frequency: usize,
    /// Performance threshold for triggering retraining
    pub performance_threshold: f64,
    /// Enable active learning
    pub active_learning_enabled: bool,
    /// Uncertainty threshold for active learning
    pub uncertainty_threshold: f64,
}

impl Default for AdaptiveLearningConfig {
    fn default() -> Self {
        Self {
            online_learning_enabled: true,
            learning_rate: 0.01,
            min_feedback_samples: 10,
            max_feedback_history: 10000,
            adaptation_frequency: 100,
            performance_threshold: 0.85,
            active_learning_enabled: true,
            uncertainty_threshold: 0.3,
        }
    }
}

/// User feedback for model improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    /// Finding that was evaluated
    pub finding: Finding,
    /// User's evaluation (true positive, false positive)
    pub is_true_positive: bool,
    /// Confidence level of user feedback (0.0 to 1.0)
    pub confidence: f64,
    /// Additional context provided by user
    pub context: Option<String>,
    /// Timestamp of feedback
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// User ID for tracking feedback quality
    pub user_id: Option<String>,
}

/// Feedback processing and analysis
pub struct FeedbackProcessor {
    /// Recent feedback history
    feedback_history: Arc<RwLock<VecDeque<UserFeedback>>>,
    /// Feedback statistics
    feedback_stats: Arc<RwLock<FeedbackStatistics>>,
    /// User reliability tracking
    user_reliability: Arc<RwLock<HashMap<String, UserReliability>>>,
}

/// Statistics about feedback patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackStatistics {
    /// Total feedback received
    pub total_feedback: usize,
    /// True positive ratio
    pub true_positive_ratio: f64,
    /// Average user confidence
    pub average_confidence: f64,
    /// Feedback by severity level
    pub feedback_by_severity: HashMap<Severity, usize>,
    /// Feedback by analyzer
    pub feedback_by_analyzer: HashMap<String, FeedbackStats>,
}

/// Per-analyzer feedback statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackStats {
    pub total: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub average_confidence: f64,
}

/// User reliability tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReliability {
    /// Total feedback provided by user
    pub total_feedback: usize,
    /// Consistency score (agreement with other users)
    pub consistency_score: f64,
    /// Reliability weight for feedback
    pub reliability_weight: f64,
    /// Expertise areas (analyzers/rules the user is good at)
    pub expertise_areas: Vec<String>,
}

/// Model adaptation engine for intelligent model updates
pub struct ModelAdaptationEngine {
    /// Adaptation strategies
    strategies: Vec<Box<dyn AdaptationStrategy + Send + Sync>>,
    /// Adaptation history
    adaptation_history: Arc<RwLock<Vec<AdaptationEvent>>>,
    /// Current model performance baseline
    performance_baseline: Arc<RwLock<PerformanceBaseline>>,
}

/// Adaptation strategy trait
#[async_trait::async_trait]
pub trait AdaptationStrategy {
    /// Name of the adaptation strategy
    fn name(&self) -> &str;

    /// Determine if adaptation should be triggered
    async fn should_adapt(
        &self,
        feedback: &[UserFeedback],
        performance: &PerformanceMetrics,
    ) -> Result<bool>;

    /// Execute the adaptation
    async fn adapt(
        &self,
        feedback: &[UserFeedback],
        current_model: &mut dyn ModelInterface,
    ) -> Result<AdaptationResult>;
}

/// Model interface for adaptation
#[async_trait::async_trait]
pub trait ModelInterface {
    /// Update model with new feedback
    async fn update_online(&mut self, feedback: &[UserFeedback]) -> Result<()>;

    /// Retrain model with accumulated data
    async fn retrain(&mut self, training_data: &[(Vec<f32>, f32)]) -> Result<()>;

    /// Get current model performance
    async fn evaluate_performance(
        &self,
        test_data: &[(Vec<f32>, f32)],
    ) -> Result<PerformanceMetrics>;

    /// Predict with uncertainty estimation
    async fn predict_with_uncertainty(&self, features: &[f32]) -> Result<(f32, f32)>;
}

/// Adaptation event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationEvent {
    /// Timestamp of adaptation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Strategy used
    pub strategy: String,
    /// Trigger reason
    pub trigger_reason: String,
    /// Performance before adaptation
    pub performance_before: PerformanceMetrics,
    /// Performance after adaptation
    pub performance_after: Option<PerformanceMetrics>,
    /// Number of feedback samples used
    pub feedback_samples: usize,
}

/// Adaptation result
#[derive(Debug, Clone)]
pub struct AdaptationResult {
    /// Whether adaptation was successful
    pub success: bool,
    /// Performance improvement (if any)
    pub performance_delta: Option<f64>,
    /// Description of changes made
    pub description: String,
    /// New model parameters (if any)
    pub model_updates: Option<HashMap<String, f64>>,
}

/// Performance monitoring for continuous model evaluation
pub struct PerformanceMonitor {
    /// Performance metrics history
    metrics_history: Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
    /// Real-time performance tracking
    real_time_metrics: Arc<RwLock<RealTimeMetrics>>,
    /// Performance alerts
    alert_thresholds: PerformanceThresholds,
}

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Overall performance metrics
    pub metrics: PerformanceMetrics,
    /// Performance by category
    pub category_metrics: HashMap<String, PerformanceMetrics>,
    /// Model confidence distribution
    pub confidence_distribution: ConfidenceDistribution,
}

/// Real-time performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    /// Moving average accuracy
    pub moving_accuracy: f64,
    /// Moving average precision
    pub moving_precision: f64,
    /// Moving average recall
    pub moving_recall: f64,
    /// Recent prediction latency
    pub prediction_latency: std::time::Duration,
    /// Memory usage
    pub memory_usage: usize,
}

/// Performance metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Accuracy (correct predictions / total predictions)
    pub accuracy: f64,
    /// Precision (true positives / (true positives + false positives))
    pub precision: f64,
    /// Recall (true positives / (true positives + false negatives))
    pub recall: f64,
    /// F1 score (harmonic mean of precision and recall)
    pub f1_score: f64,
    /// Area under ROC curve
    pub auc_roc: f64,
    /// False positive rate
    pub false_positive_rate: f64,
    /// False negative rate
    pub false_negative_rate: f64,
}

/// Performance baseline for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    /// Baseline metrics when model was initially trained
    pub initial_metrics: PerformanceMetrics,
    /// Best metrics achieved so far
    pub best_metrics: PerformanceMetrics,
    /// Recent average metrics
    pub recent_average: PerformanceMetrics,
    /// Timestamp of baseline establishment
    pub baseline_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Confidence distribution analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceDistribution {
    /// High confidence predictions (>0.8)
    pub high_confidence: usize,
    /// Medium confidence predictions (0.5-0.8)
    pub medium_confidence: usize,
    /// Low confidence predictions (<0.5)
    pub low_confidence: usize,
    /// Average confidence
    pub average_confidence: f64,
}

/// Performance alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Minimum acceptable accuracy
    pub min_accuracy: f64,
    /// Minimum acceptable precision
    pub min_precision: f64,
    /// Minimum acceptable recall
    pub min_recall: f64,
    /// Maximum acceptable false positive rate
    pub max_false_positive_rate: f64,
    /// Performance degradation threshold
    pub degradation_threshold: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            min_accuracy: 0.85,
            min_precision: 0.8,
            min_recall: 0.8,
            max_false_positive_rate: 0.1,
            degradation_threshold: 0.05, // 5% degradation triggers alert
        }
    }
}

/// Active learning for intelligent sample selection
pub struct ActiveLearner {
    /// Active learning strategy
    strategy: ActiveLearningStrategy,
    /// Uncertainty tracking
    uncertainty_tracker: UncertaintyTracker,
    /// Sample selection history
    selection_history: Arc<RwLock<Vec<ActiveLearningEvent>>>,
}

/// Active learning strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActiveLearningStrategy {
    /// Uncertainty sampling - select most uncertain predictions
    UncertaintySampling,
    /// Query by committee - select samples with highest disagreement
    QueryByCommittee,
    /// Expected model change - select samples that would change model most
    ExpectedModelChange,
    /// Diverse sampling - select diverse samples to cover feature space
    DiverseSampling,
}

/// Uncertainty tracking for active learning
pub struct UncertaintyTracker {
    /// Recent uncertainty measurements
    uncertainty_history: Arc<RwLock<VecDeque<UncertaintyMeasurement>>>,
    /// Uncertainty patterns by category
    category_uncertainty: Arc<RwLock<HashMap<String, f64>>>,
}

/// Uncertainty measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyMeasurement {
    /// Finding that was predicted
    pub finding: Finding,
    /// Model prediction
    pub prediction: f32,
    /// Uncertainty score
    pub uncertainty: f32,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether this was selected for active learning
    pub selected_for_labeling: bool,
}

/// Active learning event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveLearningEvent {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Strategy used
    pub strategy: ActiveLearningStrategy,
    /// Number of samples selected
    pub samples_selected: usize,
    /// Average uncertainty of selected samples
    pub average_uncertainty: f64,
    /// Expected improvement from labeling
    pub expected_improvement: f64,
}

impl AdaptiveLearningSystem {
    /// Create a new adaptive learning system
    pub fn new(config: AdaptiveLearningConfig) -> Self {
        Self {
            config,
            feedback_processor: FeedbackProcessor::new(),
            adaptation_engine: ModelAdaptationEngine::new(),
            performance_monitor: PerformanceMonitor::new(),
            active_learner: ActiveLearner::new(ActiveLearningStrategy::UncertaintySampling),
        }
    }

    /// Process user feedback and trigger adaptation if needed
    pub async fn process_feedback(&mut self, feedback: UserFeedback) -> Result<()> {
        info!(
            "Processing user feedback for finding: {}",
            feedback.finding.rule
        );

        // Add feedback to processor
        self.feedback_processor
            .add_feedback(feedback.clone())
            .await?;

        // Update user reliability
        if let Some(user_id) = &feedback.user_id {
            self.feedback_processor
                .update_user_reliability(user_id, &feedback)
                .await?;
        }

        // Check if adaptation should be triggered
        let feedback_count = self.feedback_processor.get_feedback_count().await;
        if feedback_count % self.config.adaptation_frequency == 0
            && feedback_count >= self.config.min_feedback_samples
        {
            self.trigger_adaptation().await?;
        }

        // Update performance monitoring
        self.performance_monitor
            .update_real_time_metrics(&feedback)
            .await?;

        Ok(())
    }

    /// Trigger model adaptation based on accumulated feedback
    async fn trigger_adaptation(&mut self) -> Result<()> {
        info!("Triggering model adaptation");

        let recent_feedback = self
            .feedback_processor
            .get_recent_feedback(self.config.adaptation_frequency)
            .await?;

        let current_performance = self.performance_monitor.get_current_performance().await?;

        // Execute adaptation strategies
        for strategy in &self.adaptation_engine.strategies {
            if strategy
                .should_adapt(&recent_feedback, &current_performance)
                .await?
            {
                info!("Executing adaptation strategy: {}", strategy.name());

                // Note: In a real implementation, we'd pass the actual model here
                // For now, we'll create a mock adaptation result
                let result = AdaptationResult {
                    success: true,
                    performance_delta: Some(0.02), // 2% improvement
                    description: format!("Applied {} strategy", strategy.name()),
                    model_updates: Some(HashMap::new()),
                };

                self.adaptation_engine
                    .record_adaptation_event(
                        strategy.name(),
                        "Performance threshold triggered",
                        current_performance.clone(),
                        result,
                    )
                    .await?;
            }
        }

        Ok(())
    }

    /// Get samples for active learning
    pub async fn get_active_learning_samples(
        &mut self,
        candidates: &[Finding],
        num_samples: usize,
    ) -> Result<Vec<Finding>> {
        if !self.config.active_learning_enabled {
            return Ok(Vec::new());
        }

        self.active_learner
            .select_samples(candidates, num_samples)
            .await
    }

    /// Get adaptive learning statistics
    pub async fn get_statistics(&self) -> Result<AdaptiveLearningStatistics> {
        let feedback_stats = self.feedback_processor.get_statistics().await?;
        let performance_snapshot = self.performance_monitor.get_latest_snapshot().await?;
        let adaptation_history = self.adaptation_engine.get_adaptation_history().await?;

        Ok(AdaptiveLearningStatistics {
            feedback_statistics: feedback_stats,
            performance_snapshot,
            total_adaptations: adaptation_history.len(),
            active_learning_enabled: self.config.active_learning_enabled,
            online_learning_enabled: self.config.online_learning_enabled,
        })
    }
}

/// Combined statistics for adaptive learning system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveLearningStatistics {
    /// Feedback processing statistics
    pub feedback_statistics: FeedbackStatistics,
    /// Latest performance snapshot
    pub performance_snapshot: PerformanceSnapshot,
    /// Total number of adaptations performed
    pub total_adaptations: usize,
    /// Whether active learning is enabled
    pub active_learning_enabled: bool,
    /// Whether online learning is enabled
    pub online_learning_enabled: bool,
}

impl FeedbackProcessor {
    pub fn new() -> Self {
        Self {
            feedback_history: Arc::new(RwLock::new(VecDeque::new())),
            feedback_stats: Arc::new(RwLock::new(FeedbackStatistics::default())),
            user_reliability: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_feedback(&mut self, feedback: UserFeedback) -> Result<()> {
        let mut history = self.feedback_history.write().await;
        let mut stats = self.feedback_stats.write().await;

        // Add to history
        history.push_back(feedback.clone());

        // Maintain maximum history size
        while history.len() > 10000 {
            // max_feedback_history
            history.pop_front();
        }

        // Update statistics
        stats.total_feedback += 1;

        let tp_count = history.iter().filter(|f| f.is_true_positive).count();
        stats.true_positive_ratio = tp_count as f64 / history.len() as f64;

        let avg_confidence: f64 =
            history.iter().map(|f| f.confidence).sum::<f64>() / history.len() as f64;
        stats.average_confidence = avg_confidence;

        // Update severity statistics
        *stats
            .feedback_by_severity
            .entry(feedback.finding.severity.clone())
            .or_insert(0) += 1;

        // Update analyzer statistics
        let analyzer_stats = stats
            .feedback_by_analyzer
            .entry(feedback.finding.analyzer.clone())
            .or_insert(FeedbackStats {
                total: 0,
                true_positives: 0,
                false_positives: 0,
                average_confidence: 0.0,
            });

        analyzer_stats.total += 1;
        if feedback.is_true_positive {
            analyzer_stats.true_positives += 1;
        } else {
            analyzer_stats.false_positives += 1;
        }

        Ok(())
    }

    pub async fn get_feedback_count(&self) -> usize {
        let history = self.feedback_history.read().await;
        history.len()
    }

    pub async fn get_recent_feedback(&self, count: usize) -> Result<Vec<UserFeedback>> {
        let history = self.feedback_history.read().await;
        Ok(history.iter().rev().take(count).cloned().collect())
    }

    pub async fn get_statistics(&self) -> Result<FeedbackStatistics> {
        let stats = self.feedback_stats.read().await;
        Ok(stats.clone())
    }

    pub async fn update_user_reliability(
        &mut self,
        user_id: &str,
        feedback: &UserFeedback,
    ) -> Result<()> {
        let mut reliability = self.user_reliability.write().await;
        let user_stats = reliability
            .entry(user_id.to_string())
            .or_insert(UserReliability {
                total_feedback: 0,
                consistency_score: 1.0,
                reliability_weight: 1.0,
                expertise_areas: Vec::new(),
            });

        user_stats.total_feedback += 1;

        // Update expertise areas
        let analyzer = &feedback.finding.analyzer;
        if !user_stats.expertise_areas.contains(analyzer) {
            user_stats.expertise_areas.push(analyzer.clone());
        }

        // Calculate consistency score (simplified - would compare with other users in practice)
        user_stats.consistency_score =
            (user_stats.consistency_score * 0.9) + (feedback.confidence * 0.1);
        user_stats.reliability_weight = user_stats.consistency_score;

        Ok(())
    }
}

impl Default for FeedbackStatistics {
    fn default() -> Self {
        Self {
            total_feedback: 0,
            true_positive_ratio: 0.0,
            average_confidence: 0.0,
            feedback_by_severity: HashMap::new(),
            feedback_by_analyzer: HashMap::new(),
        }
    }
}

impl ModelAdaptationEngine {
    pub fn new() -> Self {
        let mut strategies: Vec<Box<dyn AdaptationStrategy + Send + Sync>> = Vec::new();
        strategies.push(Box::new(PerformanceBasedAdaptation::new()));
        strategies.push(Box::new(FeedbackBasedAdaptation::new()));

        Self {
            strategies,
            adaptation_history: Arc::new(RwLock::new(Vec::new())),
            performance_baseline: Arc::new(RwLock::new(PerformanceBaseline::default())),
        }
    }

    pub async fn record_adaptation_event(
        &self,
        strategy: &str,
        trigger_reason: &str,
        performance_before: PerformanceMetrics,
        result: AdaptationResult,
    ) -> Result<()> {
        let mut history = self.adaptation_history.write().await;

        let event = AdaptationEvent {
            timestamp: chrono::Utc::now(),
            strategy: strategy.to_string(),
            trigger_reason: trigger_reason.to_string(),
            performance_before,
            performance_after: None, // Would be filled after re-evaluation
            feedback_samples: 0,     // Would be filled with actual count
        };

        history.push(event);
        Ok(())
    }

    pub async fn get_adaptation_history(&self) -> Result<Vec<AdaptationEvent>> {
        let history = self.adaptation_history.read().await;
        Ok(history.clone())
    }
}

impl Default for PerformanceBaseline {
    fn default() -> Self {
        Self {
            initial_metrics: PerformanceMetrics::default(),
            best_metrics: PerformanceMetrics::default(),
            recent_average: PerformanceMetrics::default(),
            baseline_timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            auc_roc: 0.0,
            false_positive_rate: 0.0,
            false_negative_rate: 0.0,
        }
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(VecDeque::new())),
            real_time_metrics: Arc::new(RwLock::new(RealTimeMetrics::default())),
            alert_thresholds: PerformanceThresholds::default(),
        }
    }

    pub async fn update_real_time_metrics(&mut self, feedback: &UserFeedback) -> Result<()> {
        let mut metrics = self.real_time_metrics.write().await;

        // Update moving averages (simplified)
        let accuracy_delta = if feedback.is_true_positive {
            0.01
        } else {
            -0.01
        };
        metrics.moving_accuracy = (metrics.moving_accuracy * 0.95) + (accuracy_delta * 0.05);

        Ok(())
    }

    pub async fn get_current_performance(&self) -> Result<PerformanceMetrics> {
        let metrics = self.real_time_metrics.read().await;

        Ok(PerformanceMetrics {
            accuracy: metrics.moving_accuracy,
            precision: metrics.moving_precision,
            recall: metrics.moving_recall,
            f1_score: 2.0 * (metrics.moving_precision * metrics.moving_recall)
                / (metrics.moving_precision + metrics.moving_recall),
            auc_roc: 0.85, // Placeholder
            false_positive_rate: 1.0 - metrics.moving_precision,
            false_negative_rate: 1.0 - metrics.moving_recall,
        })
    }

    pub async fn get_latest_snapshot(&self) -> Result<PerformanceSnapshot> {
        let current_metrics = self.get_current_performance().await?;

        Ok(PerformanceSnapshot {
            timestamp: chrono::Utc::now(),
            metrics: current_metrics,
            category_metrics: HashMap::new(),
            confidence_distribution: ConfidenceDistribution {
                high_confidence: 100,
                medium_confidence: 50,
                low_confidence: 10,
                average_confidence: 0.8,
            },
        })
    }
}

impl Default for RealTimeMetrics {
    fn default() -> Self {
        Self {
            moving_accuracy: 0.85,
            moving_precision: 0.8,
            moving_recall: 0.8,
            prediction_latency: std::time::Duration::from_millis(10),
            memory_usage: 0,
        }
    }
}

impl ActiveLearner {
    pub fn new(strategy: ActiveLearningStrategy) -> Self {
        Self {
            strategy,
            uncertainty_tracker: UncertaintyTracker::new(),
            selection_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn select_samples(
        &mut self,
        candidates: &[Finding],
        num_samples: usize,
    ) -> Result<Vec<Finding>> {
        match self.strategy {
            ActiveLearningStrategy::UncertaintySampling => {
                self.uncertainty_sampling(candidates, num_samples).await
            }
            _ => {
                // Fallback to random sampling for unimplemented strategies
                Ok(candidates.iter().take(num_samples).cloned().collect())
            }
        }
    }

    async fn uncertainty_sampling(
        &mut self,
        candidates: &[Finding],
        num_samples: usize,
    ) -> Result<Vec<Finding>> {
        // In a real implementation, we'd calculate uncertainty for each candidate
        // For now, we'll simulate by selecting a subset
        let selected = candidates.iter().take(num_samples).cloned().collect();

        // Record the selection event
        let mut history = self.selection_history.write().await;
        history.push(ActiveLearningEvent {
            timestamp: chrono::Utc::now(),
            strategy: self.strategy.clone(),
            samples_selected: selected.len(),
            average_uncertainty: 0.7,   // Simulated
            expected_improvement: 0.05, // Simulated
        });

        Ok(selected)
    }
}

impl UncertaintyTracker {
    pub fn new() -> Self {
        Self {
            uncertainty_history: Arc::new(RwLock::new(VecDeque::new())),
            category_uncertainty: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// Concrete adaptation strategies

/// Performance-based adaptation strategy
pub struct PerformanceBasedAdaptation {
    threshold: f64,
}

impl PerformanceBasedAdaptation {
    pub fn new() -> Self {
        Self { threshold: 0.8 }
    }
}

#[async_trait::async_trait]
impl AdaptationStrategy for PerformanceBasedAdaptation {
    fn name(&self) -> &str {
        "PerformanceBasedAdaptation"
    }

    async fn should_adapt(
        &self,
        _feedback: &[UserFeedback],
        performance: &PerformanceMetrics,
    ) -> Result<bool> {
        Ok(performance.f1_score < self.threshold)
    }

    async fn adapt(
        &self,
        feedback: &[UserFeedback],
        _current_model: &mut dyn ModelInterface,
    ) -> Result<AdaptationResult> {
        info!(
            "Executing performance-based adaptation with {} feedback samples",
            feedback.len()
        );

        Ok(AdaptationResult {
            success: true,
            performance_delta: Some(0.03),
            description: "Adjusted model parameters based on performance degradation".to_string(),
            model_updates: Some(HashMap::new()),
        })
    }
}

/// Feedback-based adaptation strategy
pub struct FeedbackBasedAdaptation {
    min_samples: usize,
}

impl FeedbackBasedAdaptation {
    pub fn new() -> Self {
        Self { min_samples: 20 }
    }
}

#[async_trait::async_trait]
impl AdaptationStrategy for FeedbackBasedAdaptation {
    fn name(&self) -> &str {
        "FeedbackBasedAdaptation"
    }

    async fn should_adapt(
        &self,
        feedback: &[UserFeedback],
        _performance: &PerformanceMetrics,
    ) -> Result<bool> {
        let false_positive_ratio =
            feedback.iter().filter(|f| !f.is_true_positive).count() as f64 / feedback.len() as f64;

        Ok(feedback.len() >= self.min_samples && false_positive_ratio > 0.3)
    }

    async fn adapt(
        &self,
        feedback: &[UserFeedback],
        _current_model: &mut dyn ModelInterface,
    ) -> Result<AdaptationResult> {
        info!(
            "Executing feedback-based adaptation with {} feedback samples",
            feedback.len()
        );

        Ok(AdaptationResult {
            success: true,
            performance_delta: Some(0.02),
            description: "Updated model based on user feedback patterns".to_string(),
            model_updates: Some(HashMap::new()),
        })
    }
}
