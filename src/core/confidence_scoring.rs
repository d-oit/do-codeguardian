//! Confidence scoring for validation results

use crate::types::Finding;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceScore(pub f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceFactors {
    pub pattern_match_strength: f64,
    pub context_relevance: f64,
    pub historical_accuracy: f64,
    pub cross_validation_score: f64,
}

#[derive(Debug, Clone)]
pub struct ConfidenceFactorsBuilder {
    factors: ConfidenceFactors,
}

impl ConfidenceFactorsBuilder {
    pub fn new() -> Self {
        Self {
            factors: ConfidenceFactors {
                pattern_match_strength: 0.5,
                context_relevance: 0.5,
                historical_accuracy: 0.5,
                cross_validation_score: 0.5,
            },
        }
    }
}

impl Default for ConfidenceFactorsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfidenceFactorsBuilder {
    pub fn pattern_match_strength(mut self, score: f64) -> Self {
        self.factors.pattern_match_strength = score.clamp(0.0, 1.0);
        self
    }

    pub fn context_relevance(mut self, score: f64) -> Self {
        self.factors.context_relevance = score.clamp(0.0, 1.0);
        self
    }

    pub fn historical_accuracy(mut self, score: f64) -> Self {
        self.factors.historical_accuracy = score.clamp(0.0, 1.0);
        self
    }

    pub fn cross_validation_score(mut self, score: f64) -> Self {
        self.factors.cross_validation_score = score.clamp(0.0, 1.0);
        self
    }

    pub fn build(self) -> ConfidenceFactors {
        self.factors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub pattern_match: f64,
    pub context: f64,
    pub historical: f64,
    pub cross_validation: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            pattern_match: 0.4,
            context: 0.3,
            historical: 0.2,
            cross_validation: 0.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdRecommendations {
    pub high_confidence: f64,
    pub medium_confidence: f64,
    pub low_confidence: f64,
    pub should_review: f64,
}

impl Default for ThresholdRecommendations {
    fn default() -> Self {
        Self {
            high_confidence: 0.8,
            medium_confidence: 0.6,
            low_confidence: 0.4,
            should_review: 0.3,
        }
    }
}

pub struct ConfidenceScorer {
    weights: ScoringWeights,
    baseline_scores: HashMap<String, f64>, // category -> accuracy
}

impl ConfidenceScorer {
    pub fn new() -> Self {
        Self {
            weights: ScoringWeights::default(),
            baseline_scores: HashMap::new(),
        }
    }
}

impl Default for ConfidenceScorer {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfidenceScorer {
    pub fn calculate_confidence(&self, factors: &ConfidenceFactors) -> ConfidenceScore {
        let score = factors.pattern_match_strength * self.weights.pattern_match
            + factors.context_relevance * self.weights.context
            + factors.historical_accuracy * self.weights.historical
            + factors.cross_validation_score * self.weights.cross_validation;

        ConfidenceScore(score.clamp(0.0, 1.0))
    }

    pub fn get_threshold_recommendations(&self, _findings: &[Finding]) -> ThresholdRecommendations {
        // Simple implementation - could be enhanced with statistical analysis
        ThresholdRecommendations::default()
    }

    pub fn update_baseline(&mut self, category: String, accuracy: f64) {
        self.baseline_scores
            .insert(category, accuracy.clamp(0.0, 1.0));
    }
}
