//! # Storage Indexer
//!
//! This module provides advanced indexing capabilities for CodeGuardian analysis results,
//! including full-text search, faceted search, and performance optimization.

use super::ResultMetadata;
use crate::types::AnalysisResults;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Advanced search index for results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Version of the search index
    pub version: String,
    /// Full-text search index (word -> result IDs)
    pub text_index: HashMap<String, HashSet<String>>,
    /// Faceted search indices
    pub facets: FacetedIndices,
    /// Trigram index for fuzzy search
    pub trigram_index: HashMap<String, HashSet<String>>,
    /// Cached search results
    pub search_cache: HashMap<String, Vec<String>>,
}

/// Faceted search indices for different attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetedIndices {
    /// Index by severity levels
    pub by_severity: HashMap<String, HashSet<String>>,
    /// Index by analyzer types
    pub by_analyzer: HashMap<String, HashSet<String>>,
    /// Index by file extensions
    pub by_file_extension: HashMap<String, HashSet<String>>,
    /// Index by finding categories
    pub by_category: HashMap<String, HashSet<String>>,
    /// Index by finding rules
    pub by_rule: HashMap<String, HashSet<String>>,
    /// Index by file paths
    pub by_file_path: HashMap<String, HashSet<String>>,
}

/// Search query builder for complex queries
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// Text search terms
    pub text_terms: Vec<String>,
    /// Required facets (AND operation)
    pub required_facets: HashMap<String, Vec<String>>,
    /// Optional facets (OR operation)
    pub optional_facets: HashMap<String, Vec<String>>,
    /// Excluded terms
    pub excluded_terms: Vec<String>,
    /// Fuzzy search enabled
    pub fuzzy_search: bool,
    /// Maximum edit distance for fuzzy search
    pub max_edit_distance: usize,
    /// Result limit
    pub limit: Option<usize>,
    /// Result offset for pagination
    pub offset: Option<usize>,
}

/// Search result with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Result ID
    pub id: String,
    /// Relevance score (0.0 - 1.0)
    pub score: f64,
    /// Matching terms that contributed to the score
    pub matching_terms: Vec<String>,
    /// Matched facets
    pub matched_facets: HashMap<String, Vec<String>>,
}

/// Storage indexer for managing search capabilities
#[derive(Default)]
pub struct StorageIndexer {
    search_index: SearchIndex,
}

impl SearchIndex {
    /// Create a new empty search index
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            text_index: HashMap::new(),
            facets: FacetedIndices::new(),
            trigram_index: HashMap::new(),
            search_cache: HashMap::new(),
        }
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl FacetedIndices {
    /// Create new empty faceted indices
    pub fn new() -> Self {
        Self {
            by_severity: HashMap::new(),
            by_analyzer: HashMap::new(),
            by_file_extension: HashMap::new(),
            by_category: HashMap::new(),
            by_rule: HashMap::new(),
            by_file_path: HashMap::new(),
        }
    }
}

impl Default for FacetedIndices {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text_terms: Vec::new(),
            required_facets: HashMap::new(),
            optional_facets: HashMap::new(),
            excluded_terms: Vec::new(),
            fuzzy_search: false,
            max_edit_distance: 2,
            limit: Some(100),
            offset: None,
        }
    }
}

impl StorageIndexer {
    /// Create a new storage indexer
    pub fn new() -> Self {
        Self {
            search_index: SearchIndex::new(),
        }
    }
}

impl StorageIndexer {
    /// Add analysis results to the search index
    pub fn index_results(
        &mut self,
        id: &str,
        results: &AnalysisResults,
        metadata: &ResultMetadata,
    ) -> Result<()> {
        // Index text content
        self.index_text_content(id, results)?;

        // Index faceted data
        self.index_faceted_data(id, results, metadata)?;

        // Build trigrams for fuzzy search
        self.build_trigrams(id, results)?;

        // Clear search cache as index has changed
        self.search_index.search_cache.clear();

        Ok(())
    }

