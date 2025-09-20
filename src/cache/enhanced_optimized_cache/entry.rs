use super::*;

/// Enhanced cache entry with pooled memory management
#[deprecated(
    since = "0.1.0",
    note = "This type is internal to the deprecated EnhancedOptimizedCache. \
            Use UnifiedCache instead."
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PooledCacheEntry {
    pub findings: Vec<Finding>,
    pub file_hash: String,
    pub config_hash: String,
    pub modified_time: u64,
    pub file_size: u64,
    pub access_count: u32,
    pub last_accessed: u64,
    pub analysis_duration_ms: u64,
}

impl PooledCacheEntry {
    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
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

    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
    pub fn update_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
    pub fn is_valid(&self, file_metadata: &super::FileMetadata, config_hash: &str) -> bool {
        self.config_hash == config_hash
            && self.modified_time == file_metadata.modified_time
            && self.file_size == file_metadata.size
            && self.file_hash == file_metadata.content_hash
    }

    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
    pub fn age_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.last_accessed)
    }

    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
    pub fn priority_score(&self) -> f64 {
        // Higher score = higher priority to keep in cache
        let access_weight = (self.access_count as f64).ln_1p();
        let recency_weight = 1.0 / (1.0 + self.age_seconds() as f64 / 3600.0); // Decay over hours
        let size_weight = 1.0 / (1.0 + self.file_size as f64 / 1024.0); // Prefer smaller files

        access_weight * recency_weight * size_weight
    }
}
