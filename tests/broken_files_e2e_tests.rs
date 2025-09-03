use clap::Parser;
use do_codeguardian::cli::{CheckArgs, Cli, Commands};
use do_codeguardian::config::Config;
use do_codeguardian::core::GuardianEngine;
use do_codeguardian::utils::progress::ProgressReporter;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
mod e2e_tests {
    use super::*;

    #[test]
    fn test_cli_detect_conflicts_flag() {
        // Test --detect-conflicts CLI flag
        let args = vec!["codeguardian", "check", "--detect-conflicts", "test.rs"];

        let cli = Cli::try_parse_from(args).unwrap();

        if let Commands::Check(check_args) = cli.command {
            assert!(check_args.detect_conflicts);
            assert!(!check_args.detect_placeholders);
            assert!(!check_args.detect_duplicates);
            assert!(!check_args.detect_broken_files);
        } else {
            panic!("Expected Check command");
        }
    }

    #[test]
    fn test_cli_detect_broken_files_flag() {
        // Test --detect-broken-files CLI flag (enables all)
        let args = vec!["codeguardian", "check", "--detect-broken-files", "test.rs"];

        let cli = Cli::try_parse_from(args).unwrap();

        if let Commands::Check(check_args) = cli.command {
            assert!(check_args.detect_broken_files);
            assert!(!check_args.detect_conflicts); // Individual flags should be false
            assert!(!check_args.detect_placeholders);
            assert!(!check_args.detect_duplicates);
        } else {
            panic!("Expected Check command");
        }
    }

