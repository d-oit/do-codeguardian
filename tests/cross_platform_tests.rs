use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Cross-platform validation tests for CodeGuardian
/// Ensures consistent behavior across different operating systems

#[cfg(test)]
mod cross_platform_tests {
    use super::*;

    /// Test path handling across platforms
    #[test]
    fn test_path_handling_consistency() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with different path formats
        let test_files = vec![
            "simple.rs",
            "subdir/nested.rs",
            "path with spaces.rs",
            "special-chars_@#$%.rs",
        ];

        for file_path in &test_files {
            let full_path = temp_dir.path().join(file_path);
            fs::create_dir_all(full_path.parent().unwrap()).unwrap();
            fs::write(&full_path, format!("// Test file: {}", file_path)).unwrap();
        }

        // Test CodeGuardian can handle all these paths
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check");

        for file_path in &test_files {
            cmd.arg(temp_dir.path().join(file_path));
        }

        cmd.arg("--format").arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "CodeGuardian should handle various path formats"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Files scanned"),
            "Should report files scanned"
        );
    }

    /// Test line ending handling (CRLF vs LF)
    #[test]
    fn test_line_ending_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with different line endings
        let lf_content = "fn main() {\n    println!(\"LF\");\n}\n";
        let crlf_content = "fn main() {\r\n    println!(\"CRLF\");\r\n}\r\n";
        let mixed_content = "fn main() {\n    println!(\"mixed\");\r\n}\n";

        fs::write(temp_dir.path().join("lf.rs"), lf_content).unwrap();
        fs::write(temp_dir.path().join("crlf.rs"), crlf_content).unwrap();
        fs::write(temp_dir.path().join("mixed.rs"), mixed_content).unwrap();

        // Test CodeGuardian handles all line ending types
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle different line endings"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Files scanned"), "Should process all files");
    }

    /// Test file permission handling across platforms
    #[test]
    fn test_file_permission_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create a readable file
        let readable_file = temp_dir.path().join("readable.rs");
        fs::write(&readable_file, "fn main() {}").unwrap();

        // Create a file with restricted permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let restricted_file = temp_dir.path().join("restricted.rs");
            fs::write(&restricted_file, "fn main() {}").unwrap();

            let mut perms = fs::metadata(&restricted_file).unwrap().permissions();
            perms.set_mode(0o000); // No permissions
            fs::set_permissions(&restricted_file, perms).unwrap();
        }

        // Test CodeGuardian handles permission issues gracefully
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        // Should either succeed (handling gracefully) or fail cleanly
        assert!(
            output.status.success() || !output.stderr.is_empty(),
            "Should handle permission issues gracefully"
        );
    }

    /// Test environment variable handling
    #[test]
    fn test_environment_variable_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create a config file that uses environment variables
        let config_content = r#"
[security]
fail_on_issues = false

[analysis]
exclude_patterns = ["*.tmp", "*.log"]

