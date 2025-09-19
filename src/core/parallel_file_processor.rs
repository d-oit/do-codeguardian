use crate::analyzers::AnalyzerRegistry;
use crate::types::Finding;
use anyhow::Result;
use memmap2::Mmap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, BufReader as AsyncBufReader};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tracing::{debug, warn};

// Constants for optimization
const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB
const BUFFER_SIZE: usize = 64 * 1024; // 64KB buffer

/// Parallel file processor with bounded concurrency and optimized I/O
pub struct ParallelFileProcessor {
    max_concurrent_files: usize,
    chunk_size: usize,
}

impl ParallelFileProcessor {
    /// Create a new parallel file processor
    pub fn new(max_concurrent_files: Option<usize>) -> Self {
        let cpu_count = num_cpus::get();
        let max_concurrent = max_concurrent_files.unwrap_or(cpu_count * 2);

        Self {
            max_concurrent_files: max_concurrent,
            chunk_size: std::cmp::max(max_concurrent / 4, 1),
        }
    }

    /// Process files in parallel with optimal batching
    pub async fn process_files_parallel(
        &self,
        files: &[PathBuf],
        analyzer_registry: &AnalyzerRegistry,
        config_hash: &str,
    ) -> Result<Vec<Finding>> {
        if files.is_empty() {
            return Ok(Vec::new());
        }

        debug!(
            "Processing {} files with {} concurrent workers",
            files.len(),
            self.max_concurrent_files
        );

        // Create semaphore to limit concurrent file operations
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_files));
        let analyzer_registry = Arc::new(analyzer_registry.clone());

        // Process files in chunks for better memory management
        let mut all_findings = Vec::new();

        for chunk in files.chunks(self.chunk_size * self.max_concurrent_files) {
            let chunk_findings = self
                .process_file_chunk(
                    chunk,
                    Arc::clone(&semaphore),
                    analyzer_registry.clone(),
                    config_hash,
                )
                .await?;

            all_findings.extend(chunk_findings);
        }

        debug!(
            "Parallel processing completed. Found {} total findings",
            all_findings.len()
        );
        Ok(all_findings)
    }

    /// Process a chunk of files in parallel
    async fn process_file_chunk(
        &self,
        files: &[PathBuf],
        semaphore: Arc<Semaphore>,
        analyzer_registry: Arc<AnalyzerRegistry>,
        config_hash: &str,
    ) -> Result<Vec<Finding>> {
        let mut tasks: Vec<JoinHandle<Result<Vec<Finding>>>> = Vec::new();

        // Spawn tasks for each file in the chunk
        for file_path in files {
            let file_path = file_path.clone();
            let semaphore = Arc::clone(&semaphore);
            let analyzer_registry = Arc::clone(&analyzer_registry);
            let config_hash = config_hash.to_string();

            let task = tokio::spawn(async move {
                // Acquire semaphore permit to limit concurrency
                let _permit = semaphore
                    .acquire()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to acquire semaphore permit: {}", e))?;

                Self::process_single_file_optimized(&file_path, &analyzer_registry, &config_hash)
                    .await
            });

            tasks.push(task);
        }

        // Collect results from all tasks
        let mut all_findings = Vec::new();
        let mut errors = 0;

        for task in tasks {
            match task.await {
                Ok(Ok(findings)) => {
                    all_findings.extend(findings);
                }
                Ok(Err(e)) => {
                    warn!("File analysis error: {}", e);
                    errors += 1;
                }
                Err(e) => {
                    warn!("Task join error: {}", e);
                    errors += 1;
                }
            }
        }

        if errors > 0 {
            debug!("Completed chunk processing with {} errors", errors);
        }

        Ok(all_findings)
    }

    /// Process a single file with optimized I/O (async buffered or memory-mapped)
    async fn process_single_file_optimized(
        file_path: &Path,
        analyzer_registry: &AnalyzerRegistry,
        _config_hash: &str,
    ) -> Result<Vec<Finding>> {
        // Get file metadata to determine optimal reading strategy
        let metadata = tokio::fs::metadata(file_path).await.map_err(|e| {
            anyhow::anyhow!("Failed to get metadata for {}: {}", file_path.display(), e)
        })?;

        let file_size = metadata.len();

        let content = if file_size > LARGE_FILE_THRESHOLD {
            // Use memory-mapped I/O for large files
            Self::read_file_memory_mapped(file_path).await?
        } else {
            // Use async buffered I/O for smaller files
            Self::read_file_async_buffered(file_path).await?
        };

        // Analyze file content
        let findings = analyzer_registry.analyze_file(file_path, &content)?;

        debug!(
            "Analyzed {} ({} bytes): {} findings",
            file_path.display(),
            file_size,
            findings.len()
        );
        Ok(findings)
    }

    /// Read file using async buffered I/O
    async fn read_file_async_buffered(file_path: &Path) -> Result<Vec<u8>> {
        let file = tokio::fs::File::open(file_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to open file {}: {}", file_path.display(), e))?;

        let mut reader = AsyncBufReader::with_capacity(BUFFER_SIZE, file);
        let mut content = Vec::new();

        reader
            .read_to_end(&mut content)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path.display(), e))?;

        Ok(content)
    }

    /// Read file using memory-mapped I/O for large files
    async fn read_file_memory_mapped(file_path: &Path) -> Result<Vec<u8>> {
        // Memory mapping is synchronous, so we spawn it on a blocking task
        let file_path = file_path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let file = File::open(&file_path).map_err(|e| {
                anyhow::anyhow!("Failed to open file {}: {}", file_path.display(), e)
            })?;

            let mmap = unsafe { Mmap::map(&file) }.map_err(|e| {
                anyhow::anyhow!("Failed to memory map file {}: {}", file_path.display(), e)
            })?;

            // Convert to owned Vec<u8> for compatibility with existing analyzer interface
            Ok(mmap.to_vec())
        })
        .await?
    }

    /// Optimized batch file reading for large numbers of files
    pub async fn batch_read_files(&self, files: &[PathBuf]) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_files));
        let mut tasks = Vec::new();

        for file_path in files {
            let file_path = file_path.clone();
            let semaphore = Arc::clone(&semaphore);

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await?;
                let content = Self::read_file_async_buffered(&file_path).await?;
                Ok::<(PathBuf, Vec<u8>), anyhow::Error>((file_path, content))
            });

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            match task.await? {
                Ok((path, content)) => results.push((path, content)),
                Err(e) => warn!("Failed to read file: {}", e),
            }
        }

        Ok(results)
    }

    /// Get optimal chunk size based on file count and system resources
    pub fn get_optimal_chunk_size(&self, total_files: usize) -> usize {
        if total_files <= self.max_concurrent_files {
            total_files
        } else {
            // Aim for 4-8 chunks per worker for good load balancing
            let chunks_per_worker = 6;
            let total_chunks = self.max_concurrent_files * chunks_per_worker;
            std::cmp::max(total_files / total_chunks, 1)
        }
    }

    /// Estimate processing time based on file count and system capabilities
    pub fn estimate_processing_time(&self, file_count: usize) -> std::time::Duration {
        // Rough estimates based on typical performance
        let base_time_per_file = std::time::Duration::from_millis(8); // Improved with optimizations
        let parallel_efficiency = 0.85; // Better efficiency with optimized I/O

        let sequential_time = base_time_per_file * file_count as u32;
        sequential_time.div_f64(self.max_concurrent_files as f64 * parallel_efficiency)
    }
}

