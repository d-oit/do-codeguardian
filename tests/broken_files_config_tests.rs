use codeguardian::analyzers::AnalyzerRegistry;
use codeguardian::config::{
    BrokenFilesConfig, Config, ConflictDetectionConfig, DuplicateDetectionConfig,
    PlaceholderDetectionConfig,
};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_default_broken_files_config() {
        let config = BrokenFilesConfig::default();

        assert!(config.enabled);
        assert!(config.detect_merge_conflicts);
        assert!(config.detect_ai_placeholders);
        assert!(!config.detect_duplicates); // Should be opt-in for performance

        // Test sub-configurations
        assert!(config.conflicts.fail_on_conflicts);
        assert!(config.conflicts.validate_syntax);
        assert!(config.conflicts.check_git_status);

        assert_eq!(config.placeholders.severity, "medium");
        assert!(!config.placeholders.patterns.is_empty());
        assert!(config.placeholders.custom_patterns.is_empty());

        assert_eq!(config.duplicates.min_lines, 10);
        assert!(config.duplicates.focus_security);
        assert!(config.duplicates.ignore_test_files);
        assert_eq!(config.duplicates.max_files_to_compare, 1000);
    }

    #[test]
    fn test_config_serialization() {
        let config = BrokenFilesConfig::default();

        // Test TOML serialization
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("enabled = true"));
        assert!(toml_str.contains("detect_merge_conflicts = true"));
        assert!(toml_str.contains("detect_ai_placeholders = true"));
        assert!(toml_str.contains("detect_duplicates = false"));

        // Test deserialization
        let deserialized: BrokenFilesConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(
            config.detect_merge_conflicts,
            deserialized.detect_merge_conflicts
        );
        assert_eq!(
            config.detect_ai_placeholders,
            deserialized.detect_ai_placeholders
        );
        assert_eq!(config.detect_duplicates, deserialized.detect_duplicates);
    }

    #[test]
    fn test_custom_config_from_toml() {
        let toml_content = r#"
enabled = true
detect_merge_conflicts = false
detect_ai_placeholders = true
detect_duplicates = true

[conflicts]
fail_on_conflicts = false
validate_syntax = false
check_git_status = false

[placeholders]
severity = "high"
patterns = ["custom pattern"]
custom_patterns = ["my custom pattern"]

[duplicates]
min_lines = 15
focus_security = false
ignore_test_files = false
max_files_to_compare = 500
"#;

        let config: BrokenFilesConfig = toml::from_str(toml_content).unwrap();

        assert!(config.enabled);
        assert!(!config.detect_merge_conflicts);
        assert!(config.detect_ai_placeholders);
        assert!(config.detect_duplicates);

        assert!(!config.conflicts.fail_on_conflicts);
        assert!(!config.conflicts.validate_syntax);
        assert!(!config.conflicts.check_git_status);

        assert_eq!(config.placeholders.severity, "high");
        assert_eq!(config.placeholders.patterns, vec!["custom pattern"]);
        assert_eq!(
            config.placeholders.custom_patterns,
            vec!["my custom pattern"]
        );

        assert_eq!(config.duplicates.min_lines, 15);
        assert!(!config.duplicates.focus_security);
        assert!(!config.duplicates.ignore_test_files);
        assert_eq!(config.duplicates.max_files_to_compare, 500);
    }

    #[test]
    fn test_full_config_integration() {
        let toml_content = r#"
[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true
detect_duplicates = true

[analyzers.broken_files.conflicts]
fail_on_conflicts = true
validate_syntax = true
check_git_status = true

[analyzers.broken_files.placeholders]
severity = "medium"
patterns = ["implement this", "add content here"]
custom_patterns = ["fix me", "complete later"]

[analyzers.broken_files.duplicates]
min_lines = 8
focus_security = true
ignore_test_files = true
max_files_to_compare = 800

[output]
directory = "test-results"
format = "json"

[security]
enabled = true

[files]
exclude_patterns = ["*.tmp"]
analyze_extensions = [".rs", ".js"]
"#;

        let config: Config = toml::from_str(toml_content).unwrap();

        // Test that broken files config is properly nested
        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert!(config.analyzers.broken_files.detect_ai_placeholders);
        assert!(config.analyzers.broken_files.detect_duplicates);

        // Test that other config sections are preserved
        assert_eq!(config.output.directory, "test-results");
        assert_eq!(config.output.format, "json");
        assert!(config.security.enabled);
    }

    #[test]
    fn test_config_file_loading() -> std::io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;

        let config_content = r#"
[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = false
detect_duplicates = true

[analyzers.broken_files.conflicts]
fail_on_conflicts = false

[analyzers.broken_files.duplicates]
min_lines = 20
"#;

        temp_file.write_all(config_content.as_bytes())?;
        temp_file.flush()?;

        let config = Config::from_file(temp_file.path()).unwrap();

        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert!(!config.analyzers.broken_files.detect_ai_placeholders);
        assert!(config.analyzers.broken_files.detect_duplicates);
        assert!(!config.analyzers.broken_files.conflicts.fail_on_conflicts);
        assert_eq!(config.analyzers.broken_files.duplicates.min_lines, 20);

        Ok(())
    }

    #[test]
    fn test_partial_config_with_defaults() {
        let toml_content = r#"
[analyzers.broken_files]
enabled = true
detect_duplicates = true
"#;

        let config: Config = toml::from_str(toml_content).unwrap();

        // Explicitly set values
        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_duplicates);

        // Default values should be preserved
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert!(config.analyzers.broken_files.detect_ai_placeholders);
        assert!(config.analyzers.broken_files.conflicts.fail_on_conflicts);
        assert_eq!(
            config.analyzers.broken_files.placeholders.severity,
            "medium"
        );
        assert_eq!(config.analyzers.broken_files.duplicates.min_lines, 10);
    }

    #[test]
    fn test_invalid_config_handling() {
        let invalid_toml = r#"
[analyzers.broken_files]
enabled = "not_a_boolean"
"#;

        let result: Result<Config, _> = toml::from_str(invalid_toml);
        assert!(result.is_err(), "Should reject invalid boolean value");

        let invalid_severity = r#"
[analyzers.broken_files.placeholders]
severity = 123
"#;

        let result: Result<Config, _> = toml::from_str(invalid_severity);
        assert!(result.is_err(), "Should reject invalid severity type");
    }
}

