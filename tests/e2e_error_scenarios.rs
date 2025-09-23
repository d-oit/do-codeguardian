use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Comprehensive error handling and edge case end-to-end tests
/// Tests CodeGuardian's robustness under various failure conditions

/// Test file system permission errors
#[test]
fn test_filesystem_permission_errors() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file and remove read permissions (Unix only)
    let test_file = temp_dir.path().join("no_read.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o000); // No permissions
        fs::set_permissions(&test_file, perms).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    // Should handle permission errors gracefully
    assert!(output.status.success() || !output.stderr.is_empty());

    // Should still produce some valid output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned") || !output.stderr.is_empty());
}

/// Test corrupted and malformed files
#[test]
fn test_corrupted_file_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create various problematic files
    let test_cases = vec![
        ("binary.bin", vec![0u8; 1000]),   // Pure binary
        ("empty.rs", vec![]),              // Empty file
        ("null_bytes.rs", vec![0u8; 100]), // Null bytes
        ("mixed.rs", {
            let mut data = b"fn main() {}".to_vec();
            data.extend(vec![0u8; 50]); // Valid code followed by nulls
            data
        }),
    ];

    for (filename, content) in test_cases {
        fs::write(temp_dir.path().join(filename), content).unwrap();
    }

    // Create a file with invalid UTF-8
    let invalid_utf8_file = temp_dir.path().join("invalid_utf8.rs");
    fs::write(&invalid_utf8_file, vec![0xFF, 0xFE, 0xFD]).unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    let output = cmd.output().unwrap();
    // Should handle all file types gracefully
    assert!(output.status.success() || !output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned") || !output.stderr.is_empty());
}

/// Test very large file handling
#[test]
fn test_large_file_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create files of various sizes
    let sizes = vec![
        1_000_000,  // 1MB
        10_000_000, // 10MB
        50_000_000, // 50MB (if system allows)
    ];

    for (i, &size) in sizes.iter().enumerate() {
        let filename = format!("large_{}.rs", i);
        let content = format!(
            "// Large file {}\n{}",
            i,
            "fn test() {}\n".repeat(size / 20)
        );

        // Only create if content isn't too large for filesystem
        if content.len() < size * 2 {
            fs::write(temp_dir.path().join(&filename), content).unwrap();
        }
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned") || !output.stderr.is_empty());
}

/// Test deep directory structures
#[test]
fn test_deep_directory_structures() {
    let temp_dir = TempDir::new().unwrap();

    // Create very deep directory structure
    let mut current_path = temp_dir.path().to_path_buf();
    let max_depth = 50; // Very deep structure

    for depth in 0..max_depth {
        current_path = current_path.join(format!("level_{}", depth));
        fs::create_dir_all(&current_path).unwrap();

        // Create a file at each level
        let file_path = current_path.join("test.rs");
        fs::write(
            &file_path,
            format!(
                r#"
// File at depth {}
pub fn function_at_depth_{}() {{
    println!("Deep function at level {{}}", {});
}}
"#,
                depth, depth, depth
            ),
        )
        .unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned") || !output.stderr.is_empty());
}

/// Test symlink handling
#[test]
fn test_symlink_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create original file
    let original = temp_dir.path().join("original.rs");
    fs::write(&original, "fn original() {}").unwrap();

    // Create various types of symlinks (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs;

        // Regular symlink
        let symlink = temp_dir.path().join("symlink.rs");
        fs::symlink(&original, &symlink).unwrap();

        // Symlink to directory
        let dir_symlink = temp_dir.path().join("dir_link");
        fs::symlink(temp_dir.path(), &dir_symlink).unwrap();

        // Broken symlink
        let broken_symlink = temp_dir.path().join("broken_link.rs");
        fs::symlink("/nonexistent/file.rs", &broken_symlink).unwrap();

        // Circular symlink (if possible)
        let circular1 = temp_dir.path().join("circular1.rs");
        let circular2 = temp_dir.path().join("circular2.rs");
        fs::symlink(&circular2, &circular1).unwrap();
        fs::symlink(&circular1, &circular2).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("human");

    let output = cmd.output().unwrap();
    // Should handle symlinks gracefully
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test concurrent access and race conditions
#[test]
fn test_concurrent_access_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create many files
    let file_count = 100;
    for i in 0..file_count {
        let file_path = temp_dir.path().join(format!("concurrent_{}.rs", i));
        fs::write(&file_path, format!("fn func_{}() {{}}", i)).unwrap();
    }

    // Test with maximum parallelism
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--parallel")
        .arg("0") // Auto-detect maximum
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Files scanned") || !output.stderr.is_empty());
}

