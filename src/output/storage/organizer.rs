//! # Results Organizer
//!
//! This module handles the organization and storage of analysis results
//! according to configurable strategies and directory structures.

use super::{OrganizationStrategy, ResultMetadata, StorageConfig, StorageIndex};
use crate::output::formatter::OutputResult;
use crate::types::{AnalysisResults, Severity};
use anyhow::Result;
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Results organizer for managing storage and organization
pub struct ResultsOrganizer {
    config: StorageConfig,
    index: StorageIndex,
    index_path: PathBuf,
}

impl ResultsOrganizer {
    /// Create a new results organizer
    pub fn new(config: StorageConfig) -> Result<Self> {
        let index_path = config
            .base_directory
            .join("index")
            .join("storage-index.json");

        // Ensure base directory exists
        fs::create_dir_all(&config.base_directory)?;
        fs::create_dir_all(config.base_directory.join("index"))?;

        // Load existing index or create new one
        let index = if index_path.exists() {
            let index_content = fs::read_to_string(&index_path)?;
            serde_json::from_str(&index_content)?
        } else {
            StorageIndex::new()
        };

        Ok(Self {
            config,
            index,
            index_path,
        })
    }

    /// Store analysis results with multiple output formats
    pub fn store_results(
        &mut self,
        results: &AnalysisResults,
        outputs: &[(String, OutputResult)], // format -> output
        project: &str,
        repository: Option<&str>,
        tags: Vec<String>,
    ) -> Result<String> {
        let result_id = Uuid::new_v4().to_string();
        let storage_path = self.determine_storage_path(project, repository, &results.timestamp)?;

        // Validate storage path for security
        self.validate_storage_path(&storage_path)?;

        // Create storage directory
        let full_storage_path = self.config.base_directory.join(&storage_path);
        fs::create_dir_all(&full_storage_path)?;

        // Store each output format
        let mut total_size = 0;
        let mut compressed_size = 0;

        for (format, output) in outputs {
            let filename = format!("{}.{}", result_id, self.get_file_extension(format));
            let file_path = full_storage_path.join(&filename);

            if self.config.enable_compression && self.should_compress(format) {
                let compressed_content = self.compress_content(&output.content)?;
                fs::write(&file_path, &compressed_content)?;
                compressed_size += compressed_content.len();
            } else {
                fs::write(&file_path, &output.content)?;
            }

            total_size += output.content.len();
        }

        // Store metadata
        let metadata_path = full_storage_path.join(format!("{}.metadata.json", result_id));
        let analysis_metadata = serde_json::to_string_pretty(results)?;
        fs::write(&metadata_path, &analysis_metadata)?;
        total_size += analysis_metadata.len();

        // Calculate checksum
        let checksum = self.calculate_checksum(results)?;

        // Create result metadata
        let metadata = ResultMetadata {
            id: result_id.clone(),
            project: project.to_string(),
            repository: repository.map(|s| s.to_string()),
            stored_at: Utc::now(),
            original_size: total_size,
            compressed_size: if compressed_size > 0 {
                Some(compressed_size)
            } else {
                None
            },
            storage_path: storage_path.clone(),
            config_hash: results.config_hash.clone(),
            finding_count: results.findings.len(),
            checksum,
            tags,
        };

        // Add to index
        self.index.add_result(metadata);

        // Save index
        self.save_index()?;

        // Apply retention policy if configured
        if let Some(retention_days) = self.config.retention_days {
            self.apply_retention_policy(retention_days)?;
        }

        Ok(result_id)
    }

    /// Retrieve results by ID
    pub fn retrieve_results(
        &self,
        id: &str,
    ) -> Result<Option<(AnalysisResults, Vec<(String, String)>)>> {
        if let Some(metadata) = self.index.by_id.get(id) {
            let storage_path = self.config.base_directory.join(&metadata.storage_path);

            // Load analysis results from metadata
            let metadata_path = storage_path.join(format!("{}.metadata.json", id));
            if !metadata_path.exists() {
                return Ok(None);
            }

            let metadata_content = fs::read_to_string(&metadata_path)?;
            let results: AnalysisResults = serde_json::from_str(&metadata_content)?;

            // Load output formats
            let mut outputs = Vec::new();
            for entry in fs::read_dir(&storage_path)? {
                let entry = entry?;
                let path = entry.path();

                if let Some(stem) = path.file_stem() {
                    if stem.to_string_lossy().starts_with(id)
                        && !path.to_string_lossy().contains("metadata")
                    {
                        if let Some(ext) = path.extension() {
                            let format = self.extension_to_format(ext.to_string_lossy().as_ref());
                            let content = if self.config.enable_compression
                                && self.should_compress(&format)
                            {
                                let compressed_content = fs::read(&path)?;
                                self.decompress_content(&compressed_content)?
                            } else {
                                fs::read_to_string(&path)?
                            };
                            outputs.push((format, content));
                        }
                    }
                }
            }

            Ok(Some((results, outputs)))
        } else {
            Ok(None)
        }
    }

