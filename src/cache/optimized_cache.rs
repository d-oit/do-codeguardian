//! Optimized cache implementation for CodeGuardian
//!
//! This module provides enhanced caching with file modification tracking,
//! intelligent eviction policies, and performance optimizations for
//! 20-40% faster repeated analysis.

use crate::types::Finding;
use anyhow::Result;
use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;

/// Enhanced cache entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub findings: Vec<Finding>,
    pub file_hash: String,
    pub config_hash: String,
    pub modified_time: u64,
    pub file_size: u64,
    pub access_count: u32,
    pub last_accessed: u64,
    pub analysis_duration_ms: u64,
    #[serde(default)]
    pub priority_score: f64, // For advanced eviction policies
}

impl CacheEntry {
    pub fn new(
        findings: Vec<Finding>,
        file_hash: String,
        config_hash: String,
        modified_time: SystemTime,
        file_size: u64,
        analysis_duration_ms: u64,
    ) -> Self {
        let modified_time_secs = modified_time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            priority_score: 0.0,
            findings,
            file_hash,
            config_hash,
            modified_time: modified_time_secs,
            file_size,
            access_count: 1,
            last_accessed: now,
            analysis_duration_ms,
        }
    }

    pub fn update_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    pub fn is_valid(&self, file_metadata: &FileMetadata, config_hash: &str) -> bool {
        self.config_hash == config_hash
            && self.modified_time == file_metadata.modified_time
            && self.file_size == file_metadata.size
            && self.file_hash == file_metadata.content_hash
    }

    pub fn age_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.last_accessed)
    }

    pub fn priority_score(&self) -> f64 {
        // Higher score = higher priority to keep in cache
        let access_weight = (self.access_count as f64).ln_1p();
        let recency_weight = 1.0 / (1.0 + self.age_seconds() as f64 / 3600.0); // Decay over hours
        let size_weight = 1.0 / (1.0 + self.file_size as f64 / 1024.0); // Prefer smaller files

        access_weight * recency_weight * size_weight
    }
}

/// File metadata for cache validation
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub modified_time: u64,
    pub size: u64,
    pub content_hash: String,
}

impl FileMetadata {
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let metadata = std::fs::metadata(file_path)?;
        let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
        let size = metadata.len();

        // Compute content hash for integrity
        let content = std::fs::read(file_path)?;
        let content_hash = Self::compute_hash(&content);

        Ok(Self {
            modified_time,
            size,
            content_hash,
        })
    }

    pub async fn from_file_async(file_path: &Path) -> Result<Self> {
        let metadata = fs::metadata(file_path).await?;
        let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
        let size = metadata.len();

        // Compute content hash for integrity
        let content = fs::read(file_path).await?;
        let content_hash = Self::compute_hash(&content);

        Ok(Self {
            modified_time,
            size,
            content_hash,
        })
    }

    pub fn compute_hash(content: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(content);
        hasher.finalize().to_hex().to_string()
    }
}

/// Optimized cache with intelligent eviction and performance tracking
pub struct OptimizedCache {
    entries: HashMap<PathBuf, CacheEntry>,
    max_entries: usize,
    max_memory_mb: usize,
    current_memory_bytes: usize,
    stats: CacheStats,
}

impl OptimizedCache {
    pub fn new(max_entries: usize, max_memory_mb: usize) -> Self {
        Self {
            max_entries,
            entries: HashMap::with_capacity(max_entries),
            max_memory_mb,
            current_memory_bytes: 0,
            stats: CacheStats::default(),
        }
    }