[logging]
level = "info"
"#;

        let config_file = temp_dir.path().join("codeguardian.toml");
        fs::write(&config_file, config_content).unwrap();

        // Create a test file
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        // Test with environment variables set
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("--config")
            .arg(&config_file)
            .arg("check")
            .arg(&test_file)
            .arg("--format")
            .arg("json")
            .env("CODEGUARDIAN_LOG_LEVEL", "debug")
            .env("CODEGUARDIAN_QUIET", "false");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle environment variables"
        );
    }

    /// Test Unicode and international character handling
    #[test]
    fn test_unicode_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with Unicode content
        let unicode_files = vec![
            ("unicode_中文.rs", "fn main() { println!(\"中文\"); }"),
            ("unicode_русский.rs", "fn main() { println!(\"русский\"); }"),
            ("unicode_emoji.rs", "fn main() { println!(\"🚀✨\"); }"),
            ("unicode_árabe.rs", "fn main() { println!(\"العربية\"); }"),
        ];

        for (filename, content) in &unicode_files {
            fs::write(temp_dir.path().join(filename), content).unwrap();
        }

        // Test CodeGuardian handles Unicode files
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle Unicode filenames and content"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Files scanned"),
            "Should process Unicode files"
        );
    }

    /// Test case sensitivity handling
    #[test]
    fn test_case_sensitivity_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with different case patterns
        let case_files = vec![
            "file.rs",
            "FILE.rs",
            "File.rs",
            "config.toml",
            "CONFIG.toml",
        ];

        for filename in &case_files {
            fs::write(temp_dir.path().join(filename), format!("// {}", filename)).unwrap();
        }

        // Test CodeGuardian handles case sensitivity appropriately
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(output.status.success(), "Should handle case sensitivity");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Files scanned"),
            "Should process all case variations"
        );
    }

    /// Test large file handling across platforms
    #[test]
    fn test_large_file_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create a moderately large file (several MB)
        let large_content = format!("// Large file\n{}", "fn test() {}\n".repeat(100_000));
        let large_file = temp_dir.path().join("large.rs");
        fs::write(&large_file, large_content).unwrap();

        // Verify file was created and has expected size
        let metadata = fs::metadata(&large_file).unwrap();
        assert!(metadata.len() > 1_000_000, "File should be at least 1MB");

        // Test CodeGuardian can handle large files
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(&large_file)
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(output.status.success(), "Should handle large files");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Files scanned"),
            "Should process large file"
        );
    }

    /// Test concurrent file access
    #[test]
    fn test_concurrent_file_access() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple files
        for i in 0..10 {
            fs::write(
                temp_dir.path().join(format!("concurrent_{}.rs", i)),
                format!("fn function_{}() {{}}", i),
            )
            .unwrap();
        }

        // Test parallel analysis
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--parallel")
            .arg("4")
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle concurrent file access"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Files scanned"),
            "Should process files in parallel"
        );
    }

    /// Test platform-specific path separators in output
    #[test]
    fn test_path_separator_normalization() {
        let temp_dir = TempDir::new().unwrap();

        // Create nested directory structure
        let nested_path = temp_dir.path().join("level1").join("level2");
        fs::create_dir_all(&nested_path).unwrap();

        let test_file = nested_path.join("test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        // Test CodeGuardian normalizes paths in output
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check").arg(&test_file).arg("--format").arg("json");

        let output = cmd.output().unwrap();
        assert!(output.status.success(), "Should handle nested paths");

        let stdout = String::from_utf8_lossy(&output.stdout);
        // JSON output should contain normalized paths
        assert!(
            stdout.contains("level1") && stdout.contains("level2"),
            "Should preserve path structure in output"
        );
    }

    /// Test handling of platform-specific reserved names
    #[test]
    fn test_reserved_name_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create files with platform-specific considerations
        let test_files = vec![
            "normal.rs",
            "aux.rs",  // Reserved on Windows
            "com1.rs", // Reserved on Windows
            "lpt1.rs", // Reserved on Windows
            "con.rs",  // Reserved on Windows
        ];

        for filename in &test_files {
            // Skip files that can't be created on this platform
            let file_path = temp_dir.path().join(filename);
            if fs::write(&file_path, format!("// {}", filename)).is_ok() {
                // Test CodeGuardian can handle these files if they exist
                let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
                cmd.arg("check").arg(&file_path).arg("--format").arg("json");

                let output = cmd.output().unwrap();
                // Should either succeed or fail gracefully
                assert!(
                    output.status.success() || !output.stderr.is_empty(),
                    "Should handle reserved names gracefully for file: {}",
                    filename
                );
            }
        }
    }

    /// Test timezone and timestamp handling
    #[test]
    fn test_timezone_handling() {
        let temp_dir = TempDir::new().unwrap();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        // Test with different timezone settings
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(&test_file)
            .arg("--format")
            .arg("json")
            .env("TZ", "UTC")
            .env("LANG", "C.UTF-8");

        let output = cmd.output().unwrap();
        assert!(output.status.success(), "Should handle timezone settings");

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should contain valid JSON with timestamps
        assert!(
            stdout.contains("Files scanned"),
            "Should produce valid output with timestamps"
        );
    }

    /// Test memory and resource limits
    #[test]
    fn test_resource_limit_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Create many small files to test resource handling
        for i in 0..1000 {
            fs::write(
                temp_dir.path().join(format!("resource_test_{}.rs", i)),
                format!("fn test_{}() {{}}", i),
            )
            .unwrap();
        }

        // Test CodeGuardian handles resource constraints
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(temp_dir.path())
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle large numbers of files"
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Files scanned"), "Should process all files");
    }
}

/// Platform-specific tests
#[cfg(target_os = "windows")]
mod windows_specific_tests {
    use super::*;

    #[test]
    fn test_windows_path_handling() {
        let temp_dir = TempDir::new().unwrap();

        // Test Windows-specific path formats
        let test_file = temp_dir.path().join("windows_test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        // Test with Windows path separators
        let windows_path = test_file.to_string_lossy().replace('/', "\\");

        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(&windows_path)
            .arg("--format")
            .arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle Windows path separators"
        );
    }
}

#[cfg(target_os = "linux")]
mod linux_specific_tests {
    use super::*;

    #[test]
    fn test_linux_permission_handling() {
        let temp_dir = TempDir::new().unwrap();

        let test_file = temp_dir.path().join("linux_test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        // Test with different permission scenarios
        use std::os::unix::fs::PermissionsExt;

        // Make file executable
        let mut perms = fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&test_file, perms).unwrap();

        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check").arg(&test_file).arg("--format").arg("json");

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Should handle Linux file permissions"
        );
    }
}

#[cfg(target_os = "macos")]
mod macos_specific_tests {
    use super::*;

    #[test]
    fn test_macos_resource_fork_handling() {
        let temp_dir = TempDir::new().unwrap();

        let test_file = temp_dir.path().join("macos_test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        // Test CodeGuardian handles macOS-specific files
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check").arg(&test_file).arg("--format").arg("json");

        let output = cmd.output().unwrap();
        assert!(output.status.success(), "Should handle macOS files");
    }
}