    /// Query results based on criteria
    pub fn query_results(&self, criteria: &super::QueryCriteria) -> Vec<&ResultMetadata> {
        let mut candidates: Vec<&ResultMetadata> = self.index.by_id.values().collect();

        // Apply filters
        if let Some(project) = &criteria.project {
            candidates.retain(|m| m.project == *project);
        }

        if let Some(repository) = &criteria.repository {
            candidates.retain(|m| m.repository.as_ref() == Some(repository));
        }

        if let Some((start, end)) = &criteria.date_range {
            candidates.retain(|m| {
                let date_str = m.stored_at.format("%Y-%m-%d").to_string();
                date_str >= *start && date_str <= *end
            });
        }

        if !criteria.tags.is_empty() {
            candidates.retain(|m| criteria.tags.iter().any(|tag| m.tags.contains(tag)));
        }

        if let Some(config_hash) = &criteria.config_hash {
            candidates.retain(|m| m.config_hash == *config_hash);
        }

        if let Some(min_findings) = criteria.min_findings {
            candidates.retain(|m| m.finding_count >= min_findings);
        }

        if let Some(max_findings) = criteria.max_findings {
            candidates.retain(|m| m.finding_count <= max_findings);
        }

        // Sort by timestamp (newest first)
        candidates.sort_by(|a, b| b.stored_at.cmp(&a.stored_at));

        // Apply pagination
        if let Some(offset) = criteria.offset {
            candidates = candidates.into_iter().skip(offset).collect();
        }

        if let Some(limit) = criteria.limit {
            candidates.truncate(limit);
        }

        candidates
    }