    /// Get cached findings if valid, otherwise return None
    pub fn get(&mut self, file_path: &Path, config_hash: &str) -> Result<Option<Vec<Finding>>> {
        self.stats.total_requests += 1;

        if let Some(entry) = self.entries.get_mut(file_path) {
            // Quick check: if config changed, entry is invalid
            if entry.config_hash != config_hash {
                self.stats.config_misses += 1;
                self.remove_entry(file_path);
                return Ok(None);
            }

            // Validate against current file state
            match FileMetadata::from_file(file_path) {
                Ok(metadata) => {
                    if entry.is_valid(&metadata, config_hash) {
                        entry.update_access();
                        self.stats.hits += 1;
                        self.stats.total_hit_time_saved_ms += entry.analysis_duration_ms;
                        return Ok(Some(entry.findings.clone()));
                    } else {
                        self.stats.file_changed_misses += 1;
                        self.remove_entry(file_path);
                    }
                }
                Err(_) => {
                    // File might have been deleted
                    self.stats.file_error_misses += 1;
                    self.remove_entry(file_path);
                }
            }
        }

        self.stats.misses += 1;
        Ok(None)
    }

    /// Store findings in cache with metadata
    pub fn put(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
        analysis_duration_ms: u64,
    ) -> Result<()> {
        let metadata = FileMetadata::from_file(file_path)?;

        let entry = CacheEntry::new(
            findings,
            metadata.content_hash,
            config_hash.to_string(),
            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(metadata.modified_time),
            metadata.size,
            analysis_duration_ms,
        );

        let entry_size = self.estimate_entry_size(&entry);

        // Check if we need to make room
        self.ensure_capacity(entry_size)?;

        // Remove existing entry if present
        if self.entries.contains_key(file_path) {
            self.remove_entry(file_path);
        }

        // Add new entry
        self.current_memory_bytes += entry_size;
        self.entries.insert(file_path.to_path_buf(), entry);
        self.stats.entries_added += 1;

        Ok(())
    }

    /// Remove an entry and update memory tracking
    fn remove_entry(&mut self, file_path: &Path) {
        if let Some(entry) = self.entries.remove(file_path) {
            let entry_size = self.estimate_entry_size(&entry);
            self.current_memory_bytes = self.current_memory_bytes.saturating_sub(entry_size);
            self.stats.entries_evicted += 1;
        }
    }

    /// Ensure cache has capacity for new entry
    fn ensure_capacity(&mut self, new_entry_size: usize) -> Result<()> {
        let max_memory_bytes = self.max_memory_mb * 1024 * 1024;

        // Check memory limit
        while self.current_memory_bytes + new_entry_size > max_memory_bytes
            || self.entries.len() >= self.max_entries
        {
            if self.entries.is_empty() {
                break;
            }

            self.evict_least_valuable_entry();
        }

        Ok(())
    }

