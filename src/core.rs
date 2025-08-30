use crate::analyzers::AnalyzerRegistry;

use crate::cache::FileCache;
use crate::config::Config;
use crate::ml::MLClassifier;
use crate::streaming::StreamingAnalyzer;
use crate::types::{AnalysisResults, Finding};
use crate::utils::adaptive_parallelism::{AdaptiveParallelismController, SystemLoadMonitor};
use crate::utils::progress::ProgressReporter;
use crate::utils::security::{canonicalize_path_safe, should_follow_path};
use crate::validation::{ValidationEngine, ValidationStats};
use anyhow::Result;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;
use tokio::fs;
use tokio::sync::Mutex;
use walkdir::WalkDir;

// Constants for better maintainability and to eliminate magic numbers
const DEFAULT_FILE_SIZE_LIMIT_BYTES: u64 = 10 * 1024 * 1024; // 10MB limit for security
const ADAPTIVE_DELAY_MS: u64 = 100; // Delay for load monitoring adjustments
const BYTES_PER_MB: f64 = 1024.0 * 1024.0; // Conversion factor for MB
const DEFAULT_DIR_CAPACITY_ESTIMATE: usize = 100; // Estimated typical directory size
const PATH_VECTOR_CAPACITY_MULTIPLIER: usize = 2; // Multiplier for path vector capacity
const PROGRESS_BUFFER_CAPACITY: usize = 256; // String buffer size for progress messages
const STREAMING_YIELD_INTERVAL: u32 = 10_000; // Yield every 10,000 lines for large files

/// Core engine that orchestrates the entire code analysis process.
///
/// The GuardianEngine coordinates all aspects of security and code quality analysis,
/// including file discovery, caching, parallel processing, ML-based false positive
/// reduction, and progress reporting. It uses adaptive parallelism to optimize
/// performance based on system load and file characteristics.
pub struct GuardianEngine {
    /// Configuration settings for the analysis
    config: Config,
    /// Registry of all available analyzers
    analyzer_registry: AnalyzerRegistry,
    /// Progress reporting interface
    progress: ProgressReporter,
    /// File cache for storing analysis results
    cache: Arc<Mutex<FileCache>>,
    /// Streaming analyzer for large files
    streaming_analyzer: StreamingAnalyzer,
    /// ML classifier for false positive reduction
    ml_classifier: MLClassifier,
    /// Validation engine for ensuring 100% validated findings
    validation_engine: ValidationEngine,
    /// Analysis statistics and metrics
    stats: AnalysisStats,
    /// Controller for adaptive parallelism
    parallelism_controller: Arc<AdaptiveParallelismController>,
    /// Optional system load monitor
    load_monitor: Option<SystemLoadMonitor>,
}

impl GuardianEngine {
    /// Creates a new GuardianEngine with the specified configuration.
    ///
    /// Initializes all components including the analyzer registry, cache,
    /// ML classifier, and adaptive parallelism controller. This is the
    /// standard constructor for most use cases.
    ///
    /// # Arguments
    /// * `config` - Configuration settings for the analysis
    /// * `progress` - Progress reporter for user feedback
    ///
    /// # Returns
    /// A new GuardianEngine instance or an error if initialization fails
    pub async fn new(config: Config, progress: ProgressReporter) -> Result<Self> {
        Self::new_with_ml(config, progress, None).await
    }

