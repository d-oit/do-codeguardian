//! # AI Enhancement Module
//!
//! This module provides AI-driven enhancements for CodeGuardian analysis results,
//! including semantic annotations, relationship mapping, and intelligent insights.

pub mod context;
pub mod enricher;
pub mod insights;
pub mod relationships;

use crate::types::AnalysisResults;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enhanced analysis results with AI-generated insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedAnalysisResults {
    /// Original analysis results
    pub base_results: AnalysisResults,
    /// AI-generated semantic annotations
    pub semantic_annotations: SemanticAnnotations,
    /// Relationship mappings between findings
    pub relationships: Vec<FindingRelationship>,
    /// AI-generated insights and recommendations
    pub insights: Vec<Insight>,
    /// Context enrichment data
    pub context: ContextData,
    /// Enhancement metadata
    pub enhancement_metadata: EnhancementMetadata,
}

/// Semantic annotations for findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnnotations {
    /// Finding classifications
    pub classifications: HashMap<String, FindingClassification>,
    /// Business impact assessments
    pub impact_assessments: HashMap<String, ImpactAssessment>,
    /// Technical debt indicators
    pub tech_debt_indicators: HashMap<String, TechDebtIndicator>,
    /// Security risk assessments
    pub security_risks: HashMap<String, SecurityRisk>,
}

/// Classification of a finding with confidence scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingClassification {
    /// Primary category (e.g., "Security", "Performance", "Maintainability")
    pub primary_category: String,
    /// Secondary categories
    pub secondary_categories: Vec<String>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Classification reasoning
    pub reasoning: String,
    /// Suggested tags
    pub suggested_tags: Vec<String>,
}

/// Business impact assessment for a finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    /// Business criticality (Critical, High, Medium, Low)
    pub criticality: String,
    /// Affected business areas
    pub affected_areas: Vec<String>,
    /// Estimated fix effort (in hours)
    pub estimated_effort: Option<f32>,
    /// Risk to business operations
    pub operational_risk: String,
    /// Compliance implications
    pub compliance_impact: Vec<String>,
}

/// Technical debt indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtIndicator {
    /// Debt type (Code Quality, Architecture, Documentation, etc.)
    pub debt_type: String,
    /// Debt level (1-10 scale)
    pub debt_level: u8,
    /// Accumulation trend (Increasing, Stable, Decreasing)
    pub trend: String,
    /// Recommended remediation strategy
    pub remediation_strategy: String,
    /// Estimated cost of addressing the debt
    pub estimated_cost: Option<f32>,
}

/// Security risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRisk {
    /// CVSS base score
    pub cvss_score: Option<f32>,
    /// Attack vector (Network, Adjacent, Local, Physical)
    pub attack_vector: Option<String>,
    /// Attack complexity (Low, High)
    pub attack_complexity: Option<String>,
    /// Exploitability assessment
    pub exploitability: String,
    /// Potential impact description
    pub impact_description: String,
    /// Mitigation recommendations
    pub mitigations: Vec<String>,
}

/// Relationship between findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingRelationship {
    /// Source finding ID
    pub source_id: String,
    /// Target finding ID
    pub target_id: String,
    /// Relationship type
    pub relationship_type: RelationshipType,
    /// Relationship strength (0.0 - 1.0)
    pub strength: f64,
    /// Description of the relationship
    pub description: String,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Types of relationships between findings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    /// One finding causes another
    Causes,
    /// Findings are related to the same root cause
    SharedRootCause,
    /// Findings affect the same component
    SameComponent,
    /// Findings have similar patterns
    SimilarPattern,
    /// Findings are duplicates
    Duplicate,
    /// One finding makes another more severe
    Amplifies,
    /// Findings mask each other
    Masks,
    /// Findings are part of the same vulnerability chain
    VulnerabilityChain,
}

/// AI-generated insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    /// Insight ID
    pub id: String,
    /// Insight type
    pub insight_type: InsightType,
    /// Title of the insight
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Priority level (Critical, High, Medium, Low)
    pub priority: String,
    /// Affected finding IDs
    pub affected_findings: Vec<String>,
    /// Recommended actions
    pub recommendations: Vec<Recommendation>,
    /// Supporting data
    pub supporting_data: HashMap<String, serde_json::Value>,
}

/// Types of AI insights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsightType {
    /// Security vulnerability pattern
    SecurityPattern,
    /// Performance bottleneck
    PerformanceIssue,
    /// Code quality concern
    QualityIssue,
    /// Architectural concern
    ArchitecturalIssue,
    /// Compliance violation
    ComplianceIssue,
    /// Maintenance burden
    MaintenanceIssue,
    /// Risk accumulation
    RiskAccumulation,
    /// Positive pattern (good practice)
    PositivePattern,
}

/// AI-generated recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation ID
    pub id: String,
    /// Action to take
    pub action: String,
    /// Priority level
    pub priority: u8,
    /// Estimated effort (in hours)
    pub estimated_effort: Option<f32>,
    /// Expected benefit
    pub expected_benefit: String,
    /// Implementation details
    pub implementation_details: Option<String>,
    /// Tools or resources needed
    pub required_resources: Vec<String>,
}