    /// Evict the least valuable entry based on priority score
    fn evict_least_valuable_entry(&mut self) {
        if let Some((path_to_remove, _)) = self
            .entries
            .iter()
            .min_by(|(_, a), (_, b)| {
                a.priority_score()
                    .partial_cmp(&b.priority_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            self.remove_entry(&path_to_remove);
        }
    }

    /// Estimate memory usage of a cache entry
    fn estimate_entry_size(&self, entry: &CacheEntry) -> usize {
        let base_size = std::mem::size_of::<CacheEntry>();
        let findings_size = entry.findings.len() * std::mem::size_of::<Finding>();
        let string_sizes = entry.file_hash.len() + entry.config_hash.len();

        // Add estimated size for Finding string fields
        let finding_strings_size: usize = entry
            .findings
            .iter()
            .map(|f| {
                f.message.len()
                    + f.description.as_ref().map_or(0, |s| s.len())
                    + f.suggestion.as_ref().map_or(0, |s| s.len())
            })
            .sum();

        base_size + findings_size + string_sizes + finding_strings_size
    }

    /// Clean up expired and invalid entries
    pub fn cleanup(&mut self, max_age_hours: u64) -> usize {
        let max_age_seconds = max_age_hours * 3600;
        let mut removed_count = 0;

        let paths_to_remove: Vec<PathBuf> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.age_seconds() >= max_age_seconds)
            .map(|(path, _)| path.clone())
            .collect();

        for path in paths_to_remove {
            self.remove_entry(&path);
            removed_count += 1;
        }

        self.stats.entries_expired += removed_count;
        removed_count as usize
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get current cache utilization
    pub fn utilization(&self) -> CacheUtilization {
        CacheUtilization {
            entry_count: self.entries.len(),
            max_entries: self.max_entries,
            memory_usage_mb: self.current_memory_bytes as f64 / (1024.0 * 1024.0),
            max_memory_mb: self.max_memory_mb as f64,
            hit_rate: self.stats.hit_rate(),
            average_entry_size_kb: if self.entries.is_empty() {
                0.0
            } else {
                (self.current_memory_bytes as f64 / self.entries.len() as f64) / 1024.0
            },
        }
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        let cleared_count = self.entries.len();
        self.entries.clear();
        self.current_memory_bytes = 0;
        self.stats.entries_evicted += cleared_count as u64;
    }

    /// Save cache to disk
    pub async fn save_to_disk(&self, cache_file: &Path) -> Result<()> {
        let serialized = serde_json::to_string_pretty(&self.entries)?;
        tokio::fs::write(cache_file, serialized).await?;
        Ok(())
    }

    /// Load cache from disk
    pub async fn load_from_disk(&mut self, cache_file: &Path) -> Result<()> {
        if !tokio::fs::try_exists(cache_file).await.unwrap_or(false) {
            return Ok(());
        }

        let content = tokio::fs::read_to_string(cache_file).await?;
        let entries: HashMap<PathBuf, CacheEntry> = serde_json::from_str(&content)?;

        // Validate and load entries
        for (path, entry) in entries {
            // Quick validation - check if file still exists and config is reasonable
            if tokio::fs::try_exists(&path).await.unwrap_or(false) && !entry.config_hash.is_empty()
            {
                let entry_size = self.estimate_entry_size(&entry);
                self.current_memory_bytes += entry_size;
                self.entries.insert(path, entry);
            }
        }

        Ok(())
    }
}

impl Default for OptimizedCache {
    fn default() -> Self {
        Self::new(1000, 100) // 1000 entries, 100MB max
    }
}

/// Cache performance statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub total_requests: u64,
    pub hits: u64,
    pub misses: u64,
    pub config_misses: u64,
    pub file_changed_misses: u64,
    pub file_error_misses: u64,
    pub entries_added: u64,
    pub entries_evicted: u64,
    pub entries_expired: u64,
    pub total_hit_time_saved_ms: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.hits as f64 / self.total_requests as f64
        }
    }

    pub fn miss_rate(&self) -> f64 {
        1.0 - self.hit_rate()
    }

    pub fn time_saved_seconds(&self) -> f64 {
        self.total_hit_time_saved_ms as f64 / 1000.0
    }

    pub fn report(&self) -> String {
        format!(
            "Cache Performance Report:\n\
             - Total requests: {}\n\
             - Hit rate: {:.1}%\n\
             - Cache hits: {} (saved {:.1}s)\n\
             - Cache misses: {} (config: {}, file changed: {}, file error: {})\n\
             - Entries added: {}\n\
             - Entries evicted: {}\n\
             - Entries expired: {}",
            self.total_requests,
            self.hit_rate() * 100.0,
            self.hits,
            self.time_saved_seconds(),
            self.misses,
            self.config_misses,
            self.file_changed_misses,
            self.file_error_misses,
            self.entries_added,
            self.entries_evicted,
            self.entries_expired
        )
    }
}

/// Current cache utilization metrics
#[derive(Debug)]
pub struct CacheUtilization {
    pub entry_count: usize,
    pub max_entries: usize,
    pub memory_usage_mb: f64,
    pub max_memory_mb: f64,
    pub hit_rate: f64,
    pub average_entry_size_kb: f64,
}