impl Default for ParallelFileProcessor {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Parallel processing statistics
#[derive(Debug, Default)]
pub struct ParallelProcessingStats {
    pub files_processed: usize,
    pub total_findings: usize,
    pub processing_time: std::time::Duration,
    pub errors: usize,
    pub average_time_per_file: std::time::Duration,
    pub throughput_files_per_second: f64,
}

impl ParallelProcessingStats {
    pub fn new(
        files_processed: usize,
        total_findings: usize,
        processing_time: std::time::Duration,
        errors: usize,
    ) -> Self {
        let average_time_per_file = if files_processed > 0 {
            processing_time / files_processed as u32
        } else {
            std::time::Duration::from_nanos(0)
        };

        let throughput_files_per_second = if processing_time.as_secs_f64() > 0.0 {
            files_processed as f64 / processing_time.as_secs_f64()
        } else {
            0.0
        };

        Self {
            files_processed,
            total_findings,
            processing_time,
            errors,
            average_time_per_file,
            throughput_files_per_second,
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Parallel Processing Report:\n\
             - Files processed: {}\n\
             - Total findings: {}\n\
             - Processing time: {:.2}s\n\
             - Errors: {}\n\
             - Average time per file: {:.2}ms\n\
             - Throughput: {:.1} files/second",
            self.files_processed,
            self.total_findings,
            self.processing_time.as_secs_f64(),
            self.errors,
            self.average_time_per_file.as_millis(),
            self.throughput_files_per_second
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_parallel_file_processor_creation() {
        let processor = ParallelFileProcessor::new(Some(4));
        assert_eq!(processor.max_concurrent_files, 4);
    }

    #[tokio::test]
    async fn test_batch_file_reading() {
        let temp_dir = tempdir().unwrap();
        let processor = ParallelFileProcessor::new(Some(2));

        // Create test files
        let mut test_files = Vec::new();
        for i in 0..5 {
            let file_path = temp_dir.path().join(format!("test_{}.txt", i));
            tokio::fs::write(&file_path, format!("test content {}", i))
                .await
                .unwrap();
            test_files.push(file_path);
        }

        let start = Instant::now();
        let results = processor.batch_read_files(&test_files).await.unwrap();
        let duration = start.elapsed();

        assert_eq!(results.len(), 5);
        assert!(duration < std::time::Duration::from_millis(100)); // Should be fast

        // Verify content
        for (i, (path, content)) in results.iter().enumerate() {
            assert!(path.to_string_lossy().contains(&format!("test_{}.txt", i)));
            assert_eq!(content, format!("test content {}", i).as_bytes());
        }
    }

    #[tokio::test]
    async fn test_large_file_memory_mapping() {
        let temp_dir = tempdir().unwrap();
        let large_file_path = temp_dir.path().join("large_test.txt");

        // Create a file larger than the threshold
        let large_content = "test content\n".repeat(200000); // ~2MB
        tokio::fs::write(&large_file_path, &large_content)
            .await
            .unwrap();

        let content = ParallelFileProcessor::read_file_memory_mapped(&large_file_path)
            .await
            .unwrap();
        assert_eq!(content, large_content.as_bytes());
    }

    #[tokio::test]
    async fn test_small_file_async_buffered() {
        let temp_dir = tempdir().unwrap();
        let small_file_path = temp_dir.path().join("small_test.txt");

        let small_content = "small test content";
        tokio::fs::write(&small_file_path, small_content)
            .await
            .unwrap();

        let content = ParallelFileProcessor::read_file_async_buffered(&small_file_path)
            .await
            .unwrap();
        assert_eq!(content, small_content.as_bytes());
    }

    #[tokio::test]
    async fn test_optimal_chunk_size_calculation() {
        let processor = ParallelFileProcessor::new(Some(4));

        assert_eq!(processor.get_optimal_chunk_size(2), 2); // Fewer files than workers
        assert_eq!(processor.get_optimal_chunk_size(100), 4); // Many files
    }

    #[tokio::test]
    async fn test_processing_time_estimation() {
        let processor = ParallelFileProcessor::new(Some(4));
        let estimated = processor.estimate_processing_time(100);

        // Should estimate reasonable processing time
        assert!(estimated > std::time::Duration::from_millis(50));
        assert!(estimated < std::time::Duration::from_secs(5));
    }

    #[test]
    fn test_parallel_processing_stats() {
        let stats = ParallelProcessingStats::new(100, 50, std::time::Duration::from_secs(2), 1);

        assert_eq!(stats.files_processed, 100);
        assert_eq!(stats.total_findings, 50);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.throughput_files_per_second, 50.0);

        let report = stats.report();
        assert!(report.contains("100"));
        assert!(report.contains("50.0 files/second"));
    }
}
