use crate::config::checklist::{
    ChecklistPriority, ChecklistRule, ChecklistRuleType, SecurityChecklist,
};
use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use std::time::SystemTime;

/// Checklist management commands
#[derive(Debug, Args)]
pub struct ChecklistArgs {
    #[command(subcommand)]
    pub command: ChecklistCommand,
}

#[derive(Debug, Subcommand)]
pub enum ChecklistCommand {
    /// Initialize a new security checklist
    Init {
        /// Output file path for the checklist
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        output: PathBuf,
        /// Include default security rules
        #[arg(long, default_value = "true")]
        include_defaults: bool,
    },
    /// Validate checklist against repository requirements
    Validate {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
        /// Repository path to validate against
        #[arg(short, long, default_value = ".")]
        repository: PathBuf,
        /// Fail on validation errors
        #[arg(long)]
        fail_on_error: bool,
    },
    /// Synchronize checklist with remote repository
    Sync {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
        /// Remote repository URL
        #[arg(short, long)]
        remote: Option<String>,
        /// Force synchronization even with conflicts
        #[arg(long)]
        force: bool,
        /// Dry run - show what would be changed
        #[arg(long)]
        dry_run: bool,
    },
    /// Add a new security rule
    AddRule {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
        /// Rule ID (unique identifier)
        #[arg(long)]
        id: String,
        /// Rule name
        #[arg(long)]
        name: String,
        /// Rule description
        #[arg(long)]
        description: String,
        /// Rule category
        #[arg(long)]
        category: String,
        /// Rule priority (critical, high, medium, low, info)
        #[arg(long, default_value = "medium")]
        priority: String,
        /// Validation pattern (regex)
        #[arg(long)]
        pattern: Option<String>,
        /// Applicable file types (comma-separated)
        #[arg(long)]
        file_types: Option<String>,
        /// Rule tags (comma-separated)
        #[arg(long)]
        tags: Option<String>,
    },
    /// Remove a security rule
    RemoveRule {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
        /// Rule ID to remove
        #[arg(long)]
        id: String,
    },
    /// List all rules or rules by category
    List {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
        /// Filter by category
        #[arg(long)]
        category: Option<String>,
        /// Filter by priority
        #[arg(long)]
        priority: Option<String>,
        /// Show only enabled rules
        #[arg(long)]
        enabled_only: bool,
        /// Output format (table, json, yaml)
        #[arg(long, default_value = "table")]
        format: String,
    },
    /// Show checklist statistics
    Stats {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
    },
    /// Export checklist to different formats
    Export {
        /// Path to checklist file
        #[arg(short, long, default_value = ".codeguardian/checklist.toml")]
        checklist: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        /// Export format (toml, json, yaml, markdown)
        #[arg(long, default_value = "json")]
        format: String,
    },
}

pub async fn handle_checklist_command(args: ChecklistArgs) -> Result<()> {
    match args.command {
        ChecklistCommand::Init {
            output,
            include_defaults,
        } => init_checklist(output, include_defaults).await,
        ChecklistCommand::Validate {
            checklist,
            repository,
            fail_on_error,
        } => validate_checklist(checklist, repository, fail_on_error).await,
        ChecklistCommand::Sync {
            checklist,
            remote,
            force,
            dry_run,
        } => sync_checklist(checklist, remote, force, dry_run).await,
        ChecklistCommand::AddRule {
            checklist,
            id,
            name,
            description,
            category,
            priority,
            pattern,
            file_types,
            tags,
        } => {
            add_rule(
                checklist,
                id,
                name,
                description,
                category,
                priority,
                pattern,
                file_types,
                tags,
            )
            .await
        }
        ChecklistCommand::RemoveRule { checklist, id } => remove_rule(checklist, id).await,
        ChecklistCommand::List {
            checklist,
            category,
            priority,
            enabled_only,
            format,
        } => list_rules(checklist, category, priority, enabled_only, format).await,
        ChecklistCommand::Stats { checklist } => show_stats(checklist).await,
        ChecklistCommand::Export {
            checklist,
            output,
            format,
        } => export_checklist(checklist, output, format).await,
    }
}

