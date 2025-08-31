use crate::analyzers::AnalyzerRegistry;
use crate::cache::FileCache;
use crate::config::Config;
use crate::streaming::StreamingAnalyzer;
use crate::types::{AnalysisResults, Finding};
use crate::utils::progress::ProgressReporter;
use crate::utils::security::{canonicalize_path_safe, should_follow_path};
use anyhow::Result;
use serde_json;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;

// Constants for file processing
const PROGRESS_UPDATE_INTERVAL: u32 = 10000; // Update progress every 10k lines

pub struct GuardianEngine {
    config: Config,
    analyzer_registry: AnalyzerRegistry,
    progress: ProgressReporter,
    cache: Arc<Mutex<FileCache>>,
    streaming_analyzer: StreamingAnalyzer,
    stats: AnalysisStats,
}

impl GuardianEngine {
    pub async fn new(config: Config, progress: ProgressReporter) -> Result<Self> {
        let cache = Arc::new(Mutex::new(FileCache::load().await?));

        Ok(Self {
            config,
            analyzer_registry: AnalyzerRegistry::new(),
            progress,
            cache,
            streaming_analyzer: StreamingAnalyzer::new(),
            stats: AnalysisStats::new(),
        })
    }

    pub async fn get_all_files(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut all_files = Vec::new();

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
        let mut files = Vec::new();

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

            // Check exclude patterns from config
            for pattern in &self.config.files.exclude_patterns {
                if name.contains(pattern.trim_start_matches('*')) {
                    return false;
                }
            }
        }

        // Check file size limits from config (security: prevent processing huge files)
        if let Ok(metadata) = path.metadata() {
            if metadata.len() > self.config.files.max_file_size_bytes {
                return false;
            }
        }