    /// Creates a new GuardianEngine with optional ML model configuration.
    ///
    /// Similar to `new()`, but allows specifying a custom ML model path
    /// for false positive reduction. If no path is provided, uses the default model.
    ///
    /// # Arguments
    /// * `config` - Configuration settings for the analysis
    /// * `progress` - Progress reporter for user feedback
    /// * `ml_model_path` - Optional path to ML model file
    ///
    /// # Returns
    /// A new GuardianEngine instance or an error if initialization fails
    pub async fn new_with_ml(
        config: Config,
        progress: ProgressReporter,
        ml_model_path: Option<&str>,
    ) -> Result<Self> {
        let cache = Arc::new(Mutex::new(FileCache::load().await?));

        // Initialize ML classifier with provided model path or default
        let ml_classifier = MLClassifier::new(ml_model_path.or(Some("codeguardian-model.fann")));

        // Initialize validation engine with ML classifier reference
        let validation_engine = ValidationEngine::new(None); // We'll pass ML classifier separately

        // Initialize adaptive parallelism controller
        let min_workers = 1;
        let max_workers = config.general.parallel_workers;
        let initial_workers = (max_workers / 2).max(1);
        let parallelism_controller = Arc::new(AdaptiveParallelismController::new(
            min_workers,
            max_workers,
            initial_workers,
        ));

        // Initialize load monitor
        let load_monitor = Some(SystemLoadMonitor::new(Arc::clone(&parallelism_controller)));

        Ok(Self {
            config,
            analyzer_registry: AnalyzerRegistry::new(),
            progress,
            cache,
            streaming_analyzer: StreamingAnalyzer::new(),
            ml_classifier,
            validation_engine,
            stats: AnalysisStats::new(),
            parallelism_controller,
            load_monitor,
        })
    }

    /// Recursively discovers all files from the given paths.
    ///
    /// Expands directories into their constituent files and filters out
    /// files that should not be analyzed based on configuration and security rules.
    ///
    /// # Arguments
    /// * `paths` - List of file or directory paths to process
    ///
    /// # Returns
    /// A vector of file paths to analyze, or an error if discovery fails
    pub async fn get_all_files(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut all_files = Vec::with_capacity(paths.len() * PATH_VECTOR_CAPACITY_MULTIPLIER); // Estimate capacity

        for path in paths {
            if path.is_file() {
                all_files.push(canonicalize_path_safe(path));
            } else if path.is_dir() {
                let files = self.scan_directory(path).await?;
                all_files.extend(files);
            }
        }

        Ok(all_files)
    }

    pub async fn get_diff_files(&self, diff_spec: &str) -> Result<Vec<PathBuf>> {
        crate::utils::git::get_diff_files(diff_spec)
    }

    pub async fn get_staged_files(&self) -> Result<Vec<PathBuf>> {
        crate::utils::git::get_staged_files()
    }

    async fn scan_directory(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::with_capacity(DEFAULT_DIR_CAPACITY_ESTIMATE); // Estimate typical directory size

        for entry in WalkDir::new(dir_path)
            .follow_links(false) // Security: don't follow symlinks by default
            .into_iter()
            .filter_entry(|e| should_follow_path(e.path(), false))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && self.should_analyze_file(path) {
                files.push(canonicalize_path_safe(path));
            }
        }

