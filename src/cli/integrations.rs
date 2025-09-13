//! Integration CLI commands

use crate::config::Config;
use crate::integrations::{IntegrationManager, IntegrationsConfig};
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct IntegrationsArgs {
    /// List all available integrations
    #[arg(long)]
    pub list: bool,

    /// Test integration health
    #[arg(long)]
    pub health_check: Option<String>,

    /// Search for duplicates across all systems
    #[arg(long)]
    pub search_duplicates: Option<String>,

    /// Create issue across multiple systems
    #[arg(long)]
    pub create_issue: bool,

    /// Issue title (required with --create-issue)
    #[arg(long)]
    pub title: Option<String>,

    /// Issue description (required with --create-issue)
    #[arg(long)]
    pub description: Option<String>,

    /// Issue type
    #[arg(long, default_value = "Bug")]
    pub issue_type: String,

    /// Project key for issue creation
    #[arg(long)]
    pub project: Option<String>,

    /// Generate unified report
    #[arg(long)]
    pub generate_report: bool,

    /// Report type
    #[arg(long, default_value = "duplicates")]
    pub report_type: String,

    /// Trigger workflow across systems
    #[arg(long)]
    pub trigger_workflow: Option<String>,

    /// Workflow parameters (JSON format)
    #[arg(long)]
    pub workflow_params: Option<String>,

    /// Initialize integration configuration
    #[arg(long)]
    pub init_config: bool,

    /// Enable specific integration
    #[arg(long)]
    pub enable: Option<String>,

    /// Disable specific integration
    #[arg(long)]
    pub disable: Option<String>,
}

pub async fn run(args: IntegrationsArgs, _config: &Config) -> Result<()> {
    // Load integrations configuration (would be part of main config in real implementation)
    let integrations_config = IntegrationsConfig::default();
    let mut manager = IntegrationManager::new(integrations_config);

    if args.list {
        list_integrations(&manager).await?;
    } else if let Some(system) = args.health_check {
        check_integration_health(&mut manager, &system).await?;
    } else if let Some(query) = args.search_duplicates {
        search_duplicates_across_systems(&mut manager, &query).await?;
    } else if args.create_issue {
        create_issue_across_systems(&mut manager, &args).await?;
    } else if args.generate_report {
        generate_unified_report(&mut manager, &args).await?;
    } else if let Some(ref workflow) = args.trigger_workflow {
        trigger_workflow_across_systems(&mut manager, workflow, &args).await?;
    } else if args.init_config {
        init_integration_config().await?;
    } else if let Some(system) = args.enable {
        enable_integration(&system).await?;
    } else if let Some(system) = args.disable {
        disable_integration(&system).await?;
    } else {
        show_integrations_help();
    }

    Ok(())
}

async fn list_integrations(_manager: &IntegrationManager) -> Result<()> {
    println!("\n🔗 CodeGuardian External System Integrations\n");

    let systems = vec![
        ("jira", "Atlassian Jira", "Issue tracking and project management"),
        ("confluence", "Atlassian Confluence", "Documentation and knowledge management"),
        ("jenkins", "Jenkins", "CI/CD automation and build management"),
        ("gitlab", "GitLab", "DevOps platform with integrated CI/CD"),
        ("bitbucket", "Bitbucket", "Git repository management and CI/CD"),
        ("azure_devops", "Azure DevOps", "Microsoft DevOps platform"),
    ];

    println!("{:<15} {:<20} {:<50} {:<10}", "System", "Name", "Description", "Status");
    println!("{}", "-".repeat(95));

    for (system_id, name, description) in systems {
        let status = "Available"; // In real implementation, check if enabled
        println!("{:<15} {:<20} {:<50} {:<10}", system_id, name, description, status);
    }

    println!("\nUse --health-check <system> to test connectivity");
    println!("Use --enable <system> to enable an integration");
    println!("Use --init-config to create default configuration");

    Ok(())
}

