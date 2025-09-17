//! Relationship graph implementation

use super::{Artifact, Relationship, RelationshipPath, RelationshipQuery, RelationshipType};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet, VecDeque};

/// Graph-based storage for relationships
pub struct RelationshipGraph {
    artifacts: HashMap<String, Artifact>,
    relationships: HashMap<String, Relationship>,
    adjacency_list: HashMap<String, Vec<String>>, // artifact_id -> relationship_ids
    reverse_adjacency: HashMap<String, Vec<String>>, // for incoming relationships
}

impl RelationshipGraph {
    pub fn new() -> Self {
        Self {
            artifacts: HashMap::new(),
            relationships: HashMap::new(),
            adjacency_list: HashMap::new(),
            reverse_adjacency: HashMap::new(),
        }
    }
}

impl Default for RelationshipGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipGraph {
    /// Add an artifact to the graph
    pub fn add_artifact(&mut self, artifact: &Artifact) -> Result<()> {
        self.artifacts.insert(artifact.id.clone(), artifact.clone());
        self.adjacency_list.entry(artifact.id.clone()).or_default();
        self.reverse_adjacency
            .entry(artifact.id.clone())
            .or_default();
        Ok(())
    }

    /// Add a relationship to the graph
    pub fn add_relationship(&mut self, relationship: &Relationship) -> Result<()> {
        // Add to relationships map
        self.relationships
            .insert(relationship.id.clone(), relationship.clone());

        // Update adjacency lists
        self.adjacency_list
            .entry(relationship.source_artifact_id.clone())
            .or_default()
            .push(relationship.id.clone());

        self.reverse_adjacency
            .entry(relationship.target_artifact_id.clone())
            .or_default()
            .push(relationship.id.clone());

        // If bidirectional, add reverse relationship
        if relationship.bidirectional {
            self.adjacency_list
                .entry(relationship.target_artifact_id.clone())
                .or_default()
                .push(relationship.id.clone());

            self.reverse_adjacency
                .entry(relationship.source_artifact_id.clone())
                .or_default()
                .push(relationship.id.clone());
        }

        Ok(())
    }