        // Check file extension against allowed extensions
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if !self
                .config
                .files
                .analyze_extensions
                .contains(&format!(".{}", ext))
            {
                return false;
            }
        }

        true
    }

    pub async fn analyze_files(
        &mut self,
        files: &[PathBuf],
        parallel: usize,
    ) -> Result<AnalysisResults> {
        let start_time = Instant::now();
        let config_hash = self.compute_config_hash();
        let mut results = AnalysisResults::new(config_hash.clone());

        // Start progress reporting
        self.progress.start_scan(files.len());

        // Process cached files
        self.process_cached_files(&mut results, files, &config_hash)
            .await?;

        // Process uncached files
        self.process_uncached_files(&mut results, files, &config_hash, parallel)
            .await?;

        // Update summary and finalize
        self.finalize_analysis(&mut results, files, start_time);

        Ok(results)
    }

    fn analyze_single_file_optimized(
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
            self.analyze_large_file_streaming(file_path, analyzer_registry, streaming_analyzer)
        } else {
            // Standard in-memory analysis for smaller files
            let content = std::fs::read(file_path)?;
            analyzer_registry.analyze_file(file_path, &content)
        }
    }

    fn analyze_large_file_streaming(
        &self,
        file_path: &Path,
        analyzer_registry: &AnalyzerRegistry,
        _streaming_analyzer: &StreamingAnalyzer,
    ) -> Result<Vec<Finding>> {
        // For now, fall back to chunked reading for large files
        // In a full implementation, this would use the streaming analyzer
        // with line-by-line or chunk-by-chunk processing

        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut all_findings = Vec::new();

        // Process file line by line to save memory
        let mut line_buffer = String::new();
        let mut line_number = 1u32;

        for line_result in reader.lines() {
            let line = line_result?;
            line_buffer.clear();
            line_buffer.push_str(&line);
            line_buffer.push('\n');

            // Analyze this line
            let line_findings =
                analyzer_registry.analyze_file(file_path, line_buffer.as_bytes())?;
            all_findings.extend(line_findings);

            line_number += 1;

            // Yield occasionally for very large files
            if line_number % PROGRESS_UPDATE_INTERVAL == 0 {
                std::thread::yield_now();
            }
        }

        Ok(all_findings)
    }

    async fn partition_cached_files(
        &self,
        files: &[PathBuf],
        config_hash: &str,
    ) -> Result<(Vec<(PathBuf, Vec<Finding>)>, Vec<PathBuf>)> {
        let mut cached_files = Vec::new();
        let mut uncached_files = Vec::new();

        if let Ok(cache_guard) = self.cache.lock() {
            for file_path in files {
                if let Some(cached_findings) =
                    cache_guard.get_cached_findings(file_path, config_hash)
                {
                    cached_files.push((file_path.clone(), cached_findings));
                } else {
                    uncached_files.push(file_path.clone());
                }
            }
        } else {
            // If cache is locked, treat all files as uncached
            uncached_files.extend_from_slice(files);
        }

        Ok((cached_files, uncached_files))
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
            self.stats.cache_hits += 1;
            self.progress
                .update(&format!("Cached: {}", file_path.display()));
        }

        Ok(())
    }

    async fn process_uncached_files(
        &mut self,
        results: &mut AnalysisResults,
        files: &[PathBuf],
        config_hash: &str,
        parallel: usize,
    ) -> Result<()> {
        let (_, uncached_files) = self.partition_cached_files(files, config_hash).await?;

        if uncached_files.is_empty() {
            return Ok(());
        }

        // Determine parallelism
        let _num_workers = if parallel == 0 {
            num_cpus::get()
        } else {
            parallel
        };

        // Process uncached files sequentially (parallel disabled for simplicity)
        let mut cache_misses = 0;
        let mut errors = 0;
        let mut findings = Vec::new();

        for file_path in &uncached_files {
            match self.analyze_single_file_optimized(
                file_path,
                &self.analyzer_registry,
                &self.streaming_analyzer,
                config_hash,
            ) {
                Ok(file_findings) => {
                    self.cache_file_findings(file_path, &file_findings, config_hash)
                        .await?;
                    findings.extend(file_findings);
                    cache_misses += 1;
                }
                Err(e) => {
                    tracing::error!("Error analyzing {}: {}", file_path.display(), e);
                    errors += 1;
                }
            }
        }

        self.stats.cache_misses += cache_misses;
        self.stats.errors += errors;

        // Add findings to results
        for finding in findings {
            results.add_finding(finding);
        }

        Ok(())
    }

    async fn cache_file_findings(
        &self,
        file_path: &Path,
        findings: &[Finding],
        config_hash: &str,
    ) -> Result<()> {
        if let Ok(mut cache_guard) = self.cache.lock() {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(cache_guard.cache_findings(
                    file_path,
                    findings.to_vec(),
                    config_hash,
                ))
            })?;
        }
        Ok(())
    }

    fn finalize_analysis(
        &mut self,
        results: &mut AnalysisResults,
        files: &[PathBuf],
        start_time: Instant,
    ) {
        // Update summary
        results.summary.total_files_scanned = files.len();
        self.stats.total_duration = start_time.elapsed();

        // Save cache asynchronously without holding the lock
        let serialized_cache = if let Ok(cache_guard) = self.cache.lock() {
            serde_json::to_string_pretty(&*cache_guard).ok()
        } else {
            None
        };

        if let Some(content) = serialized_cache {
            tokio::spawn(async move {
                #[allow(clippy::let_underscore_future)]
                let _ = tokio::fs::write(crate::cache::FileCache::CACHE_FILE, content);
            });
        }

        // Finish progress reporting
        self.progress.finish(&format!(
            "Analyzed {} files ({} cached, {} new) in {:.2}s",
            files.len(),
            self.stats.cache_hits,
            self.stats.cache_misses,
            self.stats.total_duration.as_secs_f64()
        ));
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

#[derive(Debug, Default)]
pub struct AnalysisStats {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub errors: usize,
    pub total_duration: std::time::Duration,
}

impl AnalysisStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