async fn check_integration_health(manager: &mut IntegrationManager, system: &str) -> Result<()> {
    println!("🔍 Checking health for system: {}", system);

    manager.initialize().await?;
    let health_status = manager.get_health_status().await?;

    if let Some(system_health) = health_status.system_health.get(system) {
        let status_icon = match system_health.status {
            crate::integrations::traits::HealthStatus::Healthy => "✅",
            crate::integrations::traits::HealthStatus::Degraded => "⚠️",
            crate::integrations::traits::HealthStatus::Unhealthy => "❌",
        };

        println!("{} System: {}", status_icon, system);
        println!("   Status: {:?}", system_health.status);

        if let Some(response_time) = system_health.response_time_ms {
            println!("   Response Time: {}ms", response_time);
        }

        if !system_health.features_available.is_empty() {
            println!("   Available Features: {}", system_health.features_available.join(", "));
        }

        if let Some(error) = &system_health.last_error {
            println!("   Last Error: {}", error);
        }
    } else {
        println!("❌ System '{}' not found or not configured", system);
    }

    Ok(())
}

async fn search_duplicates_across_systems(manager: &mut IntegrationManager, query: &str) -> Result<()> {
    println!("🔍 Searching for duplicates across all systems: '{}'", query);

    manager.initialize().await?;

    let search_query = crate::integrations::traits::DuplicateSearchQuery {
        title: query.to_string(),
        description: None,
        content: None,
        labels: vec![],
        project_key: None,
        issue_type: None,
        similarity_threshold: 0.7,
        max_results: 10,
        include_closed: false,
    };

    let results = manager.search_duplicates_across_systems(&search_query).await?;

    if results.is_empty() {
        println!("No duplicates found across any system.");
        return Ok(());
    }

    println!("\n📋 Found {} potential duplicates:\n", results.len());
    println!("{:<10} {:<50} {:<15} {:<10}", "System", "Title", "Status", "Score");
    println!("{}", "-".repeat(85));

    for result in results {
        println!("{:<10} {:<50} {:<15} {:<10.2}",
            result.source_system,
            truncate_string(&result.title, 48),
            result.status,
            result.similarity_score
        );
    }

    Ok(())
}

async fn create_issue_across_systems(manager: &mut IntegrationManager, args: &IntegrationsArgs) -> Result<()> {
    let title = args.title.as_ref().ok_or_else(|| anyhow::anyhow!("Title is required for issue creation"))?;
    let description = args.description.as_ref().ok_or_else(|| anyhow::anyhow!("Description is required for issue creation"))?;

    println!("📝 Creating issue across all systems: '{}'", title);

    manager.initialize().await?;

    let issue_request = crate::integrations::traits::IssueCreationRequest {
        title: title.clone(),
        description: description.clone(),
        issue_type: args.issue_type.clone(),
        priority: crate::integrations::traits::IssuePriority::Medium,
        labels: vec!["codeguardian".to_string(), "automated".to_string()],
        assignee: None,
        project_key: args.project.clone(),
        parent_issue: None,
        custom_fields: std::collections::HashMap::new(),
    };

    let results = manager.create_issue_across_systems(&issue_request).await?;

    println!("\n📋 Issue Creation Results:\n");
    println!("{:<15} {:<10} {:<30} {:<50}", "System", "Success", "Issue ID", "URL");
    println!("{}", "-".repeat(105));

    for result in results {
        let success_icon = if result.success { "✅" } else { "❌" };
        let issue_id = result.issue_id.as_deref().unwrap_or("N/A");
        let url = result.issue_url.as_deref().unwrap_or("N/A");

        println!("{} {:<14} {:<10} {:<30} {:<50}",
            success_icon, result.system, result.success, issue_id, truncate_string(url, 48));

        if let Some(error) = &result.error {
            println!("   Error: {}", error);
        }
    }

    Ok(())
}

