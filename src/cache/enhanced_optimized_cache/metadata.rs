use super::*;

/// File metadata for cache validation
#[deprecated(
    since = "0.1.0",
    note = "This type is internal to the deprecated EnhancedOptimizedCache. \
            Use UnifiedCache instead."
)]
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub modified_time: u64,
    pub size: u64,
    pub content_hash: String,
}

impl FileMetadata {
    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
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

    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
    pub async fn from_file_async(file_path: &Path) -> Result<Self> {
        let metadata = tokio::fs::metadata(file_path).await?;
        let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
        let size = metadata.len();

        // Compute content hash for integrity
        let content = tokio::fs::read(file_path).await?;
        let content_hash = Self::compute_hash(&content);

        Ok(Self {
            modified_time,
            size,
            content_hash,
        })
    }

    #[deprecated(
        since = "0.1.0",
        note = "This type is deprecated along with EnhancedOptimizedCache"
    )]
    pub fn compute_hash(content: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(content);
        hasher.finalize().to_hex().to_string()
    }
}
