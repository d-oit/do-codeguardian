//! # Parallel Output Processing Module
//!
//! This module provides parallel processing capabilities for multiple output format generation.
//! It implements chunk-level parallelization, concurrent processing pipelines, and semaphore-based
//! concurrency control for optimal performance on multi-core systems.

use crate::output::formatter::OutputResult;
use crate::output::{create_formatter, OutputFormat};
use crate::types::AnalysisResults;
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

/// Configuration for parallel output processing
#[derive(Debug, Clone)]
pub struct ParallelOutputConfig {
    /// Maximum number of concurrent format generations
    pub max_concurrent_formats: usize,
    /// Chunk size for chunk-level parallelization
    pub chunk_size: usize,
    /// Maximum number of parallel chunks
    pub max_parallel_chunks: usize,
    /// Number of threads for rayon thread pool
    pub thread_pool_size: usize,
}

impl Default for ParallelOutputConfig {
    fn default() -> Self {
        Self {
            max_concurrent_formats: num_cpus::get().min(4), // Limit to 4 concurrent formats max
            chunk_size: 1000,                               // Process 1000 items per chunk
            max_parallel_chunks: num_cpus::get(),
            thread_pool_size: num_cpus::get(),
        }
    }
}

/// Parallel output processor using rayon for CPU-intensive tasks
pub struct ParallelOutputProcessor {
    config: ParallelOutputConfig,
    thread_pool: Arc<rayon::ThreadPool>,
}

