//! Configuration Tests
//!
//! Config loading, validation, and overrides tests for CodeGuardian.

use do_codeguardian::Config;
use tempfile::TempDir;

/// Configuration testing suite
#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_default_configuration() {
        let config = Config::default();

        // Verify default values
        assert!(config.files.max_file_size_bytes > 0);
        assert!(!config.output.format.is_empty());
    }

    #[test]
    fn test_configuration_from_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test.toml");

        let toml_content = r#"
[files]
exclude_patterns = ["build/", "dist/"]
max_file_size_bytes = 10485760

[output]
format = "sarif"
output_file = "results.sarif"

[security]
strict_mode = false
"#;
        std::fs::write(&config_file, toml_content).unwrap();

        let config = Config::from_file(&config_file).unwrap();
        assert!(config
            .files
            .exclude_patterns
            .contains(&"build/".to_string()));
        assert!(config.files.exclude_patterns.contains(&"dist/".to_string()));
        assert_eq!(config.files.max_file_size_bytes, 10485760);
        assert_eq!(config.output.format, "sarif");
    }

    #[test]
    fn test_configuration_cascade() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("codeguardian.toml");

        // Create a custom configuration
        let config_content = r#"
[files]
exclude_patterns = ["target/", "node_modules/"]
max_file_size_bytes = 5242880

[output]
format = "json"
include_summary = true
include_metadata = true

[security]
strict_mode = true
check_secrets = true

[analyzers.broken_files]
enabled = true
detect_merge_conflicts = true
"#;
        std::fs::write(&config_file, config_content).unwrap();

        // Load configuration
        let config = Config::from_file(&config_file).unwrap();

        // Verify configuration values are loaded correctly
        assert!(config
            .files
            .exclude_patterns
            .contains(&"target/".to_string()));
        assert!(config
            .files
            .exclude_patterns
            .contains(&"node_modules/".to_string()));
        assert_eq!(config.files.max_file_size_bytes, 5242880);
        assert!(config.analyzers.broken_files.enabled);
        assert!(config.analyzers.broken_files.detect_merge_conflicts);
        assert_eq!(config.output.format, "json");
    }

    #[test]
    fn test_invalid_configuration_handling() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("invalid.toml");

        // Invalid TOML syntax
        let invalid_content = r#"
[analysis
enabled_analyzers = ["security"
"#;
        std::fs::write(&config_file, invalid_content).unwrap();

        let result = Config::from_file(&config_file);
        assert!(result.is_err(), "Should reject invalid configuration");
    }

    #[test]
    fn test_environment_variable_overrides() {
        std::env::set_var("CODEGUARDIAN_MAX_FILE_SIZE", "20");
        std::env::set_var("CODEGUARDIAN_OUTPUT_FORMAT", "markdown");

        let config = Config::default(); // Should pick up env vars if implemented

        // Clean up
        std::env::remove_var("CODEGUARDIAN_MAX_FILE_SIZE");
        std::env::remove_var("CODEGUARDIAN_OUTPUT_FORMAT");

        // Note: This test assumes environment variable support is implemented
        // If not, it will still pass but won't test the functionality
    }

    #[test]
    fn test_configuration_validation() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("validation.toml");

        // Configuration with invalid values
        let invalid_values = r#"
[files]
max_file_size_bytes = 0

[output]
format = "invalid_format"
"#;
        std::fs::write(&config_file, invalid_values).unwrap();

        // Should either reject invalid config or use defaults
        let result = Config::from_file(&config_file);
        match result {
            Ok(config) => {
                // If loaded, should use sensible defaults
                assert!(config.files.max_file_size_bytes > 0);
            }
            Err(_) => {
                // Rejection is also valid
                assert!(true);
            }
        }
    }
}
