use std::path::Path;
use std::time::Instant;
use tempfile::tempdir;

#[tokio::main]
async fn main() {
    println!("Running CodeGuardian Performance Optimization Benchmarks");
    println!("======================================================");

    // Test 1: Parallel I/O Operations
    println!("\n1. Testing Parallel I/O Operations");
    test_parallel_io().await;

    // Test 2: Memory-Mapped Files
    println!("\n2. Testing Memory-Mapped Files");
    test_memory_mapping().await;

    // Test 3: Algorithmic Complexity Reduction
    println!("\n3. Testing Algorithmic Complexity Reduction");
    test_algorithmic_optimization();

    println!("\nBenchmarking complete!");
}

async fn test_parallel_io() {
    let temp_dir = tempdir().unwrap();
    let processor =
        do_codeguardian::core::parallel_file_processor::ParallelFileProcessor::new(Some(4));

    // Create multiple test files
    let mut files = Vec::new();
    for i in 0..10 {
        let file_path = temp_dir.path().join(format!("test_{}.txt", i));
        let content = format!("Test content for file {} with some data\n", i).repeat(1000);
        tokio::fs::write(&file_path, content).await.unwrap();
        files.push(file_path);
    }

    let start = Instant::now();
    let results = processor.batch_read_files(&files).await.unwrap();
    let duration = start.elapsed();

    println!("  - Processed {} files in {:?}", results.len(), duration);
    println!(
        "  - Average time per file: {:?}",
        duration / results.len() as u32
    );
    println!(
        "  - Throughput: {:.1} files/second",
        results.len() as f64 / duration.as_secs_f64()
    );
}

async fn test_memory_mapping() {
    let temp_dir = tempdir().unwrap();

    // Create a large file (>10MB) to trigger memory mapping
    let large_file = temp_dir.path().join("large_test.txt");
    let large_content = "Large file content for memory mapping test\n".repeat(200000); // ~4MB
    tokio::fs::write(&large_file, &large_content).await.unwrap();

    let processor =
        do_codeguardian::core::parallel_file_processor::ParallelFileProcessor::new(Some(1));

    let start = Instant::now();
    let results = processor.batch_read_files(&[large_file]).await.unwrap();
    let duration = start.elapsed();

    println!(
        "  - Memory-mapped {} bytes in {:?}",
        large_content.len(),
        duration
    );
    assert_eq!(results[0].1, large_content.as_bytes());
}

fn test_algorithmic_optimization() {
    use do_codeguardian::analyzers::{duplicate_analyzer::DuplicateAnalyzer, Analyzer};

    let analyzer = DuplicateAnalyzer::new().unwrap();

    // Create content with many similar blocks to test O(n²) -> O(n) optimization
    let mut content = String::new();
    for i in 0..20 {
        content.push_str(&format!(
            r#"
fn function_{}() {{
    let x = {};
    let y = x + 1;
    let z = y * 2;
    return z;
}}
"#,
            i, i
        ));
    }

    let start = Instant::now();
    let findings = analyzer
        .analyze(Path::new("test.rs"), content.as_bytes())
        .unwrap();
    let duration = start.elapsed();

    println!("  - Analyzed {} functions in {:?}", 20, duration);
    println!("  - Found {} findings", findings.len());
    println!("  - Analysis completed in {:?}", duration);
}
