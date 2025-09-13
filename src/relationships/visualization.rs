//! Graph visualization for relationships

use super::{RelationshipSearchResult, Artifact, Relationship};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Graph visualizer for relationship data
pub struct GraphVisualizer {
    config: VisualizationConfig,
}

/// Configuration for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub layout_algorithm: LayoutAlgorithm,
    pub node_size_metric: NodeSizeMetric,
    pub edge_thickness_metric: EdgeThicknessMetric,
    pub color_scheme: ColorScheme,
    pub max_nodes: usize,
    pub max_edges: usize,
    pub show_labels: bool,
    pub show_edge_weights: bool,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            layout_algorithm: LayoutAlgorithm::ForceDirected,
            node_size_metric: NodeSizeMetric::RelationshipCount,
            edge_thickness_metric: EdgeThicknessMetric::Strength,
            color_scheme: ColorScheme::ArtifactType,
            max_nodes: 100,
            max_edges: 200,
            show_labels: true,
            show_edge_weights: true,
        }
    }
}

/// Layout algorithms for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutAlgorithm {
    ForceDirected,
    Hierarchical,
    Circular,
    Grid,
    Radial,
}

/// Metrics for determining node size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeSizeMetric {
    RelationshipCount,
    FileSize,
    Importance,
    Uniform,
}

/// Metrics for determining edge thickness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeThicknessMetric {
    Strength,
    Confidence,
    Uniform,
}

/// Color schemes for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    ArtifactType,
    Repository,
    System,
    RelationshipStrength,
    Custom(HashMap<String, String>),
}

/// Complete graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVisualization {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
    pub layout: LayoutData,
    pub metadata: VisualizationMetadata,
    pub interactive_features: InteractiveFeatures,
}

/// Node in the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationNode {
    pub id: String,
    pub label: String,
    pub artifact_type: String,
    pub size: f64,
    pub color: String,
    pub position: Option<Position>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub tooltip: String,
}

/// Edge in the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub relationship_type: String,
    pub thickness: f64,
    pub color: String,
    pub label: Option<String>,
    pub strength: f64,
    pub confidence: f64,
    pub bidirectional: bool,
}

/// Position coordinates for nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>, // For 3D visualizations
}

/// Layout data for the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutData {
    pub algorithm: LayoutAlgorithm,
    pub bounds: BoundingBox,
    pub scale: f64,
    pub center: Position,
}

/// Bounding box for the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

/// Metadata about the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationMetadata {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub generation_time_ms: u64,
    pub filters_applied: Vec<String>,
    pub statistics: VisualizationStatistics,
}

/// Statistics about the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationStatistics {
    pub nodes_by_type: HashMap<String, usize>,
    pub edges_by_type: HashMap<String, usize>,
    pub average_degree: f64,
    pub max_degree: usize,
    pub connected_components: usize,
}

/// Interactive features for the visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveFeatures {
    pub zoom_enabled: bool,
    pub pan_enabled: bool,
    pub node_selection: bool,
    pub edge_selection: bool,
    pub filtering: Vec<FilterOption>,
    pub search_enabled: bool,
}

/// Filter options for interactive visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOption {
    pub name: String,
    pub filter_type: FilterType,
    pub values: Vec<String>,
    pub default_value: Option<String>,
}

/// Types of filters available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    ArtifactType,
    RelationshipType,
    Repository,
    System,
    StrengthRange,
    ConfidenceRange,
}

impl GraphVisualizer {
    pub fn new() -> Self {
        Self {
            config: VisualizationConfig::default(),
        }
    }
}

impl Default for GraphVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphVisualizer {
    pub fn with_config(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Generate visualization from search results
    pub async fn generate_visualization(&self, search_result: &RelationshipSearchResult) -> Result<GraphVisualization> {
        let start_time = std::time::Instant::now();

        // Limit nodes and edges based on configuration
        let limited_relationships = self.limit_relationships(&search_result.relationships);
        let limited_artifacts = self.limit_artifacts(&search_result.artifacts, &limited_relationships);

        // Generate nodes
        let nodes = self.generate_nodes(&limited_artifacts).await?;

        // Generate edges
        let edges = self.generate_edges(&limited_relationships).await?;

        // Calculate layout
        let layout = self.calculate_layout(&nodes, &edges).await?;

        // Generate metadata
        let generation_time = start_time.elapsed().as_millis() as u64;
        let metadata = self.generate_metadata(&nodes, &edges, generation_time);

        // Configure interactive features
        let interactive_features = self.generate_interactive_features(&limited_artifacts, &limited_relationships);

        Ok(GraphVisualization {
            nodes,
            edges,
            layout,
            metadata,
            interactive_features,
        })
    }

    /// Limit relationships based on configuration
    fn limit_relationships(&self, relationships: &[Relationship]) -> Vec<Relationship> {
        let mut limited = relationships.to_vec();

        // Sort by strength descending
        limited.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap_or(std::cmp::Ordering::Equal));

        // Limit to max_edges
        limited.truncate(self.config.max_edges);

        limited
    }

