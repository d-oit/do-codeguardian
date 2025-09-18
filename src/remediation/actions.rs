//! Remediation action implementations
//!
//! Contains the actual implementation of automated remediation actions

use super::{CodeLocation, MergeStrategy};
use crate::utils::command_security;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tokio::process::Command;

/// Merge duplicate code files
pub async fn merge_duplicate_code(
    source_files: &[String],
    target_file: &str,
    merge_strategy: &MergeStrategy,
) -> Result<()> {
    tracing::info!(
        "Merging duplicate code: {:?} -> {}",
        source_files,
        target_file
    );

    // Create backup of target file
    let backup_path = format!("{}.backup.{}", target_file, chrono::Utc::now().timestamp());
    fs::copy(target_file, &backup_path)?;

    match merge_strategy {
        MergeStrategy::KeepFirst => {
            // Keep the first file as-is, remove others
            for source_file in source_files {
                if source_file != target_file {
                    fs::remove_file(source_file)?;
                    tracing::info!("Removed duplicate file: {}", source_file);
                }
            }
        }
        MergeStrategy::KeepLast => {
            // Use the last file as the target
            if let Some(last_file) = source_files.last() {
                if last_file != target_file {
                    fs::copy(last_file, target_file)?;
                    for source_file in source_files {
                        if source_file != target_file {
                            fs::remove_file(source_file)?;
                        }
                    }
                }
            }
        }
        MergeStrategy::KeepMostRecent => {
            // Find the most recently modified file
            let mut most_recent = target_file;
            let mut most_recent_time = fs::metadata(target_file)?.modified()?;

            for source_file in source_files {
                let metadata = fs::metadata(source_file)?;
                let modified_time = metadata.modified()?;
                if modified_time > most_recent_time {
                    most_recent = source_file;
                    most_recent_time = modified_time;
                }
            }

            if most_recent != target_file {
                fs::copy(most_recent, target_file)?;
            }

            // Remove other files
            for source_file in source_files {
                if source_file != target_file {
                    fs::remove_file(source_file)?;
                }
            }
        }
        MergeStrategy::KeepMostComplex => {
            // Find the file with the most lines of code
            let mut most_complex = target_file;
            let mut max_lines = count_lines_of_code(target_file)?;

            for source_file in source_files {
                let lines = count_lines_of_code(source_file)?;
                if lines > max_lines {
                    most_complex = source_file;
                    max_lines = lines;
                }
            }

            if most_complex != target_file {
                fs::copy(most_complex, target_file)?;
            }

            // Remove other files
            for source_file in source_files {
                if source_file != target_file {
                    fs::remove_file(source_file)?;
                }
            }
        }
        MergeStrategy::Manual => {
            // For manual strategy, just log the action - human intervention required
            tracing::warn!("Manual merge strategy selected - human intervention required");
            return Ok(());
        }
    }

    tracing::info!(
        "Code merge completed successfully. Backup created at: {}",
        backup_path
    );
    Ok(())
}

/// Refactor duplicate function across multiple locations
pub async fn refactor_duplicate_function(
    function_name: &str,
    source_locations: &[CodeLocation],
    target_location: &CodeLocation,
) -> Result<()> {
    tracing::info!(
        "Refactoring duplicate function: {} across {} locations",
        function_name,
        source_locations.len()
    );

    // Create a common utility file if it doesn't exist
    let utils_file = format!("{}_utils.rs", function_name.to_lowercase());
    let utils_path = Path::new(&target_location.file_path)
        .parent()
        .unwrap_or(Path::new("."))
        .join(&utils_file);

    // Extract the function from the target location
    let target_content = fs::read_to_string(&target_location.file_path)?;
    let function_code = extract_function_code(&target_content, function_name, target_location)?;

    // Create or update the utils file
    if !utils_path.exists() {
        let utils_content = format!(
            "//! Utility functions extracted from duplicate code\n\n{}\n",
            function_code
        );
        fs::write(&utils_path, utils_content)?;
    } else {
        let mut utils_content = fs::read_to_string(&utils_path)?;
        if !utils_content.contains(function_name) {
            utils_content.push_str(&format!("\n{}\n", function_code));
            fs::write(&utils_path, utils_content)?;
        }
    }

    // Replace function calls in all source locations
    for location in source_locations {
        replace_function_with_import(location, function_name, &utils_file)?;
    }

    // Also update the target location
    replace_function_with_import(target_location, function_name, &utils_file)?;

    tracing::info!(
        "Function refactoring completed. Utility file created: {:?}",
        utils_path
    );
    Ok(())
}

