//! Build script for CodeGuardian
//!
//! This script generates build-time information such as the current git commit hash
//! and sets up cargo environment variables for use in the application.

use std::process::Command;

fn main() {
    // Generate version information
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("Failed to execute git command: {}", e));

    match output {
        Ok(output) if output.status.success() => {
            let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // Validate hash format (40-character hex string)
            if git_hash.len() == 40 && git_hash.chars().all(|c| c.is_ascii_hexdigit()) {
                println!("cargo:rustc-env=GIT_HASH={}", git_hash);
            } else {
                eprintln!("Warning: Invalid git hash format: {}", git_hash);
            }
        }
        Ok(output) => {
            eprintln!("Warning: Git command failed with status: {}", output.status);
        }
        Err(e) => {
            eprintln!("Warning: {}", e);
        }
    }

    // Only rerun if Cargo.toml or Cargo.lock changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
}
