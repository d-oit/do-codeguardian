use crate::types::Finding;
use anyhow::Result;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader as AsyncBufReader};
use tokio::sync::Semaphore;

/// Threshold for switching to streaming analysis (2MB)
const STREAMING_THRESHOLD: u64 = 2 * 1024 * 1024;

/// Maximum concurrent streaming operations
const MAX_CONCURRENT_STREAMS: usize = 4;

/// Supported file types for streaming
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Text,
    Json,
    Xml,
    Csv,
    Log,
    SourceCode,
    Binary,
    Compressed,
    Unknown,
}

#[derive(Clone)]
pub struct StreamingAnalyzer {
    chunk_size: usize,
    max_concurrent_streams: usize,
    adaptive_chunking: AdaptiveChunking,
}

impl Default for StreamingAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingAnalyzer {
    pub fn new() -> Self {
        Self {
            chunk_size: 64 * 1024, // 64KB chunks
            max_concurrent_streams: MAX_CONCURRENT_STREAMS,
            adaptive_chunking: AdaptiveChunking::new(),
        }
    }

    pub fn new_with_config(chunk_size: usize, max_concurrent: usize) -> Self {
        Self {
            chunk_size,
            max_concurrent_streams: max_concurrent,
            adaptive_chunking: AdaptiveChunking::new(),
        }
    }

