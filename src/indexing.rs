use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

/// Comprehensive metadata indexing system for full-text search
/// and efficient querying of CodeGuardian analysis results.
/// Uses in-memory data structures for simplicity and performance.
pub struct ResultsIndexer {
    /// In-memory storage of findings indexed by ID
    findings: RwLock<HashMap<String, Finding>>,
    /// Full-text search index: word -> set of finding IDs
    text_index: RwLock<HashMap<String, HashSet<String>>>,
    /// Facet indexes for fast filtering
    analyzer_index: RwLock<HashMap<String, HashSet<String>>>,
    rule_index: RwLock<HashMap<String, HashSet<String>>>,
    severity_index: RwLock<HashMap<String, HashSet<String>>>,
    category_index: RwLock<HashMap<String, HashSet<String>>>,
    file_index: RwLock<HashMap<String, HashSet<String>>>,
    /// Path to persistent storage (optional)
    storage_path: Option<PathBuf>,
}

/// Search query with filtering options
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    /// Full-text search query string
    pub query: Option<String>,
    /// Filter by analyzer name
    pub analyzer: Option<String>,
    /// Filter by rule name
    pub rule: Option<String>,
    /// Filter by severity level
    pub severity: Option<Severity>,
    /// Filter by file path (supports wildcards)
    pub file: Option<String>,
    /// Filter by minimum severity
    pub min_severity: Option<Severity>,
    /// Filter by category
    pub category: Option<String>,
    /// Limit number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

/// Search result with ranking score
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The finding document
    pub finding: Finding,
    /// Search relevance score (0.0 to 1.0)
    pub score: f32,
}

impl ResultsIndexer {
    /// Create a new ResultsIndexer
    /// If storage_path is provided, findings will be persisted to disk
    pub fn new(storage_path: Option<PathBuf>) -> Self {
        Self {
            findings: RwLock::new(HashMap::new()),
            text_index: RwLock::new(HashMap::new()),
            analyzer_index: RwLock::new(HashMap::new()),
            rule_index: RwLock::new(HashMap::new()),
            severity_index: RwLock::new(HashMap::new()),
            category_index: RwLock::new(HashMap::new()),
            file_index: RwLock::new(HashMap::new()),
            storage_path,
        }
    }

    /// Create a new ResultsIndexer with persistent storage at the specified path
    pub fn with_storage<P: AsRef<Path>>(storage_path: P) -> Self {
        Self::new(Some(storage_path.as_ref().to_path_buf()))
    }

    /// Index a collection of findings
    pub async fn index_findings(&self, findings: &[Finding]) -> Result<()> {
        for finding in findings {
            self.index_finding(finding).await?;
        }
        Ok(())
    }

    /// Add or update a single finding in the index
    pub async fn index_finding(&self, finding: &Finding) -> Result<()> {
        let finding_id = finding.id.clone();

        // Remove from all indexes first
        self.remove_from_indexes(&finding_id).await;

        // Add to main storage
        {
            let mut findings = self.findings.write().await;
            findings.insert(finding_id.clone(), finding.clone());
        }

        // Add to text index
        self.add_to_text_index(&finding_id, finding).await;

        // Add to facet indexes
        self.add_to_facet_indexes(&finding_id, finding).await;

        // Persist if storage path is set
        if let Some(path) = &self.storage_path {
            self.persist_to_disk(path).await?;
        }

        Ok(())
    }

    /// Delete a finding from the index by ID
    pub async fn delete_finding(&self, finding_id: &str) -> Result<()> {
        self.remove_from_indexes(finding_id).await;

        {
            let mut findings = self.findings.write().await;
            findings.remove(finding_id);
        }

        // Persist if storage path is set
        if let Some(path) = &self.storage_path {
            self.persist_to_disk(path).await?;
        }

        Ok(())
    }

    /// Search findings with full-text search and filtering
    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let candidate_ids = self.find_candidate_ids(query).await;