    /// Delete results by ID
    pub fn delete_results(&mut self, id: &str) -> Result<bool> {
        if let Some(metadata) = self.index.by_id.get(id) {
            let storage_path = self.config.base_directory.join(&metadata.storage_path);

            // Remove all files for this result
            for entry in fs::read_dir(&storage_path)? {
                let entry = entry?;
                let path = entry.path();

                if let Some(stem) = path.file_stem() {
                    if stem.to_string_lossy().starts_with(id) {
                        fs::remove_file(&path)?;
                    }
                }
            }

            // Remove from index
            self.index.remove_result(id)?;
            self.save_index()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get storage statistics
    pub fn get_statistics(&self) -> super::StorageStatistics {
        self.index.get_statistics()
    }

    /// Cleanup old results based on retention policy
    pub fn cleanup_old_results(&mut self, retention_days: u32) -> Result<usize> {
        self.apply_retention_policy(retention_days)
    }

    /// Determine storage path based on organization strategy
    fn determine_storage_path(
        &self,
        project: &str,
        repository: Option<&str>,
        timestamp: &DateTime<Utc>,
    ) -> Result<PathBuf> {
        match &self.config.organization_strategy {
            OrganizationStrategy::ByDate => Ok(PathBuf::from(format!(
                "{}/{:02}/{:02}",
                timestamp.year(),
                timestamp.month(),
                timestamp.day()
            ))),
            OrganizationStrategy::ByProject => {
                Ok(PathBuf::from(self.sanitize_project_name(project)))
            }
            OrganizationStrategy::Hybrid => Ok(PathBuf::from(format!(
                "{}/{}/{:02}/{:02}",
                self.sanitize_project_name(project),
                timestamp.year(),
                timestamp.month(),
                timestamp.day()
            ))),
            OrganizationStrategy::HierarchicalTimeBased => {
                // Create hierarchical path: year/month/day/hour/project_hash/repo_hash
                let project_hash = self.hash_string(&self.sanitize_project_name(project));
                let repo_hash = repository
                    .map(|r| self.hash_string(r))
                    .unwrap_or_else(|| "no_repo".to_string());

                Ok(PathBuf::from(format!(
                    "{}/{:02}/{:02}/{:02}/{}/{}",
                    timestamp.year(),
                    timestamp.month(),
                    timestamp.day(),
                    timestamp.hour(),
                    project_hash,
                    repo_hash
                )))
            }
            OrganizationStrategy::Custom(pattern) => {
                // Simple template substitution
                let mut path = pattern
                    .replace("{project}", &self.sanitize_project_name(project))
                    .replace("{year}", &timestamp.year().to_string())
                    .replace("{month}", &format!("{:02}", timestamp.month()))
                    .replace("{day}", &format!("{:02}", timestamp.day()))
                    .replace("{hour}", &format!("{:02}", timestamp.hour()));

                if let Some(repo) = repository {
                    path = path.replace("{repository}", repo);
                    path = path.replace("{repo_hash}", &self.hash_string(repo));
                } else {
                    path = path.replace("{repository}", "no_repo");
                    path = path.replace("{repo_hash}", "no_repo");
                }

                Ok(PathBuf::from(path))
            }
        }
    }

    /// Sanitize project name for use in file paths
    fn sanitize_project_name(&self, project: &str) -> String {
        project
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    /// Generate a short hash for strings (used for directory names)
    fn hash_string(&self, input: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        // Take first 8 bytes (16 hex chars) for reasonable directory name length
        format!("{:x}", result)[..16].to_string()
    }

    /// Validate a storage path for security and correctness
    fn validate_storage_path(&self, path: &Path) -> Result<()> {
        // Check for path traversal attempts
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(anyhow::anyhow!(
                "Path contains parent directory references (..) which are not allowed"
            ));
        }

        // Check for absolute paths
        if path.is_absolute() {
            return Err(anyhow::anyhow!(
                "Absolute paths are not allowed in storage paths"
            ));
        }

        // Check path length (prevent extremely long paths)
        let path_str = path.to_string_lossy();
        if path_str.len() > 4096 {
            return Err(anyhow::anyhow!(
                "Storage path is too long (max 4096 characters)"
            ));
        }

        // Check for invalid characters in path components
        for component in path.components() {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_string_lossy();
                // Allow alphanumeric, hyphens, underscores, and dots
                if !name_str
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
                {
                    return Err(anyhow::anyhow!(
                        "Path component '{}' contains invalid characters",
                        name_str
                    ));
                }
            }
        }

        Ok(())
    }

    /// Get file extension for format
    fn get_file_extension(&self, format: &str) -> &str {
        match format.to_lowercase().as_str() {
            "json" => "json",
            "html" => "html",
            "markdown" => "md",
            "sarif" => "sarif",
            "yaml" => "yaml",
            _ => "txt",
        }
    }

    /// Convert file extension to format name
    fn extension_to_format(&self, extension: &str) -> String {
        match extension.to_lowercase().as_str() {
            "json" => "json".to_string(),
            "html" => "html".to_string(),
            "md" => "markdown".to_string(),
            "sarif" => "sarif".to_string(),
            "yaml" | "yml" => "yaml".to_string(),
            _ => "text".to_string(),
        }
    }

    /// Check if format should be compressed
    fn should_compress(&self, format: &str) -> bool {
        // Don't compress already compressed formats or binary formats
        !matches!(format.to_lowercase().as_str(), "zip" | "gz" | "bz2")
    }

    /// Compress content using gzip
    fn compress_content(&self, content: &str) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(content.as_bytes())?;
        Ok(encoder.finish()?)
    }

    /// Decompress content from gzip
    fn decompress_content(&self, compressed: &[u8]) -> Result<String> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(compressed);
        let mut content = String::new();
        decoder.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Calculate checksum for results
    fn calculate_checksum(&self, results: &AnalysisResults) -> Result<String> {
        let content = serde_json::to_string(results)?;
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Save index to disk
    fn save_index(&self) -> Result<()> {
        let index_content = serde_json::to_string_pretty(&self.index)?;
        fs::write(&self.index_path, index_content)?;
        Ok(())
    }

    /// Apply retention policy and clean up old results
    fn apply_retention_policy(&mut self, retention_days: u32) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days as i64);
        let mut deleted_count = 0;