    /// Detect file type based on extension and content
    pub fn detect_file_type(file_path: &Path) -> FileType {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" | "hpp" | "go" | "rb"
                | "php" => FileType::SourceCode,
                "json" => FileType::Json,
                "xml" | "html" | "htm" => FileType::Xml,
                "csv" => FileType::Csv,
                "log" => FileType::Log,
                "gz" | "bz2" | "xz" | "zst" => FileType::Compressed,
                "txt" | "md" | "rst" => FileType::Text,
                _ => FileType::Unknown,
            }
        } else {
            FileType::Unknown
        }
    }

    pub fn should_use_streaming(file_path: &Path) -> bool {
        let file_size = file_path.metadata().map(|m| m.len()).unwrap_or(0);

        // Always stream large files
        if file_size > STREAMING_THRESHOLD {
            return true;
        }

        // Stream certain file types even if small
        let file_type = Self::detect_file_type(file_path);
        matches!(
            file_type,
            FileType::Log | FileType::Csv | FileType::Compressed
        )
    }

    /// Get optimal streaming configuration for a file
    pub fn get_streaming_config(&self, file_path: &Path) -> StreamingConfig {
        let file_size = file_path.metadata().map(|m| m.len()).unwrap_or(0);
        let file_type = Self::detect_file_type(file_path);

        let chunk_size = self.adaptive_chunking.optimal_chunk_size(file_size);
        let use_parallel = file_size > 10 * 1024 * 1024; // Use parallel processing for very large files

        StreamingConfig {
            chunk_size,
            use_parallel,
            file_type,
            file_size,
        }
    }

    /// Comprehensive streaming analysis for large files
    pub async fn analyze_large_file<F>(
        &self,
        file_path: &Path,
        analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let config = self.get_streaming_config(file_path);

        match config.file_type {
            FileType::Compressed => self.analyze_compressed_file(file_path, analyzer_fn).await,
            FileType::Json => self.analyze_json_streaming(file_path, analyzer_fn).await,
            FileType::Csv => self.analyze_csv_streaming(file_path, analyzer_fn).await,
            FileType::Log => self.analyze_log_streaming(file_path, analyzer_fn).await,
            _ => {
                self.analyze_text_streaming_async(file_path, analyzer_fn, config)
                    .await
            }
        }
    }

    /// Async text file streaming analysis
    async fn analyze_text_streaming_async<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
        config: StreamingConfig,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = File::open(file_path).await?;
        let reader = AsyncBufReader::with_capacity(config.chunk_size, file);
        let mut lines = reader.lines();
        // Use adaptive capacity based on file size estimate
        let estimated_findings = (config.file_size / 100).min(10000) as usize;
        let mut all_findings = Vec::with_capacity(estimated_findings);
        let mut line_number = 1;
        let mut processed_lines = 0;

        while let Some(line) = lines.next_line().await? {
            // Analyze line by line for memory efficiency
            let mut line_findings = analyzer_fn(&line, line_number)?;
            all_findings.append(&mut line_findings);
            line_number += 1;
            processed_lines += 1;

            // Yield control periodically to prevent blocking
            if processed_lines % 1000 == 0 {
                tokio::task::yield_now().await;
            }

            // Adaptive yielding based on processing speed
            if processed_lines % 10000 == 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }

        Ok(all_findings)
    }

    /// Streaming analysis for JSON files with structure awareness
    async fn analyze_json_streaming<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = File::open(file_path).await?;
        let reader = AsyncBufReader::new(file);
        let mut lines = reader.lines();
        let mut all_findings = Vec::new();
        let mut line_number = 1;
        let mut brace_depth = 0;
        let mut current_object = String::new();

        while let Ok(Some(line)) = lines.next_line().await {
            // Track JSON structure
            brace_depth += line.chars().filter(|&c| c == '{').count() as i32;
            brace_depth -= line.chars().filter(|&c| c == '}').count() as i32;

            current_object.push_str(&line);
            current_object.push('\n');

            // Analyze complete JSON objects
            if brace_depth == 0 && !current_object.trim().is_empty() {
                let mut line_findings = analyzer_fn(&current_object, line_number)?;
                all_findings.append(&mut line_findings);
                current_object.clear();
            }

            line_number += 1;

            if line_number % 1000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(all_findings)
    }

    /// Streaming analysis for CSV files
    async fn analyze_csv_streaming<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = File::open(file_path).await?;
        let reader = AsyncBufReader::new(file);
        let mut lines = reader.lines();
        let mut all_findings = Vec::new();
        let mut line_number = 1;

        // Skip header if present
        if let Ok(Some(_header)) = lines.next_line().await {
            line_number += 1;
        }

        while let Ok(Some(line)) = lines.next_line().await {
            // Analyze CSV row
            let mut line_findings = analyzer_fn(&line, line_number)?;
            all_findings.append(&mut line_findings);
            line_number += 1;

            if line_number % 1000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(all_findings)
    }

    /// Streaming analysis for log files with timestamp awareness
    async fn analyze_log_streaming<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = File::open(file_path).await?;
        let reader = AsyncBufReader::new(file);
        let mut lines = reader.lines();
        let mut all_findings = Vec::new();
        let mut line_number = 1;
        let mut current_entry = String::new();
        let mut entry_start_line = 1;

        while let Ok(Some(line)) = lines.next_line().await {
            // Check if this is a new log entry (starts with timestamp or similar pattern)
            let is_new_entry = line.len() > 20
                && (line.chars().nth(4) == Some('-') || // ISO timestamp
                 line.chars().nth(19) == Some('.') || // Unix timestamp
                 line.starts_with("202") || // Year-based timestamp
                 line.chars().take(3).all(|c| c.is_ascii_digit())); // Date-based

            if is_new_entry && !current_entry.is_empty() {
                // Analyze previous log entry
                let mut line_findings = analyzer_fn(&current_entry, entry_start_line)?;
                all_findings.append(&mut line_findings);
                current_entry.clear();
                entry_start_line = line_number;
            }

            current_entry.push_str(&line);
            current_entry.push('\n');
            line_number += 1;

            if line_number % 1000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        // Analyze final entry
        if !current_entry.is_empty() {
            let mut line_findings = analyzer_fn(&current_entry, entry_start_line)?;
            all_findings.append(&mut line_findings);
        }

        Ok(all_findings)
    }

    /// Streaming analysis for compressed files
    async fn analyze_compressed_file<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        use std::io::Read;

        let file = std::fs::File::open(file_path)?;
        let mut all_findings = Vec::new();

        // Handle different compression formats
        let content = if file_path.extension().and_then(|e| e.to_str()) == Some("gz") {
            let mut decoder = flate2::read::GzDecoder::new(file);
            let mut decompressed = String::new();
            decoder.read_to_string(&mut decompressed)?;
            decompressed
        } else {
            // For other formats, read as text (fallback)
            let mut content = String::new();
            std::io::BufReader::new(file).read_to_string(&mut content)?;
            content
        };

        // Analyze decompressed content line by line
        for (line_number, line) in content.lines().enumerate() {
            let mut line_findings = analyzer_fn(line, line_number + 1)?;
            all_findings.append(&mut line_findings);

            if line_number % 1000 == 0 {
                tokio::task::yield_now().await;
            }
        }

        Ok(all_findings)
    }

    #[allow(dead_code)]
    pub async fn analyze_binary_chunks<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&[u8], u64) -> Result<Vec<Finding>>,
    {
        use tokio::io::AsyncReadExt;

        let mut file = File::open(file_path).await?;
        let mut buffer = vec![0u8; self.chunk_size];
        let mut offset = 0u64;
        let mut all_findings = Vec::new();

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            let chunk = &buffer[..bytes_read];
            let mut chunk_findings = analyzer_fn(chunk, offset)?;
            all_findings.append(&mut chunk_findings);

            offset += bytes_read as u64;

            // Yield control to prevent blocking
            tokio::task::yield_now().await;
        }

        Ok(all_findings)
    }

    /// Memory-efficient line-by-line analysis for text files
    pub fn analyze_text_streaming<F>(
        &self,
        file_path: &Path,
        mut analyzer_fn: F,
    ) -> Result<Vec<Finding>>
    where
        F: FnMut(&str, usize) -> Result<Vec<Finding>>,
    {
        let file = std::fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut all_findings = Vec::new();

        for (line_number, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let mut line_findings = analyzer_fn(&line, line_number + 1)?;
            all_findings.append(&mut line_findings);
        }

        Ok(all_findings)
    }

    /// Parallel streaming analysis for very large files
    pub async fn analyze_parallel_streaming<F>(
        &self,
        file_paths: &[std::path::PathBuf],
        analyzer_fn: F,
    ) -> Result<HashMap<std::path::PathBuf, Vec<Finding>>>
    where
        F: Fn(&str, usize) -> Result<Vec<Finding>> + Send + Sync + Clone + 'static,
    {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_streams));
        let mut tasks = Vec::new();
        let mut results = HashMap::new();

        for file_path in file_paths {
            let file_path = file_path.clone();
            let analyzer_fn = analyzer_fn.clone();
            let semaphore = Arc::clone(&semaphore);

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let config = Self::new().get_streaming_config(&file_path);

                let findings = if config.use_parallel {
                    Self::analyze_text_streaming_async(
                        &Self::new(),
                        &file_path,
                        analyzer_fn,
                        config,
                    )
                    .await?
                } else {
                    Self::analyze_text_streaming(&Self::new(), &file_path, analyzer_fn)?
                };

                Ok::<_, anyhow::Error>((file_path, findings))
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            match task.await {
                Ok(Ok((file_path, findings))) => {
                    results.insert(file_path, findings);
                }
                Ok(Err(e)) => {
                    eprintln!("Streaming error: {}", e);
                }
                Err(e) => {
                    eprintln!("Task join error: {}", e);
                }
            }
        }

        Ok(results)
    }

    /// Get streaming statistics
    pub fn get_streaming_stats(&self) -> StreamingStats {
        StreamingStats {
            chunk_size: self.chunk_size,
            max_concurrent_streams: self.max_concurrent_streams,
            streaming_threshold: STREAMING_THRESHOLD,
            adaptive_chunking: self.adaptive_chunking.clone(),
        }
    }
}

