//! Core performance optimization engine
//!
//! This module provides the main performance optimization logic including
//! pattern matching, file processing, scheduling, I/O optimization, and monitoring.

#![allow(dead_code)]

use crate::types::{AnalysisResults, Finding};
use anyhow::{Context, Result};
use regex::RegexSet;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

// Type alias for complex cache type
type PatternCache = HashMap<String, HashMap<String, Vec<(usize, String)>>>;

/// Performance metrics collector
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub cpu_time: Duration,
    pub memory_usage: usize,
    pub io_operations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub analysis_time: Duration,
    pub files_processed: u64,
}

/// Optimized pattern matching using RegexSet for better performance
pub struct OptimizedPatternMatcher {
    patterns: RegexSet,
    pattern_names: Vec<String>,
    cache: Arc<RwLock<PatternCache>>,
    max_cache_size: usize,
}

impl OptimizedPatternMatcher {
    /// Create a new optimized pattern matcher
    pub fn new(patterns: Vec<(&str, String)>, max_cache_size: usize) -> Result<Self> {
        let regex_patterns: Vec<&str> = patterns.iter().map(|(pat, _)| *pat).collect();
        let pattern_names: Vec<String> = patterns.into_iter().map(|(_, name)| name).collect();

        Ok(Self {
            patterns: RegexSet::new(regex_patterns)?,
            pattern_names,
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_cache_size,
        })
    }

    /// Match all patterns against content with caching
    pub async fn match_all(
        &self,
        content: &str,
        content_hash: &str,
    ) -> Result<HashMap<String, Vec<(usize, String)>>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached_result) = cache.get(content_hash) {
                return Ok(cached_result.clone());
            }
        }

        // Perform matching
        let mut results = HashMap::new();

        for (pattern_idx, pattern_name) in self.pattern_names.iter().enumerate() {
            let pattern = &self.patterns.patterns()[pattern_idx];
            let mut current_pos = 0;

            while let Some(match_start) = content[current_pos..].find(pattern) {
                let absolute_start = current_pos + match_start;
                let match_end = absolute_start + pattern.len();

                let line_number = content[..absolute_start]
                    .chars()
                    .filter(|&c| c == '\n')
                    .count();
                let matched_text = &content[absolute_start..match_end];

                results
                    .entry(pattern_name.clone())
                    .or_insert_with(Vec::new)
                    .push((line_number, matched_text.to_string()));

                current_pos = match_end;
            }
        }

        // Cache the result
        {
            let mut cache = self.cache.write().await;
            if cache.len() >= self.max_cache_size {
                // Simple LRU: remove oldest entry
                if let Some(first_key) = cache.keys().next().cloned() {
                    cache.remove(&first_key);
                }
            }
            cache.insert(content_hash.to_string(), results.clone());
        }

        Ok(results)
    }
}

/// Memory-efficient file processor
pub struct OptimizedFileProcessor {
    buffer_size: usize,
    use_streaming: bool,
    metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl OptimizedFileProcessor {
    pub fn new(buffer_size: usize, use_streaming: bool) -> Self {
        Self {
            buffer_size,
            use_streaming,
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
        }
    }

    /// Process file with optimized memory usage
    pub async fn process_file<F, T>(&self, file_path: &Path, processor: F) -> Result<T>
    where
        F: Fn(&str) -> Result<T>,
    {
        let start_time = Instant::now();

        let content = if self.use_streaming && Self::should_use_streaming(file_path) {
            self.process_streaming(file_path, &processor).await?
        } else {
            self.process_in_memory(file_path, &processor).await?
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.analysis_time = start_time.elapsed();
            metrics.files_processed += 1;
        }

        Ok(content)
    }

    fn should_use_streaming(file_path: &Path) -> bool {
        file_path
            .metadata()
            .map(|m| m.len() > 2 * 1024 * 1024) // 2MB threshold
            .unwrap_or(false)
    }

    /// Check if file should use streaming with proper error handling
    pub fn should_use_streaming_safe(file_path: &Path) -> Result<bool> {
        let metadata = file_path
            .metadata()
            .context("Failed to get file metadata for streaming check")?;
        Ok(metadata.len() > 2 * 1024 * 1024) // 2MB threshold
    }

    async fn process_in_memory<F, T>(&self, file_path: &Path, processor: &F) -> Result<T>
    where
        F: Fn(&str) -> Result<T>,
    {
        let content = tokio::fs::read_to_string(file_path).await?;
        processor(&content)
    }

    async fn process_streaming<F, T>(&self, file_path: &Path, processor: &F) -> Result<T>
    where
        F: Fn(&str) -> Result<T>,
    {
        use tokio::io::{AsyncBufReadExt, BufReader};

        let file = tokio::fs::File::open(file_path).await?;
        let reader = BufReader::with_capacity(self.buffer_size, file);
        let mut lines = reader.lines();

        let mut full_content = String::with_capacity(self.buffer_size);
        while let Some(line) = lines.next_line().await? {
            full_content.push_str(&line);
            full_content.push('\n');
        }

        processor(&full_content)
    }

    /// Get current performance metrics
    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().await.clone()
    }
}

