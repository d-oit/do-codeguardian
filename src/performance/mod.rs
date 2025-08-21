use crate::types::{Finding, AnalysisResults};
use anyhow::Result;
// Rayon import removed for now
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

/// Performance-optimized analysis engine for large codebases
pub struct PerformanceEngine {
    max_parallel_files: usize,
    memory_limit_mb: usize,
    #[allow(dead_code)]
    enable_streaming: bool,
    file_size_threshold: u64,
    progress_callback: Option<Box<dyn Fn(usize, usize) + Send + Sync>>,
}

impl Default for PerformanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceEngine {
    pub fn new() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            max_parallel_files: (cpu_count * 2).min(16), // Cap at 16 for memory reasons
            memory_limit_mb: 512, // 512MB memory limit
            enable_streaming: true,
            file_size_threshold: 5 * 1024 * 1024, // 5MB threshold for streaming
            progress_callback: None,
        }
    }

    pub fn with_parallel_limit(mut self, limit: usize) -> Self {
        self.max_parallel_files = limit;
        self
    }

    pub fn with_memory_limit(mut self, limit_mb: usize) -> Self {
        self.memory_limit_mb = limit_mb;
        self
    }

    pub fn with_streaming_threshold(mut self, threshold: u64) -> Self {
        self.file_size_threshold = threshold;
        self
    }

    #[allow(dead_code)]
    pub fn with_progress_callback<F>(mut self, callback: F) -> Self 
    where
        F: Fn(usize, usize) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    /// High-performance batch analysis for large codebases
    pub async fn analyze_batch<F>(
        &self,
        files: Vec<&Path>,
        analyzer_fn: F,
    ) -> Result<AnalysisResults>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        let start_time = Instant::now();
        let total_files = files.len();
        let processed_count = Arc::new(AtomicUsize::new(0));
        
        // Create semaphore to limit concurrent file operations
        let semaphore = Arc::new(Semaphore::new(self.max_parallel_files));
        
        // Split files into small and large categories
        let (small_files, large_files): (Vec<_>, Vec<_>) = files
            .into_iter()
            .partition(|path| {
                path.metadata()
                    .map(|m| m.len() < self.file_size_threshold)
                    .unwrap_or(true)
            });

        // Process small files in parallel batches
        let small_findings = self.process_small_files_parallel(
            small_files,
            analyzer_fn.clone(),
            semaphore.clone(),
            processed_count.clone(),
            total_files,
        ).await?;

        // Process large files with streaming
        let large_findings = self.process_large_files_streaming(
            large_files,
            analyzer_fn,
            semaphore,
            processed_count.clone(),
            total_files,
        ).await?;

        // Combine results
        let mut all_findings = small_findings;
        all_findings.extend(large_findings);

        let duration = start_time.elapsed();
        
        let mut results = AnalysisResults::new("turbo".to_string());
        for finding in all_findings {
            results.add_finding(finding);
        }
        results.summary.total_files_scanned = total_files;
        results.summary.scan_duration_ms = duration.as_millis() as u64;
        Ok(results)
    }

    async fn process_small_files_parallel<F>(
        &self,
        files: Vec<&Path>,
        analyzer_fn: F,
        semaphore: Arc<Semaphore>,
        processed_count: Arc<AtomicUsize>,
        total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        // Process small files in memory-efficient chunks
        let chunk_size = self.calculate_optimal_chunk_size(files.len());
        let mut all_findings = Vec::new();

        for chunk in files.chunks(chunk_size) {
            let chunk_paths: Vec<_> = chunk.iter().map(|&p| p.to_path_buf()).collect();
            let chunk_findings = self.process_file_chunk(
                chunk_paths,
                analyzer_fn.clone(),
                semaphore.clone(),
                processed_count.clone(),
                total_files,
            ).await?;
            
            all_findings.extend(chunk_findings);
        }

        Ok(all_findings)
    }

    async fn process_file_chunk<F>(
        &self,
        files: Vec<std::path::PathBuf>,
        analyzer_fn: F,
        semaphore: Arc<Semaphore>,
        processed_count: Arc<AtomicUsize>,
        _total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        let mut all_findings = Vec::new();
        
        for path in files {
            let _permit = semaphore.acquire().await?;
            let analyzer = analyzer_fn.clone();
            let counter = processed_count.clone();
            
            let result = tokio::task::spawn_blocking(move || {
                let content = std::fs::read(&path)?;
                analyzer(&path, &content)
            }).await??;
            
            all_findings.extend(result);
            counter.fetch_add(1, Ordering::Relaxed);
        }

        Ok(all_findings)
    }

    async fn process_large_files_streaming<F>(
        &self,
        files: Vec<&Path>,
        analyzer_fn: F,
        semaphore: Arc<Semaphore>,
        processed_count: Arc<AtomicUsize>,
        total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        let mut all_findings = Vec::new();
        
        for path in files {
            let _permit = semaphore.acquire().await?;
            
            // Use streaming analysis for large files
            let findings = self.analyze_large_file_streaming(&path, analyzer_fn.clone()).await?;
            all_findings.extend(findings);
            
            let count = processed_count.fetch_add(1, Ordering::Relaxed) + 1;
            if let Some(callback) = &self.progress_callback {
                callback(count, total_files);
            }
        }

        Ok(all_findings)
    }

    async fn analyze_large_file_streaming<F>(
        &self,
        path: &Path,
        analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync,
    {
        use tokio::io::{AsyncReadExt, BufReader};
        use tokio::fs::File;

        let file = File::open(path).await?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; self.calculate_streaming_chunk_size()];
        let mut all_findings = Vec::new();

        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            let chunk = &buffer[..bytes_read];
            let findings = analyzer_fn(path, chunk)?;
            all_findings.extend(findings);

            // Yield control to prevent blocking
            tokio::task::yield_now().await;
        }

        Ok(all_findings)
    }

    fn calculate_optimal_chunk_size(&self, total_files: usize) -> usize {
        // Balance between memory usage and parallelism
        let memory_per_file = (self.memory_limit_mb * 1024 * 1024) / self.max_parallel_files;
        let files_per_chunk = (memory_per_file / (1024 * 1024)).max(1); // At least 1MB per file
        
        files_per_chunk.min(total_files / self.max_parallel_files + 1)
    }

    fn calculate_streaming_chunk_size(&self) -> usize {
        // Use larger chunks for streaming to reduce I/O overhead
        (self.memory_limit_mb * 1024 * 1024 / self.max_parallel_files / 4).max(64 * 1024)
    }
}

