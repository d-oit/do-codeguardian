use crate::types::Finding;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;
use sha2::{Digest, Sha256};

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
}

impl FileCache {
    const CACHE_VERSION: &'static str = "1.0.0";
    const CACHE_FILE: &'static str = ".codeguardian-cache.json";

    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            cache_version: Self::CACHE_VERSION.to_string(),
        }
    }

    pub async fn load() -> Result<Self> {
        if let Ok(content) = fs::read_to_string(Self::CACHE_FILE).await {
            match serde_json::from_str::<Self>(&content) {
                Ok(cache) if cache.cache_version == Self::CACHE_VERSION => Ok(cache),
                _ => {
                    // Cache version mismatch or invalid format, start fresh
                    eprintln!("Cache version mismatch or invalid format, starting fresh");
                    Ok(Self::new())
                }
            }
        } else {
            Ok(Self::new())
        }
    }

    #[allow(dead_code)]
    pub async fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(Self::CACHE_FILE, content).await?;
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

    #[allow(dead_code)]
    pub fn cleanup_stale_entries(&mut self, max_age_days: u64) {
        let cutoff = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() - (max_age_days * 24 * 60 * 60);

        let initial_count = self.entries.len();
        self.entries.retain(|_, entry| {
            entry.cached_at
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
            let mut entries_vec: Vec<_> = self.entries.iter()
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
                println!("Cache size cleanup: removed {} entries to reduce size", removed_count);
            }
        }
    }

    /// Perform comprehensive cache maintenance
    #[allow(dead_code)]
    pub async fn perform_maintenance(&mut self, max_age_days: u64, max_size_mb: usize) -> Result<()> {
        // Clean up stale entries
        self.cleanup_stale_entries(max_age_days);
        
        // Clean up by size if needed
        self.cleanup_by_size(max_size_mb);
        
        // Save the cleaned cache
        self.save().await?;
        
        Ok(())
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