    /// Limit artifacts based on relationships and configuration
    fn limit_artifacts(&self, artifacts: &HashMap<String, Artifact>, relationships: &[Relationship]) -> HashMap<String, Artifact> {
        let mut artifact_scores = HashMap::new();

        // Calculate importance scores based on relationship count and strength
        for relationship in relationships {
            let source_score = artifact_scores.entry(relationship.source_artifact_id.clone()).or_insert(0.0);
            *source_score += relationship.strength;

            let target_score = artifact_scores.entry(relationship.target_artifact_id.clone()).or_insert(0.0);
            *target_score += relationship.strength;
        }

        // Sort artifacts by score
        let mut scored_artifacts: Vec<_> = artifact_scores.into_iter().collect();
        scored_artifacts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top artifacts up to max_nodes
        let mut limited_artifacts = HashMap::new();
        for (artifact_id, _score) in scored_artifacts.into_iter().take(self.config.max_nodes) {
            if let Some(artifact) = artifacts.get(&artifact_id) {
                limited_artifacts.insert(artifact_id, artifact.clone());
            }
        }

        limited_artifacts
    }

    /// Generate visualization nodes
    async fn generate_nodes(&self, artifacts: &HashMap<String, Artifact>) -> Result<Vec<VisualizationNode>> {
        let mut nodes = Vec::new();

        for artifact in artifacts.values() {
            let size = self.calculate_node_size(artifact, artifacts);
            let color = self.calculate_node_color(artifact);
            let tooltip = self.generate_node_tooltip(artifact);

            let node = VisualizationNode {
                id: artifact.id.clone(),
                label: artifact.name.clone(),
                artifact_type: format!("{:?}", artifact.artifact_type),
                size,
                color,
                position: None, // Will be set during layout calculation
                metadata: artifact.metadata.clone(),
                tooltip,
            };

            nodes.push(node);
        }

        Ok(nodes)
    }

    /// Generate visualization edges
    async fn generate_edges(&self, relationships: &[Relationship]) -> Result<Vec<VisualizationEdge>> {
        let mut edges = Vec::new();

        for relationship in relationships {
            let thickness = self.calculate_edge_thickness(relationship);
            let color = self.calculate_edge_color(relationship);
            let label = if self.config.show_edge_weights {
                Some(format!("{:.2}", relationship.strength))
            } else {
                None
            };

            let edge = VisualizationEdge {
                id: relationship.id.clone(),
                source: relationship.source_artifact_id.clone(),
                target: relationship.target_artifact_id.clone(),
                relationship_type: format!("{:?}", relationship.relationship_type),
                thickness,
                color,
                label,
                strength: relationship.strength,
                confidence: relationship.confidence,
                bidirectional: relationship.bidirectional,
            };

            edges.push(edge);
        }

        Ok(edges)
    }

    /// Calculate node size based on configured metric
    fn calculate_node_size(&self, artifact: &Artifact, _all_artifacts: &HashMap<String, Artifact>) -> f64 {
        match self.config.node_size_metric {
            NodeSizeMetric::RelationshipCount => {
                // This would require relationship count data
                10.0 // Default size
            },
            NodeSizeMetric::FileSize => {
                artifact.size_bytes.unwrap_or(1000) as f64 / 1000.0 + 5.0
            },
            NodeSizeMetric::Importance => {
                // Calculate importance based on various factors
                let base_size = 8.0;
                let type_multiplier = match artifact.artifact_type {
                    super::ArtifactType::SourceCode => 1.2,
                    super::ArtifactType::Configuration => 1.1,
                    super::ArtifactType::Documentation => 1.0,
                    _ => 0.9,
                };
                base_size * type_multiplier
            },
            NodeSizeMetric::Uniform => 10.0,
        }
    }