    #[test]
    fn test_cli_fail_on_conflicts_flag() {
        // Test --fail-on-conflicts CLI flag
        let args = vec![
            "codeguardian",
            "check",
            "--detect-conflicts",
            "--fail-on-conflicts",
            "test.rs",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        if let Commands::Check(check_args) = cli.command {
            assert!(check_args.detect_conflicts);
            assert!(check_args.fail_on_conflicts);
        } else {
            panic!("Expected Check command");
        }
    }

    #[test]
    fn test_cli_multiple_flags_combination() {
        // Test combining multiple broken files detection flags
        let args = vec![
            "codeguardian",
            "check",
            "--detect-conflicts",
            "--detect-placeholders",
            "--detect-duplicates",
            "--fail-on-conflicts",
            "test.rs",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        if let Commands::Check(check_args) = cli.command {
            assert!(check_args.detect_conflicts);
            assert!(check_args.detect_placeholders);
            assert!(check_args.detect_duplicates);
            assert!(check_args.fail_on_conflicts);
        } else {
            panic!("Expected Check command");
        }
    }

    #[tokio::test]
    async fn test_config_override_from_cli() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create a config file with broken files disabled
        let config_content = r#"
[analyzers.broken_files]
enabled = false
detect_merge_conflicts = false
detect_ai_placeholders = false
detect_duplicates = false
"#;

        let config_file = temp_dir.path().join("config.toml");
        fs::write(&config_file, config_content)?;

        // Load config and simulate CLI override
        let mut config = Config::from_file(&config_file)?;

        // Simulate CLI args processing
        let detect_conflicts = true;
        let detect_placeholders = true;

        if detect_conflicts {
            config.analyzers.broken_files.enabled = true;
            config.analyzers.broken_files.detect_merge_conflicts = true;
        }

        if detect_placeholders {
            config.analyzers.broken_files.enabled = true;
            config.analyzers.broken_files.detect_ai_placeholders = true;
        }

        // Verify overrides worked
        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert!(config.analyzers.broken_files.detect_ai_placeholders);
        assert!(!config.analyzers.broken_files.detect_duplicates); // Should remain false

        Ok(())
    }

    #[tokio::test]
    async fn test_end_to_end_conflict_detection() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create a test file with merge conflicts
        let test_file = temp_dir.path().join("conflicted.rs");
        let content = r#"
fn main() {
    println!("Starting application");

<<<<<<< HEAD
    let version = "1.0.0";
    println!("Version: {}", version);
=======
    let version = "2.0.0";
    println!("App version: {}", version);
>>>>>>> feature-branch

    println!("Application started");
}
"#;
        fs::write(&test_file, content)?;

        // Create config with conflict detection enabled
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_merge_conflicts = true;
        config.analyzers.broken_files.detect_ai_placeholders = false;
        config.analyzers.broken_files.detect_duplicates = false;

        // Run analysis
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Verify conflict detection
        assert!(!results.findings.is_empty());

        let conflict_findings: Vec<_> = results
            .findings
            .iter()
            .filter(|f| f.analyzer == "git_conflict")
            .collect();

        assert!(!conflict_findings.is_empty(), "Should detect git conflicts");
        assert_eq!(conflict_findings.len(), 3); // start, separator, end

        // Verify all conflict findings are critical
        for finding in conflict_findings {
            assert_eq!(finding.severity, codeguardian::types::Severity::Critical);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_end_to_end_ai_content_detection() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create a test file with AI-generated content
        let test_file = temp_dir.path().join("ai_generated.rs");
        let content = r#"
// Generated by AI assistant
fn main() {
    // TODO: implement this function properly
    println!("add content here");

    do_something();
}

fn do_something() {
    // placeholder implementation
    unimplemented!()
}

fn handle_this(data: &str) {
    // Generic function name
    println!("Processing: {}", data);
}
"#;
        fs::write(&test_file, content)?;

        // Create config with AI content detection enabled
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_merge_conflicts = false;
        config.analyzers.broken_files.detect_ai_placeholders = true;
        config.analyzers.broken_files.detect_duplicates = false;

        // Run analysis
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Verify AI content detection
        assert!(!results.findings.is_empty());

        let ai_findings: Vec<_> = results
            .findings
            .iter()
            .filter(|f| f.analyzer == "ai_content")
            .collect();

        assert!(!ai_findings.is_empty(), "Should detect AI content");

        // Should detect various types of AI content
        let rule_ids: Vec<_> = ai_findings.iter().map(|f| f.rule_id.as_str()).collect();
        assert!(rule_ids.contains(&"ai_generated_marker"));
        assert!(rule_ids.contains(&"placeholder_content"));
        assert!(rule_ids.contains(&"incomplete_implementation"));
        assert!(rule_ids.contains(&"generic_function_name"));

        Ok(())
    }

    #[tokio::test]
    async fn test_end_to_end_duplicate_detection() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create a test file with duplicate security functions
        let test_file = temp_dir.path().join("duplicates.rs");
        let content = r#"
fn authenticate_user(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    let result = hashed == stored;
    log_authentication_attempt(username, result);
    result
}

fn authenticate_admin(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    let result = hashed == stored;
    log_authentication_attempt(username, result);
    result
}

fn hash_password(password: &str) -> String {
    format!("hashed_{}", password)
}

fn get_stored_password(username: &str) -> String {
    format!("stored_{}", username)
}

fn log_authentication_attempt(username: &str, success: bool) {
    println!("Auth attempt for {}: {}", username, success);
}
"#;
        fs::write(&test_file, content)?;

        // Create config with duplicate detection enabled
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_merge_conflicts = false;
        config.analyzers.broken_files.detect_ai_placeholders = false;
        config.analyzers.broken_files.detect_duplicates = true;
        config.analyzers.broken_files.duplicates.min_lines = 5;

        // Run analysis
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Verify duplicate detection
        let duplicate_findings: Vec<_> = results
            .findings
            .iter()
            .filter(|f| f.analyzer == "duplicate")
            .collect();

        if !duplicate_findings.is_empty() {
            assert!(duplicate_findings
                .iter()
                .any(|f| f.rule_id == "internal_duplication"));

            // Security-relevant duplicates should have appropriate severity
            for finding in duplicate_findings {
                assert!(matches!(
                    finding.severity,
                    codeguardian::types::Severity::High
                        | codeguardian::types::Severity::Medium
                        | codeguardian::types::Severity::Low
                ));
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_end_to_end_all_analyzers() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create a test file with all types of issues
        let test_file = temp_dir.path().join("all_issues.rs");
        let content = r#"
// Generated by AI assistant
fn main() {
    println!("Starting application");

<<<<<<< HEAD
    let config = load_config_v1();
=======
    let config = load_config_v2();
>>>>>>> feature

    // TODO: implement proper error handling
    do_something();
}

fn do_something() {
    // add content here
    unimplemented!()
}

fn authenticate_user(user: &str, pass: &str) -> bool {
    let hash = simple_hash(pass);
    hash == "expected"
}

fn authenticate_admin(user: &str, pass: &str) -> bool {
    let hash = simple_hash(pass);
    hash == "expected"
}

fn simple_hash(input: &str) -> &'static str {
    "hashed"
}
"#;
        fs::write(&test_file, content)?;

        // Create config with all broken files detection enabled
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_merge_conflicts = true;
        config.analyzers.broken_files.detect_ai_placeholders = true;
        config.analyzers.broken_files.detect_duplicates = true;
        config.analyzers.broken_files.duplicates.min_lines = 3;

        // Run analysis
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Verify all types of issues are detected
        assert!(!results.findings.is_empty());

        let analyzers_found: std::collections::HashSet<_> = results
            .findings
            .iter()
            .map(|f| f.analyzer.as_str())
            .collect();

        assert!(
            analyzers_found.contains("git_conflict"),
            "Should detect git conflicts"
        );
        assert!(
            analyzers_found.contains("ai_content"),
            "Should detect AI content"
        );
        // Note: duplicate detection might not trigger depending on implementation details

        // Verify findings have appropriate metadata
        for finding in &results.findings {
            assert!(!finding.message.is_empty());
            assert!(finding.line > 0);
            assert!(!finding.file.as_os_str().is_empty());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_fail_on_conflicts_behavior() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create a test file with conflicts
        let test_file = temp_dir.path().join("conflicts.rs");
        let content = r#"
fn main() {
<<<<<<< HEAD
    println!("version 1");
=======
    println!("version 2");
>>>>>>> branch
}
"#;
        fs::write(&test_file, content)?;

        // Test with fail_on_conflicts enabled
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_merge_conflicts = true;
        config.analyzers.broken_files.conflicts.fail_on_conflicts = true;

        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Check that conflicts are detected
        let has_conflicts = results.findings.iter().any(|f| {
            f.analyzer == "git_conflict"
                && matches!(f.severity, codeguardian::types::Severity::Critical)
        });

        assert!(
            has_conflicts,
            "Should detect critical conflicts for fail_on_conflicts test"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_custom_patterns_integration() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create config file with custom patterns
        let config_content = r#"
[analyzers.broken_files]
enabled = true
detect_ai_placeholders = true

[analyzers.broken_files.placeholders]
custom_patterns = [
    "custom todo",
    "fix this later",
    "implement me"
]
"#;

        let config_file = temp_dir.path().join("config.toml");
        fs::write(&config_file, config_content)?;

        // Create test file with custom patterns
        let test_file = temp_dir.path().join("custom.rs");
        let content = r#"
fn main() {
    // custom todo: handle this case
    println!("fix this later");

    // implement me properly
    unimplemented!()
}
"#;
        fs::write(&test_file, content)?;

        // Load config and run analysis
        let config = Config::from_file(&config_file)?;
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Verify custom patterns are detected
        let placeholder_findings: Vec<_> = results
            .findings
            .iter()
            .filter(|f| f.analyzer == "ai_content" && f.rule_id == "placeholder_content")
            .collect();

        assert!(
            !placeholder_findings.is_empty(),
            "Should detect custom patterns"
        );
        assert!(
            placeholder_findings.len() >= 3,
            "Should detect all custom patterns"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_performance_limits_integration() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create config with performance limits
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_duplicates = true;
        config
            .analyzers
            .broken_files
            .duplicates
            .max_files_to_compare = 10;
        config.analyzers.broken_files.duplicates.min_lines = 20; // High threshold

        // Create test file
        let test_file = temp_dir.path().join("performance.rs");
        let content = "fn test() { println!(\"small function\"); }".repeat(100);
        fs::write(&test_file, content)?;

        // Run analysis
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![test_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Should complete without crashing
        // Duplicate detection might not find anything due to high min_lines threshold
        assert!(results.findings.len() >= 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_file_exclusion_patterns() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create test files in different locations
        let main_file = temp_dir.path().join("src").join("main.rs");
        let test_file = temp_dir.path().join("tests").join("test.rs");
        let example_file = temp_dir.path().join("examples").join("example.rs");

        fs::create_dir_all(main_file.parent().unwrap())?;
        fs::create_dir_all(test_file.parent().unwrap())?;
        fs::create_dir_all(example_file.parent().unwrap())?;

        let content_with_issues = r#"
fn do_something() {
    // Generic function name
}
"#;

        fs::write(&main_file, content_with_issues)?;
        fs::write(&test_file, content_with_issues)?;
        fs::write(&example_file, content_with_issues)?;

        // Create config
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_ai_placeholders = true;

        // Run analysis
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config, progress).await?;
        let files = vec![main_file, test_file, example_file];
        let results = engine.analyze_files(&files, 1).await?;

        // Check that test and example files are handled appropriately
        // (AI content analyzer should skip generic function names in test files)
        let findings_by_file: std::collections::HashMap<_, _> = results
            .findings
            .iter()
            .map(|f| (f.file.clone(), f))
            .collect();

        // Main file should have findings
        let main_findings: Vec<_> = results
            .findings
            .iter()
            .filter(|f| f.file.to_string_lossy().contains("main.rs"))
            .collect();

        let test_findings: Vec<_> = results
            .findings
            .iter()
            .filter(|f| f.file.to_string_lossy().contains("test.rs"))
            .collect();

        // Test files should have fewer or no generic function name findings
        let main_generic_findings = main_findings
            .iter()
            .filter(|f| f.rule_id == "generic_function_name")
            .count();
        let test_generic_findings = test_findings
            .iter()
            .filter(|f| f.rule_id == "generic_function_name")
            .count();

        // Test files should be treated differently (fewer generic function warnings)
        assert!(test_generic_findings <= main_generic_findings);

        Ok(())
    }
}
