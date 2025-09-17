//! Advanced linking and relationship management
//!
//! Tracks relationships between artifacts across repositories, files, and systems
//! to provide enhanced traceability and organization across the ecosystem.

pub mod graph;
pub mod impact_analysis;
pub mod metadata;
pub mod visualization;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Relationship manager for tracking artifact relationships
pub struct RelationshipManager {
    graph: graph::RelationshipGraph,
    metadata_store: metadata::MetadataStore,
    visualizer: visualization::GraphVisualizer,
    impact_analyzer: impact_analysis::ImpactAnalyzer,
    config: RelationshipConfig,
}

/// Configuration for relationship management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipConfig {
    pub enabled: bool,
    pub max_relationships_per_artifact: usize,
    pub relationship_ttl_days: u32,
    pub auto_discovery_enabled: bool,
    pub visualization_enabled: bool,
    pub impact_analysis_enabled: bool,
    pub supported_artifact_types: Vec<ArtifactType>,
    pub relationship_weights: HashMap<RelationshipType, f64>,
}

impl Default for RelationshipConfig {
    fn default() -> Self {
        let mut relationship_weights = HashMap::new();
        relationship_weights.insert(RelationshipType::Duplicate, 0.9);
        relationship_weights.insert(RelationshipType::Similar, 0.7);
        relationship_weights.insert(RelationshipType::References, 0.6);
        relationship_weights.insert(RelationshipType::DependsOn, 0.8);
        relationship_weights.insert(RelationshipType::PartOf, 0.8);
        relationship_weights.insert(RelationshipType::RelatedTo, 0.5);
        relationship_weights.insert(RelationshipType::Implements, 0.7);
        relationship_weights.insert(RelationshipType::Tests, 0.6);

        Self {
            enabled: true,
            max_relationships_per_artifact: 50,
            relationship_ttl_days: 90,
            auto_discovery_enabled: true,
            visualization_enabled: true,
            impact_analysis_enabled: true,
            supported_artifact_types: vec![
                ArtifactType::SourceCode,
                ArtifactType::Documentation,
                ArtifactType::Configuration,
                ArtifactType::Issue,
                ArtifactType::PullRequest,
                ArtifactType::TestFile,
                ArtifactType::BuildScript,
                ArtifactType::DatabaseSchema,
            ],
            relationship_weights,
        }
    }
}

/// Types of artifacts that can have relationships
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ArtifactType {
    SourceCode,
    Documentation,
    Configuration,
    Issue,
    PullRequest,
    TestFile,
    BuildScript,
    DatabaseSchema,
    ApiDefinition,
    Deployment,
    Monitoring,
    Security,
}

/// Types of relationships between artifacts
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    Duplicate,
    Similar,
    References,
    DependsOn,
    PartOf,
    RelatedTo,
    Implements,
    Tests,
    Configures,
    Monitors,
    Deploys,
    Secures,
}

/// Artifact representation in the relationship system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub artifact_type: ArtifactType,
    pub name: String,
    pub path: String,
    pub repository: Option<String>,
    pub system: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub content_hash: Option<String>,
    pub size_bytes: Option<u64>,
    pub language: Option<String>,
    pub tags: Vec<String>,
}

/// Relationship between two artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_artifact_id: String,
    pub target_artifact_id: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub bidirectional: bool,
    pub auto_discovered: bool,
}

/// Relationship discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDiscoveryResult {
    pub discovered_relationships: Vec<Relationship>,
    pub confidence_scores: HashMap<String, f64>,
    pub discovery_method: DiscoveryMethod,
    pub processing_time_ms: u64,
    pub artifacts_analyzed: usize,
}

/// Methods for discovering relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    ContentSimilarity,
    StructuralAnalysis,
    ReferenceAnalysis,
    SemanticAnalysis,
    PatternMatching,
    MachineLearning,
    UserDefined,
    ImportAnalysis,
    CallGraphAnalysis,
}