        let findings = self.findings.read().await;
        let mut results: Vec<SearchResult> = candidate_ids
            .into_iter()
            .filter_map(|id| findings.get(&id).cloned())
            .map(|finding| {
                let score = self.calculate_relevance_score(&finding, query);
                SearchResult { finding, score }
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply pagination
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(100).min(1000); // Cap at 1000 for performance

        if offset >= results.len() {
            return Ok(Vec::new());
        }

        let end = (offset + limit).min(results.len());
        Ok(results[offset..end].to_vec())
    }

    /// Get total count of findings matching the query (without retrieving documents)
    pub async fn count(&self, query: &SearchQuery) -> Result<usize> {
        let candidate_ids = self.find_candidate_ids(query).await;
        Ok(candidate_ids.len())
    }

    /// Get all unique values for a facet field (e.g., analyzers, rules, severities)
    pub async fn get_facet_values(&self, field_name: &str) -> Result<Vec<String>> {
        let index = match field_name {
            "analyzer" => &self.analyzer_index,
            "rule" => &self.rule_index,
            "severity" => &self.severity_index,
            "category" => &self.category_index,
            _ => return Err(anyhow::anyhow!("Unknown facet field: {}", field_name)),
        };

        let index = index.read().await;
        let mut values: Vec<String> = index.keys().cloned().collect();
        values.sort();
        Ok(values)
    }

    /// Get all indexed findings
    pub async fn get_all_findings(&self) -> Result<Vec<Finding>> {
        let findings = self.findings.read().await;
        Ok(findings.values().cloned().collect())
    }

    /// Clear all indexes
    pub async fn clear(&self) -> Result<()> {
        {
            let mut findings = self.findings.write().await;
            findings.clear();
        }
        {
            let mut text_index = self.text_index.write().await;
            text_index.clear();
        }
        {
            let mut analyzer_index = self.analyzer_index.write().await;
            analyzer_index.clear();
        }
        {
            let mut rule_index = self.rule_index.write().await;
            rule_index.clear();
        }
        {
            let mut severity_index = self.severity_index.write().await;
            severity_index.clear();
        }
        {
            let mut category_index = self.category_index.write().await;
            category_index.clear();
        }
        {
            let mut file_index = self.file_index.write().await;
            file_index.clear();
        }

        Ok(())
    }

    /// Find candidate finding IDs that match the query filters
    async fn find_candidate_ids(&self, query: &SearchQuery) -> HashSet<String> {
        let mut candidates: Option<HashSet<String>> = None;

        // Apply text search filter
        if let Some(query_str) = &query.query {
            let text_candidates = self.search_text(query_str).await;
            candidates = Some(self.intersect_candidates(candidates, text_candidates));
        }

        // Apply facet filters
        if let Some(analyzer) = &query.analyzer {
            let analyzer_candidates = self
                .get_facet_candidates(&self.analyzer_index, analyzer)
                .await;
            candidates = Some(self.intersect_candidates(candidates, analyzer_candidates));
        }

        if let Some(rule) = &query.rule {
            let rule_candidates = self.get_facet_candidates(&self.rule_index, rule).await;
            candidates = Some(self.intersect_candidates(candidates, rule_candidates));
        }

        if let Some(severity) = &query.severity {
            let severity_candidates = self
                .get_facet_candidates(&self.severity_index, &severity.to_string())
                .await;
            candidates = Some(self.intersect_candidates(candidates, severity_candidates));
        }

        if let Some(category) = &query.category {
            let category_candidates = self
                .get_facet_candidates(&self.category_index, category)
                .await;
            candidates = Some(self.intersect_candidates(candidates, category_candidates));
        }

        // Apply file filter
        if let Some(file_pattern) = &query.file {
            let file_candidates = self.search_files(file_pattern).await;
            candidates = Some(self.intersect_candidates(candidates, file_candidates));
        }

        // Apply minimum severity filter
        if let Some(min_severity) = &query.min_severity {
            let severity_candidates = self
                .get_minimum_severity_candidates(min_severity.clone())
                .await;
            candidates = Some(self.intersect_candidates(candidates, severity_candidates));
        }

        if let Some(candidates) = candidates {
            candidates
        } else {
            // If no filters, return all finding IDs
            let findings = self.findings.read().await;
            findings.keys().cloned().collect()
        }
    }

    /// Search for findings containing the query text
    async fn search_text(&self, query: &str) -> HashSet<String> {
        let text_index = self.text_index.read().await;
        let mut results = HashSet::new();

        // Simple word-based search (can be enhanced with stemming, etc.)
        let words: Vec<&str> = query.split_whitespace().collect();

        for word in words {
            let word_lower = word.to_lowercase();
            if let Some(finding_ids) = text_index.get(&word_lower) {
                results.extend(finding_ids.clone());
            }
        }

        results
    }

    /// Search for findings matching file pattern
    async fn search_files(&self, pattern: &str) -> HashSet<String> {
        let file_index = self.file_index.read().await;
        let mut results = HashSet::new();

        // Simple wildcard matching (* and ?)
        let regex_pattern = pattern.replace("*", ".*").replace("?", ".");
        let regex = Regex::new(&format!("(?i){}", regex_pattern)).unwrap();

        for (file_path, finding_ids) in file_index.iter() {
            if regex.is_match(file_path) {
                results.extend(finding_ids.clone());
            }
        }

        results
    }

    /// Get candidates from a facet index
    async fn get_facet_candidates(
        &self,
        index: &RwLock<HashMap<String, HashSet<String>>>,
        value: &str,
    ) -> HashSet<String> {
        let index = index.read().await;
        index.get(value).cloned().unwrap_or_default()
    }

    /// Get candidates for minimum severity filter
    async fn get_minimum_severity_candidates(&self, min_severity: Severity) -> HashSet<String> {
        let severity_values = match min_severity {
            Severity::Critical => vec!["critical"],
            Severity::High => vec!["critical", "high"],
            Severity::Medium => vec!["critical", "high", "medium"],
            Severity::Low => vec!["critical", "high", "medium", "low"],
            Severity::Info => vec!["critical", "high", "medium", "low", "info"],
        };

        let mut results = HashSet::new();
        for sev in severity_values {
            let severity_candidates = self.get_facet_candidates(&self.severity_index, sev).await;
            results.extend(severity_candidates);
        }

        results
    }

    /// Calculate relevance score for a finding based on the query
    fn calculate_relevance_score(&self, finding: &Finding, query: &SearchQuery) -> f32 {
        let mut score = 0.0;

        if let Some(query_str) = &query.query {
            // Simple term frequency scoring
            let query_words: Vec<String> = query_str
                .split_whitespace()
                .map(|w| w.to_lowercase())
                .collect();

            let mut matches = 0;
            let text_content = format!(
                "{} {} {}",
                finding.message.to_lowercase(),
                finding
                    .description
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_lowercase(),
                finding
                    .suggestion
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_lowercase()
            );

            for word in &query_words {
                if text_content.contains(word) {
                    matches += 1;
                }
            }

            score += matches as f32 / query_words.len() as f32;
        }

        // Boost score based on severity (higher severity = higher score)
        let severity_boost = match finding.severity {
            Severity::Critical => 1.0,
            Severity::High => 0.8,
            Severity::Medium => 0.6,
            Severity::Low => 0.4,
            Severity::Info => 0.2,
        };
        score += severity_boost * 0.1;

        score.min(1.0)
    }

    /// Intersect candidate sets
    fn intersect_candidates(
        &self,
        current: Option<HashSet<String>>,
        new: HashSet<String>,
    ) -> HashSet<String> {
        if let Some(current) = current {
            current.intersection(&new).cloned().collect()
        } else {
            new
        }
    }

    /// Add finding to text index
    async fn add_to_text_index(&self, finding_id: &str, finding: &Finding) {
        let mut text_index = self.text_index.write().await;

        let text_content = format!(
            "{} {} {}",
            finding.message,
            finding.description.as_ref().unwrap_or(&String::new()),
            finding.suggestion.as_ref().unwrap_or(&String::new())
        );

        // Simple tokenization (split on whitespace and punctuation)
        let words: Vec<String> = text_content
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .collect();

        for word in words {
            text_index
                .entry(word)
                .or_insert_with(HashSet::new)
                .insert(finding_id.to_string());
        }
    }

    /// Add finding to facet indexes
    async fn add_to_facet_indexes(&self, finding_id: &str, finding: &Finding) {
        {
            let mut analyzer_index = self.analyzer_index.write().await;
            analyzer_index
                .entry(finding.analyzer.clone())
                .or_insert_with(HashSet::new)
                .insert(finding_id.to_string());
        }
        {
            let mut rule_index = self.rule_index.write().await;
            rule_index
                .entry(finding.rule.clone())
                .or_insert_with(HashSet::new)
                .insert(finding_id.to_string());
        }
        {
            let mut severity_index = self.severity_index.write().await;
            severity_index
                .entry(finding.severity.to_string())
                .or_insert_with(HashSet::new)
                .insert(finding_id.to_string());
        }
        if let Some(category) = &finding.category {
            let mut category_index = self.category_index.write().await;
            category_index
                .entry(category.clone())
                .or_insert_with(HashSet::new)
                .insert(finding_id.to_string());
        }
        {
            let mut file_index = self.file_index.write().await;
            let file_path = finding.file.to_string_lossy().to_string();
            file_index
                .entry(file_path)
                .or_insert_with(HashSet::new)
                .insert(finding_id.to_string());
        }
    }

    /// Remove finding from all indexes
    async fn remove_from_indexes(&self, finding_id: &str) {
        // Remove from text index
        {
            let mut text_index = self.text_index.write().await;
            for (_word, finding_ids) in text_index.iter_mut() {
                finding_ids.remove(finding_id);
            }
            // Remove empty entries
            text_index.retain(|_word, ids| !ids.is_empty());
        }

        // Remove from facet indexes
        self.remove_from_facet_index(&self.analyzer_index, finding_id)
            .await;
        self.remove_from_facet_index(&self.rule_index, finding_id)
            .await;
        self.remove_from_facet_index(&self.severity_index, finding_id)
            .await;
        self.remove_from_facet_index(&self.category_index, finding_id)
            .await;
        self.remove_from_facet_index(&self.file_index, finding_id)
            .await;
    }

    /// Remove finding from a specific facet index
    async fn remove_from_facet_index(
        &self,
        index: &RwLock<HashMap<String, HashSet<String>>>,
        finding_id: &str,
    ) {
        let mut index = index.write().await;
        for (_key, finding_ids) in index.iter_mut() {
            finding_ids.remove(finding_id);
        }
        // Remove empty entries
        index.retain(|_key, ids| !ids.is_empty());
    }

    /// Persist index to disk (basic implementation)
    async fn persist_to_disk(&self, _path: &Path) -> Result<()> {
        // For now, this is a placeholder. In a full implementation,
        // you would serialize the indexes to disk for persistence.
        // This could be implemented using serde and tokio::fs.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_finding() -> Finding {
        Finding::new(
            "security_analyzer",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Hardcoded password detected".to_string(),
        )
        .with_description("This appears to be a hardcoded password".to_string())
        .with_category("Security".to_string())
    }

    #[tokio::test]
    async fn test_index_and_search() -> Result<()> {
        let indexer = ResultsIndexer::new(None);

        let finding = create_test_finding();
        indexer.index_finding(&finding).await?;

        // Search for the finding
        let query = SearchQuery {
            query: Some("password".to_string()),
            ..Default::default()
        };

        let results = indexer.search(&query).await?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].finding.id, finding.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_filtering() -> Result<()> {
        let indexer = ResultsIndexer::new(None);

        let finding1 = Finding::new(
            "security_analyzer",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Hardcoded password".to_string(),
        );

        let finding2 = Finding::new(
            "performance_analyzer",
            "slow_query",
            Severity::Medium,
            PathBuf::from("src/db.rs"),
            15,
            "Slow database query".to_string(),
        );

        indexer
            .index_findings(&[finding1.clone(), finding2.clone()])
            .await?;

        // Filter by analyzer
        let query = SearchQuery {
            analyzer: Some("security_analyzer".to_string()),
            ..Default::default()
        };

        let results = indexer.search(&query).await?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].finding.analyzer, "security_analyzer");

        Ok(())
    }

    #[tokio::test]
    async fn test_facet_values() -> Result<()> {
        let indexer = ResultsIndexer::new(None);

        let finding1 = create_test_finding();
        let finding2 = Finding::new(
            "performance_analyzer",
            "slow_query",
            Severity::Medium,
            PathBuf::from("src/db.rs"),
            15,
            "Slow database query".to_string(),
        );

        indexer.index_findings(&[finding1, finding2]).await?;

        let analyzers = indexer.get_facet_values("analyzer").await?;
        assert!(analyzers.contains(&"security_analyzer".to_string()));
        assert!(analyzers.contains(&"performance_analyzer".to_string()));

        Ok(())
    }
}