async fn init_checklist(output: PathBuf, include_defaults: bool) -> Result<()> {
    println!("Initializing security checklist at: {}", output.display());

    // Create directory if it doesn't exist
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let checklist = if include_defaults {
        SecurityChecklist::new()
    } else {
        SecurityChecklist {
            version: "1.0.0".to_string(),
            last_updated: SystemTime::now(),
            categories: std::collections::HashMap::new(),
            global_rules: Vec::new(),
            repository_specific: std::collections::HashMap::new(),
            synchronization_config: Default::default(),
        }
    };

    checklist.save_to_file(&output)?;

    println!("✅ Security checklist initialized successfully!");
    if include_defaults {
        println!("   - {} categories created", checklist.categories.len());
        println!("   - {} default rules added", checklist.global_rules.len());
    }
    println!("   - Saved to: {}", output.display());

    Ok(())
}

async fn validate_checklist(
    checklist_path: PathBuf,
    repository_path: PathBuf,
    fail_on_error: bool,
) -> Result<()> {
    println!("Validating checklist: {}", checklist_path.display());
    println!("Against repository: {}", repository_path.display());

    let checklist = SecurityChecklist::load_from_file(&checklist_path)?;
    let validation_result = checklist.validate_for_repository(&repository_path)?;

    println!("\n📋 Validation Results:");
    println!(
        "Status: {}",
        if validation_result.valid {
            "✅ VALID"
        } else {
            "❌ INVALID"
        }
    );

    if !validation_result.missing_rules.is_empty() {
        println!("\n🚨 Missing Rules:");
        for rule in &validation_result.missing_rules {
            println!("   - {}", rule);
        }
    }

    if !validation_result.outdated_rules.is_empty() {
        println!("\n⚠️  Outdated Rules:");
        for rule in &validation_result.outdated_rules {
            println!("   - {}", rule);
        }
    }

    if !validation_result.conflicts.is_empty() {
        println!("\n⚡ Conflicts:");
        for conflict in &validation_result.conflicts {
            println!("   - {}", conflict);
        }
    }

    if !validation_result.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for rec in &validation_result.recommendations {
            println!("   - {}", rec);
        }
    }

    if fail_on_error && !validation_result.valid {
        return Err(anyhow!("Checklist validation failed"));
    }

    println!("\n✅ Validation completed");
    Ok(())
}

async fn sync_checklist(
    checklist_path: PathBuf,
    remote: Option<String>,
    _force: bool,
    dry_run: bool,
) -> Result<()> {
    let mut checklist = SecurityChecklist::load_from_file(&checklist_path)?;

    let remote_url = remote.unwrap_or_else(|| {
        checklist
            .synchronization_config
            .central_repository
            .clone()
            .unwrap_or_else(|| "https://github.com/security-checklists/default".to_string())
    });

    println!("Synchronizing checklist with: {}", remote_url);

    if dry_run {
        println!("🔍 DRY RUN - No changes will be made");
    }

    let sync_result = checklist.synchronize_with_remote(&remote_url).await?;

    println!("\n📊 Synchronization Results:");
    println!(
        "Status: {}",
        if sync_result.success {
            "✅ SUCCESS"
        } else {
            "❌ FAILED"
        }
    );
    println!("Changes Applied: {}", sync_result.changes_applied);
    println!("Conflicts Detected: {}", sync_result.conflicts_detected);

    if !sync_result.errors.is_empty() {
        println!("\n🚨 Errors:");
        for error in &sync_result.errors {
            println!("   - {}", error);
        }
    }

    println!("\nSummary: {}", sync_result.summary);

    if !dry_run && sync_result.success {
        checklist.save_to_file(&checklist_path)?;
        println!("✅ Checklist updated and saved");
    }

    Ok(())
}

