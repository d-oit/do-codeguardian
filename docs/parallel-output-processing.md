# Parallel Output Processing

CodeGuardian now supports parallel processing for multiple output format generation, providing significant performance improvements on multi-core systems.

## Features

- **Parallel Format Generation**: Process multiple output formats concurrently using rayon
- **Chunk-Level Parallelization**: Handle large datasets with configurable chunk sizes
- **Semaphore-Based Concurrency Control**: Limit concurrent operations to prevent resource exhaustion
- **Concurrent Processing Pipelines**: Multi-stage processing with validation and storage
- **Performance Optimization**: Configurable thread pools and resource management

## Usage

### Basic Parallel Output Processing

```rust
use do_codeguardian::output::{ParallelOutputProcessor, OutputFormat};
use do_codeguardian::types::AnalysisResults;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create analysis results
    let results = AnalysisResults::new("example".to_string());

    // Create parallel processor
    let processor = ParallelOutputProcessor::new()?;

    // Process multiple formats in parallel
    let formats = vec![
        OutputFormat::Json,
        OutputFormat::Html,
        OutputFormat::Markdown,
        OutputFormat::Sarif,
    ];

    let outputs = processor.process_multiple_formats(&results, formats).await?;

    // Access results
    for (format, result) in outputs {
        println!("Generated {} output: {} bytes", format, result.content.len());
    }

    Ok(())
}
```

### Custom Configuration

```rust
use do_codeguardian::output::{ParallelOutputProcessor, ParallelOutputConfig};

let config = ParallelOutputConfig {
    max_concurrent_formats: 2,  // Limit to 2 concurrent formats
    chunk_size: 500,            // Process 500 items per chunk
    max_parallel_chunks: 4,     // Max 4 parallel chunks
    thread_pool_size: 8,        // 8 threads in rayon pool
};

let processor = ParallelOutputProcessor::with_config(config)?;
```

### Chunk-Level Parallelization

```rust
use do_codeguardian::output::ChunkParallelProcessor;

// Process large datasets in chunks
let chunk_processor = ChunkParallelProcessor::new();

// Synchronous chunk processing
let results = chunk_processor.process_chunks_sync(&large_data, |chunk| {
    // Process chunk synchronously
    Ok(processed_chunk)
})?;

// Asynchronous chunk processing
let results = chunk_processor.process_chunks_async(&large_data, |chunk| {
    async move {
        // Process chunk asynchronously
        Ok(processed_chunk)
    }
}).await?;
```

### Concurrent Pipeline Processing

```rust
// Process with full pipeline (format -> validate -> store)
let pipeline_result = processor.process_concurrent_pipeline(&results, formats).await?;

println!("Pipeline completed in {:?}", pipeline_result.total_processing_time);
println!("Pipeline efficiency: {:.1}%", pipeline_result.pipeline_efficiency * 100.0);
```

## Performance Benefits

- **Multi-core Utilization**: Leverages all available CPU cores
- **Concurrent I/O**: Parallel file operations and network requests
- **Memory Efficiency**: Configurable resource limits prevent memory exhaustion
- **Scalability**: Performance scales with core count and data size

## Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `max_concurrent_formats` | `num_cpus::get().min(4)` | Maximum concurrent format generations |
| `chunk_size` | `1000` | Items per chunk for parallel processing |
| `max_parallel_chunks` | `num_cpus::get()` | Maximum parallel chunks |
| `thread_pool_size` | `num_cpus::get()` | Rayon thread pool size |

## Benchmarks

Run performance benchmarks:

```bash
cargo bench --bench optimization_benchmarks -- parallel_output_processing
```

Expected performance improvements:
- 2-4x faster for multiple format generation
- Linear scaling with CPU core count
- Reduced memory usage through chunking
