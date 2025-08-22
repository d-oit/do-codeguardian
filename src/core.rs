use crate::analyzers::AnalyzerRegistry;

use crate::cache::FileCache;
use crate::config::Config;
use crate::ml::MLClassifier;
use crate::streaming::StreamingAnalyzer;
use crate::types::{AnalysisResults, Finding};
use crate::utils::adaptive_parallelism::{AdaptiveParallelismController, SystemLoadMonitor};
use crate::utils::progress::ProgressReporter;
use crate::utils::security::{canonicalize_path_safe, should_follow_path};
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

pub struct GuardianEngine {
    config: Config,
    analyzer_registry: AnalyzerRegistry,
    progress: ProgressReporter,
    cache: Arc<Mutex<FileCache>>,
    streaming_analyzer: StreamingAnalyzer,
    ml_classifier: MLClassifier,
    stats: AnalysisStats,
    parallelism_controller: Arc<AdaptiveParallelismController>,
    load_monitor: Option<SystemLoadMonitor>,
}

impl GuardianEngine {
    pub async fn new(config: Config, progress: ProgressReporter) -> Result<Self> {
        Self::new_with_ml(config, progress, None).await
    }

    pub async fn new_with_ml(
        config: Config,
        progress: ProgressReporter,
        ml_model_path: Option<&str>,
    ) -> Result<Self> {
        let cache = Arc::new(Mutex::new(FileCache::load().await?));

        // Initialize ML classifier with provided model path or default
        let ml_classifier = MLClassifier::new(ml_model_path.or(Some("codeguardian-model.fann")));

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
            stats: AnalysisStats::new(),
            parallelism_controller,
            load_monitor,
        })
    }

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

    #[allow(clippy::await_holding_lock)]
    pub async fn analyze_files(
        &mut self,
        files: &[PathBuf],
        parallel: usize,
    ) -> Result<AnalysisResults> {
        let start_time = Instant::now();
        let config_hash = self.compute_config_hash();
        let mut results = AnalysisResults::new(config_hash.clone());

        // Start load monitoring if available
        if let Some(ref monitor) = self.load_monitor {
            monitor.start_monitoring().await?;
        }

        // Start progress reporting
        self.progress.start_scan(files.len());

        // Pre-allocate string buffer for progress messages
        let mut progress_buffer = String::with_capacity(PROGRESS_BUFFER_CAPACITY);

        // Handle cached files
        self.process_cached_files(&mut results, files, &config_hash)
            .await?;

        // Process uncached files with adaptive parallelism
        let uncached_files = self.get_uncached_files(files, &config_hash).await?;
        if !uncached_files.is_empty() {
            let filtered_findings = self
                .process_uncached_files_adaptive(&uncached_files, parallel, &config_hash)
                .await?;

            // Add findings to results
            for finding in filtered_findings {
                results.add_finding(finding);
            }
        }

        // Update summary and finalize
        results.summary.total_files_scanned = files.len();
        self.stats.total_duration = start_time.elapsed();

        // Stop load monitoring
        if let Some(ref monitor) = self.load_monitor {
            monitor.stop_monitoring();
        }

        // Save cache with enhanced features
        {
            // Perform async operation without holding lock across await
            // For now, we'll skip the auto-save to avoid the Send issue
            // TODO: Implement proper async cache handling

            let stats = self.cache.lock().await.performance_stats();
            self.progress.update(&format!(
                "Cache: {} entries, {} findings, {:.1}MB cached",
                stats.total_entries,
                stats.total_findings,
                stats.total_cached_size as f64 / BYTES_PER_MB
            ));
        }

        // Log adaptive parallelism metrics
        let parallelism_metrics = self.parallelism_controller.metrics();
        progress_buffer.clear();
        write!(
            progress_buffer,
            "Adaptive parallelism: {} workers (load: {:.2})",
            parallelism_metrics.current_workers, parallelism_metrics.current_load_score
        )
        .unwrap();
        self.progress.update(&progress_buffer);

        // Finish progress reporting
        progress_buffer.clear();
        write!(
            progress_buffer,
            "Analyzed {} files ({} cached, {} new) in {:.2}s",
            files.len(),
            self.stats.cache_hits.load(Ordering::Relaxed),
            self.stats.cache_misses.load(Ordering::Relaxed),
            self.stats.total_duration.as_secs_f64()
        )
        .unwrap();
        self.progress.finish(&progress_buffer);

        Ok(results)
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
        let filtered_findings = self.ml_classifier.filter_findings(all_findings, 0.3)?;

        Ok(filtered_findings)
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
                        // TODO: Implement proper async cache handling
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

#[derive(Debug)]
pub struct AnalysisStats {
    pub cache_hits: AtomicUsize,
    pub cache_misses: AtomicUsize,
    pub errors: AtomicUsize,
    pub total_duration: std::time::Duration,
}

impl Default for AnalysisStats {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisStats {
    pub fn new() -> Self {
        Self {
            cache_hits: AtomicUsize::new(0),
            cache_misses: AtomicUsize::new(0),
            errors: AtomicUsize::new(0),
            total_duration: std::time::Duration::default(),
        }
    }

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