impl ParallelOutputProcessor {
    /// Create a new parallel output processor with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ParallelOutputConfig::default())
    }

    /// Create a new parallel output processor with custom configuration
    pub fn with_config(config: ParallelOutputConfig) -> Result<Self> {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(config.thread_pool_size)
            .thread_name(|i| format!("output-worker-{}", i))
            .build()?;

        Ok(Self {
            config,
            thread_pool: Arc::new(thread_pool),
        })
    }

    /// Process multiple output formats in parallel with semaphore-based concurrency control
    pub async fn process_multiple_formats(
        &self,
        results: &AnalysisResults,
        formats: Vec<OutputFormat>,
    ) -> Result<HashMap<OutputFormat, OutputResult>> {
        if formats.is_empty() {
            return Ok(HashMap::new());
        }

        let results = Arc::new(results.clone());
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_formats));

        // Create tasks for each format
        let tasks: Vec<_> = formats
            .into_iter()
            .map(|format| {
                let results = Arc::clone(&results);
                let semaphore = Arc::clone(&semaphore);
                let thread_pool = Arc::clone(&self.thread_pool);

                task::spawn(async move {
                    let _permit = semaphore
                        .acquire()
                        .await
                        .map_err(|e| anyhow!("Failed to acquire semaphore permit: {}", e))?;
                    process_single_format(&results, format, thread_pool).await
                })
            })
            .collect();

        // Collect results
        let mut results_map = HashMap::new();
        for task in tasks {
            let (format, result) = task.await??;
            results_map.insert(format, result);
        }

        Ok(results_map)
    }

    /// Process large datasets using chunk-level parallelization
    pub async fn process_chunked_data<T, F, Fut>(
        &self,
        data: &[T],
        processor: F,
    ) -> Result<Vec<ProcessedChunk>>
    where
        T: Send + Sync + Clone + 'static,
        F: Fn(Vec<T>) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<ProcessedChunk>> + Send,
    {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Split data into chunks
        let chunks: Vec<Vec<T>> = data
            .chunks(self.config.chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let semaphore = Arc::new(Semaphore::new(self.config.max_parallel_chunks));
        let processor = Arc::new(processor);

        // Process chunks in parallel with semaphore control
        let tasks: Vec<_> = chunks
            .into_iter()
            .enumerate()
            .map(|(index, chunk)| {
                let semaphore = Arc::clone(&semaphore);
                let processor = Arc::clone(&processor);

                task::spawn(async move {
                    let _permit = semaphore
                        .acquire()
                        .await
                        .map_err(|e| anyhow!("Failed to acquire chunk semaphore permit: {}", e))?;
                    let result = processor(chunk).await?;
                    Ok::<_, anyhow::Error>((index, result))
                })
            })
            .collect();

        // Collect and sort results by original order
        let mut results = Vec::new();
        for task in tasks {
            let (index, result) = task.await??;
            results.push((index, result));
        }

        results.sort_by_key(|(index, _)| *index);
        let results: Vec<_> = results.into_iter().map(|(_, result)| result).collect();

        Ok(results)
    }

    /// Process output generation with concurrent pipelines
    pub async fn process_concurrent_pipeline(
        &self,
        results: &AnalysisResults,
        formats: Vec<OutputFormat>,
    ) -> Result<ConcurrentPipelineResult> {
        let start_time = std::time::Instant::now();

        // Stage 1: Parallel format processing
        let format_results = self.process_multiple_formats(results, formats).await?;

        // Stage 2: Parallel validation (if needed)
        let validation_results = self.validate_outputs_parallel(&format_results).await?;

        // Stage 3: Parallel compression/storage (if configured)
        let storage_results = self.store_outputs_parallel(format_results).await?;

        let total_time = start_time.elapsed();

        Ok(ConcurrentPipelineResult {
            outputs: storage_results,
            validation_results: validation_results.clone(),
            total_processing_time: total_time,
            pipeline_efficiency: self.calculate_pipeline_efficiency(&validation_results),
        })
    }

    /// Validate multiple outputs in parallel
    async fn validate_outputs_parallel(
        &self,
        outputs: &HashMap<OutputFormat, OutputResult>,
    ) -> Result<HashMap<OutputFormat, ValidationResult>> {
        let outputs: Vec<_> = outputs
            .iter()
            .map(|(fmt, result)| (*fmt, result.clone()))
            .collect();

        let validation_results: Vec<_> = outputs
            .par_iter()
            .map(|(format, result)| {
                let is_valid = result.is_valid();
                let errors = result.validation_errors();
                let warnings = result.validation_warnings();

                (
                    *format,
                    ValidationResult {
                        is_valid,
                        errors: errors.to_vec(),
                        warnings: warnings.to_vec(),
                        validation_time: std::time::Duration::from_millis(1), // Placeholder
                    },
                )
            })
            .collect();

        Ok(validation_results.into_iter().collect())
    }

    /// Store outputs in parallel (placeholder for future storage implementation)
    async fn store_outputs_parallel(
        &self,
        outputs: HashMap<OutputFormat, OutputResult>,
    ) -> Result<HashMap<OutputFormat, OutputResult>> {
        // For now, just return the outputs
        // In a real implementation, this would handle compression, storage, etc.
        Ok(outputs)
    }

    /// Calculate pipeline efficiency based on validation results
    fn calculate_pipeline_efficiency(
        &self,
        validation_results: &HashMap<OutputFormat, ValidationResult>,
    ) -> f64 {
        let total_validations = validation_results.len();
        if total_validations == 0 {
            return 1.0;
        }

        let valid_count = validation_results.values().filter(|v| v.is_valid).count();
        valid_count as f64 / total_validations as f64
    }

    /// Get performance metrics for the processor
    pub fn get_performance_metrics(&self) -> ParallelPerformanceMetrics {
        ParallelPerformanceMetrics {
            thread_pool_size: self.config.thread_pool_size,
            max_concurrent_formats: self.config.max_concurrent_formats,
            chunk_size: self.config.chunk_size,
            max_parallel_chunks: self.config.max_parallel_chunks,
        }
    }
}

/// Process a single output format using the rayon thread pool
async fn process_single_format(
    results: &Arc<AnalysisResults>,
    format: OutputFormat,
    thread_pool: Arc<rayon::ThreadPool>,
) -> Result<(OutputFormat, OutputResult)> {
    let results_clone = Arc::clone(results);

    // Use spawn_blocking to run CPU-intensive formatting on rayon thread pool
    let result = task::spawn_blocking(move || {
        thread_pool.install(|| {
            let formatter = create_formatter(format);
            formatter.format(&results_clone)
        })
    })
    .await??;

    Ok((format, result))
}