        // Find results older than cutoff
        let old_results: Vec<String> = self
            .index
            .by_id
            .iter()
            .filter(|(_, metadata)| metadata.stored_at < cutoff_date)
            .map(|(id, _)| id.clone())
            .collect();

        // Delete old results
        for id in old_results {
            if self.delete_results(&id)? {
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }
}

/// Enhanced hierarchical organization structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultType {
    Analysis,
    Bulk,
    Benchmark,
    Integration,
    Security,
    Performance,
}

impl std::fmt::Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResultType::Analysis => write!(f, "analysis"),
            ResultType::Bulk => write!(f, "bulk"),
            ResultType::Benchmark => write!(f, "benchmarks"),
            ResultType::Integration => write!(f, "integration"),
            ResultType::Security => write!(f, "security"),
            ResultType::Performance => write!(f, "performance"),
        }
    }
}

/// Retention policy for different result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub analysis_retention_days: u32,
    pub bulk_retention_days: u32,
    pub benchmark_retention_days: u32,
    pub integration_retention_days: u32,
    pub security_retention_days: u32,
    pub performance_retention_days: u32,
    pub max_total_size_gb: u64,
    pub archive_old_results: bool,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            analysis_retention_days: 90,
            bulk_retention_days: 180,
            benchmark_retention_days: 365,
            integration_retention_days: 30,
            security_retention_days: 365, // Keep security results longer
            performance_retention_days: 180,
            max_total_size_gb: 100,
            archive_old_results: true,
        }
    }
}

/// Project-based organization manager
#[derive(Debug, Clone)]
pub struct ProjectOrganizer {
    project_registry: HashMap<String, ProjectMetadata>,
    base_path: PathBuf,
}

impl ProjectOrganizer {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            project_registry: HashMap::new(),
            base_path,
        }
    }

    pub fn register_project(&mut self, name: &str, metadata: ProjectMetadata) {
        self.project_registry.insert(name.to_string(), metadata);
    }

    pub fn get_project_path(&self, project_name: &str) -> PathBuf {
        let sanitized_name = self.sanitize_project_name(project_name);
        self.base_path.join("projects").join(sanitized_name)
    }

    pub fn get_repository_path(&self, repo_url: &str) -> PathBuf {
        let repo_hash = self.hash_repository_url(repo_url);
        self.base_path
            .join("repositories")
            .join(&repo_hash[..2])
            .join(&repo_hash[2..8])
            .join(&repo_hash[8..])
    }

    pub fn generate_hierarchical_path(
        &self,
        timestamp: DateTime<Utc>,
        result_type: ResultType,
        project_name: Option<&str>,
        repository_url: Option<&str>,
    ) -> PathBuf {
        let year = timestamp.year();
        let month = timestamp.month();
        let day = timestamp.day();
        let hour = timestamp.hour();

        let mut path = self.base_path.clone();

        // Add result type
        path = path.join(result_type.to_string());

        // Add time hierarchy
        path = path
            .join(format!("{:04}", year))
            .join(format!("{:02}", month))
            .join(format!("{:02}", day));

        // For analysis and security, add hourly organization
        if matches!(result_type, ResultType::Analysis | ResultType::Security) {
            path = path.join(format!("{:02}", hour));
        }

        // Add project organization if specified
        if let Some(project) = project_name {
            path = path
                .join("projects")
                .join(self.sanitize_project_name(project));
        }

        // Add repository organization if specified
        if let Some(repo) = repository_url {
            let repo_hash = self.hash_repository_url(repo);
            path = path.join("repositories").join(&repo_hash[..8]);
        }

        path
    }

    pub async fn ensure_directory(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }
        Ok(())
    }

    fn sanitize_project_name(&self, name: &str) -> String {
        name.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
            .to_lowercase()
    }

    fn hash_repository_url(&self, url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Enhanced metadata for hierarchical organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub repository_url: Option<String>,
    pub owner: String,
    pub language: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_analysis: Option<DateTime<Utc>>,
    pub total_analyses: u64,
    pub total_findings: u64,
    pub highest_severity_found: Option<Severity>,
}

impl ProjectMetadata {
    pub fn new(name: String, owner: String) -> Self {
        Self {
            name,
            repository_url: None,
            owner,
            language: None,
            tags: Vec::new(),
            created_at: Utc::now(),
            last_analysis: None,
            total_analyses: 0,
            total_findings: 0,
            highest_severity_found: None,
        }
    }