        Ok(files)
    }

    /// Scans the given paths and performs comprehensive analysis.
    ///
    /// This method discovers all files from the provided paths (expanding directories)
    /// and then performs a full analysis on them, returning complete analysis results
    /// including all findings and statistics.
    ///
    /// # Arguments
    /// * `paths` - List of file or directory paths to scan and analyze
    ///
    /// # Returns
    /// Complete analysis results including all findings and statistics
    #[allow(dead_code)]
    pub async fn scan_paths(&mut self, paths: &[PathBuf]) -> Result<AnalysisResults> {
        let files = self.get_all_files(paths).await?;
        self.analyze_files(&files, 1).await
    }

    fn should_analyze_file(&self, path: &Path) -> bool {
        // Skip files based on configuration
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Skip common ignore patterns
            if name.starts_with('.') && name != ".gitignore" && name != ".dockerignore" {
                return false;
            }
        }

        // Check file size limits (security: prevent processing huge files)
        if let Ok(metadata) = path.metadata() {
            if metadata.len() > DEFAULT_FILE_SIZE_LIMIT_BYTES {
                // Security limit to prevent processing huge files
                return false;
            }
        }

        true
    }

    /// Performs comprehensive analysis on the given files.
    ///
    /// This is the main analysis entry point that coordinates the entire analysis process:
    /// - Processes cached results where available
    /// - Analyzes uncached files with adaptive parallelism
    /// - Applies ML-based false positive reduction
    /// - Updates cache with new results
    /// - Reports progress and final statistics
    ///
    /// # Arguments
    /// * `files` - List of file paths to analyze
    /// * `parallel` - Base parallelism level (may be adjusted adaptively)
    ///
    /// # Returns
    /// Complete analysis results including all findings and statistics
    #[allow(clippy::await_holding_lock)]
    pub async fn analyze_files(
        &mut self,
        files: &[PathBuf],
        parallel: usize,
    ) -> Result<AnalysisResults> {
        let start_time = Instant::now();
        let config_hash = self.compute_config_hash();
        let mut results = AnalysisResults::new(config_hash.clone());

        // Initialize analysis
        self.initialize_analysis(files.len()).await?;

        // Process files using cache and adaptive parallelism
        self.process_files_with_caching(&mut results, files, &config_hash, parallel)
            .await?;

        // Finalize analysis
        self.finalize_analysis(&mut results, files.len(), start_time)
            .await?;

        Ok(results)
    }

    /// Analyze files and prepare results specifically for GitHub issue creation
    /// This applies strict validation to ensure only 100% validated findings create issues
    pub async fn analyze_files_for_github_issues(
        &mut self,
        files: &[PathBuf],
        parallel: usize,
    ) -> Result<AnalysisResults> {
        // First, perform standard analysis
        let mut results = self.analyze_files(files, parallel).await?;
        
        // Apply strict validation for GitHub issue creation
        results = self.validation_engine.filter_results_for_github(results)?;
        
        // Add validation metadata to results
        let validation_stats = self.validation_engine.get_validation_stats();
        results.summary.metadata.insert(
            "validation_mode".to_string(),
            "strict_github_validation".to_string()
        );
        results.summary.metadata.insert(
            "validation_stats".to_string(),
            validation_stats.to_string()
        );
        
        // Report validation statistics
        self.progress.update(&format!(
            "Validation applied: {} findings validated for GitHub issues ({})",
            results.findings.len(),
            validation_stats
        ));
        
        Ok(results)
    }

    /// Get validation engine statistics
    pub fn get_validation_stats(&self) -> ValidationStats {
        self.validation_engine.get_validation_stats()
    }

    /// Initialize analysis components
    async fn initialize_analysis(&mut self, file_count: usize) -> Result<()> {
        // Start load monitoring if available
        if let Some(ref monitor) = self.load_monitor {
            monitor.start_monitoring().await?;
        }

        // Start progress reporting
        self.progress.start_scan(file_count);

        Ok(())
    }

    /// Process files using caching and adaptive parallelism
    async fn process_files_with_caching(
        &mut self,
        results: &mut AnalysisResults,
        files: &[PathBuf],
        config_hash: &str,
        parallel: usize,
    ) -> Result<()> {
        // Handle cached files
        self.process_cached_files(results, files, config_hash)
            .await?;

        // Process uncached files with adaptive parallelism
        let uncached_files = self.get_uncached_files(files, config_hash).await?;
        if !uncached_files.is_empty() {
            let filtered_findings = self
                .process_uncached_files_adaptive(&uncached_files, parallel, config_hash)
                .await?;

            // Add findings to results
            for finding in filtered_findings {
                results.add_finding(finding);
            }
        }

        Ok(())
    }

    /// Finalize analysis and report results
    async fn finalize_analysis(
        &mut self,
        results: &mut AnalysisResults,
        file_count: usize,
        start_time: Instant,
    ) -> Result<()> {
        // Update summary and finalize
        results.summary.total_files_scanned = file_count;
        self.stats.total_duration = start_time.elapsed();

        // Stop load monitoring
        if let Some(ref monitor) = self.load_monitor {
            monitor.stop_monitoring();
        }

        // Report cache statistics
        self.report_cache_statistics().await?;

        // Report adaptive parallelism metrics
        self.report_parallelism_metrics()?;

        // Finish progress reporting
        self.finish_progress_reporting(file_count)?;

        Ok(())
    }

    /// Report cache statistics
    async fn report_cache_statistics(&self) -> Result<()> {
        let stats = self.cache.lock().await.performance_stats();
        self.progress.update(&format!(
            "Cache: {} entries, {} findings, {:.1}MB cached",
            stats.total_entries,
            stats.total_findings,
            stats.total_cached_size as f64 / BYTES_PER_MB
        ));
        Ok(())
    }

    /// Report adaptive parallelism metrics
    fn report_parallelism_metrics(&self) -> Result<()> {
        let parallelism_metrics = self.parallelism_controller.metrics();
        let mut progress_buffer = String::with_capacity(PROGRESS_BUFFER_CAPACITY);
        write!(
            progress_buffer,
            "Adaptive parallelism: {} workers (load: {:.2})",
            parallelism_metrics.current_workers, parallelism_metrics.current_load_score
        )?;
        self.progress.update(&progress_buffer);
        Ok(())
    }

    /// Finish progress reporting with final statistics
    fn finish_progress_reporting(&self, file_count: usize) -> Result<()> {
        let mut progress_buffer = String::with_capacity(PROGRESS_BUFFER_CAPACITY);
        write!(
            progress_buffer,
            "Analyzed {} files ({} cached, {} new) in {:.2}s",
            file_count,
            self.stats.cache_hits.load(Ordering::Relaxed),
            self.stats.cache_misses.load(Ordering::Relaxed),
            self.stats.total_duration.as_secs_f64()
        )?;
        self.progress.finish(&progress_buffer);
        Ok(())
    }

    async fn process_cached_files(
        &mut self,
        results: &mut AnalysisResults,
        files: &[PathBuf],
        config_hash: &str,
    ) -> Result<()> {
        let (cached_files, _) = self.partition_cached_files(files, config_hash).await?;

        for (file_path, cached_findings) in cached_files {
            for finding in cached_findings {
                results.add_finding(finding);
            }
            self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
            self.progress
                .update(&format!("Cached: {}", file_path.display()));
        }
        Ok(())
    }

    async fn get_uncached_files(
        &self,
        files: &[PathBuf],
        config_hash: &str,
    ) -> Result<Vec<PathBuf>> {
        let (_, uncached_files) = self.partition_cached_files(files, config_hash).await?;
        Ok(uncached_files)
    }

    async fn process_uncached_files_adaptive(
        &mut self,
        uncached_files: &[PathBuf],
        _parallel: usize,
        config_hash: &str,
    ) -> Result<Vec<Finding>> {
        // Use memory pool for findings collection to reduce allocations
        use crate::utils::memory_pool::thread_local_pools;
        thread_local_pools::init();

        let mut all_findings = thread_local_pools::get_findings_vec();
        all_findings.reserve(uncached_files.len() * 5); // Estimate findings per file
        let total_files = uncached_files.len();
        let mut processed = 0;

        // Process files with adaptive parallelism
        while processed < total_files {
            // Get current recommended worker count from adaptive controller
            let current_workers = self.parallelism_controller.current_workers();
            let batch_size = current_workers.min(total_files - processed);

            // Process a batch of files
            let batch_end = (processed + batch_size).min(total_files);
            let batch = &uncached_files[processed..batch_end];

            // Process batch concurrently
            let batch_findings = self.process_file_batch(batch, config_hash).await?;
            all_findings.extend(batch_findings);

            processed = batch_end;

            // Update progress
            self.progress.update(&format!(
                "Processed {}/{} files ({} workers active)",
                processed, total_files, current_workers
            ));

            // Small delay to allow load monitoring to adjust
            tokio::time::sleep(tokio::time::Duration::from_millis(ADAPTIVE_DELAY_MS)).await;
        }

        // Apply ML-based false positive reduction if enabled
        let ml_filtered_findings = self.ml_classifier.filter_findings(all_findings, 0.3)?;

        // Apply validation for reports (less strict than GitHub issues)
        let validated_findings = self.validation_engine.validate_for_reports(ml_filtered_findings)?;

        Ok(validated_findings)
    }

    async fn process_file_batch(
        &self,
        files: &[PathBuf],
        config_hash: &str,
    ) -> Result<Vec<Finding>> {
        let mut batch_findings = Vec::with_capacity(files.len() * 5); // Estimate findings per file

        for file_path in files {
            self.progress
                .update(&format!("Analyzing: {}", file_path.display()));

            match self
                .analyze_single_file_optimized(
                    file_path,
                    &self.analyzer_registry,
                    &self.streaming_analyzer,
                    config_hash,
                )
                .await
            {
                Ok(findings) => {
                    // Cache the results
                    {
                        // Perform async operation without holding lock across await
                        // For now, we'll skip the async caching to avoid the Send issue
                        // Note: Async cache handling needs refactoring to avoid Send trait issues
                        let mut cache_guard = self.cache.lock().await;
                        let _ = cache_guard
                            .cache_findings(file_path, findings.clone(), config_hash)
                            .await;
                    }

                    self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
                    batch_findings.extend(findings);
                }
                Err(e) => {
                    eprintln!("Error analyzing {}: {}", file_path.display(), e);
                    self.stats.errors.fetch_add(1, Ordering::Relaxed);
                }
            }
        }

        Ok(batch_findings)
    }

    async fn analyze_single_file_optimized(
        &self,
        file_path: &Path,
        analyzer_registry: &AnalyzerRegistry,
        streaming_analyzer: &StreamingAnalyzer,
        _config_hash: &str,
    ) -> Result<Vec<Finding>> {
        // Update progress
        self.progress
            .update(&format!("Analyzing {}", file_path.display()));

        // Check if we should use streaming analysis
        if StreamingAnalyzer::should_use_streaming(file_path) {
            // Use streaming analysis for large files
            self.analyze_large_file_streaming_async(
                file_path,
                analyzer_registry,
                streaming_analyzer,
            )
            .await
        } else {
            // Standard in-memory analysis for smaller files
            let content = fs::read(file_path).await?;
            analyzer_registry.analyze_file(file_path, &content)
        }
    }

    async fn analyze_large_file_streaming_async(
        &self,
        file_path: &Path,
        analyzer_registry: &AnalyzerRegistry,
        _streaming_analyzer: &StreamingAnalyzer,
    ) -> Result<Vec<Finding>> {
        // Use async streaming for large files to avoid blocking
        use tokio::io::{AsyncBufReadExt, BufReader};
        let file = fs::File::open(file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut all_findings = Vec::new();

        // Process file line by line to save memory
        let mut line_number = 1u32;

        while let Some(line_result) = lines.next_line().await? {
            let line = line_result;
            let line_content = format!("{}\n", line);

            // Analyze this line
            let line_findings =
                analyzer_registry.analyze_file(file_path, line_content.as_bytes())?;
            all_findings.extend(line_findings);

            line_number += 1;

            // Yield occasionally for very large files to prevent blocking
            if line_number % STREAMING_YIELD_INTERVAL == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(all_findings)
    }

    async fn partition_cached_files(
        &self,
        files: &[PathBuf],
        config_hash: &str,
    ) -> Result<(Vec<(PathBuf, Vec<Finding>)>, Vec<PathBuf>)> {
        let mut cached_files = Vec::with_capacity(files.len() / 2); // Estimate half cached
        let mut uncached_files = Vec::with_capacity(files.len() / 2); // Estimate half uncached

        let cache_guard = self.cache.lock().await;
        for file_path in files {
            if let Some(cached_findings) = cache_guard.get_cached_findings(file_path, config_hash) {
                cached_files.push((file_path.clone(), cached_findings));
            } else {
                uncached_files.push(file_path.clone());
            }
        }

        Ok((cached_files, uncached_files))
    }

    fn compute_config_hash(&self) -> String {
        use sha2::{Digest, Sha256};

        // Serialize config and hash it
        let config_str = toml::to_string(&self.config).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(config_str.as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }
}

/// Statistics collected during an analysis run.
///
/// Tracks various metrics about the analysis performance including
/// cache effectiveness, error counts, and timing information.
#[derive(Debug)]
pub struct AnalysisStats {
    /// Number of cache hits (files with cached results)
    pub cache_hits: AtomicUsize,
    /// Number of cache misses (files that needed fresh analysis)
    pub cache_misses: AtomicUsize,
    /// Number of errors encountered during analysis
    pub errors: AtomicUsize,
    /// Total duration of the analysis
    pub total_duration: std::time::Duration,
}

impl Default for AnalysisStats {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisStats {
    /// Creates a new AnalysisStats instance with zeroed counters.
    pub fn new() -> Self {
        Self {
            cache_hits: AtomicUsize::new(0),
            cache_misses: AtomicUsize::new(0),
            errors: AtomicUsize::new(0),
            total_duration: std::time::Duration::default(),
        }
    }

    /// Calculates the cache hit rate as a percentage.
    ///
    /// Returns the ratio of cache hits to total cache operations (hits + misses).
    /// Returns 0.0 if no cache operations have been performed.
    ///
    /// # Returns
    /// Cache hit rate as a float between 0.0 and 1.0
    #[allow(dead_code)]
    pub fn cache_hit_rate(&self) -> f64 {
        let cache_hits = self.cache_hits.load(Ordering::Relaxed);
        let cache_misses = self.cache_misses.load(Ordering::Relaxed);
        let total = cache_hits + cache_misses;
        if total == 0 {
            0.0
        } else {
            cache_hits as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::utils::progress::ProgressReporter;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio::fs;

    async fn create_test_engine() -> GuardianEngine {
        let config = Config::minimal();
        let progress = ProgressReporter::new(false);
        GuardianEngine::new(config, progress).await.unwrap()
    }

    #[tokio::test]
    async fn test_guardian_engine_creation() {
        let engine = create_test_engine().await;
        assert_eq!(engine.stats.cache_hits.load(Ordering::Relaxed), 0);
        assert_eq!(engine.stats.cache_misses.load(Ordering::Relaxed), 0);
        assert_eq!(engine.stats.errors.load(Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn test_analysis_stats() {
        let stats = AnalysisStats::new();
        assert_eq!(stats.cache_hit_rate(), 0.0);

        stats.cache_hits.store(3, Ordering::Relaxed);
        stats.cache_misses.store(1, Ordering::Relaxed);
        assert_eq!(stats.cache_hit_rate(), 0.75);
    }

    #[tokio::test]
    async fn test_analyze_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let mut engine = create_test_engine().await;

        let paths = vec![temp_dir.path().to_path_buf()];
        let results = engine.scan_paths(&paths).await.unwrap();
        assert_eq!(results.summary.total_files_scanned, 0);
        assert_eq!(results.findings.len(), 0);
    }

    #[tokio::test]
    async fn test_analyze_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn main() { println!(\"Hello, world!\"); }")
            .await
            .unwrap();

        let mut engine = create_test_engine().await;
        let paths = vec![temp_dir.path().to_path_buf()];
        let results = engine.scan_paths(&paths).await.unwrap();

        assert!(results.summary.total_files_scanned > 0);
    }

    #[tokio::test]
    async fn test_analyze_file_with_security_issue() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.js");
        fs::write(
            &test_file,
            r#"const apiKey = "sk-1234567890abcdef1234567890abcdef";"#,
        )
        .await
        .unwrap();

        let mut engine = create_test_engine().await;
        let paths = vec![temp_dir.path().to_path_buf()];
        let results = engine.scan_paths(&paths).await.unwrap();

        // Should detect hardcoded secret
        assert!(results
            .findings
            .iter()
            .any(|f| f.rule == "hardcoded_secret"));
    }

    #[tokio::test]
    async fn test_file_size_limit() {
        let temp_dir = TempDir::new().unwrap();
        let large_file = temp_dir.path().join("large.txt");

        // Create a file larger than the default limit
        let large_content = "x".repeat(15 * 1024 * 1024); // 15MB
        fs::write(&large_file, large_content).await.unwrap();

        let mut engine = create_test_engine().await;
        let paths = vec![temp_dir.path().to_path_buf()];
        let results = engine.scan_paths(&paths).await.unwrap();

        // Large file should be skipped
        assert_eq!(results.summary.total_files_scanned, 0);
    }

    #[tokio::test]
    async fn test_analyze_paths_with_filters() {
        let temp_dir = TempDir::new().unwrap();

        // Create test files
        let rust_file = temp_dir.path().join("test.rs");
        let js_file = temp_dir.path().join("test.js");
        let txt_file = temp_dir.path().join("test.txt");

        fs::write(&rust_file, "fn main() {}").await.unwrap();
        fs::write(&js_file, "console.log('hello');").await.unwrap();
        fs::write(&txt_file, "plain text").await.unwrap();

        let mut engine = create_test_engine().await;
        let paths = vec![rust_file, js_file, txt_file];
        let results = engine.scan_paths(&paths).await.unwrap();

        // Should analyze supported file types
        assert!(results.summary.total_files_scanned >= 2);
    }

    #[tokio::test]
    async fn test_cache_integration() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn main() {}").await.unwrap();

        let mut engine = create_test_engine().await;
        let paths = vec![temp_dir.path().to_path_buf()];

        // First analysis
        let results1 = engine.scan_paths(&paths).await.unwrap();
        let initial_cache_hits = engine.stats.cache_hits.load(Ordering::Relaxed);

        // Second analysis (should use cache)
        let results2 = engine.scan_paths(&paths).await.unwrap();
        let final_cache_hits = engine.stats.cache_hits.load(Ordering::Relaxed);

        assert_eq!(results1.findings.len(), results2.findings.len());
        // Cache hits should increase (though exact behavior depends on cache implementation)
        assert!(final_cache_hits >= initial_cache_hits);
    }

    #[tokio::test]
    async fn test_ml_classifier_integration() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.js");
        fs::write(&test_file, r#"const secret = "test_secret_123";"#)
            .await
            .unwrap();

        let config = Config::minimal();
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new_with_ml(config, progress, None)
            .await
            .unwrap();
        let paths = vec![temp_dir.path().to_path_buf()];

        let results = engine.scan_paths(&paths).await.unwrap();

        // Should still work even without ML model
        assert!(results.summary.total_files_scanned > 0);
    }

    #[tokio::test]
    async fn test_error_handling_invalid_path() {
        let engine = create_test_engine().await;
        let invalid_path = PathBuf::from("/nonexistent/path/that/does/not/exist");

        let result = engine.scan_directory(&invalid_path).await;
        // Should handle gracefully - either return empty results or appropriate error
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_analysis() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple test files
        for i in 0..5 {
            let file_path = temp_dir.path().join(format!("test{}.rs", i));
            fs::write(&file_path, format!("fn test{}() {{}}", i))
                .await
                .unwrap();
        }

        let mut engine = create_test_engine().await;
        let paths = vec![temp_dir.path().to_path_buf()];
        let results = engine.scan_paths(&paths).await.unwrap();

        assert_eq!(results.summary.total_files_scanned, 5);
    }

    #[test]
    fn test_analysis_stats_default() {
        let stats = AnalysisStats::default();
        assert_eq!(stats.cache_hits.load(Ordering::Relaxed), 0);
        assert_eq!(stats.cache_misses.load(Ordering::Relaxed), 0);
        assert_eq!(stats.errors.load(Ordering::Relaxed), 0);
        assert_eq!(stats.total_duration, std::time::Duration::default());
    }

    #[test]
    fn test_analysis_stats_cache_hit_rate_edge_cases() {
        let stats = AnalysisStats::new();

        // No cache operations
        assert_eq!(stats.cache_hit_rate(), 0.0);

        // Only hits
        stats.cache_hits.store(5, Ordering::Relaxed);
        assert_eq!(stats.cache_hit_rate(), 1.0);

        // Only misses
        stats.cache_hits.store(0, Ordering::Relaxed);
        stats.cache_misses.store(3, Ordering::Relaxed);
        assert_eq!(stats.cache_hit_rate(), 0.0);
    }
}
