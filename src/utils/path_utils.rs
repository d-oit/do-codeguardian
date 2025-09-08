//! Common path utilities for consistent path handling across CLI commands

use crate::config::Config;
use crate::utils::security::canonicalize_path_safe;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Validate that a file exists and return canonicalized path
pub fn validate_file_path(file_path: &Path, context: &str) -> Result<PathBuf> {
    if !file_path.exists() {
        return Err(anyhow::anyhow!(
            "{} file does not exist: {}",
            context,
            file_path.display()
        ));
    }

    let canonical = canonicalize_path_safe(file_path);
    Ok(canonical)
}

/// Validate that a file has the expected extension
pub fn validate_file_extension(file_path: &Path, expected_ext: &str, context: &str) -> Result<()> {
    if let Some(extension) = file_path.extension() {
        if extension != expected_ext.trim_start_matches('.') {
            tracing::warn!(
                "{} file does not have expected .{} extension: {}",
                context,
                expected_ext,
                file_path.display()
            );
        }
    } else {
        tracing::warn!(
            "{} file has no extension, expected .{}: {}",
            context,
            expected_ext,
            file_path.display()
        );
    }
    Ok(())
}

/// Resolve output path using configured output directory if needed
pub fn resolve_output_path(
    provided_path: &Path,
    default_filename: &str,
    config: &Config,
) -> PathBuf {
    let output_dir = &config.output.directory;

    // If using default path, use configured output directory
    if provided_path == Path::new(default_filename) {
        return PathBuf::from(output_dir).join(default_filename);
    }

    // If relative path and not starting with ./ or ../, use output directory
    if provided_path.is_relative()
        && !provided_path.starts_with("./")
        && !provided_path.starts_with("../")
    {
        return PathBuf::from(output_dir).join(provided_path);
    }

    provided_path.to_path_buf()
}

/// Ensure output directory exists for a given path
pub async fn ensure_output_directory(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    Ok(())
}

/// Resolve input path using configured output directory if needed
pub fn resolve_input_path(
    provided_path: &Path,
    default_filename: &str,
    config: &Config,
) -> PathBuf {
    let output_dir = &config.output.directory;

    // If using default path, use configured output directory
    if provided_path == Path::new(default_filename) {
        return PathBuf::from(output_dir).join(default_filename);
    }

    provided_path.to_path_buf()
}
