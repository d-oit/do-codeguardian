use do_codeguardian::{analyze_files, Config};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_security_analysis() {
        // Create test files
        let test_files = create_test_files();

        // Test file analysis using the main analyze_files function
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        println!("File paths: {:?}", file_paths);
        let config = Config::default();
        let results = analyze_files(&file_paths, &config).await.unwrap();
        println!("Files analyzed: {}", results.files_analyzed);
        assert!(results.files_analyzed > 0);

        println!("✅ Basic security analysis test passed");
    }

    #[tokio::test]
    async fn test_configuration_loading() {
        // Test default configuration
        let config = Config::default();
        assert!(config.security.max_file_size > 0);

        // Test configuration from file (if it exists)
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let config_content = r#"
[security]
fail_on_issues = true
min_severity = "medium"
max_file_size = 5242880

[git]
conventional_commits = true
commit_template = "{type}({scope}): {description}"
require_signed_commits = false

[analysis]
exclude_patterns = ["*.log", "*.tmp"]
analyze_binaries = false
analysis_timeout = 300

[logging]
level = "info"
log_to_file = false
"#;

        std::fs::write(&config_path, config_content).unwrap();

        let loaded_config = Config::from_file(&config_path).unwrap();
        assert_eq!(loaded_config.security.fail_on_issues, true);
        assert_eq!(loaded_config.security.min_severity, "medium");

        println!("✅ Configuration loading test passed");
    }

    #[tokio::test]
    async fn test_large_file_analysis() {
        // Create a large test file
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "line 1\nline 2\nfn test() {}\nline 4\n".repeat(1000);
        temp_file.write_all(content.as_bytes()).unwrap();

        // Test analysis of large file
        let file_paths = vec![temp_file.path().to_path_buf()];
        let config = Config::default();
        let results = analyze_files(&file_paths, &config).await.unwrap();

        assert!(results.files_analyzed == 1);
        // The analysis should complete without errors
        assert!(results.duration_ms > 0);

        println!("✅ Large file analysis test passed");
    }

    #[tokio::test]
    async fn test_multiple_file_types() {
        // Create test files of different types
        let test_files = create_multiple_file_types();

        // Test analysis of multiple file types
        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        let config = Config::default();
        let results = analyze_files(&file_paths, &config).await.unwrap();

        assert_eq!(results.files_analyzed, file_paths.len());
        assert!(results.duration_ms > 0);

        println!("✅ Multiple file types test passed");
    }

    #[tokio::test]
    async fn test_end_to_end_workflow() {
        // Create test files
        let test_files = create_test_files();

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        // Measure performance
        let start_time = std::time::Instant::now();
        let config = Config::default();
        let results = analyze_files(&file_paths, &config).await.unwrap();
        let duration = start_time.elapsed();

        // Validate results
        assert_eq!(results.files_analyzed, file_paths.len());
        assert!(results.duration_ms > 0);
        assert!(duration.as_millis() > 0); // Should complete in reasonable time

        println!("✅ End-to-end workflow test passed in {:?}", duration);
        println!("   Files processed: {}", results.files_analyzed);
        println!("   Issues found: {}", results.issues.len());
    }

    #[test]
    fn test_basic_functionality() {
        // Simple test to ensure basic functionality works
        let test_files = create_test_files();

        // Just verify we can create and access test files
        assert!(!test_files.is_empty());

        for (file, _name, _size) in &test_files {
            assert!(file.path().exists());
            let content = std::fs::read_to_string(file.path()).unwrap();
            assert!(!content.is_empty());
        }

        // Files should be cleaned up automatically by tempfile
        println!("✅ Basic functionality test passed");
    }

    fn create_test_files() -> Vec<(NamedTempFile, String, usize)> {
        let mut files = Vec::new();

        // Small Rust file
        let content1 = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(content1.as_bytes()).unwrap();
        files.push((file1, "test.rs".to_string(), content1.len()));

        // Medium JSON file
        let content2 = r#"{"users": [{"id": 1, "name": "test"}], "config": {"debug": true}}"#;
        let mut file2 = NamedTempFile::new().unwrap();
        file2.write_all(content2.as_bytes()).unwrap();
        files.push((file2, "data.json".to_string(), content2.len()));

        files
    }

    fn create_multiple_file_types() -> Vec<(NamedTempFile, String, usize)> {
        let mut files = Vec::new();

        // Rust file
        let content1 = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
        let mut file1 = NamedTempFile::new().unwrap();
        file1.write_all(content1.as_bytes()).unwrap();
        files.push((file1, "test.rs".to_string(), content1.len()));

        // Python file
        let content2 =
            "def main():\n    print('Hello, world!')\n\nif __name__ == '__main__':\n    main()";
        let mut file2 = NamedTempFile::new().unwrap();
        file2.write_all(content2.as_bytes()).unwrap();
        files.push((file2, "test.py".to_string(), content2.len()));

        // JavaScript file
        let content3 = "function main() {\n    console.log('Hello, world!');\n}\n\nmain();";
        let mut file3 = NamedTempFile::new().unwrap();
        file3.write_all(content3.as_bytes()).unwrap();
        files.push((file3, "test.js".to_string(), content3.len()));

        files
    }

    #[test]
    fn test_security_patterns() {
        // Test that security analysis detects basic patterns
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = b"fn main() {\n    let password = \"secret123\";\n    println!(\"Password: {}\", password);\n}";
        temp_file.write_all(content).unwrap();

        let file_paths = vec![temp_file.path().to_path_buf()];
        let config = Config::default();

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(analyze_files(&file_paths, &config));

        assert!(result.is_ok());
        let analysis = result.unwrap();

        // Should detect the hardcoded password
        assert!(analysis.issues.len() > 0);

        println!(
            "✅ Security patterns test passed - detected {} issues",
            analysis.issues.len()
        );
    }
}