    /// Remove results from the search index
    pub fn remove_from_index(&mut self, id: &str) -> Result<()> {
        // Remove from text index
        for word_set in self.search_index.text_index.values_mut() {
            word_set.remove(id);
        }

        // Remove empty entries
        self.search_index
            .text_index
            .retain(|_, ids| !ids.is_empty());

        // Remove from faceted indices
        self.remove_from_faceted_indices(id);

        // Remove from trigram index
        for trigram_set in self.search_index.trigram_index.values_mut() {
            trigram_set.remove(id);
        }

        // Remove empty trigram entries
        self.search_index
            .trigram_index
            .retain(|_, ids| !ids.is_empty());

        // Clear search cache
        self.search_index.search_cache.clear();

        Ok(())
    }

    /// Perform a search query
    pub fn search(&mut self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // Check cache first
        let cache_key = self.generate_cache_key(query);
        if let Some(cached_ids) = self.search_index.search_cache.get(&cache_key) {
            return Ok(self.build_search_results(cached_ids, query));
        }

        let mut candidate_ids = HashSet::new();
        let mut scores = HashMap::new();

        // Text search
        if !query.text_terms.is_empty() {
            let text_matches = self.search_text_terms(
                &query.text_terms,
                query.fuzzy_search,
                query.max_edit_distance,
            )?;
            for (id, score) in text_matches {
                candidate_ids.insert(id.clone());
                *scores.entry(id).or_insert(0.0) += score;
            }
        } else {
            // If no text terms, start with all indexed results
            for ids in self.search_index.text_index.values() {
                candidate_ids.extend(ids.iter().cloned());
            }
            for id in &candidate_ids {
                scores.insert(id.clone(), 0.5); // Base score for non-text matches
            }
        }

        // Apply required facets (AND)
        for (facet_type, values) in &query.required_facets {
            let facet_matches = self.search_facet(facet_type, values, true)?;
            candidate_ids.retain(|id| facet_matches.contains(id));

            // Boost scores for facet matches
            for id in &facet_matches {
                if let Some(score) = scores.get_mut(id) {
                    *score += 0.1;
                }
            }
        }

        // Apply optional facets (OR)
        for (facet_type, values) in &query.optional_facets {
            let facet_matches = self.search_facet(facet_type, values, false)?;

            // Boost scores for optional facet matches
            for id in &facet_matches {
                if candidate_ids.contains(id) {
                    if let Some(score) = scores.get_mut(id) {
                        *score += 0.05;
                    }
                }
            }
        }

        // Apply exclusions
        for excluded_term in &query.excluded_terms {
            if let Some(excluded_ids) = self.search_index.text_index.get(excluded_term) {
                for excluded_id in excluded_ids {
                    candidate_ids.remove(excluded_id);
                    scores.remove(excluded_id);
                }
            }
        }

        // Convert to sorted results
        let mut results: Vec<_> = candidate_ids
            .into_iter()
            .map(|id| (id.clone(), scores.get(&id).copied().unwrap_or(0.0)))
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Apply pagination
        if let Some(offset) = query.offset {
            results = results.into_iter().skip(offset).collect();
        }

        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        // Cache results
        let result_ids: Vec<String> = results.iter().map(|(id, _)| id.clone()).collect();
        self.search_index
            .search_cache
            .insert(cache_key, result_ids.clone());

        // Build search results with metadata
        Ok(self.build_search_results(&result_ids, query))
    }

    /// Get search suggestions based on partial input
    pub fn get_suggestions(&self, partial_input: &str, limit: usize) -> Vec<String> {
        let partial_lower = partial_input.to_lowercase();
        let mut suggestions = Vec::new();

        // Find matching terms in text index
        for term in self.search_index.text_index.keys() {
            if term.starts_with(&partial_lower) {
                suggestions.push(term.clone());
            }
        }

        // Sort by frequency (number of results containing the term)
        suggestions.sort_by(|a, b| {
            let a_count = self
                .search_index
                .text_index
                .get(a)
                .map(|s| s.len())
                .unwrap_or(0);
            let b_count = self
                .search_index
                .text_index
                .get(b)
                .map(|s| s.len())
                .unwrap_or(0);
            b_count.cmp(&a_count)
        });

        suggestions.truncate(limit);
        suggestions
    }