    /// Calculate node color based on configured scheme
    fn calculate_node_color(&self, artifact: &Artifact) -> String {
        match &self.config.color_scheme {
            ColorScheme::ArtifactType => {
                match artifact.artifact_type {
                    super::ArtifactType::SourceCode => "#3498db".to_string(),
                    super::ArtifactType::Documentation => "#2ecc71".to_string(),
                    super::ArtifactType::Configuration => "#f39c12".to_string(),
                    super::ArtifactType::Issue => "#e74c3c".to_string(),
                    super::ArtifactType::TestFile => "#9b59b6".to_string(),
                    _ => "#95a5a6".to_string(),
                }
            },
            ColorScheme::Repository => {
                // Generate color based on repository name hash
                let hash = self.hash_string(artifact.repository.as_deref().unwrap_or("default"));
                self.hash_to_color(hash)
            },
            ColorScheme::System => {
                let hash = self.hash_string(artifact.system.as_deref().unwrap_or("default"));
                self.hash_to_color(hash)
            },
            ColorScheme::RelationshipStrength => "#3498db".to_string(), // Not applicable for nodes
            ColorScheme::Custom(colors) => {
                colors.get(&format!("{:?}", artifact.artifact_type))
                    .cloned()
                    .unwrap_or_else(|| "#95a5a6".to_string())
            },
        }
    }

    /// Calculate edge thickness based on configured metric
    fn calculate_edge_thickness(&self, relationship: &Relationship) -> f64 {
        match self.config.edge_thickness_metric {
            EdgeThicknessMetric::Strength => {
                1.0 + (relationship.strength * 4.0) // 1-5 pixel range
            },
            EdgeThicknessMetric::Confidence => {
                1.0 + (relationship.confidence * 4.0)
            },
            EdgeThicknessMetric::Uniform => 2.0,
        }
    }

    /// Calculate edge color
    fn calculate_edge_color(&self, relationship: &Relationship) -> String {
        match relationship.relationship_type {
            super::RelationshipType::Duplicate => "#e74c3c".to_string(),
            super::RelationshipType::Similar => "#f39c12".to_string(),
            super::RelationshipType::References => "#3498db".to_string(),
            super::RelationshipType::DependsOn => "#9b59b6".to_string(),
            super::RelationshipType::Tests => "#2ecc71".to_string(),
            _ => "#95a5a6".to_string(),
        }
    }

    /// Generate tooltip for node
    fn generate_node_tooltip(&self, artifact: &Artifact) -> String {
        format!(
            "Name: {}\nType: {:?}\nPath: {}\nRepository: {}\nCreated: {}",
            artifact.name,
            artifact.artifact_type,
            artifact.path,
            artifact.repository.as_deref().unwrap_or("N/A"),
            artifact.created_at.format("%Y-%m-%d %H:%M")
        )
    }

    /// Calculate layout for nodes and edges
    async fn calculate_layout(&self, nodes: &[VisualizationNode], edges: &[VisualizationEdge]) -> Result<LayoutData> {
        match self.config.layout_algorithm {
            LayoutAlgorithm::ForceDirected => self.calculate_force_directed_layout(nodes, edges).await,
            LayoutAlgorithm::Hierarchical => self.calculate_hierarchical_layout(nodes, edges).await,
            LayoutAlgorithm::Circular => self.calculate_circular_layout(nodes, edges).await,
            LayoutAlgorithm::Grid => self.calculate_grid_layout(nodes, edges).await,
            LayoutAlgorithm::Radial => self.calculate_radial_layout(nodes, edges).await,
        }
    }

    /// Calculate force-directed layout
    async fn calculate_force_directed_layout(&self, _nodes: &[VisualizationNode], _edges: &[VisualizationEdge]) -> Result<LayoutData> {
        // Simplified force-directed layout
        let bounds = BoundingBox {
            min_x: 0.0,
            max_x: 800.0,
            min_y: 0.0,
            max_y: 600.0,
        };

        Ok(LayoutData {
            algorithm: LayoutAlgorithm::ForceDirected,
            bounds,
            scale: 1.0,
            center: Position { x: 400.0, y: 300.0, z: None },
        })
    }

    /// Calculate hierarchical layout
    async fn calculate_hierarchical_layout(&self, _nodes: &[VisualizationNode], _edges: &[VisualizationEdge]) -> Result<LayoutData> {
        let bounds = BoundingBox {
            min_x: 0.0,
            max_x: 1000.0,
            min_y: 0.0,
            max_y: 800.0,
        };

        Ok(LayoutData {
            algorithm: LayoutAlgorithm::Hierarchical,
            bounds,
            scale: 1.0,
            center: Position { x: 500.0, y: 400.0, z: None },
        })
    }

    /// Calculate circular layout
    async fn calculate_circular_layout(&self, _nodes: &[VisualizationNode], _edges: &[VisualizationEdge]) -> Result<LayoutData> {
        let bounds = BoundingBox {
            min_x: -300.0,
            max_x: 300.0,
            min_y: -300.0,
            max_y: 300.0,
        };

        Ok(LayoutData {
            algorithm: LayoutAlgorithm::Circular,
            bounds,
            scale: 1.0,
            center: Position { x: 0.0, y: 0.0, z: None },
        })
    }

