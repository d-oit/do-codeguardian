//! # Results Storage and Organization Module
//!
//! This module provides efficient storage, indexing, and retrieval capabilities
//! for CodeGuardian analysis results. It implements hierarchical organization
//! with support for large-scale result sets.

pub mod compression;
pub mod indexer;
pub mod organizer;
pub mod retriever;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Storage configuration for results organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base directory for storing results
    pub base_directory: PathBuf,
    /// Organization strategy (by_date, by_project, hybrid)
    pub organization_strategy: OrganizationStrategy,
    /// Enable compression for stored results
    pub enable_compression: bool,
    /// Maximum results per directory
    pub max_results_per_directory: usize,
    /// Enable automatic indexing
    pub enable_indexing: bool,
    /// Retention policy in days
    pub retention_days: Option<u32>,
    /// Enable result deduplication
    pub enable_deduplication: bool,
}

/// Organization strategies for results storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrganizationStrategy {
    /// Organize by date (YYYY/MM/DD)
    ByDate,
    /// Organize by project name
    ByProject,
    /// Hybrid: project -> date
    Hybrid,
    /// Hierarchical time-based: year/month/day/hour with repository hashing
    HierarchicalTimeBased,
    /// Custom organization pattern
    Custom(String),
}

/// Result storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMetadata {
    /// Unique identifier for the result
    pub id: String,
    /// Project name
    pub project: String,
    /// Repository identifier (e.g., owner/repo or hashed path)
    pub repository: Option<String>,
    /// Timestamp when result was stored
    pub stored_at: DateTime<Utc>,
    /// Original file size in bytes
    pub original_size: usize,
    /// Compressed size if compression is enabled
    pub compressed_size: Option<usize>,
    /// Storage path relative to base directory
    pub storage_path: PathBuf,
    /// Configuration hash used for this analysis
    pub config_hash: String,
    /// Number of findings in this result
    pub finding_count: usize,
    /// Checksum for integrity verification
    pub checksum: String,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Storage index for fast retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIndex {
    /// Index version for compatibility
    pub version: String,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    /// Results indexed by ID
    pub by_id: HashMap<String, ResultMetadata>,
    /// Results indexed by project
    pub by_project: HashMap<String, Vec<String>>,
    /// Results indexed by repository
    pub by_repository: HashMap<String, Vec<String>>,
    /// Results indexed by date
    pub by_date: HashMap<String, Vec<String>>,
    /// Results indexed by tags
    pub by_tags: HashMap<String, Vec<String>>,
    /// Full-text search index
    pub search_index: HashMap<String, Vec<String>>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_directory: PathBuf::from("analysis-results"),
            organization_strategy: OrganizationStrategy::HierarchicalTimeBased,
            enable_compression: true,
            max_results_per_directory: 1000,
            enable_indexing: true,
            retention_days: Some(365),
            enable_deduplication: true,
        }
    }
}

impl StorageIndex {
    /// Create a new empty storage index
    pub fn new() -> Self {
        Self {
            version: "1.1.0".to_string(),
            last_updated: Utc::now(),
            by_id: HashMap::new(),
            by_project: HashMap::new(),
            by_repository: HashMap::new(),
            by_date: HashMap::new(),
            by_tags: HashMap::new(),
            search_index: HashMap::new(),
        }
    }
}

impl Default for StorageIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageIndex {
    /// Add a result to the index
    pub fn add_result(&mut self, metadata: ResultMetadata) {
        let id = metadata.id.clone();
        let project = metadata.project.clone();
        let repository = metadata.repository.clone();
        let date_key = metadata.stored_at.format("%Y-%m-%d").to_string();

        // Add to by_id index
        self.by_id.insert(id.clone(), metadata.clone());

        // Add to by_project index
        self.by_project.entry(project).or_default().push(id.clone());

        // Add to by_repository index
        if let Some(repo) = repository {
            self.by_repository.entry(repo).or_default().push(id.clone());
        }

        // Add to by_date index
        self.by_date.entry(date_key).or_default().push(id.clone());

        // Add to by_tags index
        for tag in &metadata.tags {
            self.by_tags
                .entry(tag.clone())
                .or_default()
                .push(id.clone());
        }

        self.last_updated = Utc::now();
    }