/// Configuration for streaming analysis
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    pub chunk_size: usize,
    pub use_parallel: bool,
    pub file_type: FileType,
    pub file_size: u64,
}

/// Streaming statistics and configuration
#[derive(Debug)]
pub struct StreamingStats {
    pub chunk_size: usize,
    pub max_concurrent_streams: usize,
    pub streaming_threshold: u64,
    pub adaptive_chunking: AdaptiveChunking,
}

/// Adaptive chunk size based on file size and available memory
#[derive(Debug, Clone)]
pub struct AdaptiveChunking {
    base_chunk_size: usize,
    max_chunk_size: usize,
    available_memory: usize,
}

impl Default for AdaptiveChunking {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveChunking {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            base_chunk_size: 64 * 1024,  // 64KB
            max_chunk_size: 1024 * 1024, // 1MB
            available_memory: Self::estimate_available_memory(),
        }
    }

    #[allow(dead_code)]
    pub fn optimal_chunk_size(&self, file_size: u64) -> usize {
        // Adaptive chunking based on file size and available memory
        let memory_based_chunk = self.available_memory / 10; // Use 10% of available memory
        let file_based_chunk = (file_size / 100).max(self.base_chunk_size as u64) as usize;

        memory_based_chunk
            .min(file_based_chunk)
            .min(self.max_chunk_size)
            .max(self.base_chunk_size)
    }

    #[allow(dead_code)]
    fn estimate_available_memory() -> usize {
        // Simple heuristic - in production, use proper memory detection
        #[cfg(target_os = "linux")]
        {
            if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
                for line in meminfo.lines() {
                    if line.starts_with("MemAvailable:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return kb * 1024; // Convert to bytes
                            }
                        }
                    }
                }
            }
        }

        // Fallback: assume 1GB available
        1024 * 1024 * 1024
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    // use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_streaming_analysis() {
        let mut temp_file = NamedTempFile::new().unwrap();

        // Write test content
        let content = "line 1\nline 2\nline 3\n".repeat(1000);
        temp_file.write_all(content.as_bytes()).unwrap();

        let analyzer = StreamingAnalyzer::new();
        let findings = analyzer
            .analyze_large_file(temp_file.path(), |line, line_num| {
                if line.contains("line 2") {
                    Ok(vec![Finding::new(
                        "test",
                        "test_rule",
                        crate::types::Severity::Info,
                        temp_file.path().to_path_buf(),
                        line_num as u32,
                        "Found line 2".to_string(),
                    )])
                } else {
                    Ok(vec![])
                }
            })
            .await
            .unwrap();

        assert_eq!(findings.len(), 1000); // Should find "line 2" in each repetition
    }

    #[test]
    fn test_adaptive_chunking() {
        let chunking = AdaptiveChunking::new();

        // Small file should use base chunk size
        let small_chunk = chunking.optimal_chunk_size(1024);
        assert_eq!(small_chunk, chunking.base_chunk_size);

        // Large file should use larger chunks but not exceed max
        let large_chunk = chunking.optimal_chunk_size(100 * 1024 * 1024);
        assert!(large_chunk > chunking.base_chunk_size);
        assert!(large_chunk <= chunking.max_chunk_size);
    }
}