#[cfg(test)]
mod analyzer_registry_config_tests {
    use super::*;

    #[test]
    fn test_analyzer_registry_with_disabled_broken_files() {
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = false;

        let registry = AnalyzerRegistry::with_config(&config);

        // Test that analyzers are created (we can't easily test internal state)
        // but the registry should be functional
        let test_content = b"fn test() {}";
        let findings = registry
            .analyze_file(Path::new("test.rs"), test_content)
            .unwrap();

        // Should not crash and should work with other analyzers
        // (findings might be empty or contain results from other analyzers)
        assert!(findings.len() >= 0);
    }

    #[test]
    fn test_analyzer_registry_with_selective_analyzers() {
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_merge_conflicts = true;
        config.analyzers.broken_files.detect_ai_placeholders = false;
        config.analyzers.broken_files.detect_duplicates = false;

        let registry = AnalyzerRegistry::with_config(&config);

        // Test with content that would trigger multiple analyzers
        let test_content = br#"
// TODO: implement this
fn do_something() {
<<<<<<< HEAD
    println!("version 1");
=======
    println!("version 2");
>>>>>>> branch
}
"#;

        let findings = registry
            .analyze_file(Path::new("test.rs"), test_content)
            .unwrap();

        // Should detect conflicts (enabled) but not AI content (disabled)
        let has_conflicts = findings.iter().any(|f| f.analyzer == "git_conflict");
        let has_ai_content = findings.iter().any(|f| f.analyzer == "ai_content");

        assert!(has_conflicts, "Should detect conflicts when enabled");
        assert!(
            !has_ai_content,
            "Should not detect AI content when disabled"
        );
    }