/// Relationship query for searching relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipQuery {
    pub artifact_id: Option<String>,
    pub artifact_type: Option<ArtifactType>,
    pub relationship_type: Option<RelationshipType>,
    pub min_strength: Option<f64>,
    pub min_confidence: Option<f64>,
    pub repository: Option<String>,
    pub system: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
    pub include_transitive: bool,
    pub max_depth: Option<u32>,
}

/// Relationship search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipSearchResult {
    pub relationships: Vec<Relationship>,
    pub artifacts: HashMap<String, Artifact>,
    pub total_count: usize,
    pub query_time_ms: u64,
    pub relationship_paths: Vec<RelationshipPath>,
}

/// Path between artifacts through relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPath {
    pub source_artifact_id: String,
    pub target_artifact_id: String,
    pub path: Vec<String>,          // Artifact IDs in the path
    pub relationships: Vec<String>, // Relationship IDs in the path
    pub total_strength: f64,
    pub path_length: usize,
}

impl RelationshipManager {
    pub fn new(config: RelationshipConfig) -> Self {
        Self {
            graph: graph::RelationshipGraph::new(),
            metadata_store: metadata::MetadataStore::new(),
            visualizer: visualization::GraphVisualizer::new(),
            impact_analyzer: impact_analysis::ImpactAnalyzer::new(),
            config,
        }
    }

    /// Add an artifact to the relationship system
    pub async fn add_artifact(&mut self, artifact: Artifact) -> Result<()> {
        // Store artifact metadata
        self.metadata_store.store_artifact(&artifact).await?;

        // Add to graph
        self.graph.add_artifact(&artifact)?;

        // Auto-discover relationships if enabled
        if self.config.auto_discovery_enabled {
            self.discover_relationships_for_artifact(&artifact.id)
                .await?;
        }

        tracing::info!("Added artifact to relationship system: {}", artifact.id);
        Ok(())
    }

    /// Add a relationship between artifacts
    pub async fn add_relationship(&mut self, relationship: Relationship) -> Result<()> {
        // Validate relationship
        self.validate_relationship(&relationship)?;

        // Store relationship
        self.graph.add_relationship(&relationship)?;
        self.metadata_store
            .store_relationship(&relationship)
            .await?;

        tracing::info!(
            "Added relationship: {} -> {} ({})",
            relationship.source_artifact_id,
            relationship.target_artifact_id,
            format!("{:?}", relationship.relationship_type)
        );

        Ok(())
    }