    pub fn update_analysis_stats(&mut self, findings_count: u64, highest_severity: Severity) {
        self.last_analysis = Some(Utc::now());
        self.total_analyses += 1;
        self.total_findings += findings_count;

        // Update highest severity if this is worse
        if let Some(current_highest) = &self.highest_severity_found {
            if severity_level(&highest_severity) > severity_level(current_highest) {
                self.highest_severity_found = Some(highest_severity);
            }
        } else {
            self.highest_severity_found = Some(highest_severity);
        }
    }
}

/// Enhanced results organizer with hierarchical capabilities
pub struct HierarchicalResultsOrganizer {
    config: StorageConfig,
    retention_policy: RetentionPolicy,
    project_organizer: ProjectOrganizer,
    index: StorageIndex,
}

impl HierarchicalResultsOrganizer {
    pub fn new(config: StorageConfig, retention_policy: RetentionPolicy) -> Result<Self> {
        let project_organizer = ProjectOrganizer::new(config.base_directory.clone());
        let index = StorageIndex::new();

        // Ensure base directories exist
        std::fs::create_dir_all(&config.base_directory)?;
        std::fs::create_dir_all(config.base_directory.join("index"))?;

        Ok(Self {
            config,
            retention_policy,
            project_organizer,
            index,
        })
    }

    pub async fn store_result_hierarchical(
        &mut self,
        result: &OutputResult,
        analysis_results: &AnalysisResults,
        result_type: ResultType,
        project_name: Option<&str>,
        repository_url: Option<&str>,
    ) -> Result<PathBuf> {
        let timestamp = Utc::now();

        // Generate hierarchical path
        let base_path = self.project_organizer.generate_hierarchical_path(
            timestamp,
            result_type.clone(),
            project_name,
            repository_url,
        );

        // Ensure directory exists
        self.project_organizer.ensure_directory(&base_path).await?;

        // Generate unique filename
        let result_id = Uuid::new_v4().to_string();
        let filename = format!("{}_{}.json", result_id, timestamp.format("%Y%m%d_%H%M%S"));
        let file_path = base_path.join(filename);

        // Store the result
        let content = serde_json::to_string_pretty(result)?;
        tokio::fs::write(&file_path, &content).await?;

        // Create enhanced metadata
        let metadata = self.create_enhanced_metadata(
            &result_id,
            &file_path,
            analysis_results,
            result_type,
            project_name,
            repository_url,
            timestamp,
        );

        // Update index
        let result_metadata = ResultMetadata {
            id: result_id.clone(),
            project: project_name.unwrap_or("unknown").to_string(),
            repository: repository_url.map(|s| s.to_string()),
            stored_at: timestamp,
            original_size: 0, // Will be updated after file is written
            compressed_size: None,
            storage_path: file_path.clone(),
            config_hash: blake3::hash(format!("{:?}", self.config).as_bytes()).to_string(),
            finding_count: analysis_results.findings.len(),
            checksum: blake3::hash(content.as_bytes()).to_string(),
            tags: Vec::new(),
        };
        self.index.add_result(result_metadata);

        // Update project metadata if applicable
        if let Some(project) = project_name {
            self.update_project_metadata(project, analysis_results)
                .await?;
        }

        // Store metadata file alongside result
        let metadata_path = file_path.with_extension("metadata.json");
        let metadata_content = serde_json::to_string_pretty(&metadata)?;
        tokio::fs::write(metadata_path, metadata_content).await?;

        Ok(file_path)
    }