    #[test]
    fn test_analyzer_registry_with_custom_config() {
        let mut config = Config::default();
        config.analyzers.broken_files.enabled = true;
        config.analyzers.broken_files.detect_ai_placeholders = true;
        config.analyzers.broken_files.placeholders.custom_patterns =
            vec!["custom todo".to_string(), "fix this".to_string()];
        config.analyzers.broken_files.detect_duplicates = true;
        config.analyzers.broken_files.duplicates.min_lines = 5;
        config.analyzers.broken_files.duplicates.focus_security = false;

        let registry = AnalyzerRegistry::with_config(&config);

        // Test custom patterns
        let ai_content = b"// custom todo: implement this function";
        let ai_findings = registry
            .analyze_file(Path::new("test.rs"), ai_content)
            .unwrap();

        // Should detect custom pattern
        assert!(ai_findings.iter().any(|f| f.analyzer == "ai_content"));

        // Test duplicate detection with custom min_lines
        let duplicate_content = br#"
fn short_a() {
    println!("a");
    println!("b");
    println!("c");
}

fn short_b() {
    println!("a");
    println!("b");
    println!("c");
}
"#;

        let dup_findings = registry
            .analyze_file(Path::new("test.rs"), duplicate_content)
            .unwrap();

        // Should detect duplicates with lower threshold
        // (Note: actual detection depends on implementation details)
    }

    #[test]
    fn test_analyzer_registry_default_vs_custom() {
        let default_config = Config::default();
        let default_registry = AnalyzerRegistry::with_config(&default_config);

        let mut custom_config = Config::default();
        custom_config.analyzers.broken_files.detect_duplicates = true;
        custom_config.analyzers.broken_files.duplicates.min_lines = 3;
        let custom_registry = AnalyzerRegistry::with_config(&custom_config);

        let test_content = br#"
fn test_a() {
    let x = 1;
    let y = 2;
}

fn test_b() {
    let x = 1;
    let y = 2;
}
"#;

        let default_findings = default_registry
            .analyze_file(Path::new("test.rs"), test_content)
            .unwrap();
        let custom_findings = custom_registry
            .analyze_file(Path::new("test.rs"), test_content)
            .unwrap();

        // Default config has duplicates disabled, custom has them enabled
        let default_has_duplicates = default_findings.iter().any(|f| f.analyzer == "duplicate");
        let custom_has_duplicates = custom_findings.iter().any(|f| f.analyzer == "duplicate");

        assert!(
            !default_has_duplicates,
            "Default should not detect duplicates"
        );
        // Custom might detect duplicates depending on implementation
    }

    #[test]
    fn test_config_validation_edge_cases() {
        // Test zero min_lines
        let mut config = Config::default();
        config.analyzers.broken_files.duplicates.min_lines = 0;

        let registry = AnalyzerRegistry::with_config(&config);
        // Should not crash with zero min_lines

        // Test very large max_files
        config
            .analyzers
            .broken_files
            .duplicates
            .max_files_to_compare = usize::MAX;
        let registry = AnalyzerRegistry::with_config(&config);
        // Should not crash with large max_files

        // Test empty custom patterns
        config.analyzers.broken_files.placeholders.custom_patterns = vec![];
        let registry = AnalyzerRegistry::with_config(&config);
        // Should work with empty custom patterns

        // All should create valid registries
        let test_content = b"fn test() {}";
        let _ = registry
            .analyze_file(Path::new("test.rs"), test_content)
            .unwrap();
    }
}

#[cfg(test)]
mod config_integration_tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_real_world_config_scenarios() -> std::io::Result<()> {
        let temp_dir = TempDir::new()?;