    /// Get facet statistics
    pub fn get_facet_statistics(&self) -> HashMap<String, HashMap<String, usize>> {
        let mut stats = HashMap::new();

        // Severity statistics
        let mut severity_stats = HashMap::new();
        for (severity, ids) in &self.search_index.facets.by_severity {
            severity_stats.insert(severity.clone(), ids.len());
        }
        stats.insert("severity".to_string(), severity_stats);

        // Analyzer statistics
        let mut analyzer_stats = HashMap::new();
        for (analyzer, ids) in &self.search_index.facets.by_analyzer {
            analyzer_stats.insert(analyzer.clone(), ids.len());
        }
        stats.insert("analyzer".to_string(), analyzer_stats);

        // File extension statistics
        let mut extension_stats = HashMap::new();
        for (extension, ids) in &self.search_index.facets.by_file_extension {
            extension_stats.insert(extension.clone(), ids.len());
        }
        stats.insert("file_extension".to_string(), extension_stats);

        stats
    }

    /// Index text content from analysis results
    fn index_text_content(&mut self, id: &str, results: &AnalysisResults) -> Result<()> {
        let mut words = HashSet::new();

        // Index finding messages and descriptions
        for finding in &results.findings {
            // Tokenize message
            self.tokenize_text(&finding.message, &mut words);

            // Tokenize description if available
            if let Some(description) = &finding.description {
                self.tokenize_text(description, &mut words);
            }

            // Index rule name
            self.tokenize_text(&finding.rule, &mut words);

            // Index file path
            self.tokenize_text(&finding.file.to_string_lossy(), &mut words);
        }

        // Add words to index
        for word in words {
            self.search_index
                .text_index
                .entry(word)
                .or_default()
                .insert(id.to_string());
        }

        Ok(())
    }

    /// Index faceted data
    fn index_faceted_data(
        &mut self,
        id: &str,
        results: &AnalysisResults,
        _metadata: &ResultMetadata,
    ) -> Result<()> {
        for finding in &results.findings {
            // Index by severity
            self.search_index
                .facets
                .by_severity
                .entry(finding.severity.to_string())
                .or_default()
                .insert(id.to_string());

            // Index by analyzer
            self.search_index
                .facets
                .by_analyzer
                .entry(finding.analyzer.clone())
                .or_default()
                .insert(id.to_string());

            // Index by file extension
            if let Some(extension) = finding.file.extension() {
                self.search_index
                    .facets
                    .by_file_extension
                    .entry(extension.to_string_lossy().to_string())
                    .or_default()
                    .insert(id.to_string());
            }

            // Index by category
            if let Some(category) = &finding.category {
                self.search_index
                    .facets
                    .by_category
                    .entry(category.clone())
                    .or_default()
                    .insert(id.to_string());
            }

            // Index by rule
            self.search_index
                .facets
                .by_rule
                .entry(finding.rule.clone())
                .or_default()
                .insert(id.to_string());

            // Index by file path
            self.search_index
                .facets
                .by_file_path
                .entry(finding.file.to_string_lossy().to_string())
                .or_default()
                .insert(id.to_string());
        }

        Ok(())
    }

    /// Build trigrams for fuzzy search
    fn build_trigrams(&mut self, id: &str, results: &AnalysisResults) -> Result<()> {
        let mut text_content = String::new();

        for finding in &results.findings {
            text_content.push_str(&finding.message);
            text_content.push(' ');
            if let Some(description) = &finding.description {
                text_content.push_str(description);
                text_content.push(' ');
            }
        }

        let trigrams = self.generate_trigrams(&text_content.to_lowercase());
        for trigram in trigrams {
            self.search_index
                .trigram_index
                .entry(trigram)
                .or_default()
                .insert(id.to_string());
        }

        Ok(())
    }