/// Result of processing a chunk of data
#[derive(Debug, Clone)]
pub struct ProcessedChunk {
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

/// Result of concurrent pipeline processing
#[derive(Debug)]
pub struct ConcurrentPipelineResult {
    pub outputs: HashMap<OutputFormat, OutputResult>,
    pub validation_results: HashMap<OutputFormat, ValidationResult>,
    pub total_processing_time: std::time::Duration,
    pub pipeline_efficiency: f64,
}

/// Validation result for an output
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub validation_time: std::time::Duration,
}

/// Performance metrics for parallel processing
#[derive(Debug, Clone)]
pub struct ParallelPerformanceMetrics {
    pub thread_pool_size: usize,
    pub max_concurrent_formats: usize,
    pub chunk_size: usize,
    pub max_parallel_chunks: usize,
}

/// Chunk-level parallel processor for large datasets
pub struct ChunkParallelProcessor {
    config: ParallelOutputConfig,
}

impl ChunkParallelProcessor {
    pub fn new() -> Self {
        Self {
            config: ParallelOutputConfig::default(),
        }
    }
}

impl Default for ChunkParallelProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkParallelProcessor {
    pub fn with_config(config: ParallelOutputConfig) -> Self {
        Self { config }
    }

    /// Process data in chunks using parallel processing (synchronous version)
    pub fn process_chunks_sync<T, F>(&self, data: &[T], processor: F) -> Result<Vec<ProcessedChunk>>
    where
        T: Send + Sync + Clone + 'static,
        F: Fn(&[T]) -> Result<ProcessedChunk> + Send + Sync,
    {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Create chunks
        let chunks: Vec<Vec<T>> = data
            .chunks(self.config.chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        // Process chunks in parallel using rayon
        let results: Vec<ProcessedChunk> = chunks
            .par_iter()
            .map(|chunk| processor(chunk))
            .collect::<Result<Vec<_>>>()?;

        Ok(results)
    }

    /// Process data in chunks using parallel processing (async version)
    pub async fn process_chunks_async<T, F, Fut>(
        &self,
        data: &[T],
        processor: F,
    ) -> Result<Vec<ProcessedChunk>>
    where
        T: Send + Sync + Clone + 'static,
        F: Fn(Vec<T>) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = Result<ProcessedChunk>> + Send,
    {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Create chunks
        let chunks: Vec<Vec<T>> = data
            .chunks(self.config.chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let semaphore = Arc::new(Semaphore::new(self.config.max_parallel_chunks));
        let processor = Arc::new(processor);

        // Process chunks in parallel with semaphore control
        let tasks: Vec<_> = chunks
            .into_iter()
            .map(|chunk| {
                let semaphore = Arc::clone(&semaphore);
                let processor = Arc::clone(&processor);

                task::spawn(async move {
                    let _permit = semaphore
                        .acquire()
                        .await
                        .map_err(|e| anyhow!("Failed to acquire chunk semaphore permit: {}", e))?;
                    processor(chunk).await
                })
            })
            .collect();

        // Collect results
        let mut results = Vec::new();
        for task in tasks {
            results.push(task.await??);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AnalysisResults;

    #[tokio::test]
    async fn test_parallel_output_processor_creation() {
        let processor = ParallelOutputProcessor::new().unwrap();
        assert_eq!(
            processor.config.max_concurrent_formats,
            num_cpus::get().min(4)
        );
    }

    #[tokio::test]
    async fn test_process_multiple_formats_empty() {
        let processor = ParallelOutputProcessor::new().unwrap();
        let results = AnalysisResults::new("test".to_string());
        let formats = vec![];

        let result = processor
            .process_multiple_formats(&results, formats)
            .await
            .unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_chunk_parallel_processor() {
        let processor = ChunkParallelProcessor::new();
        assert_eq!(processor.config.chunk_size, 1000);
    }

    #[test]
    fn test_parallel_performance_metrics() {
        let config = ParallelOutputConfig::default();
        let processor = ParallelOutputProcessor::with_config(config.clone()).unwrap();
        let metrics = processor.get_performance_metrics();

        assert_eq!(metrics.thread_pool_size, config.thread_pool_size);
        assert_eq!(
            metrics.max_concurrent_formats,
            config.max_concurrent_formats
        );
    }
}