    /// Remove a result from the index
    fn remove_result(&mut self, id: &str) -> Result<()> {
        if let Some(metadata) = self.by_id.remove(id) {
            // Remove from project index
            if let Some(project_results) = self.by_project.get_mut(&metadata.project) {
                project_results.retain(|x| x != id);
                if project_results.is_empty() {
                    self.by_project.remove(&metadata.project);
                }
            }

            // Remove from repository index
            if let Some(repository) = &metadata.repository {
                if let Some(repo_results) = self.by_repository.get_mut(repository) {
                    repo_results.retain(|x| x != id);
                    if repo_results.is_empty() {
                        self.by_repository.remove(repository);
                    }
                }
            }

            // Remove from date index
            let date_key = metadata.stored_at.format("%Y-%m-%d").to_string();
            if let Some(date_results) = self.by_date.get_mut(&date_key) {
                date_results.retain(|x| x != id);
                if date_results.is_empty() {
                    self.by_date.remove(&date_key);
                }
            }

            // Remove from tags index
            for tag in &metadata.tags {
                if let Some(tag_results) = self.by_tags.get_mut(tag) {
                    tag_results.retain(|x| x != id);
                    if tag_results.is_empty() {
                        self.by_tags.remove(tag);
                    }
                }
            }

            self.last_updated = Utc::now();
        }

        Ok(())
    }

    /// Find results by project
    fn find_by_project(&self, project: &str) -> Vec<&ResultMetadata> {
        if let Some(ids) = self.by_project.get(project) {
            ids.iter().filter_map(|id| self.by_id.get(id)).collect()
        } else {
            Vec::new()
        }
    }

    /// Find results by date range
    fn find_by_date_range(&self, start: &str, end: &str) -> Vec<&ResultMetadata> {
        let mut results = Vec::new();

        for (date, ids) in &self.by_date {
            if date.as_str() >= start && date.as_str() <= end {
                for id in ids {
                    if let Some(metadata) = self.by_id.get(id) {
                        results.push(metadata);
                    }
                }
            }
        }

        results.sort_by(|a, b| a.stored_at.cmp(&b.stored_at));
        results
    }

    /// Find results by repository
    fn find_by_repository(&self, repository: &str) -> Vec<&ResultMetadata> {
        if let Some(ids) = self.by_repository.get(repository) {
            ids.iter().filter_map(|id| self.by_id.get(id)).collect()
        } else {
            Vec::new()
        }
    }

    /// Find results by tags
    fn find_by_tags(&self, tags: &[String]) -> Vec<&ResultMetadata> {
        let mut result_ids = Vec::new();

        for tag in tags {
            if let Some(ids) = self.by_tags.get(tag) {
                result_ids.extend(ids.iter());
            }
        }

        // Remove duplicates and get metadata
        result_ids.sort();
        result_ids.dedup();

        result_ids
            .iter()
            .filter_map(|id| self.by_id.get(*id))
            .collect()
    }

    /// Get storage statistics
    fn get_statistics(&self) -> StorageStatistics {
        let total_results = self.by_id.len();
        let total_projects = self.by_project.len();
        let total_size: usize = self.by_id.values().map(|m| m.original_size).sum();
        let compressed_size: usize = self.by_id.values().filter_map(|m| m.compressed_size).sum();

        StorageStatistics {
            total_results,
            total_projects,
            total_size,
            compressed_size: if compressed_size > 0 {
                Some(compressed_size)
            } else {
                None
            },
            compression_ratio: if compressed_size > 0 {
                Some((total_size as f64 - compressed_size as f64) / total_size as f64)
            } else {
                None
            },
            oldest_result: self.by_id.values().map(|m| m.stored_at).min(),
            newest_result: self.by_id.values().map(|m| m.stored_at).max(),
        }
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatistics {
    pub total_results: usize,
    pub total_projects: usize,
    pub total_size: usize,
    pub compressed_size: Option<usize>,
    pub compression_ratio: Option<f64>,
    pub oldest_result: Option<DateTime<Utc>>,
    pub newest_result: Option<DateTime<Utc>>,
}

/// Query criteria for finding results
#[derive(Debug, Clone, Default)]
pub struct QueryCriteria {
    pub project: Option<String>,
    pub repository: Option<String>,
    pub date_range: Option<(String, String)>,
    pub tags: Vec<String>,
    pub config_hash: Option<String>,
    pub min_findings: Option<usize>,
    pub max_findings: Option<usize>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// Re-export enhanced hierarchical types for easy access
pub use organizer::{
    DateRange, EnhancedResultMetadata, HierarchicalResultsOrganizer, ProjectMetadata,
    ProjectOrganizer, ResultType, RetentionPolicy, SearchQuery, SearchResult,
};
