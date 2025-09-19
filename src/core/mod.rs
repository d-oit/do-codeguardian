//! Core functionality for CodeGuardian security analysis

pub mod ai_processor;
pub mod confidence_scoring;
pub mod conflict_resolution;
pub mod guardian_engine;
pub mod manual_review;
pub mod parallel_file_processor;
pub mod performance_monitor;
pub mod resource_manager;
pub mod result_aggregation;
pub mod retention;
pub mod swarm_orchestrator;
pub mod swarm_types;
pub mod task_decomposition;
pub mod validation_pipeline;

pub use ai_processor::AIProcessor;
pub use confidence_scoring::{
    ConfidenceFactors, ConfidenceFactorsBuilder, ConfidenceScore, ConfidenceScorer, ScoringWeights,
    ThresholdRecommendations,
};
pub use guardian_engine::{AnalysisStats, GuardianEngine};
pub use manual_review::{
    ManualReviewWorkflow, ReviewConfig, ReviewDecision, ReviewItem, ReviewPriority,
    ReviewStatistics, ReviewStatus, ReviewerFeedback,
};
pub use parallel_file_processor::*;
pub use retention::{CleanupReport, IntegrityReport, RepairReport, RetentionManager};
pub use validation_pipeline::{
    ValidationConfig, ValidationContext, ValidationLayer, ValidationMetrics, ValidationPipeline,
    ValidationResult, ValidationStatus,
};
