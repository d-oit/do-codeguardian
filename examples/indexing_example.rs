use do_codeguardian::indexing::{ResultsIndexer, SearchQuery};
use do_codeguardian::types::{Finding, Severity};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("CodeGuardian Results Indexing Example");
    println!("=====================================");

    // Create a new indexer
    let indexer = ResultsIndexer::new(None);

    // Create some sample findings
    let findings = vec![
        Finding::new(
            "security_analyzer",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Hardcoded password detected in configuration".to_string(),
        )
        .with_description("Found hardcoded password in config file".to_string())
        .with_category("Security".to_string()),
        Finding::new(
            "performance_analyzer",
            "slow_query",
            Severity::Medium,
            PathBuf::from("src/db.rs"),
            15,
            "Database query may be slow due to missing index".to_string(),
        )
        .with_description("Consider adding database index for better performance".to_string())
        .with_category("Performance".to_string()),
        Finding::new(
            "security_analyzer",
            "sql_injection",
            Severity::Critical,
            PathBuf::from("src/api.rs"),
            78,
            "Potential SQL injection vulnerability".to_string(),
        )
        .with_description("User input is directly concatenated into SQL query".to_string())
        .with_category("Security".to_string()),
    ];

    // Index the findings
    println!("Indexing {} findings...", findings.len());
    indexer.index_findings(&findings).await?;

    // Perform various searches
    println!("\n1. Full-text search for 'password':");
    let query = SearchQuery {
        query: Some("password".to_string()),
        ..Default::default()
    };
    let results = indexer.search(&query).await?;
    for result in &results {
        println!(
            "  - {} (score: {:.2})",
            result.finding.message, result.score
        );
    }

    println!("\n2. Filter by analyzer 'security_analyzer':");
    let query = SearchQuery {
        analyzer: Some("security_analyzer".to_string()),
        ..Default::default()
    };
    let results = indexer.search(&query).await?;
    for result in &results {
        println!(
            "  - {} ({})",
            result.finding.message, result.finding.severity
        );
    }

    println!("\n3. Filter by minimum severity 'High':");
    let query = SearchQuery {
        min_severity: Some(Severity::High),
        ..Default::default()
    };
    let results = indexer.search(&query).await?;
    for result in &results {
        println!(
            "  - {} ({})",
            result.finding.message, result.finding.severity
        );
    }

    println!("\n4. Complex query: security issues with 'SQL' in text:");
    let query = SearchQuery {
        query: Some("SQL".to_string()),
        analyzer: Some("security_analyzer".to_string()),
        ..Default::default()
    };
    let results = indexer.search(&query).await?;
    for result in &results {
        println!(
            "  - {} ({})",
            result.finding.message, result.finding.severity
        );
    }

    // Get facet values
    println!("\n5. Available analyzers:");
    let analyzers = indexer.get_facet_values("analyzer").await?;
    for analyzer in &analyzers {
        println!("  - {}", analyzer);
    }

    println!("\n6. Available severities:");
    let severities = indexer.get_facet_values("severity").await?;
    for severity in &severities {
        println!("  - {}", severity);
    }

    // Count total findings
    let total_count = indexer.count(&SearchQuery::default()).await?;
    println!("\nTotal indexed findings: {}", total_count);

    println!("\nIndexing example completed successfully!");
    Ok(())
}