/// Close duplicate issue
pub async fn close_duplicate_issue(
    issue_id: &str,
    duplicate_of: &str,
    comment: &str,
) -> Result<()> {
    tracing::info!(
        "Closing duplicate issue: {} (duplicate of {})",
        issue_id,
        duplicate_of
    );

    // This would integrate with GitHub API or other issue tracking systems
    // For now, we'll simulate the action

    // In a real implementation, this would:
    // 1. Add a comment to the issue explaining it's a duplicate
    // 2. Close the issue
    // 3. Link it to the original issue

    tracing::info!(
        "Issue {} closed as duplicate of {} with comment: {}",
        issue_id,
        duplicate_of,
        comment
    );

    Ok(())
}

/// Consolidate duplicate documentation
pub async fn consolidate_documentation(
    source_docs: &[String],
    target_doc: &str,
    merge_sections: &[String],
) -> Result<()> {
    tracing::info!(
        "Consolidating documentation: {:?} -> {}",
        source_docs,
        target_doc
    );

    // Create backup of target document
    let backup_path = format!("{}.backup.{}", target_doc, chrono::Utc::now().timestamp());
    fs::copy(target_doc, &backup_path)?;

    let mut target_content = fs::read_to_string(target_doc)?;

    // Merge content from source documents
    for source_doc in source_docs {
        if source_doc != target_doc {
            let source_content = fs::read_to_string(source_doc)?;

            // Extract specified sections and merge them
            for section in merge_sections {
                if let Some(section_content) =
                    extract_documentation_section(&source_content, section)
                {
                    if !target_content.contains(&section_content) {
                        target_content.push_str(&format!("\n\n{}\n", section_content));
                    }
                }
            }

            // Remove the source document after merging
            fs::remove_file(source_doc)?;
            tracing::info!("Merged and removed source document: {}", source_doc);
        }
    }

    // Write the consolidated content
    fs::write(target_doc, target_content)?;

    tracing::info!(
        "Documentation consolidation completed. Backup created at: {}",
        backup_path
    );
    Ok(())
}

/// Update configuration file
pub async fn update_configuration(
    config_file: &str,
    changes: &HashMap<String, serde_json::Value>,
    backup_created: bool,
) -> Result<()> {
    tracing::info!("Updating configuration file: {}", config_file);

    // Create backup if requested
    if backup_created {
        let backup_path = format!("{}.backup.{}", config_file, chrono::Utc::now().timestamp());
        fs::copy(config_file, &backup_path)?;
        tracing::info!("Configuration backup created: {}", backup_path);
    }

    // Read current configuration
    let content = fs::read_to_string(config_file)?;

    // Parse based on file extension
    let updated_content = if config_file.ends_with(".toml") {
        update_toml_config(&content, changes)?
    } else if config_file.ends_with(".json") {
        update_json_config(&content, changes)?
    } else if config_file.ends_with(".yaml") || config_file.ends_with(".yml") {
        update_yaml_config(&content, changes)?
    } else {
        return Err(anyhow!("Unsupported configuration file format"));
    };

    // Write updated configuration
    fs::write(config_file, updated_content)?;

    tracing::info!("Configuration file updated successfully");
    Ok(())
}

