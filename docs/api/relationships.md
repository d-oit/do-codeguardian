# Relationships API Documentation

This document provides comprehensive API documentation for CodeGuardian's advanced relationship management system, introduced in v0.2.0.

## Table of Contents

- [Relationship Manager](#relationship-manager)
- [Artifacts](#artifacts)
- [Relationships](#relationships)
- [Discovery Methods](#discovery-methods)
- [Graph Operations](#graph-operations)
- [Impact Analysis](#impact-analysis)
- [Visualization](#visualization)
- [Query Interface](#query-interface)
- [Metadata Management](#metadata-management)

## Relationship Manager

### RelationshipManager

```rust
pub struct RelationshipManager {
    graph: graph::RelationshipGraph,
    metadata_store: metadata::MetadataStore,
    visualizer: visualization::GraphVisualizer,
    impact_analyzer: impact_analysis::ImpactAnalyzer,
    config: RelationshipConfig,
}
```

**Methods:**
```rust
impl RelationshipManager {
    pub fn new(config: RelationshipConfig) -> Self;
    pub async fn add_artifact(&mut self, artifact: Artifact) -> Result<()>;
    pub async fn add_relationship(&mut self, relationship: Relationship) -> Result<()>;
    pub async fn discover_relationships_for_artifact(&mut self, artifact_id: &str) -> Result<RelationshipDiscoveryResult>;
    pub async fn search_relationships(&self, query: RelationshipQuery) -> Result<RelationshipSearchResult>;
    pub async fn analyze_impact(&self, artifact_id: &str, change_type: impact_analysis::ChangeType) -> Result<impact_analysis::ImpactAnalysisResult>;
    pub async fn generate_visualization(&self, query: RelationshipQuery) -> Result<visualization::GraphVisualization>;
    pub async fn get_statistics(&self) -> Result<RelationshipStatistics>;
    pub async fn cleanup_old_relationships(&mut self) -> Result<usize>;
}
```

### RelationshipConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipConfig {
    pub enabled: bool,
    pub max_relationships_per_artifact: usize,
    pub relationship_ttl_days: u32,
    pub auto_discovery_enabled: bool,
    pub visualization_enabled: bool,
    pub impact_analysis_enabled: bool,
    pub supported_artifact_types: Vec<ArtifactType>,
    pub relationship_weights: HashMap<String, f64>,
}
```

## Artifacts

### Artifact

```rust
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
```

### ArtifactType

```rust
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
```

### Artifact Management Example

```rust
use do_codeguardian::relationships::{RelationshipManager, Artifact, ArtifactType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = RelationshipManager::new(config);

    // Add source code artifact
    let source_artifact = Artifact {
        id: "src-main-rs".to_string(),
        artifact_type: ArtifactType::SourceCode,
        name: "main.rs".to_string(),
        path: "src/main.rs".to_string(),
        repository: Some("my-project".to_string()),
        system: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: HashMap::from([
            ("language".to_string(), serde_json::json!("rust")),
            ("lines_of_code".to_string(), serde_json::json!(150)),
        ]),
        content_hash: Some("abc123...".to_string()),
        size_bytes: Some(2048),
        language: Some("rust".to_string()),
        tags: vec!["main".to_string(), "entry-point".to_string()],
    };

    manager.add_artifact(source_artifact).await?;

    // Add documentation artifact
    let doc_artifact = Artifact {
        id: "readme-md".to_string(),
        artifact_type: ArtifactType::Documentation,
        name: "README.md".to_string(),
        path: "README.md".to_string(),
        repository: Some("my-project".to_string()),
        system: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        metadata: HashMap::from([
            ("format".to_string(), serde_json::json!("markdown")),
            ("sections".to_string(), serde_json::json!(["introduction", "usage", "api"])),
        ]),
        content_hash: Some("def456...".to_string()),
        size_bytes: Some(1024),
        language: None,
        tags: vec!["documentation".to_string(), "readme".to_string()],
    };

    manager.add_artifact(doc_artifact).await?;
    println!("Artifacts added successfully");

    Ok(())
}
```

## Relationships

### Relationship

```rust
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
```

### RelationshipType

```rust
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
```

### Relationship Creation Example

```rust
use do_codeguardian::relationships::{RelationshipManager, Relationship, RelationshipType};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = RelationshipManager::new(config);

    // Create dependency relationship
    let dependency_relationship = Relationship {
        id: Uuid::new_v4().to_string(),
        source_artifact_id: "src-main-rs".to_string(),
        target_artifact_id: "src-lib-rs".to_string(),
        relationship_type: RelationshipType::DependsOn,
        strength: 0.9,
        confidence: 0.95,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: "auto_discovery".to_string(),
        metadata: HashMap::from([
            ("import_type".to_string(), serde_json::json!("direct")),
            ("functions_used".to_string(), serde_json::json!(["parse_config", "validate_input"])),
        ]),
        bidirectional: false,
        auto_discovered: true,
    };

    manager.add_relationship(dependency_relationship).await?;

    // Create test relationship
    let test_relationship = Relationship {
        id: Uuid::new_v4().to_string(),
        source_artifact_id: "tests-main-rs".to_string(),
        target_artifact_id: "src-main-rs".to_string(),
        relationship_type: RelationshipType::Tests,
        strength: 0.8,
        confidence: 0.9,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: "auto_discovery".to_string(),
        metadata: HashMap::from([
            ("test_type".to_string(), serde_json::json!("unit_test")),
            ("coverage".to_string(), serde_json::json!(85.5)),
        ]),
        bidirectional: false,
        auto_discovered: true,
    };

    manager.add_relationship(test_relationship).await?;
    println!("Relationships added successfully");

    Ok(())
}
```

## Discovery Methods

### RelationshipDiscoveryResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDiscoveryResult {
    pub discovered_relationships: Vec<Relationship>,
    pub confidence_scores: HashMap<String, f64>,
    pub discovery_method: DiscoveryMethod,
    pub processing_time_ms: u64,
    pub artifacts_analyzed: usize,
}
```

### DiscoveryMethod

```rust
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
```

### Auto-Discovery Example

```rust
use do_codeguardian::relationships::RelationshipManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut manager = RelationshipManager::new(config);

    // Discover relationships for an artifact
    let discovery_result = manager.discover_relationships_for_artifact("src-main-rs").await?;

    println!("Discovery completed:");
    println!("- Method: {:?}", discovery_result.discovery_method);
    println!("- Processing time: {}ms", discovery_result.processing_time_ms);
    println!("- Artifacts analyzed: {}", discovery_result.artifacts_analyzed);
    println!("- Relationships discovered: {}", discovery_result.discovered_relationships.len());

    for relationship in &discovery_result.discovered_relationships {
        println!("  - {} -> {} ({:?}, strength: {:.2}, confidence: {:.2})",
                 relationship.source_artifact_id,
                 relationship.target_artifact_id,
                 relationship.relationship_type,
                 relationship.strength,
                 relationship.confidence);
    }

    Ok(())
}
```

## Graph Operations

### RelationshipGraph

```rust
pub struct RelationshipGraph {
    nodes: HashMap<String, graph::Node>,
    edges: HashMap<String, graph::Edge>,
    adjacency_list: HashMap<String, Vec<String>>,
}
```

**Methods:**
```rust
impl RelationshipGraph {
    pub fn new() -> Self;
    pub fn add_artifact(&mut self, artifact: &Artifact) -> Result<()>;
    pub fn add_relationship(&mut self, relationship: &Relationship) -> Result<()>;
    pub fn search_relationships(&self, query: &RelationshipQuery) -> Result<Vec<Relationship>>;
    pub fn find_paths(&self, query: &RelationshipQuery) -> Result<Vec<RelationshipPath>>;
    pub fn get_related_artifacts(&self, artifact_id: &str, max_depth: u32) -> Result<Vec<String>>;
    pub fn calculate_centrality(&self, artifact_id: &str) -> Result<f64>;
}
```

### RelationshipPath

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPath {
    pub source_artifact_id: String,
    pub target_artifact_id: String,
    pub path: Vec<String>,          // Artifact IDs in the path
    pub relationships: Vec<String>, // Relationship IDs in the path
    pub total_strength: f64,
    pub path_length: usize,
}
```

### Graph Operations Example

```rust
use do_codeguardian::relationships::{RelationshipManager, RelationshipQuery, RelationshipType};

// Find all artifacts related to a specific artifact
let related_artifacts = manager.graph.get_related_artifacts("src-main-rs", 2)?;
println!("Related artifacts: {:?}", related_artifacts);

// Search for specific relationship types
let query = RelationshipQuery {
    artifact_id: Some("src-main-rs".to_string()),
    relationship_type: Some(RelationshipType::DependsOn),
    min_strength: Some(0.7),
    ..Default::default()
};

let relationships = manager.search_relationships(query).await?;
println!("Found {} dependency relationships", relationships.relationships.len());

// Find paths between artifacts
let path_query = RelationshipQuery {
    artifact_id: Some("src-main-rs".to_string()),
    include_transitive: true,
    max_depth: Some(3),
    ..Default::default()
};

let search_result = manager.search_relationships(path_query).await?;
println!("Found {} relationship paths", search_result.relationship_paths.len());

for path in &search_result.relationship_paths {
    println!("Path: {} -> {} (length: {}, strength: {:.2})",
             path.source_artifact_id,
             path.target_artifact_id,
             path.path_length,
             path.total_strength);
}
```

## Impact Analysis

### ImpactAnalysisResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisResult {
    pub artifact_id: String,
    pub change_type: ChangeType,
    pub affected_artifacts: Vec<AffectedArtifact>,
    pub risk_level: RiskLevel,
    pub estimated_effort: u32,
    pub recommendations: Vec<String>,
    pub analysis_timestamp: DateTime<Utc>,
}
```

### ChangeType

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Modify,
    Delete,
    Rename,
    Move,
    Refactor,
    SecurityUpdate,
    BreakingChange,
}
```

### AffectedArtifact

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedArtifact {
    pub artifact_id: String,
    pub impact_level: ImpactLevel,
    pub relationship_strength: f64,
    pub required_actions: Vec<String>,
}
```

### ImpactLevel

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}
```

### Impact Analysis Example

```rust
use do_codeguardian::relationships::{RelationshipManager, impact_analysis::ChangeType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = RelationshipManager::new(config);

    // Analyze impact of modifying main.rs
    let impact_result = manager.analyze_impact("src-main-rs", ChangeType::Modify).await?;

    println!("Impact Analysis for modifying src-main-rs:");
    println!("- Risk Level: {:?}", impact_result.risk_level);
    println!("- Estimated Effort: {} hours", impact_result.estimated_effort);
    println!("- Affected Artifacts: {}", impact_result.affected_artifacts.len());

    for affected in &impact_result.affected_artifacts {
        println!("  - {} (impact: {:?}, strength: {:.2})",
                 affected.artifact_id,
                 affected.impact_level,
                 affected.relationship_strength);

        for action in &affected.required_actions {
            println!("    * {}", action);
        }
    }

    println!("Recommendations:");
    for recommendation in &impact_result.recommendations {
        println!("- {}", recommendation);
    }

    Ok(())
}
```

## Visualization

### GraphVisualization

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVisualization {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
    pub metadata: VisualizationMetadata,
}
```

### VisualizationNode

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationNode {
    pub id: String,
    pub label: String,
    pub node_type: ArtifactType,
    pub size: f64,
    pub color: String,
    pub position: Option<NodePosition>,
}
```

### VisualizationEdge

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEdge {
    pub source: String,
    pub target: String,
    pub label: String,
    pub weight: f64,
    pub edge_type: RelationshipType,
}
```

### Visualization Example

```rust
use do_codeguardian::relationships::{RelationshipManager, RelationshipQuery};

// Generate visualization for all relationships
let query = RelationshipQuery {
    limit: Some(100),
    ..Default::default()
};

let visualization = manager.generate_visualization(query).await?;

println!("Generated visualization:");
println!("- Nodes: {}", visualization.nodes.len());
println!("- Edges: {}", visualization.edges.len());

// Export to different formats
use do_codeguardian::relationships::visualization::GraphVisualizer;

let visualizer = GraphVisualizer::new();

// Export as GraphML
let graphml_data = visualizer.export_to_format(&visualization, ExportFormat::GraphML).await?;

// Export as DOT (for Graphviz)
let dot_data = visualizer.export_to_format(&visualization, ExportFormat::DOT).await?;

// Export as JSON
let json_data = visualizer.export_to_format(&visualization, ExportFormat::Json).await?;
```

## Query Interface

### RelationshipQuery

```rust
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
```

### RelationshipSearchResult

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipSearchResult {
    pub relationships: Vec<Relationship>,
    pub artifacts: HashMap<String, Artifact>,
    pub total_count: usize,
    pub query_time_ms: u64,
    pub relationship_paths: Vec<RelationshipPath>,
}
```

### Query Examples

```rust
use do_codeguardian::relationships::{RelationshipManager, RelationshipQuery, RelationshipType, ArtifactType};

// Find all dependencies of an artifact
let dependency_query = RelationshipQuery {
    artifact_id: Some("src-main-rs".to_string()),
    relationship_type: Some(RelationshipType::DependsOn),
    include_transitive: true,
    max_depth: Some(3),
    ..Default::default()
};

let dependencies = manager.search_relationships(dependency_query).await?;
println!("Found {} dependencies", dependencies.relationships.len());

// Find all test files for source code
let test_query = RelationshipQuery {
    artifact_type: Some(ArtifactType::SourceCode),
    relationship_type: Some(RelationshipType::Tests),
    ..Default::default()
};

let test_relationships = manager.search_relationships(test_query).await?;
println!("Found {} test relationships", test_relationships.relationships.len());

// Find artifacts with high relationship strength
let strong_relationships_query = RelationshipQuery {
    min_strength: Some(0.8),
    min_confidence: Some(0.9),
    limit: Some(50),
    ..Default::default()
};

let strong_relationships = manager.search_relationships(strong_relationships_query).await?;
println!("Found {} strong relationships", strong_relationships.relationships.len());
```

## Metadata Management

### MetadataStore

```rust
pub struct MetadataStore {
    artifacts: HashMap<String, Artifact>,
    relationships: HashMap<String, Relationship>,
    indexes: MetadataIndexes,
}
```

**Methods:**
```rust
impl MetadataStore {
    pub fn new() -> Self;
    pub async fn store_artifact(&mut self, artifact: &Artifact) -> Result<()>;
    pub async fn get_artifact(&self, id: &str) -> Result<Option<Artifact>>;
    pub async fn store_relationship(&mut self, relationship: &Relationship) -> Result<()>;
    pub async fn get_relationship(&self, id: &str) -> Result<Option<Relationship>>;
    pub async fn get_all_artifacts(&self) -> Result<Vec<Artifact>>;
    pub async fn count_artifacts(&self) -> Result<usize>;
    pub async fn count_artifacts_by_type(&self) -> Result<HashMap<ArtifactType, usize>>;
}
```

### Metadata Management Example

```rust
use do_codeguardian::relationships::RelationshipManager;

// Get statistics
let stats = manager.get_statistics().await?;

println!("Relationship Statistics:");
println!("- Total artifacts: {}", stats.total_artifacts);
println!("- Total relationships: {}", stats.total_relationships);
println!("- Average relationships per artifact: {:.2}", stats.average_relationships_per_artifact);
println!("- Last updated: {}", stats.last_updated);

// Count artifacts by type
for (artifact_type, count) in &stats.artifacts_by_type {
    println!("- {:?}: {}", artifact_type, count);
}

// Count relationships by type
for (relationship_type, count) in &stats.relationships_by_type {
    println!("- {:?}: {}", relationship_type, count);
}

// Cleanup old relationships
let removed_count = manager.cleanup_old_relationships().await?;
println!("Cleaned up {} old relationships", removed_count);
```

## CLI Integration

### Relationships Commands

```bash
# Discover relationships for an artifact
codeguardian relationships discover src/main.rs

# Search for relationships
codeguardian relationships search --type DependsOn --min-strength 0.7

# Analyze impact of changes
codeguardian relationships analyze src/main.rs --change-type modify

# Generate visualization
codeguardian relationships visualize --output relationships.dot

# Get statistics
codeguardian relationships stats

# Find relationship paths
codeguardian relationships paths src/main.rs src/lib.rs
```

### CLI Examples

```bash
# Discover relationships with verbose output
codeguardian relationships discover src/ --verbose

# Search for duplicate relationships across the project
codeguardian relationships search --type Duplicate --repository my-project

# Analyze impact of deleting a file
codeguardian relationships analyze src/old_file.rs --change-type delete

# Generate visualization with custom filters
codeguardian relationships visualize --min-strength 0.8 --max-depth 3 --output graph.png

# Get detailed statistics
codeguardian relationships stats --format json

# Find all paths between two artifacts
codeguardian relationships paths src/main.rs tests/main.rs --max-length 5
```

## Configuration Examples

### Basic Configuration

```toml
[relationships]
enabled = true
max_relationships_per_artifact = 50
relationship_ttl_days = 90
auto_discovery_enabled = true
visualization_enabled = true
impact_analysis_enabled = true

[relationships.relationship_weights]
Duplicate = 0.9
Similar = 0.7
References = 0.6
DependsOn = 0.8
```

### Advanced Configuration

```toml
[relationships]
enabled = true
max_relationships_per_artifact = 100
relationship_ttl_days = 180
auto_discovery_enabled = true
visualization_enabled = true
impact_analysis_enabled = true
supported_artifact_types = [
    "SourceCode",
    "Documentation",
    "Configuration",
    "Issue",
    "PullRequest",
    "TestFile",
    "BuildScript"
]

[relationships.relationship_weights]
Duplicate = 0.95
Similar = 0.75
References = 0.65
DependsOn = 0.85
PartOf = 0.8
RelatedTo = 0.5
Implements = 0.7
Tests = 0.6
```

## Performance Considerations

1. **Indexing**: Maintain efficient indexes for fast queries
2. **Caching**: Cache frequently accessed artifacts and relationships
3. **Batch Operations**: Use batch operations for bulk updates
4. **Memory Management**: Limit in-memory graph size for large repositories
5. **Async Processing**: Use async processing for discovery operations
6. **Database Optimization**: Use appropriate database indexes and partitioning
7. **Query Optimization**: Optimize queries with appropriate filters and limits

## Security Considerations

1. **Access Control**: Implement proper access controls for relationship data
2. **Data Sanitization**: Sanitize all artifact and relationship metadata
3. **Audit Logging**: Log all relationship operations and changes
4. **Privacy**: Respect privacy concerns when analyzing relationships
5. **Data Retention**: Implement appropriate data retention policies
6. **Secure Storage**: Encrypt sensitive relationship metadata
7. **Input Validation**: Validate all inputs and artifact identifiers

## Best Practices

1. **Incremental Discovery**: Use incremental discovery to avoid full rescans
2. **Relationship Validation**: Regularly validate and clean up invalid relationships
3. **Metadata Enrichment**: Enrich artifacts with relevant metadata for better analysis
4. **Visualization Optimization**: Optimize visualizations for large graphs
5. **Query Performance**: Monitor and optimize query performance
6. **Backup and Recovery**: Implement backup and recovery for relationship data
7. **Versioning**: Track versions of artifacts and relationships over time

## Integration Examples

### CI/CD Pipeline Integration

```yaml
name: Relationship Analysis
on:
  push:
    branches: [ main ]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Analyze Relationships
        run: |
          codeguardian relationships discover . --output relationships.json

      - name: Check for Breaking Changes
        run: |
          IMPACT=$(codeguardian relationships analyze src/main.rs --change-type modify --format json)
          if echo "$IMPACT" | jq -e '.risk_level == "Critical"'; then
            echo "Critical impact detected"
            exit 1
          fi

      - name: Generate Visualization
        run: |
          codeguardian relationships visualize --output relationships.dot
          dot -Tpng relationships.dot -o relationships.png

      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: relationship-analysis
          path: |
            relationships.json
            relationships.png
```

### Development Workflow Integration

```bash
# Before making changes, analyze impact
codeguardian relationships analyze src/auth.rs --change-type modify

# After adding new code, discover new relationships
codeguardian relationships discover src/new_feature.rs

# Before refactoring, check dependencies
codeguardian relationships search --artifact src/old_module.rs --type DependsOn

# Generate documentation of system architecture
codeguardian relationships visualize --output architecture.dot
dot -Tsvg architecture.dot -o architecture.svg
```

## Error Handling

### RelationshipError

```rust
#[derive(Debug, thiserror::Error)]
pub enum RelationshipError {
    #[error("Artifact not found: {0}")]
    ArtifactNotFound(String),

    #[error("Relationship not found: {0}")]
    RelationshipNotFound(String),

    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),

    #[error("Discovery failed: {0}")]
    DiscoveryError(String),

    #[error("Graph operation failed: {0}")]
    GraphError(String),

    #[error("Visualization failed: {0}")]
    VisualizationError(String),

    #[error("Impact analysis failed: {0}")]
    ImpactAnalysisError(String),

    #[error("Query failed: {0}")]
    QueryError(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}
```

### Error Handling Example

```rust
use do_codeguardian::relationships::{RelationshipManager, RelationshipError};

match manager.add_artifact(artifact).await {
    Ok(()) => println!("Artifact added successfully"),
    Err(RelationshipError::ArtifactNotFound(id)) => {
        eprintln!("Artifact {} not found", id);
    }
    Err(RelationshipError::InvalidRelationship(msg)) => {
        eprintln!("Invalid relationship: {}", msg);
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Migration Guide

### From v0.1.0 to v0.2.0

**New Features:**
- Advanced relationship discovery and management
- Impact analysis for changes
- Graph visualization capabilities
- Metadata storage and indexing
- Query interface for complex searches

**Configuration Changes:**
```toml
# Old configuration (v0.1.0)
[analysis]
enable_relationships = false

# New configuration (v0.2.0)
[relationships]
enabled = true
auto_discovery_enabled = true
visualization_enabled = true
impact_analysis_enabled = true
```

**API Changes:**
- New `RelationshipManager` replaces simple relationship tracking
- Enhanced query capabilities with `RelationshipQuery`
- New visualization and impact analysis APIs
- Improved metadata management

**Migration Steps:**
1. Update configuration to use new `[relationships]` section
2. Replace old relationship APIs with new `RelationshipManager`
3. Update queries to use new `RelationshipQuery` structure
4. Implement visualization and impact analysis as needed
5. Update error handling for new error types