/// Context data for enhanced understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextData {
    /// Project context information
    pub project_context: ProjectContext,
    /// Code context for findings
    pub code_context: HashMap<String, CodeContext>,
    /// Historical context
    pub historical_context: HistoricalContext,
    /// Environmental context
    pub environment_context: EnvironmentContext,
}

/// Project-level context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    /// Project type (web, mobile, desktop, etc.)
    pub project_type: Option<String>,
    /// Primary programming languages
    pub languages: Vec<String>,
    /// Frameworks and libraries used
    pub frameworks: Vec<String>,
    /// Project maturity level
    pub maturity_level: Option<String>,
    /// Team size estimate
    pub team_size: Option<u32>,
    /// Development methodology
    pub development_methodology: Option<String>,
}

/// Code context for a specific finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeContext {
    /// Function or method context
    pub function_context: Option<String>,
    /// Class or module context
    pub class_context: Option<String>,
    /// File purpose/role
    pub file_role: Option<String>,
    /// Code complexity metrics
    pub complexity_metrics: HashMap<String, f32>,
    /// Dependencies and imports
    pub dependencies: Vec<String>,
}

/// Historical context for trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalContext {
    /// Previous analysis results summary
    pub previous_results: Vec<HistoricalResult>,
    /// Trend analysis
    pub trends: Vec<Trend>,
    /// Recurring issues
    pub recurring_issues: Vec<RecurringIssue>,
}

/// Historical analysis result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalResult {
    /// Analysis timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Number of findings by severity
    pub findings_by_severity: HashMap<String, usize>,
    /// Config hash used
    pub config_hash: String,
}

/// Trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    /// Metric being tracked
    pub metric: String,
    /// Trend direction (Improving, Worsening, Stable)
    pub direction: String,
    /// Trend strength (0.0 - 1.0)
    pub strength: f64,
    /// Time period for the trend
    pub time_period: String,
}

/// Recurring issue pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringIssue {
    /// Issue pattern description
    pub pattern: String,
    /// Number of occurrences
    pub occurrence_count: u32,
    /// Files commonly affected
    pub common_files: Vec<String>,
    /// Suggested systematic fix
    pub systematic_fix: Option<String>,
}

/// Environmental context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentContext {
    /// Target deployment environment
    pub deployment_environment: Option<String>,
    /// Performance requirements
    pub performance_requirements: Vec<String>,
    /// Security requirements
    pub security_requirements: Vec<String>,
    /// Compliance requirements
    pub compliance_requirements: Vec<String>,
}

/// Enhancement metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementMetadata {
    /// Enhancement engine version
    pub engine_version: String,
    /// Processing timestamp
    pub processed_at: chrono::DateTime<chrono::Utc>,
    /// Processing duration in milliseconds
    pub processing_duration_ms: u64,
    /// Models used for enhancement
    pub models_used: Vec<String>,
    /// Enhancement quality score
    pub quality_score: f64,
    /// Confidence distribution
    pub confidence_distribution: HashMap<String, f64>,
}

/// Configuration for AI enhancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEnhancementConfig {
    /// Enable semantic enrichment
    pub enable_semantic_enrichment: bool,
    /// Enable relationship detection
    pub enable_relationship_detection: bool,
    /// Enable insight generation
    pub enable_insight_generation: bool,
    /// Enable context analysis
    pub enable_context_analysis: bool,
    /// Minimum confidence threshold for insights
    pub min_confidence_threshold: f64,
    /// Maximum processing time in seconds
    pub max_processing_time: u32,
    /// Enable historical analysis
    pub enable_historical_analysis: bool,
    /// Model configuration
    pub model_config: ModelConfig,
}

/// AI model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Classification model path
    pub classification_model: Option<String>,
    /// Relationship model path
    pub relationship_model: Option<String>,
    /// Context model path
    pub context_model: Option<String>,
    /// Use pre-trained models
    pub use_pretrained: bool,
    /// Model cache directory
    pub cache_directory: Option<String>,
}

impl Default for AIEnhancementConfig {
    fn default() -> Self {
        Self {
            enable_semantic_enrichment: true,
            enable_relationship_detection: true,
            enable_insight_generation: true,
            enable_context_analysis: false, // Disabled by default due to complexity
            min_confidence_threshold: 0.7,
            max_processing_time: 300,          // 5 minutes
            enable_historical_analysis: false, // Disabled by default
            model_config: ModelConfig::default(),
        }
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            classification_model: None,
            relationship_model: None,
            context_model: None,
            use_pretrained: true,
            cache_directory: Some(".codeguardian/models".to_string()),
        }
    }
}

/// Main AI enhancement engine
pub trait AIEnhancementEngine: Send + Sync {
    /// Enhance analysis results with AI capabilities
    fn enhance_results(
        &self,
        results: &AnalysisResults,
        config: &AIEnhancementConfig,
    ) -> Result<EnhancedAnalysisResults>;

    /// Get enhancement capabilities
    fn get_capabilities(&self) -> Vec<String>;

    /// Check if the engine is available and ready
    fn is_available(&self) -> bool;
}

/// Create an AI enhancement engine based on available capabilities
pub fn create_enhancement_engine() -> Result<Box<dyn AIEnhancementEngine>> {
    // For now, return a basic implementation
    // In the future, this could detect available ML libraries and models
    Ok(Box::new(enricher::BasicEnhancementEngine::new()?))
}