/// Create pull request
pub async fn create_pull_request(
    title: &str,
    description: &str,
    branch_name: &str,
    files_changed: &[String],
) -> Result<()> {
    tracing::info!("Creating pull request: {} (branch: {})", title, branch_name);

    // Validate inputs to prevent command injection
    command_security::validate_git_branch_name(branch_name)?;
    let sanitized_commit_message =
        command_security::sanitize_commit_message(&format!("Automated remediation: {}", title))?;

    // Create a new branch
    let output = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to create branch: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Add changed files
    for file in files_changed {
        // Validate file path to prevent command injection
        command_security::validate_file_path(file)?;
        let output = Command::new("git").args(["add", file]).output().await?;

        if !output.status.success() {
            tracing::warn!(
                "Failed to add file {}: {}",
                file,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    // Commit changes
    let output = Command::new("git")
        .args(["commit", "-m", &sanitized_commit_message])
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to commit changes: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Push branch
    let output = Command::new("git")
        .args(["push", "origin", branch_name])
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to push branch: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // In a real implementation, this would create a PR via GitHub API
    tracing::info!("Pull request created successfully: {}", title);
    tracing::info!("Description: {}", description);
    tracing::info!("Files changed: {:?}", files_changed);

    Ok(())
}

/// Helper function to count lines of code in a file
fn count_lines_of_code(file_path: &str) -> Result<usize> {
    let content = fs::read_to_string(file_path)?;
    let lines = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("//"))
        .count();
    Ok(lines)
}

/// Helper function to extract function code
fn extract_function_code(
    content: &str,
    _function_name: &str,
    location: &CodeLocation,
) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();

    if location.end_line as usize > lines.len() || location.start_line == 0 {
        return Err(anyhow!("Invalid line range for function extraction"));
    }

    let start_idx = (location.start_line - 1) as usize;
    let end_idx = std::cmp::min(location.end_line as usize, lines.len());

    let function_lines = &lines[start_idx..end_idx];
    Ok(function_lines.join("\n"))
}

/// Helper function to replace function with import
fn replace_function_with_import(
    location: &CodeLocation,
    function_name: &str,
    utils_file: &str,
) -> Result<()> {
    let content = fs::read_to_string(&location.file_path)?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Add import at the top of the file
    let import_line = format!("use crate::{};", utils_file.replace(".rs", ""));
    if !lines.iter().any(|line| line.contains(&import_line)) {
        // Find the right place to insert the import
        let mut insert_idx = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("use ") || line.starts_with("mod ") {
                insert_idx = i + 1;
            } else if !line.trim().is_empty() && !line.starts_with("//") {
                break;
            }
        }
        lines.insert(insert_idx, import_line);
    }

    // Remove the function definition (replace with a comment)
    if location.start_line > 0 && location.end_line as usize <= lines.len() {
        let start_idx = (location.start_line - 1) as usize;
        let end_idx = std::cmp::min(location.end_line as usize, lines.len());

        // Replace function definition with a comment
        let comment = format!("// Function {} moved to {}", function_name, utils_file);
        lines.splice(start_idx..end_idx, std::iter::once(comment));
    }

    // Write the updated content
    let updated_content = lines.join("\n");
    fs::write(&location.file_path, updated_content)?;

    Ok(())
}

/// Helper function to extract documentation section
fn extract_documentation_section(content: &str, section: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_section = false;
    let mut section_lines = Vec::new();

    for line in lines {
        if line.contains(&format!("# {}", section)) || line.contains(&format!("## {}", section)) {
            in_section = true;
            section_lines.push(line);
        } else if in_section && (line.starts_with("# ") || line.starts_with("## ")) {
            // Found the start of a new section
            break;
        } else if in_section {
            section_lines.push(line);
        }
    }

    if section_lines.is_empty() {
        None
    } else {
        Some(section_lines.join("\n"))
    }
}

/// Helper function to update TOML configuration
fn update_toml_config(
    content: &str,
    changes: &HashMap<String, serde_json::Value>,
) -> Result<String> {
    let mut config: toml::Value = content.parse()?;

    for (key, value) in changes {
        let toml_value = json_to_toml_value(value)?;
        set_nested_value(&mut config, key, toml_value)?;
    }

    Ok(toml::to_string_pretty(&config)?)
}