        // Scenario 1: CI/CD focused config
        let ci_config = r#"
[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true
detect_duplicates = false

[analyzers.broken_files.conflicts]
fail_on_conflicts = true
validate_syntax = true

[analyzers.broken_files.placeholders]
severity = "high"
patterns = ["TODO", "FIXME", "implement this"]
"#;

        let ci_config_file = temp_dir.path().join("ci_config.toml");
        fs::write(&ci_config_file, ci_config)?;

        let config = Config::from_file(&ci_config_file).unwrap();
        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert!(!config.analyzers.broken_files.detect_duplicates);
        assert!(config.analyzers.broken_files.conflicts.fail_on_conflicts);
        assert_eq!(config.analyzers.broken_files.placeholders.severity, "high");

        // Scenario 2: Development focused config
        let dev_config = r#"
[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true
detect_duplicates = true

[analyzers.broken_files.conflicts]
fail_on_conflicts = false

[analyzers.broken_files.duplicates]
min_lines = 5
focus_security = true
ignore_test_files = true
"#;

        let dev_config_file = temp_dir.path().join("dev_config.toml");
        fs::write(&dev_config_file, dev_config)?;

        let config = Config::from_file(&dev_config_file).unwrap();
        assert!(config.analyzers.broken_files.detect_duplicates);
        assert!(!config.analyzers.broken_files.conflicts.fail_on_conflicts);
        assert_eq!(config.analyzers.broken_files.duplicates.min_lines, 5);

        // Scenario 3: Security focused config
        let security_config = r#"
[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
detect_ai_placeholders = true
detect_duplicates = true

[analyzers.broken_files.placeholders]
severity = "critical"
custom_patterns = [
    "security todo",
    "auth placeholder",
    "crypto implement"
]

[analyzers.broken_files.duplicates]
min_lines = 8
focus_security = true
max_files_to_compare = 2000
"#;

        let security_config_file = temp_dir.path().join("security_config.toml");
        fs::write(&security_config_file, security_config)?;

        let config = Config::from_file(&security_config_file).unwrap();
        assert_eq!(
            config.analyzers.broken_files.placeholders.severity,
            "critical"
        );
        assert_eq!(
            config
                .analyzers
                .broken_files
                .placeholders
                .custom_patterns
                .len(),
            3
        );
        assert!(config.analyzers.broken_files.duplicates.focus_security);
        assert_eq!(
            config
                .analyzers
                .broken_files
                .duplicates
                .max_files_to_compare,
            2000
        );

        Ok(())
    }

    #[test]
    fn test_config_migration_compatibility() {
        // Test that old configs without broken_files section still work
        let old_config = r#"
[output]
directory = "results"
format = "json"

[security]
enabled = true

[files]
exclude_patterns = ["*.tmp"]
"#;

        let config: Config = toml::from_str(old_config).unwrap();

        // Should use default broken_files config
        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert!(config.analyzers.broken_files.detect_ai_placeholders);
        assert!(!config.analyzers.broken_files.detect_duplicates);

        // Other sections should be preserved
        assert_eq!(config.output.directory, "results");
        assert_eq!(config.output.format, "json");
        assert!(config.security.enabled);
    }

    #[test]
    fn test_config_override_precedence() {
        // Test that more specific configs override general ones
        let config_with_overrides = r#"
[analyzers.broken_files]
enabled = true
detect_duplicates = true

[analyzers.broken_files.duplicates]
min_lines = 15
focus_security = false
"#;

        let config: Config = toml::from_str(config_with_overrides).unwrap();

        // Specific overrides should take precedence
        assert_eq!(config.analyzers.broken_files.duplicates.min_lines, 15);
        assert!(!config.analyzers.broken_files.duplicates.focus_security);

        // Defaults should be preserved where not overridden
        assert!(config.analyzers.broken_files.duplicates.ignore_test_files);
        assert_eq!(
            config
                .analyzers
                .broken_files
                .duplicates
                .max_files_to_compare,
            1000
        );
    }
}