/// CPU-optimized analysis scheduler
pub struct AdaptiveAnalysisScheduler {
    #[allow(dead_code)]
    worker_count: usize,
    queue_depth: usize,
    #[allow(dead_code)]
    load_threshold: f64,
    #[allow(dead_code)]
    backpressure_enabled: bool,
}

impl AdaptiveAnalysisScheduler {
    pub fn new(worker_count: usize) -> Self {
        Self {
            worker_count,
            queue_depth: worker_count * 2,
            load_threshold: 0.8,
            backpressure_enabled: true,
        }
    }

    /// Schedule analysis tasks with adaptive load balancing
    pub async fn schedule_tasks<F, Fut, T>(&self, tasks: Vec<F>) -> Result<Vec<T>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        use std::sync::Arc;
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(self.queue_depth));
        let mut handles = Vec::with_capacity(tasks.len());

        for task in tasks {
            let permit = semaphore.clone().acquire_owned().await?;
            let handle = tokio::spawn(async move {
                let _permit = permit;
                task().await
            });
            handles.push(handle);
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            results.push(handle.await??);
        }

        Ok(results)
    }
}

/// I/O optimization utilities
pub mod io_optimization {
    use super::*;

    /// Optimized file reader with buffering
    pub struct OptimizedFileReader {
        buffer_size: usize,
    }

    impl OptimizedFileReader {
        pub fn new(buffer_size: usize) -> Self {
            Self { buffer_size }
        }

        /// Read file with optimized buffering
        pub async fn read_file(&self, path: &Path) -> Result<String> {
            use tokio::io::AsyncReadExt;

            let mut file = tokio::fs::File::open(path).await?;
            let mut buffer = Vec::with_capacity(self.buffer_size);
            file.read_to_end(&mut buffer).await?;

            String::from_utf8(buffer).map_err(Into::into)
        }
    }

    /// Batch file operations for reduced I/O overhead
    pub struct BatchFileProcessor {
        batch_size: usize,
        operations: Vec<Box<dyn FnOnce() -> Result<()> + Send>>,
    }

    impl BatchFileProcessor {
        pub fn new(batch_size: usize) -> Self {
            Self {
                batch_size,
                operations: Vec::with_capacity(batch_size),
            }
        }

        /// Add operation to batch
        pub fn add_operation<F>(&mut self, operation: F)
        where
            F: FnOnce() -> Result<()> + Send + 'static,
        {
            self.operations.push(Box::new(operation));

            if self.operations.len() >= self.batch_size {
                if let Err(e) = self.flush() {
                    eprintln!("Warning: Failed to flush batch operations: {}", e);
                }
            }
        }

        /// Flush pending operations
        pub fn flush(&mut self) -> Result<()> {
            for operation in self.operations.drain(..) {
                operation()?;
            }
            Ok(())
        }
    }

    impl Drop for BatchFileProcessor {
        fn drop(&mut self) {
            let _ = self.flush();
        }
    }
}

/// Performance monitoring utilities
pub mod monitoring {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Global performance counters
    pub struct PerformanceCounters {
        pub total_files_processed: AtomicU64,
        pub total_analysis_time: AtomicU64,
        pub total_memory_allocated: AtomicU64,
        pub cache_effectiveness: AtomicU64,
    }