    /// Tokenize text into searchable words
    fn tokenize_text(&self, text: &str, words: &mut HashSet<String>) {
        let cleaned = text
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c.is_whitespace() {
                    c
                } else {
                    ' '
                }
            })
            .collect::<String>();

        for word in cleaned.split_whitespace() {
            if word.len() >= 2 && !self.is_stop_word(word) {
                words.insert(word.to_string());
            }
        }
    }

    /// Check if word is a stop word
    fn is_stop_word(&self, word: &str) -> bool {
        matches!(
            word,
            "the"
                | "is"
                | "at"
                | "which"
                | "on"
                | "and"
                | "or"
                | "but"
                | "in"
                | "with"
                | "to"
                | "for"
                | "of"
                | "as"
                | "by"
        )
    }

    /// Generate trigrams for fuzzy search
    fn generate_trigrams(&self, text: &str) -> HashSet<String> {
        let mut trigrams = HashSet::new();
        let chars: Vec<char> = format!("  {}  ", text).chars().collect();

        for window in chars.windows(3) {
            let trigram: String = window.iter().collect();
            trigrams.insert(trigram);
        }

        trigrams
    }

    /// Search text terms with optional fuzzy matching
    fn search_text_terms(
        &self,
        terms: &[String],
        fuzzy: bool,
        max_edit_distance: usize,
    ) -> Result<HashMap<String, f64>> {
        let mut matches = HashMap::new();

        for term in terms {
            let term_lower = term.to_lowercase();

            // Exact matches
            if let Some(ids) = self.search_index.text_index.get(&term_lower) {
                for id in ids {
                    *matches.entry(id.clone()).or_insert(0.0) += 1.0;
                }
            }

            // Fuzzy matches if enabled
            if fuzzy {
                let fuzzy_matches = self.fuzzy_search(&term_lower, max_edit_distance);
                for (id, score) in fuzzy_matches {
                    *matches.entry(id).or_insert(0.0) += score * 0.8; // Slightly lower score for fuzzy matches
                }
            }
        }

        Ok(matches)
    }

    /// Perform fuzzy search using trigrams
    fn fuzzy_search(&self, term: &str, _max_edit_distance: usize) -> HashMap<String, f64> {
        let mut matches = HashMap::new();
        let term_trigrams = self.generate_trigrams(term);

        // Find candidates using trigram similarity
        let mut candidates = HashMap::new();
        for trigram in &term_trigrams {
            if let Some(ids) = self.search_index.trigram_index.get(trigram) {
                for id in ids {
                    *candidates.entry(id.clone()).or_insert(0) += 1;
                }
            }
        }

        // Calculate similarity scores
        for (id, trigram_matches) in candidates {
            let similarity = trigram_matches as f64 / term_trigrams.len() as f64;
            if similarity >= 0.3 {
                // Threshold for fuzzy matches
                matches.insert(id, similarity);
            }
        }

        matches
    }

    /// Search within a specific facet
    fn search_facet(
        &self,
        facet_type: &str,
        values: &[String],
        _required: bool,
    ) -> Result<HashSet<String>> {
        let mut matches = HashSet::new();

        let facet_index = match facet_type {
            "severity" => &self.search_index.facets.by_severity,
            "analyzer" => &self.search_index.facets.by_analyzer,
            "file_extension" => &self.search_index.facets.by_file_extension,
            "category" => &self.search_index.facets.by_category,
            "rule" => &self.search_index.facets.by_rule,
            "file_path" => &self.search_index.facets.by_file_path,
            _ => return Ok(matches),
        };

        for value in values {
            if let Some(ids) = facet_index.get(value) {
                matches.extend(ids.iter().cloned());
            }
        }

        Ok(matches)
    }

    /// Remove ID from all faceted indices
    fn remove_from_faceted_indices(&mut self, id: &str) {
        // Helper macro to remove from facet indices
        macro_rules! remove_from_facet {
            ($facet:expr) => {
                for ids in $facet.values_mut() {
                    ids.remove(id);
                }
                $facet.retain(|_, ids| !ids.is_empty());
            };
        }

        remove_from_facet!(self.search_index.facets.by_severity);
        remove_from_facet!(self.search_index.facets.by_analyzer);
        remove_from_facet!(self.search_index.facets.by_file_extension);
        remove_from_facet!(self.search_index.facets.by_category);
        remove_from_facet!(self.search_index.facets.by_rule);
        remove_from_facet!(self.search_index.facets.by_file_path);
    }

    /// Generate cache key for search query
    fn generate_cache_key(&self, query: &SearchQuery) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        query.text_terms.hash(&mut hasher);
        // Note: HashMap doesn't implement Hash, so we'll create a simple representation
        format!("query_{:x}", hasher.finish())
    }

    /// Build search results from IDs
    fn build_search_results(
        &self,
        result_ids: &[String],
        query: &SearchQuery,
    ) -> Vec<SearchResult> {
        result_ids
            .iter()
            .enumerate()
            .map(|(index, id)| {
                let score = 1.0 - (index as f64 * 0.01); // Simple scoring based on position

                SearchResult {
                    id: id.clone(),
                    score: score.max(0.0),
                    matching_terms: query.text_terms.clone(), // Simplified - would need more detailed tracking
                    matched_facets: HashMap::new(), // Simplified - would need more detailed tracking
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Finding, Severity};
    use std::path::PathBuf;

    fn create_test_results() -> AnalysisResults {
        let mut results = AnalysisResults::new("test_config".to_string());

        let finding = Finding::new(
            "test_analyzer",
            "test_rule",
            Severity::High,
            PathBuf::from("test.rs"),
            10,
            "Test finding message with security vulnerability".to_string(),
        )
        .with_description("This is a security issue that needs attention".to_string());

        results.add_finding(finding);
        results
    }

    #[test]
    fn test_indexer_creation() -> Result<(), Box<dyn std::error::Error>> {
        let indexer = StorageIndexer::new();
        assert_eq!(indexer.search_index.version, "1.0.0");
    }

    #[test]
    fn test_index_and_search() -> Result<(), Box<dyn std::error::Error>> {
        let mut indexer = StorageIndexer::new();
        let results = create_test_results();
        let metadata = ResultMetadata {
            id: "test_1".to_string(),
            project: "test_project".to_string(),
            repository: None,
            stored_at: chrono::Utc::now(),
            original_size: 1000,
            compressed_size: None,
            storage_path: PathBuf::from("test"),
            config_hash: "test_hash".to_string(),
            finding_count: 1,
            checksum: "test_checksum".to_string(),
            tags: vec!["test".to_string()],
        };

        indexer.index_results("test_1", &results, &metadata)?;

        let query = SearchQuery {
            text_terms: vec!["security".to_string()],
            ..Default::default()
        };

        let search_results = indexer.search(&query)?;
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, "test_1");
    }

    #[test]
    fn test_faceted_search() -> Result<(), Box<dyn std::error::Error>> {
        let mut indexer = StorageIndexer::new();
        let results = create_test_results();
        let metadata = ResultMetadata {
            id: "test_1".to_string(),
            project: "test_project".to_string(),
            repository: None,
            stored_at: chrono::Utc::now(),
            original_size: 1000,
            compressed_size: None,
            storage_path: PathBuf::from("test"),
            config_hash: "test_hash".to_string(),
            finding_count: 1,
            checksum: "test_checksum".to_string(),
            tags: vec!["test".to_string()],
        };

        indexer.index_results("test_1", &results, &metadata)?;

        let mut required_facets = HashMap::new();
        required_facets.insert("severity".to_string(), vec!["high".to_string()]);

        let query = SearchQuery {
            required_facets,
            ..Default::default()
        };

        let search_results = indexer.search(&query)?;
        assert_eq!(search_results.len(), 1);
    }

    #[test]
    fn test_suggestions() -> Result<(), Box<dyn std::error::Error>> {
        let mut indexer = StorageIndexer::new();
        let results = create_test_results();
        let metadata = ResultMetadata {
            id: "test_1".to_string(),
            project: "test_project".to_string(),
            repository: None,
            stored_at: chrono::Utc::now(),
            original_size: 1000,
            compressed_size: None,
            storage_path: PathBuf::from("test"),
            config_hash: "test_hash".to_string(),
            finding_count: 1,
            checksum: "test_checksum".to_string(),
            tags: vec!["test".to_string()],
        };

        indexer.index_results("test_1", &results, &metadata)?;

        let suggestions = indexer.get_suggestions("sec", 5);
        assert!(suggestions.iter().any(|s| s.contains("security")));
    }
}