    /// Discover relationships for a specific artifact
    pub async fn discover_relationships_for_artifact(
        &mut self,
        artifact_id: &str,
    ) -> Result<RelationshipDiscoveryResult> {
        let start_time = std::time::Instant::now();

        let artifact = self
            .metadata_store
            .get_artifact(artifact_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Artifact not found: {}", artifact_id))?;

        let mut discovered_relationships = Vec::new();
        let mut confidence_scores = HashMap::new();
        #[allow(unused_assignments)]
        let mut artifacts_analyzed = 0;

        // Get all other artifacts for comparison
        let all_artifacts = self.metadata_store.get_all_artifacts().await?;
        artifacts_analyzed = all_artifacts.len();

        for other_artifact in &all_artifacts {
            if other_artifact.id == artifact.id {
                continue;
            }

            // Discover relationships using different methods
            if let Some(relationships) = self
                .discover_content_similarity(&artifact, other_artifact)
                .await?
            {
                for rel in relationships {
                    confidence_scores.insert(rel.id.clone(), rel.confidence);
                    discovered_relationships.push(rel);
                }
            }

            if let Some(relationships) = self
                .discover_structural_relationships(&artifact, other_artifact)
                .await?
            {
                for rel in relationships {
                    confidence_scores.insert(rel.id.clone(), rel.confidence);
                    discovered_relationships.push(rel);
                }
            }

            if let Some(relationships) = self
                .discover_reference_relationships(&artifact, other_artifact)
                .await?
            {
                for rel in relationships {
                    confidence_scores.insert(rel.id.clone(), rel.confidence);
                    discovered_relationships.push(rel);
                }
            }
        }

        // Add discovered relationships to the graph
        for relationship in &discovered_relationships {
            self.add_relationship(relationship.clone()).await?;
        }

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(RelationshipDiscoveryResult {
            discovered_relationships,
            confidence_scores,
            discovery_method: DiscoveryMethod::ContentSimilarity,
            processing_time_ms: processing_time,
            artifacts_analyzed,
        })
    }

    /// Discover relationships based on content similarity
    async fn discover_content_similarity(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<Option<Vec<Relationship>>> {
        // Only compare artifacts of the same type for content similarity
        if artifact1.artifact_type != artifact2.artifact_type {
            return Ok(None);
        }

        // Calculate content similarity
        let similarity = self
            .calculate_content_similarity(artifact1, artifact2)
            .await?;

        if similarity < 0.7 {
            return Ok(None);
        }

        let relationship_type = if similarity > 0.95 {
            RelationshipType::Duplicate
        } else if similarity > 0.8 {
            RelationshipType::Similar
        } else {
            RelationshipType::RelatedTo
        };

        let relationship = Relationship {
            id: Uuid::new_v4().to_string(),
            source_artifact_id: artifact1.id.clone(),
            target_artifact_id: artifact2.id.clone(),
            relationship_type,
            strength: similarity,
            confidence: similarity * 0.9, // Slightly lower confidence than similarity
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: "auto_discovery".to_string(),
            metadata: HashMap::from([
                (
                    "discovery_method".to_string(),
                    serde_json::Value::String("content_similarity".to_string()),
                ),
                (
                    "similarity_score".to_string(),
                    serde_json::Value::Number(serde_json::Number::from_f64(similarity).unwrap()),
                ),
            ]),
            bidirectional: true,
            auto_discovered: true,
        };

        Ok(Some(vec![relationship]))
    }

    /// Discover structural relationships
    async fn discover_structural_relationships(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<Option<Vec<Relationship>>> {
        let mut relationships = Vec::new();

        // Check for dependency relationships
        if self
            .has_dependency_relationship(artifact1, artifact2)
            .await?
        {
            let relationship = Relationship {
                id: Uuid::new_v4().to_string(),
                source_artifact_id: artifact1.id.clone(),
                target_artifact_id: artifact2.id.clone(),
                relationship_type: RelationshipType::DependsOn,
                strength: 0.8,
                confidence: 0.85,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by: "auto_discovery".to_string(),
                metadata: HashMap::from([(
                    "discovery_method".to_string(),
                    serde_json::Value::String("structural_analysis".to_string()),
                )]),
                bidirectional: false,
                auto_discovered: true,
            };
            relationships.push(relationship);
        }

        // Check for test relationships
        if self.has_test_relationship(artifact1, artifact2).await? {
            let relationship = Relationship {
                id: Uuid::new_v4().to_string(),
                source_artifact_id: artifact1.id.clone(),
                target_artifact_id: artifact2.id.clone(),
                relationship_type: RelationshipType::Tests,
                strength: 0.9,
                confidence: 0.9,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by: "auto_discovery".to_string(),
                metadata: HashMap::from([(
                    "discovery_method".to_string(),
                    serde_json::Value::String("structural_analysis".to_string()),
                )]),
                bidirectional: false,
                auto_discovered: true,
            };
            relationships.push(relationship);
        }

        if relationships.is_empty() {
            Ok(None)
        } else {
            Ok(Some(relationships))
        }
    }

    /// Discover reference relationships
    async fn discover_reference_relationships(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<Option<Vec<Relationship>>> {
        // Check if artifact1 references artifact2
        if self
            .has_reference_relationship(artifact1, artifact2)
            .await?
        {
            let relationship = Relationship {
                id: Uuid::new_v4().to_string(),
                source_artifact_id: artifact1.id.clone(),
                target_artifact_id: artifact2.id.clone(),
                relationship_type: RelationshipType::References,
                strength: 0.7,
                confidence: 0.8,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by: "auto_discovery".to_string(),
                metadata: HashMap::from([(
                    "discovery_method".to_string(),
                    serde_json::Value::String("reference_analysis".to_string()),
                )]),
                bidirectional: false,
                auto_discovered: true,
            };
            Ok(Some(vec![relationship]))
        } else {
            Ok(None)
        }
    }

    /// Calculate content similarity between artifacts
    async fn calculate_content_similarity(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<f64> {
        // Use content hashes if available
        if let (Some(hash1), Some(hash2)) = (&artifact1.content_hash, &artifact2.content_hash) {
            if hash1 == hash2 {
                return Ok(1.0);
            }
        }

        // Use ML-based similarity if available
        #[cfg(feature = "ml")]
        {
            if let Ok(similarity) = self.calculate_ml_similarity(artifact1, artifact2).await {
                return Ok(similarity);
            }
        }

        // Fallback to simple text similarity
        self.calculate_text_similarity(artifact1, artifact2).await
    }

    #[cfg(feature = "ml")]
    async fn calculate_ml_similarity(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<f64> {
        // Use the pattern recognition engine for similarity calculation
        use crate::ml::pattern_recognition::PatternRecognitionEngine;

        // This would integrate with the ML pattern recognition system
        // For now, return a placeholder
        Ok(0.5)
    }

    async fn calculate_text_similarity(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<f64> {
        // Simple text similarity based on artifact names and paths
        let name_similarity = self.calculate_string_similarity(&artifact1.name, &artifact2.name);
        let path_similarity = self.calculate_string_similarity(&artifact1.path, &artifact2.path);

        Ok((name_similarity + path_similarity) / 2.0)
    }

    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f64 {
        let words1: HashSet<&str> = s1.split_whitespace().collect();
        let words2: HashSet<&str> = s2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Check for dependency relationships
    async fn has_dependency_relationship(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<bool> {
        // Check if artifact1 depends on artifact2 based on file paths, imports, etc.
        if artifact1.path.contains(&artifact2.name) || artifact2.path.contains(&artifact1.name) {
            return Ok(true);
        }

        // Check for import/include relationships
        // This would require content analysis
        Ok(false)
    }

    /// Check for test relationships
    async fn has_test_relationship(
        &self,
        artifact1: &Artifact,
        artifact2: &Artifact,
    ) -> Result<bool> {
        // Check if artifact1 is a test for artifact2
        if artifact1.artifact_type == ArtifactType::TestFile
            && (artifact1.path.contains(&artifact2.name)
                || artifact1.name.contains(&artifact2.name))
        {
            return Ok(true);
        }
        Ok(false)
    }

    /// Check for reference relationships
    async fn has_reference_relationship(
        &self,
        _artifact1: &Artifact,
        _artifact2: &Artifact,
    ) -> Result<bool> {
        // Check if artifact1 references artifact2
        // This would require content analysis to find references
        Ok(false)
    }

    /// Search for relationships
    pub async fn search_relationships(
        &self,
        query: RelationshipQuery,
    ) -> Result<RelationshipSearchResult> {
        let start_time = std::time::Instant::now();

        let relationships = self.graph.search_relationships(&query)?;
        let mut artifacts = HashMap::new();

        // Collect all artifacts involved in the relationships
        for relationship in &relationships {
            if let Some(artifact) = self
                .metadata_store
                .get_artifact(&relationship.source_artifact_id)
                .await?
            {
                artifacts.insert(artifact.id.clone(), artifact);
            }
            if let Some(artifact) = self
                .metadata_store
                .get_artifact(&relationship.target_artifact_id)
                .await?
            {
                artifacts.insert(artifact.id.clone(), artifact);
            }
        }

        // Find relationship paths if requested
        let relationship_paths = if query.include_transitive {
            self.find_relationship_paths(&query).await?
        } else {
            Vec::new()
        };

        let query_time = start_time.elapsed().as_millis() as u64;

        Ok(RelationshipSearchResult {
            total_count: relationships.len(),
            relationships,
            artifacts,
            query_time_ms: query_time,
            relationship_paths,
        })
    }

    /// Find paths between artifacts through relationships
    async fn find_relationship_paths(
        &self,
        query: &RelationshipQuery,
    ) -> Result<Vec<RelationshipPath>> {
        // Use graph algorithms to find paths
        self.graph.find_paths(query).await
    }

    /// Generate visualization of relationships
    pub async fn generate_visualization(
        &self,
        query: RelationshipQuery,
    ) -> Result<visualization::GraphVisualization> {
        if !self.config.visualization_enabled {
            return Err(anyhow::anyhow!("Visualization is disabled"));
        }

        let search_result = self.search_relationships(query).await?;
        self.visualizer.generate_visualization(&search_result).await
    }

    /// Perform impact analysis for changes
    pub async fn analyze_impact(
        &self,
        artifact_id: &str,
        change_type: impact_analysis::ChangeType,
    ) -> Result<impact_analysis::ImpactAnalysisResult> {
        if !self.config.impact_analysis_enabled {
            return Err(anyhow::anyhow!("Impact analysis is disabled"));
        }

        self.impact_analyzer
            .analyze_impact(artifact_id, change_type, &self.graph)
            .await
    }

    /// Validate a relationship before adding it
    fn validate_relationship(&self, relationship: &Relationship) -> Result<()> {
        // Check if source and target are different
        if relationship.source_artifact_id == relationship.target_artifact_id {
            return Err(anyhow::anyhow!("Self-relationships are not allowed"));
        }

        // Check strength and confidence bounds
        if relationship.strength < 0.0 || relationship.strength > 1.0 {
            return Err(anyhow::anyhow!(
                "Relationship strength must be between 0.0 and 1.0"
            ));
        }

        if relationship.confidence < 0.0 || relationship.confidence > 1.0 {
            return Err(anyhow::anyhow!(
                "Relationship confidence must be between 0.0 and 1.0"
            ));
        }

        Ok(())
    }

    /// Get relationship statistics
    pub async fn get_statistics(&self) -> Result<RelationshipStatistics> {
        let total_artifacts = self.metadata_store.count_artifacts().await?;
        let total_relationships = self.graph.count_relationships()?;
        let relationships_by_type = self.graph.count_relationships_by_type()?;
        let artifacts_by_type = self.metadata_store.count_artifacts_by_type().await?;

        Ok(RelationshipStatistics {
            total_artifacts,
            total_relationships,
            relationships_by_type,
            artifacts_by_type,
            average_relationships_per_artifact: if total_artifacts > 0 {
                total_relationships as f64 / total_artifacts as f64
            } else {
                0.0
            },
            last_updated: Utc::now(),
        })
    }

    /// Clean up old relationships
    pub async fn cleanup_old_relationships(&mut self) -> Result<usize> {
        let cutoff_date =
            Utc::now() - chrono::Duration::days(self.config.relationship_ttl_days as i64);
        let removed_count = self.graph.remove_relationships_older_than(cutoff_date)?;

        tracing::info!("Cleaned up {} old relationships", removed_count);
        Ok(removed_count)
    }
}

/// Statistics about relationships in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipStatistics {
    pub total_artifacts: usize,
    pub total_relationships: usize,
    pub relationships_by_type: HashMap<RelationshipType, usize>,
    pub artifacts_by_type: HashMap<ArtifactType, usize>,
    pub average_relationships_per_artifact: f64,
    pub last_updated: DateTime<Utc>,
}
