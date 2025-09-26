#![allow(clippy::disallowed_methods)]

use crate::types::Finding;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub mod enhanced_optimized_cache;
pub mod memory_pool;
pub mod optimized_cache;
pub mod regex_cache;
pub mod unified_cache;

// Re-export the unified cache for easy access
use tokio::fs;
pub use unified_cache::{CacheStrategyType, MemoryPoolSizes, UnifiedCache, UnifiedCacheConfig};

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
    pub entries: HashMap<PathBuf, CacheEntry>,
    pub cache_version: String,
    #[serde(skip)]
    pub cache_path: PathBuf,
}

impl Default for FileCache {
    fn default() -> Self {
        Self::new(PathBuf::from(".codeguardian/cache/cache.json"))
    }
}

impl FileCache {
    const CACHE_VERSION: &'static str = "1.0.0";

    pub fn new(cache_path: PathBuf) -> Self {
        Self {
            entries: HashMap::new(),
            cache_version: Self::CACHE_VERSION.to_string(),
            cache_path,
        }
    }

    pub async fn load(cache_path: PathBuf) -> Result<Self> {
        // Ensure directory exists
        if let Some(parent) = cache_path.parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }

        if let Ok(content) = fs::read_to_string(&cache_path).await {
            match serde_json::from_str::<Self>(&content) {
                Ok(mut cache) if cache.cache_version == Self::CACHE_VERSION => {
                    cache.cache_path = cache_path;
                    Ok(cache)
                }
                _ => {
                    // Cache version mismatch or invalid format, start fresh
                    tracing::warn!("Cache version mismatch or invalid format, starting fresh");
                    Ok(Self::new(cache_path))
                }
            }
        } else {
            Ok(Self::new(cache_path))
        }
    }

    pub async fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&self.cache_path, content).await?;
        Ok(())
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
        Ok(())
    }

    fn compute_content_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())[..16].to_string()
    }

    pub fn cleanup_stale_entries(
        &mut self,
        max_age_days: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cutoff = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            - (max_age_days * 24 * 60 * 60);

        self.entries.retain(|_, entry| {
            entry
                .cached_at
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() > cutoff)
                .unwrap_or(false)
        });
        Ok(())
    }

    pub fn stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.entries.len(),
            cache_size_bytes: self.estimate_size(),
        }
    }

    fn estimate_size(&self) -> usize {
        // Rough estimation of cache size in memory
        self.entries.len() * 1024 // Approximate 1KB per entry
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub cache_size_bytes: usize,
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