    fn create_enhanced_metadata(
        &self,
        result_id: &str,
        file_path: &Path,
        analysis_results: &AnalysisResults,
        result_type: ResultType,
        project_name: Option<&str>,
        repository_url: Option<&str>,
        timestamp: DateTime<Utc>,
    ) -> EnhancedResultMetadata {
        // Define a type alias for the complex return type

        let highest_severity = analysis_results
            .findings
            .iter()
            .map(|f| &f.severity)
            .max()
            .cloned()
            .unwrap_or(Severity::Info);

        let analyzers_used: Vec<String> = analysis_results
            .findings
            .iter()
            .map(|f| f.analyzer.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        EnhancedResultMetadata {
            id: result_id.to_string(),
            file_path: file_path.to_path_buf(),
            timestamp,
            result_type,
            project_name: project_name.map(|s| s.to_string()),
            repository_url: repository_url.map(|s| s.to_string()),
            findings_count: analysis_results.findings.len(),
            highest_severity,
            analyzers_used,
            file_size_bytes: 0, // Will be updated after file is written
            schema_version: analysis_results.schema_version.clone(),
            tool_metadata: analysis_results.tool_metadata.clone(),
        }
    }

    async fn update_project_metadata(
        &mut self,
        project_name: &str,
        analysis_results: &AnalysisResults,
    ) -> Result<()> {
        let highest_severity = analysis_results
            .findings
            .iter()
            .map(|f| &f.severity)
            .max()
            .cloned()
            .unwrap_or(Severity::Info);

        // Get or create project metadata
        if let Some(project_metadata) = self
            .project_organizer
            .project_registry
            .get_mut(project_name)
        {
            project_metadata
                .update_analysis_stats(analysis_results.findings.len() as u64, highest_severity);
        } else {
            let mut new_project =
                ProjectMetadata::new(project_name.to_string(), "unknown".to_string());
            new_project
                .update_analysis_stats(analysis_results.findings.len() as u64, highest_severity);
            self.project_organizer
                .register_project(project_name, new_project);
        }

        Ok(())
    }

    pub fn search_results(&self, query: &SearchQuery) -> Vec<SearchResult> {
        let mut results = Vec::new();

        // Search through the index by_id map for now
        // Implement comprehensive search using all available indices
        let mut combined_results = HashMap::new();

        // Search by project name
        if let Some(project) = &query.project {
            if let Some(project_results) = self.index.by_project.get(project) {
                for result_id in project_results {
                    if let Some(metadata) = self.index.by_id.get(result_id) {
                        let score = self.calculate_relevance_score_basic(metadata, query);
                        combined_results.insert(result_id.clone(), (metadata.clone(), score));
                    }
                }
            }
        }

        // Convert to sorted results
        let mut sorted_results: Vec<_> = combined_results.into_values().collect();
        sorted_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if !sorted_results.is_empty() {
            let search_results: Vec<SearchResult> = sorted_results
                .into_iter()
                .take(query.limit)
                .map(|(metadata, score)| {
                    let enhanced_metadata = EnhancedResultMetadata {
                        id: metadata.id.clone(),
                        file_path: metadata.storage_path.clone(),
                        timestamp: metadata.stored_at,
                        result_type: ResultType::Analysis,
                        project_name: Some(metadata.project.clone()),
                        repository_url: metadata.repository.clone(),
                        findings_count: metadata.finding_count,
                        highest_severity: Severity::Info,
                        analyzers_used: Vec::new(),
                        file_size_bytes: metadata.original_size as u64,
                        schema_version: "1.0.0".to_string(),
                        tool_metadata: crate::types::ToolMetadata {
                            name: "CodeGuardian".to_string(),
                            version: env!("CARGO_PKG_VERSION").to_string(),
                            config_hash: metadata.config_hash.clone(),
                            timestamp: metadata.stored_at,
                        },
                    };
                    SearchResult {
                        id: metadata.id.clone(),
                        metadata: enhanced_metadata,
                        score,
                    }
                })
                .collect();
            return search_results;
        }

        // Fallback to old search method if no results found
        for (id, metadata) in &self.index.by_id {
            // Convert ResultMetadata to EnhancedResultMetadata for compatibility
            let enhanced_metadata = EnhancedResultMetadata {
                id: metadata.id.clone(),
                file_path: metadata.storage_path.clone(),
                timestamp: metadata.stored_at,
                result_type: ResultType::Analysis, // Default, could be improved
                project_name: Some(metadata.project.clone()),
                repository_url: metadata.repository.clone(),
                findings_count: metadata.finding_count,
                highest_severity: Severity::Info, // Default
                analyzers_used: Vec::new(),       // Not available in basic metadata
                file_size_bytes: metadata.original_size as u64,
                schema_version: "1.0.0".to_string(), // Default
                tool_metadata: crate::types::ToolMetadata {
                    name: "CodeGuardian".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    config_hash: metadata.config_hash.clone(),
                    timestamp: metadata.stored_at,
                },
            };

            if self.matches_query(&enhanced_metadata, query) {
                results.push(SearchResult {
                    id: id.clone(),
                    metadata: enhanced_metadata.clone(),
                    score: self.calculate_relevance_score(&enhanced_metadata, query),
                });
            }
        }

        // Sort by relevance score (highest first)
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    fn matches_query(&self, metadata: &EnhancedResultMetadata, query: &SearchQuery) -> bool {
        // Project filter
        if let Some(ref project_filter) = query.project {
            if let Some(ref project) = metadata.project_name {
                if !project.contains(project_filter) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Severity filter
        if let Some(ref severity_filter) = query.severity {
            if severity_level(&metadata.highest_severity) < severity_level(severity_filter) {
                return false;
            }
        }

        // Date range filter
        if let Some(ref date_range) = query.date_range {
            match date_range {
                DateRange::After(after) => {
                    if metadata.timestamp <= *after {
                        return false;
                    }
                }
                DateRange::Before(before) => {
                    if metadata.timestamp >= *before {
                        return false;
                    }
                }
                DateRange::Between(start, end) => {
                    if metadata.timestamp < *start || metadata.timestamp > *end {
                        return false;
                    }
                }
            }
        }

        // Analyzer filter
        if let Some(ref analyzer_filter) = query.analyzer {
            if !metadata
                .analyzers_used
                .iter()
                .any(|a| a.contains(analyzer_filter))
            {
                return false;
            }
        }

        true
    }

    fn calculate_relevance_score(
        &self,
        metadata: &EnhancedResultMetadata,
        _query: &SearchQuery,
    ) -> f64 {
        let mut score = 1.0;

        // Boost recent results
        let days_old = (Utc::now() - metadata.timestamp).num_days() as f64;
        score *= 1.0 / (1.0 + days_old / 30.0); // Decay over 30 days

        // Boost higher severity
        score *= match metadata.highest_severity {
            Severity::Critical => 5.0,
            Severity::High => 3.0,
            Severity::Medium => 2.0,
            Severity::Low => 1.5,
            Severity::Info => 1.0,
        };

        // Boost results with more findings (but not linearly)
        score *= 1.0 + (metadata.findings_count as f64).ln();

        score
    }

    fn calculate_relevance_score_basic(
        &self,
        metadata: &ResultMetadata,
        _query: &SearchQuery,
    ) -> f64 {
        let mut score = 1.0;

        // Boost recent results
        let days_old = (Utc::now() - metadata.stored_at).num_days() as f64;
        score *= 1.0 / (1.0 + days_old / 30.0); // Decay over 30 days

        // Boost results with more findings
        score *= 1.0 + (metadata.finding_count as f64).ln();

        score
    }
}

/// Enhanced metadata structure for hierarchical organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedResultMetadata {
    pub id: String,
    pub file_path: PathBuf,
    pub timestamp: DateTime<Utc>,
    pub result_type: ResultType,
    pub project_name: Option<String>,
    pub repository_url: Option<String>,
    pub findings_count: usize,
    pub highest_severity: Severity,
    pub analyzers_used: Vec<String>,
    pub file_size_bytes: u64,
    pub schema_version: String,
    pub tool_metadata: crate::types::ToolMetadata,
}

/// Search query structure for finding results
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub project: Option<String>,
    pub repository: Option<String>,
    pub severity: Option<Severity>,
    pub analyzer: Option<String>,
    pub date_range: Option<DateRange>,
    pub result_type: Option<ResultType>,
    pub text: Option<String>,
    pub limit: usize,
}

/// Date range for search queries
#[derive(Debug, Clone)]
pub enum DateRange {
    After(DateTime<Utc>),
    Before(DateTime<Utc>),
    Between(DateTime<Utc>, DateTime<Utc>),
}

/// Search result structure
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub metadata: EnhancedResultMetadata,
    pub score: f64,
}

/// Helper function to convert severity to numeric level for comparison
fn severity_level(severity: &Severity) -> u8 {
    match severity {
        Severity::Critical => 5,
        Severity::High => 4,
        Severity::Medium => 3,
        Severity::Low => 2,
        Severity::Info => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_results() -> AnalysisResults {
        let mut results = AnalysisResults::new("test_config".to_string());

        let finding = Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test finding message".to_string(),
        );

        results.add_finding(finding);
        results
    }

    fn create_test_config(temp_dir: &TempDir) -> StorageConfig {
        StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            organization_strategy: OrganizationStrategy::Hybrid,
            enable_compression: false, // Disable for simpler testing
            max_results_per_directory: 1000,
            enable_indexing: true,
            retention_days: Some(30),
            enable_deduplication: true,
        }
    }

