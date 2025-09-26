use do_codeguardian::analyzers::Analyzer;
use std::path::Path;
use std::time::Instant;
use tempfile::tempdir;

// Test the optimized parallel file processor
#[tokio::test]
async fn test_optimized_parallel_processor() {
    let temp_dir = tempdir().unwrap();

    // Create test files of different sizes
    let small_file = temp_dir.path().join("small.txt");
    let large_file = temp_dir.path().join("large.txt");

    // Small file content
    tokio::fs::write(&small_file, "small content")
        .await
        .unwrap();

    // Large file content (should trigger memory mapping)
    let large_content = "large content\n".repeat(100000); // ~1.2MB
    tokio::fs::write(&large_file, &large_content).await.unwrap();

    let files = vec![small_file, large_file];

    // Test the optimized processor
    let processor =
        do_codeguardian::core::parallel_file_processor::ParallelFileProcessor::new(Some(2));

    let start = Instant::now();
    let results = processor.batch_read_files(&files).await.unwrap();
    let duration = start.elapsed();

    assert_eq!(results.len(), 2);
    assert!(duration < std::time::Duration::from_millis(500)); // Should be fast

    println!("Optimized parallel processing took: {:?}", duration);
}

// Test duplicate analyzer optimization
#[test]
fn test_duplicate_analyzer_optimization() {
    use do_codeguardian::analyzers::duplicate_analyzer::DuplicateAnalyzer;

    let analyzer = DuplicateAnalyzer::new().unwrap();

    // Create test content with duplicate blocks
    let content = r#"
fn func1() {
    let x = 1;
    let y = 2;
    let z = x + y;
}

fn func2() {
    let x = 1;
    let y = 2;
    let z = x + y;
}
"#;

    let findings = analyzer
        .analyze(Path::new("test.rs"), content.as_bytes())
        .unwrap();

    // Should detect duplicates
    assert!(!findings.is_empty());
    println!("Found {} duplicate findings", findings.len());
}

#[test]
fn test_cross_file_duplicate_optimization() {
    use do_codeguardian::analyzers::cross_file_duplicate_analyzer::CrossFileDuplicateAnalyzer;

    let mut analyzer = CrossFileDuplicateAnalyzer::new().unwrap();

    // Analyze files with similar content
    let content1 = r#"
fn authenticate() {
    let user = "test";
    let pass = "secret";
    validate(user, pass);
}
"#;

    let content2 = r#"
fn login() {
    let user = "test";
    let pass = "secret";
    validate(user, pass);
}
"#;

    analyzer
        .analyze_file(Path::new("auth1.rs"), content1)
        .unwrap();
    analyzer
        .analyze_file(Path::new("auth2.rs"), content2)
        .unwrap();

    let duplicates = analyzer.find_cross_file_duplicates().unwrap();

    // Should detect cross-file duplicates
    assert!(!duplicates.is_empty());
    println!("Found {} cross-file duplicates", duplicates.len());
}