    /// Calculate grid layout
    async fn calculate_grid_layout(&self, nodes: &[VisualizationNode], _edges: &[VisualizationEdge]) -> Result<LayoutData> {
        let grid_size = (nodes.len() as f64).sqrt().ceil() as usize;
        let cell_size = 100.0;
        let total_size = grid_size as f64 * cell_size;

        let bounds = BoundingBox {
            min_x: 0.0,
            max_x: total_size,
            min_y: 0.0,
            max_y: total_size,
        };

        Ok(LayoutData {
            algorithm: LayoutAlgorithm::Grid,
            bounds,
            scale: 1.0,
            center: Position { x: total_size / 2.0, y: total_size / 2.0, z: None },
        })
    }

    /// Calculate radial layout
    async fn calculate_radial_layout(&self, _nodes: &[VisualizationNode], _edges: &[VisualizationEdge]) -> Result<LayoutData> {
        let radius = 250.0;
        let bounds = BoundingBox {
            min_x: -radius,
            max_x: radius,
            min_y: -radius,
            max_y: radius,
        };

        Ok(LayoutData {
            algorithm: LayoutAlgorithm::Radial,
            bounds,
            scale: 1.0,
            center: Position { x: 0.0, y: 0.0, z: None },
        })
    }

    /// Generate visualization metadata
    fn generate_metadata(&self, nodes: &[VisualizationNode], edges: &[VisualizationEdge], generation_time_ms: u64) -> VisualizationMetadata {
        let mut nodes_by_type = HashMap::new();
        for node in nodes {
            *nodes_by_type.entry(node.artifact_type.clone()).or_insert(0) += 1;
        }

        let mut edges_by_type = HashMap::new();
        for edge in edges {
            *edges_by_type.entry(edge.relationship_type.clone()).or_insert(0) += 1;
        }

        let total_degree: usize = nodes.len() * 2; // Simplified calculation
        let average_degree = if nodes.is_empty() { 0.0 } else { total_degree as f64 / nodes.len() as f64 };

        VisualizationMetadata {
            total_nodes: nodes.len(),
            total_edges: edges.len(),
            generated_at: chrono::Utc::now(),
            generation_time_ms,
            filters_applied: vec![], // Would be populated based on actual filters
            statistics: VisualizationStatistics {
                nodes_by_type,
                edges_by_type,
                average_degree,
                max_degree: 10, // Simplified
                connected_components: 1, // Simplified
            },
        }
    }

    /// Generate interactive features
    fn generate_interactive_features(&self, artifacts: &HashMap<String, Artifact>, relationships: &[Relationship]) -> InteractiveFeatures {
        let mut artifact_types = HashSet::new();
        let mut repositories = HashSet::new();
        let mut systems = HashSet::new();

        for artifact in artifacts.values() {
            artifact_types.insert(format!("{:?}", artifact.artifact_type));
            if let Some(repo) = &artifact.repository {
                repositories.insert(repo.clone());
            }
            if let Some(system) = &artifact.system {
                systems.insert(system.clone());
            }
        }

        let mut relationship_types = HashSet::new();
        for relationship in relationships {
            relationship_types.insert(format!("{:?}", relationship.relationship_type));
        }

        let mut filtering = Vec::new();

        filtering.push(FilterOption {
            name: "Artifact Type".to_string(),
            filter_type: FilterType::ArtifactType,
            values: artifact_types.into_iter().collect(),
            default_value: None,
        });

        filtering.push(FilterOption {
            name: "Relationship Type".to_string(),
            filter_type: FilterType::RelationshipType,
            values: relationship_types.into_iter().collect(),
            default_value: None,
        });

        if !repositories.is_empty() {
            filtering.push(FilterOption {
                name: "Repository".to_string(),
                filter_type: FilterType::Repository,
                values: repositories.into_iter().collect(),
                default_value: None,
            });
        }

        if !systems.is_empty() {
            filtering.push(FilterOption {
                name: "System".to_string(),
                filter_type: FilterType::System,
                values: systems.into_iter().collect(),
                default_value: None,
            });
        }

        InteractiveFeatures {
            zoom_enabled: true,
            pan_enabled: true,
            node_selection: true,
            edge_selection: true,
            filtering,
            search_enabled: true,
        }
    }

    /// Hash string to generate consistent colors
    fn hash_string(&self, s: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Convert hash to color
    fn hash_to_color(&self, hash: u64) -> String {
        let r = ((hash >> 16) & 0xFF) as u8;
        let g = ((hash >> 8) & 0xFF) as u8;
        let b = (hash & 0xFF) as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
}

use std::collections::HashSet;
