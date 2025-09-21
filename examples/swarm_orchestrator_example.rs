//! Example usage of the CodeGuardian Swarm Orchestrator Framework

use do_codeguardian::core::swarm_orchestrator::SwarmOrchestrator;
use do_codeguardian::core::swarm_types::{ConflictResolutionStrategy, Priority, SwarmConfig};
use do_codeguardian::core::task_decomposition::AnalysisRequest;
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CodeGuardian Swarm Orchestrator Example");
    println!("==========================================");

    // Create swarm configuration (using simple parallel mode)
    let config = SwarmConfig {
        max_concurrent_tasks: 10,
        max_memory_mb: 2048,
        max_cpu_percent: 80.0,
        task_timeout: Duration::from_secs(300),
        enable_resource_monitoring: false, // Disable for simple mode
        enable_performance_tracking: false, // Disable for simple mode
        conflict_resolution_strategy: ConflictResolutionStrategy::PriorityBased,
    };

    // Initialize the swarm orchestrator
    let mut orchestrator = SwarmOrchestrator::new(config)?;

    // Define analysis request
    let mut parameters = HashMap::new();
    parameters.insert("security_level".to_string(), "high".to_string());
    parameters.insert("performance_mode".to_string(), "balanced".to_string());

    let request = AnalysisRequest {
        target_files: vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "src/core/mod.rs".to_string(),
            "Cargo.toml".to_string(),
        ],
        parameters,
        analysis_types: vec![
            "security_analyzer".to_string(),
            "performance_analyzer".to_string(),
            "quality_analyzer".to_string(),
        ],
        priority: Priority::High,
    };

    println!("📋 Analysis Request:");
    println!("  - Target files: {}", request.target_files.len());
    println!("  - Analysis types: {:?}", request.analysis_types);
    println!("  - Priority: {:?}", request.priority);

    // Execute the analysis
    println!("\n⚡ Executing swarm analysis...");
    let start_time = std::time::Instant::now();

    let results = orchestrator.execute_analysis(request).await?;

    let execution_time = start_time.elapsed();

    // Display results
    println!("\n📊 Analysis Results:");
    println!("==================");
    println!("Execution time: {:.2}s", execution_time.as_secs_f64());
    println!("Total findings: {}", results.aggregated_findings.len());
    println!("Conflicts resolved: {}", results.conflicts.len());
    println!(
        "Tasks completed: {}",
        results.execution_summary.completed_tasks
    );
    println!("Tasks failed: {}", results.execution_summary.failed_tasks);

    // Display performance metrics
    println!("\n⚡ Performance Metrics:");
    println!("=====================");
    println!(
        "Total execution time: {:.2}s",
        results
            .performance_metrics
            .total_execution_time
            .as_secs_f64()
    );
    println!(
        "Average task time: {:.2}s",
        results.performance_metrics.average_task_time.as_secs_f64()
    );
    println!(
        "Max concurrent tasks: {}",
        results.performance_metrics.max_concurrent_tasks
    );
    println!(
        "Peak memory usage: {} MB",
        results.performance_metrics.peak_memory_usage_mb
    );
    println!(
        "Tasks completed: {}",
        results.performance_metrics.tasks_completed
    );
    println!("Tasks failed: {}", results.performance_metrics.tasks_failed);

    // Display findings by category
    if !results.aggregated_findings.is_empty() {
        println!("\n🔍 Findings Summary:");
        println!("===================");

        let mut category_counts = HashMap::new();
        let mut severity_counts = HashMap::new();

        for finding in &results.aggregated_findings {
            *category_counts.entry(finding.category.clone()).or_insert(0) += 1;
            *severity_counts.entry(finding.severity.clone()).or_insert(0) += 1;
        }

        println!("By Category:");
        for (category, count) in category_counts {
            if let Some(cat) = category {
                println!("  {}: {}", cat, count);
            } else {
                println!("  (uncategorized): {}", count);
            }
        }

        println!("\nBy Severity:");
        for (severity, count) in severity_counts {
            println!("  {}: {}", severity, count);
        }
    }

    // Display conflicts if any
    if !results.conflicts.is_empty() {
        println!("\n⚠️  Conflicts Resolved:");
        println!("====================");
        for conflict in &results.conflicts {
            println!("  Finding: {}", conflict.finding_id);
            println!("  Agents involved: {}", conflict.agent_ids.len());
            println!("  Resolution strategy: {:?}", conflict.resolution_strategy);
        }
    }

    // Get final orchestrator status
    let status = orchestrator.get_status().await;
    println!("\n📈 Final Orchestrator Status:");
    println!("===========================");
    println!("Active tasks: {}", status.active_tasks);
    println!("Pending tasks: {}", status.pending_tasks);
    println!("Completed tasks: {}", status.completed_tasks);
    println!("Total agents: {}", status.total_agents);
    println!(
        "Current CPU usage: {} cores",
        status.resource_usage.current_cpu_usage
    );
    println!(
        "Current memory usage: {} MB",
        status.resource_usage.current_memory_usage_mb
    );

    // Shutdown gracefully
    orchestrator.shutdown().await?;
    println!("\n✅ Swarm orchestrator shutdown complete");

    Ok(())
}