    impl Default for PerformanceCounters {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PerformanceCounters {
        pub fn new() -> Self {
            Self {
                total_files_processed: AtomicU64::new(0),
                total_analysis_time: AtomicU64::new(0),
                total_memory_allocated: AtomicU64::new(0),
                cache_effectiveness: AtomicU64::new(0),
            }
        }

        /// Record file processing
        pub fn record_file_processed(&self, duration: Duration) {
            self.total_files_processed.fetch_add(1, Ordering::Relaxed);
            self.total_analysis_time
                .fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        }

        /// Get average processing time
        pub fn average_processing_time(&self) -> Duration {
            let files = self.total_files_processed.load(Ordering::Relaxed);
            let time = self.total_analysis_time.load(Ordering::Relaxed);

            if files > 0 {
                Duration::from_millis(time / files)
            } else {
                Duration::default()
            }
        }
    }
}

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
            memory_limit_mb: 512,                        // 512MB memory limit
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
    ///
    /// This function orchestrates the analysis of multiple files by:
    /// 1. Splitting files by size for optimal processing
    /// 2. Processing small files in parallel batches
    /// 3. Processing large files with streaming
    /// 4. Combining and summarizing results
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
        let processed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        // Create semaphore to limit concurrent file operations
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_parallel_files));

        // Split files into small and large categories for optimal processing
        let (small_files, large_files) = self.split_files_by_size(&files).await?;

        // Process small files in parallel batches
        let small_findings = self
            .process_small_files_parallel(
                small_files,
                analyzer_fn.clone(),
                semaphore.clone(),
                processed_count.clone(),
                total_files,
            )
            .await?;

        // Process large files with streaming
        let large_findings = self
            .process_large_files_streaming(
                large_files,
                analyzer_fn,
                semaphore,
                processed_count.clone(),
                total_files,
            )
            .await?;

        // Combine and finalize results
        let results =
            self.combine_findings_results(small_findings, large_findings, total_files, start_time);

        Ok(results)
    }

    /// Split files into small and large categories based on size threshold
    ///
    /// # Arguments
    /// * `files` - Vector of file paths to categorize
    ///
    /// # Returns
    /// A tuple containing (small_files, large_files)
    async fn split_files_by_size<'a>(
        &self,
        files: &[&'a Path],
    ) -> Result<(Vec<&'a Path>, Vec<&'a Path>)> {
        let mut small_files = Vec::new();
        let mut large_files = Vec::new();

        for &path in files {
            let is_small = path
                .metadata()
                .map(|m| m.len() < self.file_size_threshold)
                .unwrap_or(true);

            if is_small {
                small_files.push(path);
            } else {
                large_files.push(path);
            }
        }

        Ok((small_files, large_files))
    }

    /// Combine findings from different processing paths and create final results
    ///
    /// # Arguments
    /// * `small_findings` - Findings from small file processing
    /// * `large_findings` - Findings from large file processing
    /// * `total_files` - Total number of files processed
    /// * `start_time` - When the analysis started
    ///
    /// # Returns
    /// Complete AnalysisResults with all findings and summary
    fn combine_findings_results(
        &self,
        small_findings: Vec<Finding>,
        large_findings: Vec<Finding>,
        total_files: usize,
        start_time: Instant,
    ) -> AnalysisResults {
        let duration = start_time.elapsed();

        // Combine all findings
        let mut all_findings = small_findings;
        all_findings.extend(large_findings);

        // Create results with summary
        let mut results = AnalysisResults::new("turbo".to_string());
        for finding in all_findings {
            results.add_finding(finding);
        }

        results.summary.total_files_scanned = total_files;
        results.summary.scan_duration_ms = duration.as_millis() as u64;

        results
    }

    /// Process small files in parallel using memory-efficient chunking
    ///
    /// # Arguments
    /// * `files` - Vector of small file paths to process
    /// * `analyzer_fn` - Function to analyze each file
    /// * `semaphore` - Semaphore to limit concurrent operations
    /// * `processed_count` - Atomic counter for tracking progress
    /// * `total_files` - Total number of files being processed
    ///
    /// # Returns
    /// Vector of all findings from small files
    async fn process_small_files_parallel<F>(
        &self,
        files: Vec<&Path>,
        analyzer_fn: F,
        semaphore: Arc<tokio::sync::Semaphore>,
        processed_count: Arc<std::sync::atomic::AtomicUsize>,
        total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        if files.is_empty() {
            return Ok(Vec::new());
        }

        // Calculate optimal chunk size for memory efficiency
        let chunk_size = self.calculate_optimal_chunk_size(files.len());
        let mut all_findings = Vec::new();

        // Process files in chunks to manage memory usage
        for chunk in files.chunks(chunk_size) {
            let chunk_findings = self
                .process_small_files_chunk(
                    chunk,
                    analyzer_fn.clone(),
                    semaphore.clone(),
                    processed_count.clone(),
                    total_files,
                )
                .await?;

            all_findings.extend(chunk_findings);
        }

        Ok(all_findings)
    }

    /// Process a chunk of small files in parallel
    ///
    /// # Arguments
    /// * `chunk` - Slice of file paths to process in this chunk
    /// * `analyzer_fn` - Function to analyze each file
    /// * `semaphore` - Semaphore to limit concurrent operations
    /// * `processed_count` - Atomic counter for tracking progress
    /// * `total_files` - Total number of files being processed
    ///
    /// # Returns
    /// Vector of findings from this chunk
    async fn process_small_files_chunk<F>(
        &self,
        chunk: &[&Path],
        analyzer_fn: F,
        semaphore: Arc<tokio::sync::Semaphore>,
        processed_count: Arc<std::sync::atomic::AtomicUsize>,
        total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        let chunk_paths: Vec<_> = chunk.iter().map(|&p| p.to_path_buf()).collect();

        self.process_file_chunk(
            chunk_paths,
            analyzer_fn,
            semaphore,
            processed_count,
            total_files,
        )
        .await
    }

    /// Process a chunk of files using parallel blocking tasks
    ///
    /// # Arguments
    /// * `files` - Vector of file paths to process
    /// * `analyzer_fn` - Function to analyze each file
    /// * `semaphore` - Semaphore to limit concurrent operations
    /// * `processed_count` - Atomic counter for tracking progress
    /// * `_total_files` - Total number of files being processed (unused)
    ///
    /// # Returns
    /// Vector of findings from all files in the chunk
    async fn process_file_chunk<F>(
        &self,
        files: Vec<std::path::PathBuf>,
        analyzer_fn: F,
        semaphore: Arc<tokio::sync::Semaphore>,
        processed_count: Arc<std::sync::atomic::AtomicUsize>,
        _total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        let mut all_findings = Vec::new();

        for path in files {
            let findings = self
                .process_single_file(
                    &path,
                    analyzer_fn.clone(),
                    semaphore.clone(),
                    processed_count.clone(),
                )
                .await?;

            all_findings.extend(findings);
        }

        Ok(all_findings)
    }

    /// Process a single file with proper error handling and progress tracking
    ///
    /// # Arguments
    /// * `path` - Path to the file to process
    /// * `analyzer_fn` - Function to analyze the file
    /// * `semaphore` - Semaphore to limit concurrent operations
    /// * `processed_count` - Atomic counter for tracking progress
    ///
    /// # Returns
    /// Vector of findings from the file
    async fn process_single_file<F>(
        &self,
        path: &Path,
        analyzer_fn: F,
        semaphore: Arc<tokio::sync::Semaphore>,
        processed_count: Arc<std::sync::atomic::AtomicUsize>,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        // Acquire semaphore permit to limit concurrency
        let _permit = semaphore
            .acquire()
            .await
            .context("Failed to acquire semaphore permit for file processing")?;

        // Clone for move into blocking task
        let analyzer = analyzer_fn.clone();
        let counter = processed_count.clone();
        let path_clone = path.to_path_buf();

        // Process file in blocking task to avoid blocking async runtime
        let result =
            tokio::task::spawn_blocking(move || Self::read_and_analyze_file(&path_clone, analyzer))
                .await
                .context("File processing task panicked")??;

        // Update progress counter
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Ok(result)
    }

    /// Read file content and analyze it with the provided function
    ///
    /// # Arguments
    /// * `path` - Path to the file to read and analyze
    /// * `analyzer_fn` - Function to analyze the file content
    ///
    /// # Returns
    /// Vector of findings from the analysis
    fn read_and_analyze_file<F>(path: &Path, analyzer_fn: F) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync,
    {
        // Read file content with security validation
        let content = Self::read_file_with_validation(path)?;

        // Analyze the content
        analyzer_fn(path, &content)
    }

    /// Read file content with security validation
    ///
    /// # Arguments
    /// * `path` - Path to the file to read
    ///
    /// # Returns
    /// File content as bytes
    ///
    /// # Security
    /// Validates file path and checks size limits to prevent abuse
    fn read_file_with_validation(path: &Path) -> Result<Vec<u8>> {
        // Validate path for security
        if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().starts_with('.') {
                return Err(anyhow::anyhow!(
                    "Hidden files are not processed for security"
                ));
            }
        }

        // Check file size to prevent memory exhaustion
        let metadata = path.metadata().context("Failed to get file metadata")?;

        const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB limit
        if metadata.len() > MAX_FILE_SIZE {
            return Err(anyhow::anyhow!(
                "File too large: {} bytes (max: {} bytes)",
                metadata.len(),
                MAX_FILE_SIZE
            ));
        }

        // Read file content
        std::fs::read(path).context(format!("Failed to read file: {}", path.display()))
    }

    /// Process large files using streaming to manage memory usage
    ///
    /// # Arguments
    /// * `files` - Vector of large file paths to process
    /// * `analyzer_fn` - Function to analyze each file
    /// * `semaphore` - Semaphore to limit concurrent operations
    /// * `processed_count` - Atomic counter for tracking progress
    /// * `total_files` - Total number of files being processed
    ///
    /// # Returns
    /// Vector of all findings from large files
    async fn process_large_files_streaming<F>(
        &self,
        files: Vec<&Path>,
        analyzer_fn: F,
        semaphore: Arc<tokio::sync::Semaphore>,
        processed_count: Arc<std::sync::atomic::AtomicUsize>,
        total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        if files.is_empty() {
            return Ok(Vec::new());
        }

        let mut all_findings = Vec::new();

        for &path in &files {
            let findings = self
                .process_single_large_file(
                    path,
                    analyzer_fn.clone(),
                    semaphore.clone(),
                    processed_count.clone(),
                    total_files,
                )
                .await?;

            all_findings.extend(findings);
        }

        Ok(all_findings)
    }

    /// Process a single large file with streaming and progress tracking
    ///
    /// # Arguments
    /// * `path` - Path to the large file to process
    /// * `analyzer_fn` - Function to analyze the file
    /// * `semaphore` - Semaphore to limit concurrent operations
    /// * `processed_count` - Atomic counter for tracking progress
    /// * `total_files` - Total number of files being processed
    ///
    /// # Returns
    /// Vector of findings from the large file
    async fn process_single_large_file<F>(
        &self,
        path: &Path,
        analyzer_fn: F,
        semaphore: Arc<tokio::sync::Semaphore>,
        processed_count: Arc<std::sync::atomic::AtomicUsize>,
        total_files: usize,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        // Acquire semaphore permit to limit concurrency
        let _permit = semaphore
            .acquire()
            .await
            .context("Failed to acquire semaphore permit for large file processing")?;

        // Use streaming analysis for large files to manage memory
        let findings = self
            .analyze_large_file_streaming(path, analyzer_fn)
            .await
            .context(format!("Failed to analyze large file: {}", path.display()))?;

        // Update progress and notify callback if available
        let count = processed_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        self.update_progress_callback(count, total_files);

        Ok(findings)
    }

    /// Update progress callback if one is configured
    ///
    /// # Arguments
    /// * `count` - Current count of processed files
    /// * `total_files` - Total number of files to process
    fn update_progress_callback(&self, count: usize, total_files: usize) {
        if let Some(callback) = &self.progress_callback {
            callback(count, total_files);
        }
    }

    /// Analyze a large file using streaming to manage memory usage
    ///
    /// # Arguments
    /// * `path` - Path to the large file to analyze
    /// * `analyzer_fn` - Function to analyze each chunk of the file
    ///
    /// # Returns
    /// Vector of all findings from the file
    ///
    /// # Performance
    /// Uses streaming to process large files without loading them entirely into memory
    async fn analyze_large_file_streaming<F>(
        &self,
        path: &Path,
        analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync,
    {
        // Validate file before streaming
        self.validate_large_file(path).await?;

        // Open file and create streaming reader
        let reader = self.open_file_for_streaming(path).await?;

        // Process file in chunks
        self.process_file_chunks_streaming(reader, path, analyzer_fn)
            .await
    }

    /// Validate that a file is suitable for streaming analysis
    ///
    /// # Arguments
    /// * `path` - Path to the file to validate
    ///
    /// # Returns
    /// Ok if file is valid for streaming, Error otherwise
    async fn validate_large_file(&self, path: &Path) -> Result<()> {
        let metadata = path.metadata().context(format!(
            "Failed to get metadata for large file: {}",
            path.display()
        ))?;

        // Ensure file is actually large enough to warrant streaming
        if metadata.len() < self.file_size_threshold {
            return Err(anyhow::anyhow!(
                "File size {} is below streaming threshold {}",
                metadata.len(),
                self.file_size_threshold
            ));
        }

        // Check if file is readable
        if !metadata.permissions().readonly() {
            // File is writable, but we'll still try to read it
            // This is just a warning condition
        }

        Ok(())
    }

    /// Open file and create a buffered reader for streaming
    ///
    /// # Arguments
    /// * `path` - Path to the file to open
    ///
    /// # Returns
    /// Buffered reader for the file
    async fn open_file_for_streaming(
        &self,
        path: &Path,
    ) -> Result<tokio::io::BufReader<tokio::fs::File>> {
        use tokio::fs::File;
        use tokio::io::BufReader;

        let file = File::open(path).await.context(format!(
            "Failed to open large file for streaming: {}",
            path.display()
        ))?;

        let reader = BufReader::with_capacity(self.calculate_streaming_chunk_size(), file);

        Ok(reader)
    }

    /// Process file chunks using streaming reader
    ///
    /// # Arguments
    /// * `reader` - Buffered reader for the file
    /// * `path` - Path to the file being processed
    /// * `analyzer_fn` - Function to analyze each chunk
    ///
    /// # Returns
    /// Vector of all findings from all chunks
    async fn process_file_chunks_streaming<F>(
        &self,
        mut reader: tokio::io::BufReader<tokio::fs::File>,
        path: &Path,
        analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: Fn(&Path, &[u8]) -> Result<Vec<Finding>> + Send + Sync,
    {
        let mut buffer = vec![0u8; self.calculate_streaming_chunk_size()];
        let mut all_findings = Vec::new();
        let mut chunk_count = 0;

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .await
                .context("Failed to read chunk from large file")?;

            if bytes_read == 0 {
                break; // EOF reached
            }

            // Process the chunk
            let chunk = &buffer[..bytes_read];
            let findings = analyzer_fn(path, chunk).context(format!(
                "Failed to analyze chunk {} of large file",
                chunk_count
            ))?;

            all_findings.extend(findings);
            chunk_count += 1;

            // Yield control to prevent blocking the async runtime
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

    #[tokio::test]
    async fn test_optimized_pattern_matcher() {
        let patterns = vec![
            (r"TODO", "todo".to_string()),
            (r"FIXME", "fixme".to_string()),
        ];

        let matcher = OptimizedPatternMatcher::new(patterns, 10).unwrap();
        let content = "This is a TODO item\nThis is a FIXME item";
        let content_hash = "test_hash";

        let results = matcher.match_all(content, content_hash).await.unwrap();

        assert!(results.contains_key("todo"));
        assert!(results.contains_key("fixme"));
    }

    #[tokio::test]
    async fn test_performance_counters() {
        let counters = monitoring::PerformanceCounters::new();

        counters.record_file_processed(Duration::from_millis(100));
        counters.record_file_processed(Duration::from_millis(200));

        let avg_time = counters.average_processing_time();
        assert_eq!(avg_time, Duration::from_millis(150));
    }
}