async fn add_rule(
    checklist_path: PathBuf,
    id: String,
    name: String,
    description: String,
    category: String,
    priority: String,
    pattern: Option<String>,
    file_types: Option<String>,
    tags: Option<String>,
) -> Result<()> {
    let mut checklist = SecurityChecklist::load_from_file(&checklist_path)?;

    let priority = match priority.to_lowercase().as_str() {
        "critical" => ChecklistPriority::Critical,
        "high" => ChecklistPriority::High,
        "medium" => ChecklistPriority::Medium,
        "low" => ChecklistPriority::Low,
        "info" => ChecklistPriority::Info,
        _ => {
            return Err(anyhow!(
                "Invalid priority. Use: critical, high, medium, low, info"
            ))
        }
    };

    let applicable_file_types = file_types
        .map(|ft| ft.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let rule_tags = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let rule = ChecklistRule {
        id: id.clone(),
        name,
        description,
        category,
        priority,
        rule_type: ChecklistRuleType::CodePattern,
        validation_pattern: pattern,
        remediation_steps: Vec::new(),
        applicable_file_types,
        tags: rule_tags,
        enabled: true,
        last_modified: SystemTime::now(),
    };

    checklist.add_or_update_rule(rule)?;
    checklist.save_to_file(&checklist_path)?;

    println!("✅ Rule '{}' added successfully", id);
    Ok(())
}

async fn remove_rule(checklist_path: PathBuf, id: String) -> Result<()> {
    let mut checklist = SecurityChecklist::load_from_file(&checklist_path)?;

    if checklist.remove_rule(&id)? {
        checklist.save_to_file(&checklist_path)?;
        println!("✅ Rule '{}' removed successfully", id);
    } else {
        println!("⚠️  Rule '{}' not found", id);
    }

    Ok(())
}

async fn list_rules(
    checklist_path: PathBuf,
    category: Option<String>,
    priority: Option<String>,
    enabled_only: bool,
    format: String,
) -> Result<()> {
    let checklist = SecurityChecklist::load_from_file(&checklist_path)?;

    let mut rules: Vec<&ChecklistRule> = checklist.global_rules.iter().collect();

    // Apply filters
    if let Some(cat) = &category {
        rules.retain(|rule| rule.category == *cat);
    }

    if let Some(prio) = &priority {
        let priority_filter = match prio.to_lowercase().as_str() {
            "critical" => ChecklistPriority::Critical,
            "high" => ChecklistPriority::High,
            "medium" => ChecklistPriority::Medium,
            "low" => ChecklistPriority::Low,
            "info" => ChecklistPriority::Info,
            _ => return Err(anyhow!("Invalid priority filter")),
        };
        rules.retain(|rule| rule.priority == priority_filter);
    }

    if enabled_only {
        rules.retain(|rule| rule.enabled);
    }

    match format.as_str() {
        "table" => print_rules_table(&rules),
        "json" => print_rules_json(&rules)?,
        "yaml" => print_rules_yaml(&rules)?,
        _ => return Err(anyhow!("Unsupported format. Use: table, json, yaml")),
    }

    Ok(())
}

async fn show_stats(checklist_path: PathBuf) -> Result<()> {
    let checklist = SecurityChecklist::load_from_file(&checklist_path)?;

    println!("📊 Checklist Statistics");
    println!("========================");
    println!("Version: {}", checklist.version);
    println!("Categories: {}", checklist.categories.len());
    println!("Total Rules: {}", checklist.global_rules.len());

    // Count by priority
    let mut priority_counts = std::collections::HashMap::new();
    for rule in &checklist.global_rules {
        *priority_counts.entry(&rule.priority).or_insert(0) += 1;
    }

    println!("\nRules by Priority:");
    for (priority, count) in priority_counts {
        println!("  {:?}: {}", priority, count);
    }

    // Count by category
    let mut category_counts = std::collections::HashMap::new();
    for rule in &checklist.global_rules {
        *category_counts.entry(&rule.category).or_insert(0) += 1;
    }

    println!("\nRules by Category:");
    for (category, count) in category_counts {
        println!("  {}: {}", category, count);
    }

    // Enabled/disabled count
    let enabled_count = checklist.global_rules.iter().filter(|r| r.enabled).count();
    let disabled_count = checklist.global_rules.len() - enabled_count;

    println!("\nRule Status:");
    println!("  Enabled: {}", enabled_count);
    println!("  Disabled: {}", disabled_count);

    Ok(())
}

async fn export_checklist(checklist_path: PathBuf, output: PathBuf, format: String) -> Result<()> {
    let checklist = SecurityChecklist::load_from_file(&checklist_path)?;

    let content = match format.as_str() {
        "toml" => toml::to_string_pretty(&checklist)?,
        "json" => serde_json::to_string_pretty(&checklist)?,
        "yaml" => serde_yaml::to_string(&checklist)?,
        "markdown" => generate_markdown_export(&checklist)?,
        _ => {
            return Err(anyhow!(
                "Unsupported format. Use: toml, json, yaml, markdown"
            ))
        }
    };

    std::fs::write(&output, content)?;
    println!("✅ Checklist exported to: {}", output.display());

    Ok(())
}

fn print_rules_table(rules: &[&ChecklistRule]) {
    println!("📋 Security Rules");
    println!("=================");
    println!(
        "{:<12} {:<30} {:<15} {:<10} {:<8}",
        "ID", "Name", "Category", "Priority", "Enabled"
    );
    println!("{}", "-".repeat(80));

    for rule in rules {
        println!(
            "{:<12} {:<30} {:<15} {:<10} {:<8}",
            rule.id,
            if rule.name.len() > 28 {
                &rule.name[..28]
            } else {
                &rule.name
            },
            rule.category,
            format!("{:?}", rule.priority),
            if rule.enabled { "✅" } else { "❌" }
        );
    }

    println!("\nTotal: {} rules", rules.len());
}

fn print_rules_json(rules: &[&ChecklistRule]) -> Result<()> {
    let json = serde_json::to_string_pretty(rules)?;
    println!("{}", json);
    Ok(())
}

fn print_rules_yaml(rules: &[&ChecklistRule]) -> Result<()> {
    let yaml = serde_yaml::to_string(rules)?;
    println!("{}", yaml);
    Ok(())
}

fn generate_markdown_export(checklist: &SecurityChecklist) -> Result<String> {
    let mut markdown = String::new();

    markdown.push_str(&format!("# Security Checklist v{}\n\n", checklist.version));
    markdown.push_str(&format!("Generated: {:?}\n\n", checklist.last_updated));

    // Categories
    markdown.push_str("## Categories\n\n");
    for (name, category) in &checklist.categories {
        markdown.push_str(&format!("### {}\n", name));
        markdown.push_str(&format!("**Description:** {}\n", category.description));
        markdown.push_str(&format!("**Priority:** {:?}\n\n", category.priority));
    }

    // Rules by category
    markdown.push_str("## Rules\n\n");
    for (category_name, _) in &checklist.categories {
        let category_rules: Vec<_> = checklist
            .global_rules
            .iter()
            .filter(|rule| rule.category == *category_name)
            .collect();

        if !category_rules.is_empty() {
            markdown.push_str(&format!("### {} Rules\n\n", category_name));

            for rule in category_rules {
                markdown.push_str(&format!("#### {} ({})\n", rule.name, rule.id));
                markdown.push_str(&format!("**Priority:** {:?}\n", rule.priority));
                markdown.push_str(&format!("**Description:** {}\n", rule.description));

                if !rule.remediation_steps.is_empty() {
                    markdown.push_str("**Remediation:**\n");
                    for step in &rule.remediation_steps {
                        markdown.push_str(&format!("- {}\n", step));
                    }
                }

                if !rule.tags.is_empty() {
                    markdown.push_str(&format!("**Tags:** {}\n", rule.tags.join(", ")));
                }

                markdown.push_str("\n");
            }
        }
    }

    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_init_checklist() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_checklist.toml");

        let result = init_checklist(output_path.clone(), true).await;
        assert!(result.is_ok());
        assert!(output_path.exists());

        // Verify the checklist was created correctly
        let checklist = SecurityChecklist::load_from_file(&output_path).unwrap();
        assert!(!checklist.categories.is_empty());
        assert!(!checklist.global_rules.is_empty());
    }

    #[tokio::test]
    async fn test_add_and_remove_rule() {
        let temp_dir = TempDir::new().unwrap();
        let checklist_path = temp_dir.path().join("test_checklist.toml");

        // Initialize checklist
        init_checklist(checklist_path.clone(), true).await.unwrap();

        // Add a rule
        let result = add_rule(
            checklist_path.clone(),
            "TEST_001".to_string(),
            "Test Rule".to_string(),
            "Test description".to_string(),
            "authentication".to_string(),
            "high".to_string(),
            Some("test_pattern".to_string()),
            Some("rs,js".to_string()),
            Some("test,security".to_string()),
        )
        .await;
        assert!(result.is_ok());

        // Verify rule was added
        let checklist = SecurityChecklist::load_from_file(&checklist_path).unwrap();
        assert!(checklist.global_rules.iter().any(|r| r.id == "TEST_001"));

        // Remove the rule
        let result = remove_rule(checklist_path.clone(), "TEST_001".to_string()).await;
        assert!(result.is_ok());

        // Verify rule was removed
        let checklist = SecurityChecklist::load_from_file(&checklist_path).unwrap();
        assert!(!checklist.global_rules.iter().any(|r| r.id == "TEST_001"));
    }
}