/// Memory-efficient file iterator for large codebases
pub struct LargeCodebaseIterator {
    ignore_walker: ignore::Walk,
    size_threshold: u64,
    max_files: Option<usize>,
    processed: usize,
}

impl LargeCodebaseIterator {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            ignore_walker: ignore::Walk::new(root),
            size_threshold: 100 * 1024 * 1024, // 100MB max file size
            max_files: None,
            processed: 0,
        }
    }

    pub fn with_size_limit(mut self, limit: u64) -> Self {
        self.size_threshold = limit;
        self
    }

    pub fn with_file_limit(mut self, limit: usize) -> Self {
        self.max_files = Some(limit);
        self
    }

    pub fn collect_files(mut self) -> Result<Vec<std::path::PathBuf>> {
        let mut files = Vec::new();
        
        for entry in self.ignore_walker {
            if let Some(max) = self.max_files {
                if self.processed >= max {
                    break;
                }
            }

            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                // Check file size
                if let Ok(metadata) = path.metadata() {
                    if metadata.len() <= self.size_threshold {
                        files.push(path.to_path_buf());
                        self.processed += 1;
                    }
                }
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_performance_engine() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create test files
        for i in 0..10 {
            let file_path = temp_dir.path().join(format!("test_{}.rs", i));
            fs::write(&file_path, format!("// Test file {}\nfn main() {{}}", i)).unwrap();
        }

        let engine = PerformanceEngine::new()
            .with_parallel_limit(4)
            .with_memory_limit(64);

        let files: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

        let file_refs: Vec<_> = files.iter().map(|p| p.as_path()).collect();

        let results = engine
            .analyze_batch(file_refs, |_path, content| {
                // Simple analyzer that counts lines
                let line_count = content.iter().filter(|&&b| b == b'\n').count();
                if line_count > 0 {
                    Ok(vec![Finding::new(
                        "test",
                        "line_count",
                        crate::types::Severity::Info,
                        _path.to_path_buf(),
                        1,
                        format!("File has {} lines", line_count),
                    )])
                } else {
                    Ok(vec![])
                }
            })
            .await
            .unwrap();

        assert_eq!(results.summary.total_files_scanned, 10);
        assert!(results.summary.scan_duration_ms > 0);
    }
}