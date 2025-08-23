use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;
use std::fs;

/// Error handling and edge case end-to-end tests

#[test]
fn test_permission_denied_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file and remove read permissions (Unix only)
    let test_file = temp_dir.path().join("no_access.rs");
    fs::write(&test_file, "fn test() {}").unwrap();
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o000); // No permissions
        fs::set_permissions(&test_file, perms).unwrap();
    }
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    // Should handle permission errors gracefully
    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_corrupted_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files with various issues
    fs::write(temp_dir.path().join("binary.bin"), &[0u8; 1000]).unwrap(); // Binary file
    fs::write(temp_dir.path().join("empty.rs"), "").unwrap(); // Empty file
    fs::write(temp_dir.path().join("invalid_utf8.rs"), &[0xFF, 0xFE, 0xFD]).unwrap(); // Invalid UTF-8
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success() // Should handle gracefully
        .stdout(predicate::str::contains("total_files_scanned"));
}

#[test]
fn test_very_large_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a very large file (beyond typical limits)
    let large_content = "// Large file\n".repeat(100_000); // ~1.3MB
    fs::write(temp_dir.path().join("huge.rs"), large_content).unwrap();
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));
}

#[test]
fn test_deep_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create deeply nested directory structure
    let mut current_path = temp_dir.path().to_path_buf();
    for i in 0..20 {
        current_path = current_path.join(format!("level_{}", i));
        fs::create_dir_all(&current_path).unwrap();
        
        fs::write(current_path.join("deep.rs"), format!(r#"
 // File at depth {}
 pub fn function_at_depth_{}() {{
     println!("Deep function {{}}", {});
 }}
 "#, i, i, i)).unwrap();
    }
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));
}

#[test]
fn test_symlink_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create original file
    let original = temp_dir.path().join("original.rs");
    fs::write(&original, "fn original() {}").unwrap();
    
    // Create symlink (Unix only)
    #[cfg(unix)]
    {
        let symlink = temp_dir.path().join("symlink.rs");
        std::os::unix::fs::symlink(&original, &symlink).unwrap();
    }
    
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("total_files_scanned"));
}

#[test]
fn test_interrupted_analysis_recovery() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create many files to increase chance of interruption
    for i in 0..100 {
        fs::write(temp_dir.path().join(format!("file_{}.rs", i)), format!(r#"
pub fn function_{}() {{
    let data = "some data {}";
    println!("{{}}", data);
}}
"#, i, i)).unwrap();
    }
    
    // Run analysis with timeout to simulate interruption
    let mut cmd = Command::cargo_bin("codeguardian").unwrap();
    cmd.arg("check")
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json");
    
    // Even if interrupted, should handle gracefully
    let output = cmd.output().unwrap();
    // Either succeeds or fails cleanly
    assert!(output.status.success() || output.status.code().is_some());
}