    /// Search for relationships based on query
    pub fn search_relationships(&self, query: &RelationshipQuery) -> Result<Vec<Relationship>> {
        let mut results = Vec::new();

        for relationship in self.relationships.values() {
            if self.matches_query(relationship, query) {
                results.push(relationship.clone());
            }
        }

        // Apply limit if specified
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    /// Check if a relationship matches the query criteria
    fn matches_query(&self, relationship: &Relationship, query: &RelationshipQuery) -> bool {
        // Check artifact ID
        if let Some(artifact_id) = &query.artifact_id {
            if relationship.source_artifact_id != *artifact_id
                && relationship.target_artifact_id != *artifact_id
            {
                return false;
            }
        }

        // Check relationship type
        if let Some(rel_type) = &query.relationship_type {
            if relationship.relationship_type != *rel_type {
                return false;
            }
        }

        // Check minimum strength
        if let Some(min_strength) = query.min_strength {
            if relationship.strength < min_strength {
                return false;
            }
        }

        // Check minimum confidence
        if let Some(min_confidence) = query.min_confidence {
            if relationship.confidence < min_confidence {
                return false;
            }
        }

        // Check date range
        if let Some(after) = query.created_after {
            if relationship.created_at < after {
                return false;
            }
        }

        if let Some(before) = query.created_before {
            if relationship.created_at > before {
                return false;
            }
        }

        // Check artifact type
        if let Some(artifact_type) = &query.artifact_type {
            let source_matches = self
                .artifacts
                .get(&relationship.source_artifact_id)
                .map(|a| a.artifact_type == *artifact_type)
                .unwrap_or(false);
            let target_matches = self
                .artifacts
                .get(&relationship.target_artifact_id)
                .map(|a| a.artifact_type == *artifact_type)
                .unwrap_or(false);

            if !source_matches && !target_matches {
                return false;
            }
        }

        // Check repository
        if let Some(repository) = &query.repository {
            let source_matches = self
                .artifacts
                .get(&relationship.source_artifact_id)
                .and_then(|a| a.repository.as_ref())
                .map(|r| r == repository)
                .unwrap_or(false);
            let target_matches = self
                .artifacts
                .get(&relationship.target_artifact_id)
                .and_then(|a| a.repository.as_ref())
                .map(|r| r == repository)
                .unwrap_or(false);

            if !source_matches && !target_matches {
                return false;
            }
        }

        // Check system
        if let Some(system) = &query.system {
            let source_matches = self
                .artifacts
                .get(&relationship.source_artifact_id)
                .and_then(|a| a.system.as_ref())
                .map(|s| s == system)
                .unwrap_or(false);
            let target_matches = self
                .artifacts
                .get(&relationship.target_artifact_id)
                .and_then(|a| a.system.as_ref())
                .map(|s| s == system)
                .unwrap_or(false);

            if !source_matches && !target_matches {
                return false;
            }
        }

        true
    }

    /// Find paths between artifacts
    pub async fn find_paths(&self, query: &RelationshipQuery) -> Result<Vec<RelationshipPath>> {
        let mut paths = Vec::new();

        if let Some(source_id) = &query.artifact_id {
            let max_depth = query.max_depth.unwrap_or(3);

            // Use BFS to find paths
            let found_paths = self.bfs_paths(source_id, max_depth)?;
            paths.extend(found_paths);
        }

        Ok(paths)
    }

    /// Breadth-first search to find paths
    fn bfs_paths(&self, start_id: &str, max_depth: u32) -> Result<Vec<RelationshipPath>> {
        let mut paths = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Initialize with starting artifact
        queue.push_back((
            start_id.to_string(),
            vec![start_id.to_string()],
            vec![],
            0u32,
            1.0f64,
        ));

        while let Some((current_id, path, relationships, depth, total_strength)) = queue.pop_front()
        {
            if depth >= max_depth {
                continue;
            }

            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            // Get outgoing relationships
            if let Some(relationship_ids) = self.adjacency_list.get(&current_id) {
                for rel_id in relationship_ids {
                    if let Some(relationship) = self.relationships.get(rel_id) {
                        let next_id = if relationship.source_artifact_id == current_id {
                            &relationship.target_artifact_id
                        } else {
                            &relationship.source_artifact_id
                        };

                        if !path.contains(next_id) {
                            let mut new_path = path.clone();
                            new_path.push(next_id.clone());

                            let mut new_relationships = relationships.clone();
                            new_relationships.push(rel_id.clone());

                            let new_strength = total_strength * relationship.strength;

                            // Add as a path if it's not just the starting point
                            if new_path.len() > 1 {
                                paths.push(RelationshipPath {
                                    source_artifact_id: start_id.to_string(),
                                    target_artifact_id: next_id.clone(),
                                    path: new_path.clone(),
                                    relationships: new_relationships.clone(),
                                    total_strength: new_strength,
                                    path_length: new_path.len() - 1,
                                });
                            }

                            // Continue searching from this node
                            queue.push_back((
                                next_id.clone(),
                                new_path,
                                new_relationships,
                                depth + 1,
                                new_strength,
                            ));
                        }
                    }
                }
            }
        }

        // Sort paths by strength (descending)
        paths.sort_by(|a, b| {
            b.total_strength
                .partial_cmp(&a.total_strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(paths)
    }

    /// Count total relationships
    pub fn count_relationships(&self) -> Result<usize> {
        Ok(self.relationships.len())
    }

    /// Count relationships by type
    pub fn count_relationships_by_type(&self) -> Result<HashMap<RelationshipType, usize>> {
        let mut counts = HashMap::new();

        for relationship in self.relationships.values() {
            *counts
                .entry(relationship.relationship_type.clone())
                .or_insert(0) += 1;
        }

        Ok(counts)
    }

    /// Remove relationships older than a certain date
    pub fn remove_relationships_older_than(&mut self, cutoff_date: DateTime<Utc>) -> Result<usize> {
        let mut to_remove = Vec::new();

        for (id, relationship) in &self.relationships {
            if relationship.created_at < cutoff_date {
                to_remove.push(id.clone());
            }
        }

        let removed_count = to_remove.len();

        for id in to_remove {
            self.remove_relationship(&id)?;
        }

        Ok(removed_count)
    }

    /// Remove a specific relationship
    fn remove_relationship(&mut self, relationship_id: &str) -> Result<()> {
        if let Some(relationship) = self.relationships.remove(relationship_id) {
            // Remove from adjacency lists
            if let Some(outgoing) = self
                .adjacency_list
                .get_mut(&relationship.source_artifact_id)
            {
                outgoing.retain(|id| id != relationship_id);
            }

            if let Some(incoming) = self
                .reverse_adjacency
                .get_mut(&relationship.target_artifact_id)
            {
                incoming.retain(|id| id != relationship_id);
            }

            // If bidirectional, remove reverse entries too
            if relationship.bidirectional {
                if let Some(outgoing) = self
                    .adjacency_list
                    .get_mut(&relationship.target_artifact_id)
                {
                    outgoing.retain(|id| id != relationship_id);
                }

                if let Some(incoming) = self
                    .reverse_adjacency
                    .get_mut(&relationship.source_artifact_id)
                {
                    incoming.retain(|id| id != relationship_id);
                }
            }
        }

        Ok(())
    }

    /// Get neighbors of an artifact
    pub fn get_neighbors(&self, artifact_id: &str) -> Result<Vec<String>> {
        let mut neighbors = HashSet::new();

        // Get outgoing relationships
        if let Some(relationship_ids) = self.adjacency_list.get(artifact_id) {
            for rel_id in relationship_ids {
                if let Some(relationship) = self.relationships.get(rel_id) {
                    if relationship.source_artifact_id == artifact_id {
                        neighbors.insert(relationship.target_artifact_id.clone());
                    } else {
                        neighbors.insert(relationship.source_artifact_id.clone());
                    }
                }
            }
        }

        // Get incoming relationships
        if let Some(relationship_ids) = self.reverse_adjacency.get(artifact_id) {
            for rel_id in relationship_ids {
                if let Some(relationship) = self.relationships.get(rel_id) {
                    if relationship.target_artifact_id == artifact_id {
                        neighbors.insert(relationship.source_artifact_id.clone());
                    } else {
                        neighbors.insert(relationship.target_artifact_id.clone());
                    }
                }
            }
        }

        Ok(neighbors.into_iter().collect())
    }

    /// Get strongly connected components
    pub fn get_strongly_connected_components(&self) -> Result<Vec<Vec<String>>> {
        // Simplified implementation - in practice, you'd use Tarjan's or Kosaraju's algorithm
        let mut components = Vec::new();
        let mut visited = HashSet::new();

        for artifact_id in self.artifacts.keys() {
            if !visited.contains(artifact_id) {
                let component = self.dfs_component(artifact_id, &mut visited)?;
                if component.len() > 1 {
                    components.push(component);
                }
            }
        }

        Ok(components)
    }

    /// DFS to find connected component
    fn dfs_component(&self, start_id: &str, visited: &mut HashSet<String>) -> Result<Vec<String>> {
        let mut component = Vec::new();
        let mut stack = vec![start_id.to_string()];

        while let Some(current_id) = stack.pop() {
            if visited.contains(&current_id) {
                continue;
            }

            visited.insert(current_id.clone());
            component.push(current_id.clone());

            // Add neighbors to stack
            for neighbor_id in self.get_neighbors(&current_id)? {
                if !visited.contains(&neighbor_id) {
                    stack.push(neighbor_id);
                }
            }
        }

        Ok(component)
    }
}
