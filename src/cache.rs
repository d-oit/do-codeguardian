use crate::types::Finding;
use anyhow::Result;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub path: PathBuf,
    pub mtime: SystemTime,
    pub size: u64,
    pub content_hash: String,
    pub config_hash: String,
    pub findings: Vec<Finding>,
    pub cached_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCache {
    entries: HashMap<PathBuf, CacheEntry>,
    cache_version: String,
    #[serde(default)]
    compressed: bool,
    #[serde(default)]
    auto_save: bool,
    #[serde(default)]
    max_entries: usize,
}

impl Default for FileCache {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCache {
    const CACHE_VERSION: &'static str = "2.0.0"; // Updated version for new features
    const CACHE_FILE: &'static str = ".codeguardian-cache.json.gz"; // Compressed by default
    const CACHE_FILE_LEGACY: &'static str = ".codeguardian-cache.json"; // For migration

    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            cache_version: Self::CACHE_VERSION.to_string(),
            compressed: true,
            auto_save: true,
            max_entries: 10000, // Default limit
        }
    }

    pub fn new_with_config(compressed: bool, auto_save: bool, max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            cache_version: Self::CACHE_VERSION.to_string(),
            compressed,
            auto_save,
            max_entries,
        }
    }

    pub async fn load() -> Result<Self> {
        // Try compressed cache first
        if let Ok(content) = Self::load_compressed_cache().await {
            return Ok(content);
        }

        // Try legacy uncompressed cache
        if let Ok(content) = fs::read_to_string(Self::CACHE_FILE_LEGACY).await {
            match serde_json::from_str::<Self>(&content) {
                Ok(mut cache) => {
                    if cache.cache_version != Self::CACHE_VERSION {
                        eprintln!(
                            "Cache version mismatch, migrating from {} to {}",
                            cache.cache_version,
                            Self::CACHE_VERSION
                        );
                        cache = Self::migrate_cache(cache);
                    }
                    // Save in new compressed format
                    if let Err(e) = cache.save().await {
                        eprintln!("Warning: Failed to save migrated cache: {}", e);
                    }
                    Ok(cache)
                }
                Err(e) => {
                    eprintln!("Invalid legacy cache format: {}, starting fresh", e);
                    Ok(Self::new())
                }
            }
        } else {
            Ok(Self::new())
        }
    }

    async fn load_compressed_cache() -> Result<Self> {
        let compressed_data = fs::read(Self::CACHE_FILE).await?;
        let mut decoder = GzDecoder::new(&compressed_data[..]);
        let mut json_data = String::new();
        decoder.read_to_string(&mut json_data)?;

        match serde_json::from_str::<Self>(&json_data) {
            Ok(mut cache) => {
                if cache.cache_version != Self::CACHE_VERSION {
                    eprintln!(
                        "Compressed cache version mismatch, migrating from {} to {}",
                        cache.cache_version,
                        Self::CACHE_VERSION
                    );
                    cache = Self::migrate_cache(cache);
                }
                Ok(cache)
            }
            Err(e) => {
                eprintln!("Invalid compressed cache format: {}, starting fresh", e);
                Ok(Self::new())
            }
        }
    }

    fn migrate_cache(mut old_cache: Self) -> Self {
        // Update version and set new defaults
        old_cache.cache_version = Self::CACHE_VERSION.to_string();
        old_cache.compressed = true;
        old_cache.auto_save = true;
        old_cache.max_entries = 10000;

        // Clean up any invalid entries during migration
        old_cache.entries.retain(|path, _| path.exists());

        old_cache
    }

    pub async fn save(&self) -> Result<()> {
        if self.compressed {
            self.save_compressed().await
        } else {
            let content = serde_json::to_string_pretty(self)?;
            fs::write(Self::CACHE_FILE, content).await?;
            Ok(())
        }
    }

    async fn save_compressed(&self) -> Result<()> {
        let json_data = serde_json::to_string(self)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json_data.as_bytes())?;
        let compressed_data = encoder.finish()?;

        fs::write(Self::CACHE_FILE, compressed_data).await?;
        Ok(())
    }

    /// Save cache if auto-save is enabled and there are changes
    pub async fn auto_save(&self) -> Result<()> {
        if self.auto_save {
            self.save().await?;
        }
        Ok(())
    }

    /// Force save regardless of auto-save setting
    pub async fn force_save(&self) -> Result<()> {
        self.save().await
    }

    pub fn is_cached(&self, file_path: &Path, config_hash: &str) -> Option<&CacheEntry> {
        let entry = self.entries.get(file_path)?;

        // Check if config changed
        if entry.config_hash != config_hash {
            return None;
        }

        // Check if file metadata changed
        if let Ok(metadata) = file_path.metadata() {
            if let Ok(mtime) = metadata.modified() {
                if entry.mtime != mtime || entry.size != metadata.len() {
                    return None;
                }
            }
        }

        Some(entry)
    }

    pub fn get_cached_findings(&self, file_path: &Path, config_hash: &str) -> Option<Vec<Finding>> {
        self.is_cached(file_path, config_hash)
            .map(|entry| entry.findings.clone())
    }

    pub async fn cache_findings(
        &mut self,
        file_path: &Path,
        findings: Vec<Finding>,
        config_hash: &str,
    ) -> Result<()> {
        // Check if we should cache this file (size and entry limits)
        if !self.should_cache_file(file_path) {
            return Ok(());
        }

        let metadata = file_path.metadata()?;
        let mtime = metadata.modified()?;
        let size = metadata.len();

        // Compute content hash for additional verification
        let content = fs::read(file_path).await?;
        let content_hash = self.compute_content_hash(&content);

        let entry = CacheEntry {
            path: file_path.to_path_buf(),
            mtime,
            size,
            content_hash,
            config_hash: config_hash.to_string(),
            findings,
            cached_at: SystemTime::now(),
        };

        self.entries.insert(file_path.to_path_buf(), entry);

        // Enforce max entries limit
        self.enforce_size_limit();

        Ok(())
    }

    fn should_cache_file(&self, file_path: &Path) -> bool {
        // Don't cache very large files
        if let Ok(metadata) = file_path.metadata() {
            if metadata.len() > 50 * 1024 * 1024 {
                // 50MB limit
                return false;
            }
        }

        // Don't cache temporary or generated files
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            if file_name.starts_with('.')
                || file_name.ends_with(".tmp")
                || file_name.ends_with(".temp")
                || file_name.contains("generated")
            {
                return false;
            }
        }

        true
    }

    fn enforce_size_limit(&mut self) {
        if self.entries.len() > self.max_entries {
            // Remove oldest entries to stay within limit
            let to_remove = self.entries.len() - self.max_entries;
            let mut entries_by_age: Vec<_> = self
                .entries
                .iter()
                .map(|(path, entry)| (path.clone(), entry.cached_at))
                .collect();
            entries_by_age.sort_by_key(|(_, cached_at)| *cached_at);

            for (path, _) in entries_by_age.into_iter().take(to_remove) {
                self.entries.remove(&path);
            }
        }
    }

    fn compute_content_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    #[allow(dead_code)]
    pub fn cleanup_stale_entries(&mut self, max_age_days: u64) {
        let cutoff = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - (max_age_days * 24 * 60 * 60);

        let initial_count = self.entries.len();
        self.entries.retain(|_, entry| {
            entry
                .cached_at
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() > cutoff)
                .unwrap_or(false)
        });

        let removed_count = initial_count - self.entries.len();
        if removed_count > 0 {
            println!("Cache cleanup: removed {} stale entries", removed_count);
        }
    }

    /// Cleanup cache based on size limit
    #[allow(dead_code)]
    pub fn cleanup_by_size(&mut self, max_size_mb: usize) {
        let max_size_bytes = max_size_mb * 1024 * 1024;
        let current_size = self.estimate_size();

        if current_size > max_size_bytes {
            // Remove oldest entries first - collect paths to remove
            let mut entries_vec: Vec<_> = self
                .entries
                .iter()
                .map(|(path, entry)| (path.clone(), entry.cached_at))
                .collect();
            entries_vec.sort_by_key(|(_, cached_at)| *cached_at);

            let target_size = max_size_bytes * 80 / 100; // Target 80% of max size
            let mut current_size = self.estimate_size();
            let mut removed_count = 0;

            for (path, _) in entries_vec {
                if current_size <= target_size {
                    break;
                }

                if self.entries.remove(&path).is_some() {
                    // Rough estimate: each entry is about 1KB
                    current_size = current_size.saturating_sub(1024);
                    removed_count += 1;
                }
            }

            if removed_count > 0 {
                println!(
                    "Cache size cleanup: removed {} entries to reduce size",
                    removed_count
                );
            }
        }
    }

    /// Perform comprehensive cache maintenance
    pub async fn perform_maintenance(
        &mut self,
        max_age_days: u64,
        max_size_mb: usize,
    ) -> Result<()> {
        // Clean up stale entries
        self.cleanup_stale_entries(max_age_days);

        // Clean up by size if needed
        self.cleanup_by_size(max_size_mb);

        // Save the cleaned cache
        self.save().await?;

        Ok(())
    }

    /// Warm up cache for frequently accessed files
    pub async fn warm_up(&mut self, files: &[PathBuf], config_hash: &str) -> Result<()> {
        for file_path in files {
            if self.is_cached(file_path, config_hash).is_none() {
                // File not cached, we could pre-analyze it here
                // For now, just ensure the file is accessible
                if file_path.exists() {
                    // Could implement pre-analysis here if needed
                }
            }
        }
        Ok(())
    }

    /// Get cache performance statistics
    pub fn performance_stats(&self) -> CachePerformanceStats {
        let total_entries = self.entries.len();
        let mut total_findings = 0;
        let mut total_size = 0u64;
        let mut oldest_entry = SystemTime::now();
        let mut newest_entry = SystemTime::UNIX_EPOCH;

        for entry in self.entries.values() {
            total_findings += entry.findings.len();
            total_size += entry.size;
            oldest_entry = oldest_entry.min(entry.cached_at);
            newest_entry = newest_entry.max(entry.cached_at);
        }

        let avg_findings_per_file = if total_entries > 0 {
            total_findings as f64 / total_entries as f64
        } else {
            0.0
        };

        CachePerformanceStats {
            total_entries,
            total_findings,
            total_cached_size: total_size,
            avg_findings_per_file,
            oldest_entry,
            newest_entry,
            compression_enabled: self.compressed,
            auto_save_enabled: self.auto_save,
        }
    }

    /// Optimize cache for better performance
    pub fn optimize(&mut self) {
        // Rebuild hashmap with better capacity
        let new_capacity = (self.entries.len() * 4 / 3).max(16);
        let mut new_entries = HashMap::with_capacity(new_capacity);
        new_entries.extend(self.entries.drain());
        self.entries = new_entries;
    }

    #[allow(dead_code)]
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.entries.len(),
            cache_size_bytes: self.estimate_size(),
        }
    }

    #[allow(dead_code)]
    fn estimate_size(&self) -> usize {
        // Rough estimation of cache size in memory
        self.entries.len() * 1024 // Approximate 1KB per entry
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CacheStats {
    pub total_entries: usize,
    pub cache_size_bytes: usize,
}

#[derive(Debug)]
pub struct CachePerformanceStats {
    pub total_entries: usize,
    pub total_findings: usize,
    pub total_cached_size: u64,
    pub avg_findings_per_file: f64,
    pub oldest_entry: SystemTime,
    pub newest_entry: SystemTime,
    pub compression_enabled: bool,
    pub auto_save_enabled: bool,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache: {} entries, ~{:.1}KB",
            self.total_entries,
            self.cache_size_bytes as f64 / 1024.0
        )
    }
}