async fn generate_unified_report(manager: &mut IntegrationManager, args: &IntegrationsArgs) -> Result<()> {
    println!("📊 Generating unified report: {}", args.report_type);

    manager.initialize().await?;

    let report_request = crate::integrations::traits::ReportRequest {
        report_type: args.report_type.clone(),
        start_date: None,
        end_date: None,
        filters: std::collections::HashMap::new(),
        include_details: false,
    };

    let report = manager.generate_unified_report(&report_request).await?;

    println!("\n📊 Unified Report - {}", report.report_type);
    println!("Generated: {}", report.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("\n📈 Summary:");
    println!("  Total Systems: {}", report.summary.total_systems);
    println!("  Total Issues: {}", report.summary.total_issues);
    println!("  Total Duplicates: {}", report.summary.total_duplicates);
    println!("  Duplicate Rate: {:.2}%", report.summary.duplicate_rate);

    if !report.system_reports.is_empty() {
        println!("\n📋 System Breakdown:");
        println!("{:<15} {:<12} {:<12} {:<12}", "System", "Issues", "Duplicates", "Rate %");
        println!("{}", "-".repeat(55));

        for (system, system_report) in &report.system_reports {
            println!("{:<15} {:<12} {:<12} {:<12.2}",
                system,
                system_report.total_issues,
                system_report.duplicates_found,
                system_report.duplicate_rate
            );
        }
    }

    Ok(())
}

async fn trigger_workflow_across_systems(manager: &mut IntegrationManager, workflow: &str, args: &IntegrationsArgs) -> Result<()> {
    println!("🚀 Triggering workflow '{}' across systems", workflow);

    manager.initialize().await?;

    let parameters = if let Some(params_json) = &args.workflow_params {
        serde_json::from_str(params_json)?
    } else {
        std::collections::HashMap::new()
    };

    let workflow_request = crate::integrations::traits::WorkflowTriggerRequest {
        workflow_name: workflow.to_string(),
        branch: None,
        parameters,
        trigger_reason: "CodeGuardian automated trigger".to_string(),
    };

    let results = manager.trigger_workflows(&workflow_request).await?;

    println!("\n🚀 Workflow Trigger Results:\n");
    println!("{:<15} {:<10} {:<30} {:<50}", "System", "Success", "Workflow ID", "URL");
    println!("{}", "-".repeat(105));

    for result in results {
        let success_icon = if result.success { "✅" } else { "❌" };
        let workflow_id = result.workflow_id.as_deref().unwrap_or("N/A");
        let url = result.workflow_url.as_deref().unwrap_or("N/A");

        println!("{} {:<14} {:<10} {:<30} {:<50}",
            success_icon, result.system, result.success, workflow_id, truncate_string(url, 48));

        if let Some(error) = &result.error {
            println!("   Error: {}", error);
        }
    }

    Ok(())
}

async fn init_integration_config() -> Result<()> {
    println!("🔧 Initializing integration configuration...");

    let config = IntegrationsConfig::default();
    let config_toml = toml::to_string_pretty(&config)?;

    println!("\n📝 Default integration configuration:\n");
    println!("{}", config_toml);

    println!("\n💡 To enable integrations:");
    println!("1. Add this configuration to your codeguardian.toml file");
    println!("2. Update the authentication credentials for each system");
    println!("3. Set enabled = true for the systems you want to use");
    println!("4. Use --enable <system> to enable specific integrations");

    Ok(())
}

async fn enable_integration(system: &str) -> Result<()> {
    println!("✅ Enabling integration: {}", system);
    println!("💡 Update your configuration file to set {}.enabled = true", system);
    Ok(())
}

async fn disable_integration(system: &str) -> Result<()> {
    println!("❌ Disabling integration: {}", system);
    println!("💡 Update your configuration file to set {}.enabled = false", system);
    Ok(())
}

fn show_integrations_help() {
    println!("\n🔗 CodeGuardian External System Integrations\n");
    println!("Available commands:");
    println!("  --list                       List all available integrations");
    println!("  --health-check <system>      Test integration connectivity");
    println!("  --search-duplicates <query>  Search for duplicates across all systems");
    println!("  --create-issue               Create issue across multiple systems");
    println!("    --title <title>            Issue title (required)");
    println!("    --description <desc>       Issue description (required)");
    println!("    --project <key>            Project key for issue creation");
    println!("  --generate-report            Generate unified report");
    println!("    --report-type <type>       Report type (default: duplicates)");
    println!("  --trigger-workflow <name>    Trigger workflow across systems");
    println!("    --workflow-params <json>   Workflow parameters in JSON format");
    println!("  --init-config                Initialize integration configuration");
    println!("  --enable <system>            Enable specific integration");
    println!("  --disable <system>           Disable specific integration");
    println!("\nSupported systems: jira, confluence, jenkins, gitlab, bitbucket, azure_devops");
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