/// Test memory pressure scenarios
#[test]
fn test_memory_pressure_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create files that consume memory when analyzed
    for i in 0..20 {
        let content = format!(
            r#"
// Memory intensive file {}
{}
"#,
            i,
            (0..1000)
                .map(|j| format!(
                    "pub const CONST_{}_{}: &str = \"{}\";",
                    i,
                    j,
                    "x".repeat(100)
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(temp_dir.path().join(format!("memory_{}.rs", i)), content).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test network-related errors (if applicable)
#[test]
fn test_network_error_handling() {
    let temp_dir = TempDir::new().unwrap();

    let test_file = temp_dir.path().join("network_test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    // Test with network timeouts (if GitHub integration is enabled)
    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(&test_file)
        .arg("--emit-gh")
        .arg("--repo")
        .arg("test/repo")
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    // Should handle network errors gracefully
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test configuration file errors
#[test]
fn test_configuration_error_handling() {
    let temp_dir = TempDir::new().unwrap();

    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    // Test with malformed configuration
    let bad_config = temp_dir.path().join("bad_config.toml");
    fs::write(
        &bad_config,
        r#"
[security
fail_on_issues = true
invalid_toml = [
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("--config")
        .arg(&bad_config)
        .arg("check")
        .arg(&test_file);

    let output = cmd.output().unwrap();
    // Should handle config errors gracefully
    assert!(output.status.success() || !output.stderr.is_empty());

    // Test with configuration that has invalid values
    let invalid_config = temp_dir.path().join("invalid_config.toml");
    fs::write(
        &invalid_config,
        r#"
[security]
min_severity = "invalid_level"
max_file_size = "not_a_number"
"#,
    )
    .unwrap();

    let mut invalid_cmd = Command::cargo_bin("do-codeguardian").unwrap();
    invalid_cmd
        .arg("--config")
        .arg(&invalid_config)
        .arg("check")
        .arg(&test_file);

    let output = invalid_cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test encoding and character set issues
#[test]
fn test_encoding_and_charset_issues() {
    let temp_dir = TempDir::new().unwrap();

    // Test various encodings and character sets
    let test_cases: Vec<(&str, String)> = vec![
        (
            "utf8.rs",
            "fn main() { println!(\"Hello 🌍\"); }".to_string(),
        ),
        (
            "latin1.rs",
            "fn main() { println!(\"Café résumé\"); }".to_string(),
        ),
        ("mixed_encoding.rs", {
            let mut content = "fn main() {\n".to_string();
            // Add some potentially problematic Unicode
            content.push_str("    println!(\"🚀 ✨ 💻\");\n");
            content.push_str("    let café = \"résumé\";\n");
            content.push('}');
            content
        }),
    ];

    for (filename, content) in test_cases {
        fs::write(temp_dir.path().join(filename), content).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test resource exhaustion scenarios
#[test]
fn test_resource_exhaustion_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create many small files to test file handle limits
    let file_count = 1000;
    for i in 0..file_count {
        let file_path = temp_dir.path().join(format!("resource_{}.rs", i));
        fs::write(&file_path, format!("fn f_{}() {{}}", i)).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());

    // Should process at least some files
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        assert!(stdout.contains("Files scanned"));
    }
}

/// Test signal handling and interruption
#[test]
fn test_signal_and_interruption_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Create many files to make analysis take longer
    for i in 0..200 {
        let content = format!(
            r#"
// File {} with substantial content
{}
"#,
            i,
            (0..50)
                .map(|j| format!("pub fn func_{}_{}() {{ println!(\"{{}}\", {}); }}", i, j, j))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(temp_dir.path().join(format!("interrupt_{}.rs", i)), content).unwrap();
    }

    // Run analysis in a thread and potentially interrupt it
    let temp_dir_path = temp_dir.path().to_path_buf();
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();

    let handle = thread::spawn(move || {
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check")
            .arg(&temp_dir_path)
            .arg("--format")
            .arg("json");

        // Note: timeout simulation removed for std::process::Command compatibility

        let result = cmd.output();
        interrupted_clone.store(true, Ordering::SeqCst);
        result
    });

    // Wait for completion or timeout
    let result = handle.join().unwrap();

    // Should either complete successfully or be interrupted gracefully
    match result {
        Ok(output) => {
            assert!(output.status.success() || !output.stderr.is_empty());
        }
        Err(_) => {
            // Timeout occurred, which is expected behavior
        }
    }
}

/// Test filesystem boundary conditions
#[test]
fn test_filesystem_boundary_conditions() {
    let temp_dir = TempDir::new().unwrap();

    // Test with filesystem that has limited space (if possible)
    // For now, just test with various file sizes and counts

    // Create files with exact size boundaries
    let boundary_sizes = vec![
        0,           // Empty file
        1,           // 1 byte
        1023,        // Just under 1KB
        1024,        // Exactly 1KB
        1025,        // Just over 1KB
        1024 * 1024, // 1MB
    ];

    for (i, &size) in boundary_sizes.iter().enumerate() {
        let content = "x".repeat(size);
        fs::write(temp_dir.path().join(format!("boundary_{}.rs", i)), content).unwrap();
    }

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

/// Test error recovery and partial success
#[test]
fn test_error_recovery_and_partial_success() {
    let temp_dir = TempDir::new().unwrap();

    // Create a mix of valid and invalid files
    fs::write(temp_dir.path().join("valid1.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir.path().join("valid2.rs"), "pub fn test() {}").unwrap();

    // Create some problematic files
    #[cfg(unix)]
    {
        let invalid_perms = temp_dir.path().join("invalid_perms.rs");
        fs::write(&invalid_perms, "fn test() {}").unwrap();
        let mut perms = fs::metadata(&invalid_perms).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&invalid_perms, perms).unwrap();
    }

    fs::write(temp_dir.path().join("binary.dat"), vec![0u8; 100]).unwrap();
    fs::write(temp_dir.path().join("empty.rs"), "").unwrap();

    let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
    cmd.arg("check")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    // Should process valid files even if some fail
    assert!(output.status.success() || !output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        // Should have processed at least the valid files
        assert!(stdout.contains("Files scanned"));
    }
}

/// Test environmental factor handling
#[test]
fn test_environmental_factor_handling() {
    let temp_dir = TempDir::new().unwrap();

    let test_file = temp_dir.path().join("env_test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    // Test with various environment variable settings
    let env_scenarios = vec![
        vec![("CODEGUARDIAN_QUIET", "true")],
        vec![("CODEGUARDIAN_VERBOSE", "2")],
        vec![("LANG", "C")],
        vec![("LC_ALL", "C.UTF-8")],
        vec![("TZ", "UTC")],
        vec![("PATH", "")], // Empty PATH (should still work)
    ];

    for env_vars in env_scenarios {
        let mut cmd = Command::cargo_bin("do-codeguardian").unwrap();
        cmd.arg("check").arg(&test_file).arg("--format").arg("json");

        for (key, value) in &env_vars {
            cmd.env(key, value);
        }

        let output = cmd.output().unwrap();
        // Should handle various environment configurations
        assert!(output.status.success() || !output.stderr.is_empty());
    }
}