impl CacheUtilization {
    pub fn entry_utilization_percentage(&self) -> f64 {
        if self.max_entries == 0 {
            0.0
        } else {
            (self.entry_count as f64 / self.max_entries as f64) * 100.0
        }
    }

    pub fn memory_utilization_percentage(&self) -> f64 {
        if self.max_memory_mb == 0.0 {
            0.0
        } else {
            (self.memory_usage_mb / self.max_memory_mb) * 100.0
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Cache Utilization:\n\
             - Entries: {}/{} ({:.1}%)\n\
             - Memory: {:.1}/{:.1} MB ({:.1}%)\n\
             - Hit rate: {:.1}%\n\
             - Average entry size: {:.1} KB",
            self.entry_count,
            self.max_entries,
            self.entry_utilization_percentage(),
            self.memory_usage_mb,
            self.max_memory_mb,
            self.memory_utilization_percentage(),
            self.hit_rate * 100.0,
            self.average_entry_size_kb
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_cache_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = OptimizedCache::new(10, 10);
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.rs");

        std::fs::write(&test_file, "fn test() {}")?;

        let findings = vec![];
        let config_hash = "test_config";

        // Put and get
        cache.put(&test_file, findings.clone(), config_hash, 100)?;
        let result = cache.get(&test_file, config_hash)?;

        assert!(result.is_some());
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().hit_rate(), 1.0);
        Ok(())
    }

    #[test]
    fn test_cache_invalidation_on_file_change() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = OptimizedCache::new(10, 10);
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.rs");

        std::fs::write(&test_file, "fn test() {}")?;

        let findings = vec![];
        let config_hash = "test_config";

        cache.put(&test_file, findings, config_hash, 100)?;

        // Modify file
        std::thread::sleep(Duration::from_millis(10));
        std::fs::write(&test_file, "fn test() { println!(); }")?;

        // Should miss due to file change
        let result = cache.get(&test_file, config_hash)?;
        assert!(result.is_none());
        assert_eq!(cache.stats().file_changed_misses, 1);
        Ok(())
    }

    #[test]
    fn test_cache_eviction() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = OptimizedCache::new(2, 1); // Very small cache
        let temp_dir = tempdir()?;

        // Add entries that exceed capacity
        for i in 0..3 {
            let test_file = temp_dir.path().join(format!("test_{}.rs", i));
        std::fs::write(&test_file, "fn test() {}").unwrap();

            let findings = vec![];
            cache.put(&test_file, findings, "config", 100)?;
        }

        // Should have evicted some entries
        assert!(cache.stats().entries_evicted > 0);
        assert!(cache.entries.len() <= 2);
    }

    #[test]
    fn test_cache_cleanup() -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = OptimizedCache::new(10, 10);
        let temp_dir = tempdir()?;
        let test_file = temp_dir.path().join("test.rs");

        std::fs::write(&test_file, "fn test() {}")?;

        let findings = vec![];
        cache.put(&test_file, findings, "config", 100)?;

        // Cleanup with 0 max age should remove all entries
        let removed = cache.cleanup(0);
        assert_eq!(removed, 1);
        assert!(cache.entries.is_empty());
    }

    #[tokio::test]
    async fn test_cache_persistence() {
        let temp_dir = tempdir().unwrap();
        let cache_file = temp_dir.path().join("cache.json");
        let test_file = temp_dir.path().join("test.rs");

        std::fs::write(&test_file, "fn test() {}")?;

        // Create and populate cache
        let mut cache = OptimizedCache::new(10, 10);
        let findings = vec![];
        cache.put(&test_file, findings, "config", 100)?;

        // Save to disk
        cache.save_to_disk(&cache_file).await?;
        assert!(cache_file.exists());

        // Load into new cache
        let mut new_cache = OptimizedCache::new(10, 10);
        new_cache.load_from_disk(&cache_file).await?;

        // Should have loaded the entry
        assert!(!new_cache.entries.is_empty());
        Ok(())
    }
}