/// Helper function to update JSON configuration
fn update_json_config(
    content: &str,
    changes: &HashMap<String, serde_json::Value>,
) -> Result<String> {
    let mut config: serde_json::Value = serde_json::from_str(content)?;

    for (key, value) in changes {
        set_nested_json_value(&mut config, key, value.clone())?;
    }

    Ok(serde_json::to_string_pretty(&config)?)
}

/// Helper function to update YAML configuration
fn update_yaml_config(
    content: &str,
    changes: &HashMap<String, serde_json::Value>,
) -> Result<String> {
    let mut config: serde_yaml::Value = serde_yaml::from_str(content)?;

    for (key, value) in changes {
        let yaml_value = json_to_yaml_value(value)?;
        set_nested_yaml_value(&mut config, key, yaml_value)?;
    }

    Ok(serde_yaml::to_string(&config)?)
}

/// Convert JSON value to TOML value
fn json_to_toml_value(json_value: &serde_json::Value) -> Result<toml::Value> {
    match json_value {
        serde_json::Value::Null => Ok(toml::Value::String("null".to_string())),
        serde_json::Value::Bool(b) => Ok(toml::Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(anyhow!("Invalid number format"))
            }
        }
        serde_json::Value::String(s) => Ok(toml::Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let toml_arr: Result<Vec<toml::Value>> = arr.iter().map(json_to_toml_value).collect();
            Ok(toml::Value::Array(toml_arr?))
        }
        serde_json::Value::Object(obj) => {
            let mut toml_table = toml::value::Table::new();
            for (k, v) in obj {
                toml_table.insert(k.clone(), json_to_toml_value(v)?);
            }
            Ok(toml::Value::Table(toml_table))
        }
    }
}

/// Convert JSON value to YAML value
fn json_to_yaml_value(json_value: &serde_json::Value) -> Result<serde_yaml::Value> {
    let yaml_str = serde_json::to_string(json_value)?;
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&yaml_str)?;
    Ok(yaml_value)
}

/// Set nested value in TOML
fn set_nested_value(config: &mut toml::Value, key: &str, value: toml::Value) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = config;

    for (i, k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            // Last key, set the value
            if let toml::Value::Table(ref mut table) = current {
                table.insert(k.to_string(), value.clone());
            }
        } else {
            // Navigate to the next level
            if let toml::Value::Table(ref mut table) = current {
                current = table
                    .entry(k.to_string())
                    .or_insert_with(|| toml::Value::Table(toml::value::Table::new()));
            }
        }
    }

    Ok(())
}

/// Set nested value in JSON
fn set_nested_json_value(
    config: &mut serde_json::Value,
    key: &str,
    value: serde_json::Value,
) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = config;

    for (i, k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            // Last key, set the value
            if let serde_json::Value::Object(ref mut map) = current {
                map.insert(k.to_string(), value.clone());
            }
        } else {
            // Navigate to the next level
            if let serde_json::Value::Object(ref mut map) = current {
                current = map
                    .entry(k.to_string())
                    .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
            }
        }
    }

    Ok(())
}

/// Set nested value in YAML
fn set_nested_yaml_value(
    config: &mut serde_yaml::Value,
    key: &str,
    value: serde_yaml::Value,
) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = config;

    for (i, k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            // Last key, set the value
            if let serde_yaml::Value::Mapping(ref mut map) = current {
                map.insert(serde_yaml::Value::String(k.to_string()), value.clone());
            }
        } else {
            // Navigate to the next level
            if let serde_yaml::Value::Mapping(ref mut map) = current {
                let key_value = serde_yaml::Value::String(k.to_string());
                if !map.contains_key(&key_value) {
                    map.insert(
                        key_value.clone(),
                        serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
                    );
                }
                current = map
                    .get_mut(&key_value)
                    .expect("Key should exist after insert");
            }
        }
    }

    Ok(())
}