    #[test]
    fn test_organizer_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);

        let organizer = ResultsOrganizer::new(config).unwrap();
        assert!(temp_dir.path().join("index").exists());
    }

    #[test]
    fn test_store_and_retrieve_results() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let mut organizer = ResultsOrganizer::new(config).unwrap();

        let results = create_test_results();
        let outputs = vec![(
            "json".to_string(),
            crate::output::formatter::OutputResult::new(
                "{}".to_string(),
                "json",
                "test_config".to_string(),
            ),
        )];

        let result_id = organizer
            .store_results(
                &results,
                &outputs,
                "test_project",
                Some("test/repo"),
                vec!["test".to_string()],
            )
            .unwrap();

        assert!(!result_id.is_empty());

        // Retrieve and verify
        let retrieved = organizer.retrieve_results(&result_id).unwrap();
        assert!(retrieved.is_some());

        let (retrieved_results, retrieved_outputs) = retrieved.unwrap();
        assert_eq!(retrieved_results.config_hash, results.config_hash);
        assert_eq!(retrieved_outputs.len(), 1);
    }

    #[test]
    fn test_organization_strategies() {
        let temp_dir = TempDir::new().unwrap();
        let mut organizer = ResultsOrganizer::new(StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            organization_strategy: OrganizationStrategy::ByDate,
            ..Default::default()
        })
        .unwrap();

        let timestamp = Utc::now();
        let path = organizer
            .determine_storage_path("test_project", Some("test/repo"), &timestamp)
            .unwrap();

        assert!(path
            .to_string_lossy()
            .contains(&timestamp.year().to_string()));
    }

    #[test]
    fn test_hierarchical_time_based_organization() {
        let temp_dir = TempDir::new().unwrap();
        let mut organizer = ResultsOrganizer::new(StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            organization_strategy: OrganizationStrategy::HierarchicalTimeBased,
            ..Default::default()
        })
        .unwrap();

        let timestamp = Utc::now();
        let path = organizer
            .determine_storage_path("test_project", Some("test/repo"), &timestamp)
            .unwrap();

        let path_str = path.to_string_lossy();
        assert!(path_str.contains(&timestamp.year().to_string()));
        assert!(path_str.contains(&format!("{:02}", timestamp.month())));
        assert!(path_str.contains(&format!("{:02}", timestamp.day())));
        assert!(path_str.contains(&format!("{:02}", timestamp.hour())));

        // Should contain hashed components
        let components: Vec<&str> = path_str.split('/').collect();
        assert_eq!(components.len(), 6); // year/month/day/hour/project_hash/repo_hash
    }

    #[test]
    fn test_path_validation() {
        let temp_dir = TempDir::new().unwrap();
        let organizer = ResultsOrganizer::new(StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            organization_strategy: OrganizationStrategy::ByDate,
            ..Default::default()
        })
        .unwrap();

        // Valid path
        let valid_path = PathBuf::from("2024/01/01");
        assert!(organizer.validate_storage_path(&valid_path).is_ok());

        // Invalid path with parent directory
        let invalid_path = PathBuf::from("2024/../etc/passwd");
        assert!(organizer.validate_storage_path(&invalid_path).is_err());

        // Invalid path with special characters
        let invalid_path2 = PathBuf::from("2024/01/01/file<script>.json");
        assert!(organizer.validate_storage_path(&invalid_path2).is_err());
    }

    #[test]
    fn test_query_results() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let mut organizer = ResultsOrganizer::new(config).unwrap();

        let results = create_test_results();
        let outputs = vec![(
            "json".to_string(),
            crate::output::formatter::OutputResult::new(
                "{}".to_string(),
                "json",
                "test_config".to_string(),
            ),
        )];

        organizer
            .store_results(
                &results,
                &outputs,
                "test_project",
                Some("test/repo"),
                vec!["test".to_string()],
            )
            .unwrap();

        let criteria = super::super::QueryCriteria {
            project: Some("test_project".to_string()),
            repository: Some("test/repo".to_string()),
            ..Default::default()
        };

        let found_results = organizer.query_results(&criteria);
        assert_eq!(found_results.len(), 1);
        assert_eq!(found_results[0].project, "test_project");
    }
}
