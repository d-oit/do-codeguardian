//! Core functionality for CodeGuardian security analysis

pub mod confidence_scoring;
pub mod guardian_engine;
pub mod manual_review;
pub mod parallel_file_processor;
pub mod validation_pipeline;

pub use confidence_scoring::{
    ConfidenceFactors, ConfidenceFactorsBuilder, ConfidenceScore, ConfidenceScorer, ScoringWeights,
    ThresholdRecommendations,
};
pub use guardian_engine::{AnalysisStats, GuardianEngine};
pub use manual_review::{
    ManualReviewWorkflow, ReviewConfig, ReviewDecision, ReviewItem, ReviewPriority,
    ReviewStatistics, ReviewStatus, ReviewerFeedback,
};
pub use validation_pipeline::{
    ValidationConfig, ValidationContext, ValidationLayer, ValidationMetrics, ValidationPipeline,
    ValidationResult, ValidationStatus,
};
