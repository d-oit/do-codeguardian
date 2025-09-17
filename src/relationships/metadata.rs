//! Metadata storage for artifacts and relationships

use super::{Artifact, ArtifactType, Relationship};
use anyhow::Result;
use std::collections::HashMap;

/// In-memory metadata store (in production, this would use a database)
pub struct MetadataStore {
    artifacts: HashMap<String, Artifact>,
    relationships: HashMap<String, Relationship>,
}

impl MetadataStore {
    pub fn new() -> Self {
        Self {
            artifacts: HashMap::new(),
            relationships: HashMap::new(),
        }
    }
}

impl Default for MetadataStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataStore {
    /// Store an artifact
    pub async fn store_artifact(&mut self, artifact: &Artifact) -> Result<()> {
        self.artifacts.insert(artifact.id.clone(), artifact.clone());
        Ok(())
    }

    /// Get an artifact by ID
    pub async fn get_artifact(&self, id: &str) -> Result<Option<Artifact>> {
        Ok(self.artifacts.get(id).cloned())
    }

    /// Get all artifacts
    pub async fn get_all_artifacts(&self) -> Result<Vec<Artifact>> {
        Ok(self.artifacts.values().cloned().collect())
    }

    /// Store a relationship
    pub async fn store_relationship(&mut self, relationship: &Relationship) -> Result<()> {
        self.relationships
            .insert(relationship.id.clone(), relationship.clone());
        Ok(())
    }

    /// Get a relationship by ID
    pub async fn get_relationship(&self, id: &str) -> Result<Option<Relationship>> {
        Ok(self.relationships.get(id).cloned())
    }

    /// Count total artifacts
    pub async fn count_artifacts(&self) -> Result<usize> {
        Ok(self.artifacts.len())
    }

    /// Count artifacts by type
    pub async fn count_artifacts_by_type(&self) -> Result<HashMap<ArtifactType, usize>> {
        let mut counts = HashMap::new();

        for artifact in self.artifacts.values() {
            *counts.entry(artifact.artifact_type.clone()).or_insert(0) += 1;
        }

        Ok(counts)
    }

    /// Search artifacts by criteria
    pub async fn search_artifacts(
        &self,
        criteria: &ArtifactSearchCriteria,
    ) -> Result<Vec<Artifact>> {
        let mut results = Vec::new();

        for artifact in self.artifacts.values() {
            if self.matches_criteria(artifact, criteria) {
                results.push(artifact.clone());
            }
        }

        Ok(results)
    }

    fn matches_criteria(&self, artifact: &Artifact, criteria: &ArtifactSearchCriteria) -> bool {
        if let Some(artifact_type) = &criteria.artifact_type {
            if artifact.artifact_type != *artifact_type {
                return false;
            }
        }

        if let Some(repository) = &criteria.repository {
            if artifact.repository.as_ref() != Some(repository) {
                return false;
            }
        }

        if let Some(system) = &criteria.system {
            if artifact.system.as_ref() != Some(system) {
                return false;
            }
        }

        if let Some(name_pattern) = &criteria.name_pattern {
            if !artifact.name.contains(name_pattern) {
                return false;
            }
        }

        if let Some(path_pattern) = &criteria.path_pattern {
            if !artifact.path.contains(path_pattern) {
                return false;
            }
        }

        if let Some(tags) = &criteria.tags {
            for tag in tags {
                if !artifact.tags.contains(tag) {
                    return false;
                }
            }
        }

        true
    }
}

/// Search criteria for artifacts
#[derive(Debug, Clone)]
pub struct ArtifactSearchCriteria {
    pub artifact_type: Option<ArtifactType>,
    pub repository: Option<String>,
    pub system: Option<String>,
    pub name_pattern: Option<String>,
    pub path_pattern: Option<String>,
    pub tags: Option<Vec<String>>,
}
