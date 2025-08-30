use crate::cli::InitArgs;
use crate::config::Config;
use anyhow::Result;
use tokio::fs;

// Constants for configuration templates
const CI_ANALYSIS_TIMEOUT_SECS: u64 = 600; // 10 minutes

pub async fn run(args: InitArgs, _config: &Config) -> Result<()> {
    let config_path = "codeguardian.toml";

    // Check if config already exists
    if fs::metadata(config_path).await.is_ok() {
        tracing::info!("Configuration file already exists at {}", config_path);
        tracing::info!("Use --force to overwrite (not implemented yet)");
        return Ok(());
    }

    // Create default configuration
    let config = if args.default {
        Config::default()
    } else if let Some(template) = &args.template {
        create_from_template(template)?
    } else {
        create_interactive_config().await?
    };

    // Save configuration
    let toml_content = toml::to_string_pretty(&config)?;
    fs::write(config_path, toml_content).await?;

    tracing::info!("✅ Created configuration file: {}", config_path);
    tracing::info!("Next steps:");
    tracing::info!("1. Review and customize the configuration");
    tracing::info!("2. Run: codeguardian check .");
    tracing::info!("3. Set up CI integration with GitHub Actions");

    Ok(())
}

fn create_from_template(template: &str) -> Result<Config> {
    match template {
        "minimal" => {
            let mut config = Config::default();
            config.analysis.exclude_patterns = vec![
                "*.log".to_string(),
                "*.tmp".to_string(),
                "target/".to_string(),
                "node_modules/".to_string(),
                ".git/".to_string(),
            ];
            Ok(config)
        }
        "security" => {
            let mut config = Config::default();
            config.security.fail_on_issues = true;
            config.security.min_severity = "medium".to_string();
            Ok(config)
        }
        "ci" => {
            let mut config = Config::default();
            config.analysis.analysis_timeout = CI_ANALYSIS_TIMEOUT_SECS;
            Ok(config)
        }
        _ => {
            tracing::warn!("Unknown template: {}", template);
            tracing::info!("Available templates: minimal, security, ci");
            Ok(Config::default())
        }
    }
}

async fn create_interactive_config() -> Result<Config> {
    tracing::info!("🚀 CodeGuardian Configuration Setup");
    tracing::info!("===================================");

    tracing::info!("This wizard will create a basic configuration.");
    tracing::info!("You can customize it later by editing codeguardian.toml");

    // Create a basic configuration
    let config = Config::default();

    tracing::info!("✅ Basic configuration created!");
    tracing::info!("📝 You can edit codeguardian.toml to customize settings.");

    Ok(config)
}
