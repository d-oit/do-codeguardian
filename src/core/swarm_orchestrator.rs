//! Main swarm orchestrator implementation for coordinating parallel security analysis

use crate::core::conflict_resolution::ConflictResolver;
use crate::core::performance_monitor::{PerformanceConfig, SwarmPerformanceMonitor};
use crate::core::resource_manager::{PendingTask, ResourceAwareScheduler, ResourceManager};
use crate::core::result_aggregation::PriorityBasedResolver;
use crate::core::result_aggregation::{AggregationConfig, ResultAggregator};
use crate::core::swarm_types::{
    Priority, ResourceRequirements, SwarmAgent, SwarmConfig, SwarmError, SwarmPerformanceMetrics,
    SwarmResults, SwarmState, SwarmTask, TaskResult, TaskStatus,
};
use crate::core::task_decomposition::{AnalysisRequest, DecompositionStrategy, TaskDecomposer};

use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

/// Main swarm orchestrator for coordinating parallel analysis tasks
pub struct SwarmOrchestrator {
    config: SwarmConfig,
    state: Arc<Mutex<SwarmState>>,
    task_decomposer: TaskDecomposer,
    result_aggregator: ResultAggregator,
    conflict_resolver: ConflictResolver,
    resource_manager: Option<Arc<ResourceManager>>,
    performance_monitor: Option<Arc<SwarmPerformanceMonitor>>,
    scheduler: Option<ResourceAwareScheduler>,
    task_sender: Option<mpsc::UnboundedSender<SwarmTask>>,
    task_receiver: Option<mpsc::UnboundedReceiver<SwarmTask>>,
    result_sender: Option<mpsc::UnboundedSender<TaskResult>>,
    result_receiver: Option<mpsc::UnboundedReceiver<TaskResult>>,
}

impl SwarmOrchestrator {
    /// Create a new swarm orchestrator
    pub fn new(config: SwarmConfig) -> Result<Self, SwarmError> {
        let state = Arc::new(Mutex::new(SwarmState::new(config.clone())));

        let task_decomposer = TaskDecomposer::new(config.clone(), DecompositionStrategy::Hybrid);
        let result_aggregator = ResultAggregator::new(
            AggregationConfig {
                deduplicate_findings: true,
                conflict_resolution_strategy: config.conflict_resolution_strategy,
                max_conflicts_to_resolve: 100,
                aggregation_timeout: Duration::from_secs(30),
            },
            Arc::new(PriorityBasedResolver::new(Self::default_agent_priorities())),
        );

        let conflict_resolver = ConflictResolver::new(
            config.conflict_resolution_strategy,
            Self::default_agent_priorities(),
            Self::default_confidence_thresholds(),
        );

        // Only initialize complex components if resource monitoring or performance tracking is enabled
        let (
            resource_manager,
            performance_monitor,
            scheduler,
            task_sender,
            task_receiver,
            result_sender,
            result_receiver,
        ) = if config.enable_resource_monitoring || config.enable_performance_tracking {
            let performance_config = PerformanceConfig {
                cpu_threshold_percent: 80.0,
                memory_threshold_mb: 1024.0,
                io_threshold_operations: 1000,
                max_history_size: 1000,
                monitoring_interval_ms: 1000,
            };

            let performance_monitor =
                Some(Arc::new(SwarmPerformanceMonitor::new(performance_config)));
            let resource_manager = Some(Arc::new(ResourceManager::new(config.clone())));
            let scheduler = Some(ResourceAwareScheduler::new(
                resource_manager.as_ref().unwrap().clone(),
            ));

            let (task_sender, task_receiver) = mpsc::unbounded_channel();
            let (result_sender, result_receiver) = mpsc::unbounded_channel();

            (
                resource_manager,
                performance_monitor,
                scheduler,
                Some(task_sender),
                Some(task_receiver),
                Some(result_sender),
                Some(result_receiver),
            )
        } else {
            (None, None, None, None, None, None, None)
        };

        Ok(Self {
            config,
            state,
            task_decomposer,
            result_aggregator,
            conflict_resolver,
            resource_manager,
            performance_monitor,
            scheduler,
            task_sender,
            task_receiver,
            result_sender,
            result_receiver,
        })
    }

    /// Execute a comprehensive analysis using the swarm
    pub async fn execute_analysis(
        &mut self,
        request: AnalysisRequest,
    ) -> Result<SwarmResults, SwarmError> {
        // Check if simple parallel mode is enabled (when resource monitoring and performance tracking are disabled)
        if !self.config.enable_resource_monitoring && !self.config.enable_performance_tracking {
            return self.execute_analysis_simple(request).await;
        }

        info!(
            "Starting swarm analysis with {} target files",
            request.target_files.len()
        );

        // Step 1: Decompose the analysis task into parallel subtasks
        let tasks = self.task_decomposer.decompose_task(&request).await?;
        info!("Decomposed into {} parallel tasks", tasks.len());

        // Step 2: Register agents for the tasks
        self.register_agents_for_tasks(&tasks).await?;

        // Step 3: Execute tasks in parallel with resource management
        let task_results = self.execute_tasks_parallel(tasks).await?;

        // Step 4: Aggregate and resolve conflicts in results
        let aggregated_results = self
            .result_aggregator
            .aggregate_results(task_results, self.get_performance_metrics().await)
            .await?;

        // Step 5: Generate final performance metrics
        let _final_metrics = if let Some(pm) = &self.performance_monitor {
            Some(pm.generate_performance_report().await?)
        } else {
            None
        };

        info!(
            "Swarm analysis completed: {} findings, {} conflicts resolved",
            aggregated_results.aggregated_findings.len(),
            aggregated_results.conflicts.len()
        );

        Ok(aggregated_results)
    }

    /// Execute analysis using simple parallel processing without complex resource management
    pub async fn execute_analysis_simple(
        &mut self,
        request: AnalysisRequest,
    ) -> Result<SwarmResults, SwarmError> {
        info!(
            "Starting simple parallel analysis with {} target files",
            request.target_files.len()
        );

        // Step 1: Decompose the analysis task into parallel subtasks
        let tasks = self.task_decomposer.decompose_task(&request).await?;
        info!("Decomposed into {} parallel tasks", tasks.len());

        // Step 2: Register agents for the tasks
        self.register_agents_for_tasks(&tasks).await?;

        // Step 3: Execute tasks in parallel using simple futures::join_all
        let task_results = self.execute_tasks_simple_parallel(tasks).await?;

        // Step 4: Simple aggregation without complex conflict resolution
        let aggregated_results = self.aggregate_results_simple(task_results).await?;

        info!(
            "Simple parallel analysis completed: {} findings",
            aggregated_results.aggregated_findings.len()
        );

        Ok(aggregated_results)
    }

    /// Register swarm agents for the given tasks
    async fn register_agents_for_tasks(&self, tasks: &[SwarmTask]) -> Result<(), SwarmError> {
        let mut state = self.state.lock().await;

        for task in tasks {
            if !state.agent_registry.contains_key(&task.agent_type) {
                // In a real implementation, you'd instantiate the appropriate agent
                // For now, we'll create a placeholder
                let agent = Arc::new(MockSwarmAgent::new(&task.agent_type));
                state.agent_registry.insert(task.agent_type.clone(), agent);
            }
        }

        Ok(())
    }

    /// Execute tasks in parallel with resource management
    async fn execute_tasks_parallel(
        &self,
        tasks: Vec<SwarmTask>,
    ) -> Result<Vec<TaskResult>, SwarmError> {
        // If complex components are not available, fall back to simple parallel execution
        if self.resource_manager.is_none() || self.performance_monitor.is_none() {
            return self.execute_tasks_simple_parallel(tasks).await;
        }

        let resource_manager =
            self.resource_manager
                .as_ref()
                .ok_or(SwarmError::ConfigurationError(
                    "Resource manager not available".to_string(),
                ))?;
        let scheduler = self
            .scheduler
            .as_ref()
            .ok_or(SwarmError::ConfigurationError(
                "Scheduler not available".to_string(),
            ))?;

        let mut handles = Vec::new();
        let mut pending_tasks = Vec::new();

        // Convert tasks to pending tasks for scheduling
        for task in tasks {
            let pending = PendingTask {
                task_id: task.id.clone(),
                priority: task.priority,
                requirements: self.get_resource_requirements_for_task(&task),
            };
            pending_tasks.push((task, pending));
        }

        // Sort by priority
        pending_tasks.sort_by(|a, b| b.1.priority.cmp(&a.1.priority));

        let mut results = Vec::new();

        // Execute tasks with concurrency control
        for (task, pending) in pending_tasks {
            // Check resource availability
            let availability = resource_manager
                .check_resource_availability(&pending.requirements)
                .await;

            if availability.overall_available {
                // Start task immediately
                let handle = self.spawn_task_execution(task);
                handles.push(handle);
            } else {
                // Schedule for later
                scheduler.schedule_task(pending).await?;
            }
        }

        // Wait for initial batch to complete and process pending tasks
        for handle in handles {
            match timeout(Duration::from_secs(300), handle).await {
                Ok(result) => {
                    if let Ok(task_result) = result {
                        results.push(task_result);
                    }
                }
                Err(_) => {
                    warn!("Task execution timed out");
                }
            }
        }

        // Process any remaining pending tasks
        scheduler.process_pending_tasks().await?;

        Ok(results)
    }

    /// Execute tasks in simple parallel processing using futures::join_all
    async fn execute_tasks_simple_parallel(
        &self,
        tasks: Vec<SwarmTask>,
    ) -> Result<Vec<TaskResult>, SwarmError> {
        let start_time = std::time::Instant::now();

        // Create futures for each task
        let task_futures = tasks.into_iter().map(|task| {
            let state = self.state.clone();
            async move {
                let task_start = std::time::Instant::now();

                // Get the agent and execute the task
                let result = {
                    let state_lock = state.lock().await;
                    if let Some(agent) = state_lock.agent_registry.get(&task.agent_type) {
                        match timeout(task.timeout, agent.execute_task(task.clone())).await {
                            Ok(Ok(result)) => result,
                            Ok(Err(e)) => TaskResult {
                                task_id: task.id,
                                agent_id: task.agent_type,
                                status: TaskStatus::Failed,
                                findings: Vec::new(),
                                metrics: crate::core::swarm_types::TaskMetrics {
                                    cpu_usage_percent: 0.0,
                                    memory_usage_mb: 0.0,
                                    io_operations: 0,
                                    network_requests: 0,
                                    processed_files: 0,
                                    processed_lines: 0,
                                },
                                error_message: Some(format!("Task execution failed: {}", e)),
                                execution_time: task_start.elapsed(),
                            },
                            Err(_) => TaskResult {
                                task_id: task.id,
                                agent_id: task.agent_type,
                                status: TaskStatus::Failed,
                                findings: Vec::new(),
                                metrics: crate::core::swarm_types::TaskMetrics {
                                    cpu_usage_percent: 0.0,
                                    memory_usage_mb: 0.0,
                                    io_operations: 0,
                                    network_requests: 0,
                                    processed_files: 0,
                                    processed_lines: 0,
                                },
                                error_message: Some("Task execution timed out".to_string()),
                                execution_time: task_start.elapsed(),
                            },
                        }
                    } else {
                        let agent_type = task.agent_type.clone();
                        TaskResult {
                            task_id: task.id,
                            agent_id: agent_type.clone(),
                            status: TaskStatus::Failed,
                            findings: Vec::new(),
                            metrics: crate::core::swarm_types::TaskMetrics {
                                cpu_usage_percent: 0.0,
                                memory_usage_mb: 0.0,
                                io_operations: 0,
                                network_requests: 0,
                                processed_files: 0,
                                processed_lines: 0,
                            },
                            error_message: Some(format!("No agent found for type: {}", agent_type)),
                            execution_time: task_start.elapsed(),
                        }
                    }
                };

                result
            }
        });

        // Execute all tasks in parallel
        let results = join_all(task_futures).await;

        info!(
            "Simple parallel execution completed in {:?} with {} results",
            start_time.elapsed(),
            results.len()
        );

        Ok(results)
    }

    /// Simple result aggregation without complex conflict resolution
    async fn aggregate_results_simple(
        &self,
        task_results: Vec<TaskResult>,
    ) -> Result<SwarmResults, SwarmError> {
        let mut all_findings = Vec::new();
        let mut conflicts = Vec::new();
        let mut total_execution_time = Duration::from_secs(0);
        let mut completed_tasks = 0;
        let mut failed_tasks = 0;

        for result in &task_results {
            all_findings.extend(result.findings.clone());
            total_execution_time += result.execution_time;

            match result.status {
                TaskStatus::Completed => completed_tasks += 1,
                TaskStatus::Failed => failed_tasks += 1,
                _ => {}
            }
        }

        // Simple deduplication by finding ID
        let mut seen_ids = std::collections::HashSet::new();
        let mut unique_findings = Vec::new();

        for finding in all_findings {
            if seen_ids.insert(finding.id.clone()) {
                unique_findings.push(finding);
            } else {
                // This is a conflict - same finding from different agents
                conflicts.push(crate::core::swarm_types::ConflictInfo {
                    finding_id: finding.id.clone(),
                    conflicting_findings: vec![finding.clone()], // Simplified - just store one
                    agent_ids: vec![finding.analyzer.clone()],
                    resolution_strategy:
                        crate::core::swarm_types::ConflictResolutionStrategy::PriorityBased,
                });
            }
        }

        let performance_metrics = SwarmPerformanceMetrics {
            total_execution_time,
            average_task_time: if task_results.is_empty() {
                Duration::from_secs(0)
            } else {
                total_execution_time / task_results.len() as u32
            },
            max_concurrent_tasks: task_results.len(), // All executed concurrently
            total_cpu_usage_percent: 0.0,             // Not tracked in simple mode
            peak_memory_usage_mb: 0.0,
            total_io_operations: 0,
            total_network_requests: 0,
            tasks_completed: completed_tasks as u64,
            tasks_failed: failed_tasks as u64,
        };

        let execution_summary = crate::core::swarm_types::ExecutionSummary {
            total_tasks: task_results.len(),
            completed_tasks,
            failed_tasks,
            cancelled_tasks: 0,
            total_findings: unique_findings.len(),
            unique_findings: unique_findings.len(),
            conflicts_resolved: conflicts.len(),
        };

        Ok(SwarmResults {
            task_results,
            aggregated_findings: unique_findings,
            conflicts,
            performance_metrics,
            execution_summary,
        })
    }

    /// Spawn a task for execution
    fn spawn_task_execution(&self, task: SwarmTask) -> tokio::task::JoinHandle<TaskResult> {
        let state = self.state.clone();
        let result_sender = self.result_sender.clone();
        let performance_monitor = self.performance_monitor.clone();
        let resource_manager = self.resource_manager.clone();

        tokio::spawn(async move {
            // Start performance monitoring if available
            if let Some(pm) = &performance_monitor {
                let _ = pm.start_task_monitoring(&task.id).await;
            }

            // Allocate resources if resource manager is available
            if let Some(rm) = &resource_manager {
                let _allocation = match rm
                    .allocate_resources(
                        &task.id,
                        &ResourceRequirements {
                            cpu_cores: 1,
                            memory_mb: 256,
                            io_priority: task.priority,
                            network_bandwidth_mbps: 10,
                        },
                    )
                    .await
                {
                    Ok(alloc) => alloc,
                    Err(e) => {
                        return TaskResult {
                            task_id: task.id,
                            agent_id: task.agent_type,
                            status: TaskStatus::Failed,
                            findings: Vec::new(),
                            metrics: crate::core::swarm_types::TaskMetrics {
                                cpu_usage_percent: 0.0,
                                memory_usage_mb: 0.0,
                                io_operations: 0,
                                network_requests: 0,
                                processed_files: 0,
                                processed_lines: 0,
                            },
                            error_message: Some(format!("Resource allocation failed: {}", e)),
                            execution_time: Duration::from_secs(0),
                        };
                    }
                };
            }

            let start_time = std::time::Instant::now();

            // Execute the task
            let result = {
                let state_lock = state.lock().await;
                if let Some(agent) = state_lock.agent_registry.get(&task.agent_type) {
                    match timeout(task.timeout, agent.execute_task(task.clone())).await {
                        Ok(Ok(result)) => result,
                        Ok(Err(e)) => TaskResult {
                            task_id: task.id,
                            agent_id: task.agent_type,
                            status: TaskStatus::Failed,
                            findings: Vec::new(),
                            metrics: crate::core::swarm_types::TaskMetrics {
                                cpu_usage_percent: 0.0,
                                memory_usage_mb: 0.0,
                                io_operations: 0,
                                network_requests: 0,
                                processed_files: 0,
                                processed_lines: 0,
                            },
                            error_message: Some(format!("Task execution failed: {}", e)),
                            execution_time: start_time.elapsed(),
                        },
                        Err(_) => TaskResult {
                            task_id: task.id,
                            agent_id: task.agent_type,
                            status: TaskStatus::Failed,
                            findings: Vec::new(),
                            metrics: crate::core::swarm_types::TaskMetrics {
                                cpu_usage_percent: 0.0,
                                memory_usage_mb: 0.0,
                                io_operations: 0,
                                network_requests: 0,
                                processed_files: 0,
                                processed_lines: 0,
                            },
                            error_message: Some("Task execution timed out".to_string()),
                            execution_time: start_time.elapsed(),
                        },
                    }
                } else {
                    let agent_type = task.agent_type.clone();
                    TaskResult {
                        task_id: task.id,
                        agent_id: agent_type.clone(),
                        status: TaskStatus::Failed,
                        findings: Vec::new(),
                        metrics: crate::core::swarm_types::TaskMetrics {
                            cpu_usage_percent: 0.0,
                            memory_usage_mb: 0.0,
                            io_operations: 0,
                            network_requests: 0,
                            processed_files: 0,
                            processed_lines: 0,
                        },
                        error_message: Some(format!("No agent found for type: {}", agent_type)),
                        execution_time: start_time.elapsed(),
                    }
                }
            };

            // Release resources if resource manager is available
            if let Some(rm) = &resource_manager {
                let _ = rm.release_resources(&result.task_id).await;
            }

            // Complete performance monitoring if available
            if let Some(pm) = &performance_monitor {
                let _ = pm.complete_task_monitoring(&result.task_id, &result).await;
            }

            // Send result if sender is available
            if let Some(sender) = &result_sender {
                let _ = sender.send(result.clone());
            }

            result
        })
    }

    /// Get resource requirements for a task
    fn get_resource_requirements_for_task(&self, task: &SwarmTask) -> ResourceRequirements {
        // Determine requirements based on task type and priority
        let base_cpu = match task.priority {
            Priority::Low => 1,
            Priority::Medium => 1,
            Priority::High => 2,
            Priority::Critical => 4,
        };

        let base_memory = match task.priority {
            Priority::Low => 128,
            Priority::Medium => 256,
            Priority::High => 512,
            Priority::Critical => 1024,
        };

        ResourceRequirements {
            cpu_cores: base_cpu,
            memory_mb: base_memory,
            io_priority: task.priority,
            network_bandwidth_mbps: 10,
        }
    }

    /// Get current performance metrics
    async fn get_performance_metrics(&self) -> SwarmPerformanceMetrics {
        if let Some(_pm) = &self.performance_monitor {
            // This would collect actual metrics from the performance monitor
            SwarmPerformanceMetrics {
                total_execution_time: Duration::from_secs(0), // Would be calculated
                average_task_time: Duration::from_secs(0),
                max_concurrent_tasks: 0,
                total_cpu_usage_percent: 0.0,
                peak_memory_usage_mb: 0.0,
                total_io_operations: 0,
                total_network_requests: 0,
                tasks_completed: 0,
                tasks_failed: 0,
            }
        } else {
            // Return default metrics for simple mode
            SwarmPerformanceMetrics {
                total_execution_time: Duration::from_secs(0),
                average_task_time: Duration::from_secs(0),
                max_concurrent_tasks: 0,
                total_cpu_usage_percent: 0.0,
                peak_memory_usage_mb: 0.0,
                total_io_operations: 0,
                total_network_requests: 0,
                tasks_completed: 0,
                tasks_failed: 0,
            }
        }
    }

    /// Get orchestrator status
    pub async fn get_status(&self) -> OrchestratorStatus {
        let state = self.state.lock().await;
        let resource_stats = if let Some(rm) = &self.resource_manager {
            rm.get_resource_stats().await
        } else {
            // Default resource stats for simple mode
            crate::core::resource_manager::ResourceStats {
                current_cpu_usage: 0,
                current_memory_usage_mb: 0,
                current_io_operations: 0,
                active_tasks: 0,
                max_cpu_cores: 0,
                max_memory_mb: 0,
                max_io_operations: 0,
                tracked_allocations: 0,
            }
        };

        OrchestratorStatus {
            active_tasks: state.active_tasks.len(),
            pending_tasks: 0, // Would track this
            completed_tasks: state.completed_tasks.len(),
            total_agents: state.agent_registry.len(),
            resource_usage: resource_stats,
        }
    }

    /// Shutdown the orchestrator gracefully
    pub async fn shutdown(&mut self) -> Result<(), SwarmError> {
        info!("Shutting down swarm orchestrator");

        // Close channels if they exist
        if let Some(receiver) = &mut self.task_receiver {
            // Wait for any remaining tasks to complete
            while (receiver.recv().await).is_some() {
                // Process any remaining tasks
            }
        }

        Ok(())
    }

    /// Default agent priorities for conflict resolution
    fn default_agent_priorities() -> HashMap<String, Priority> {
        let mut priorities = HashMap::new();
        priorities.insert("security_analyzer".to_string(), Priority::High);
        priorities.insert("vulnerability_analyzer".to_string(), Priority::High);
        priorities.insert("performance_analyzer".to_string(), Priority::Medium);
        priorities.insert("quality_analyzer".to_string(), Priority::Medium);
        priorities.insert("dependency_analyzer".to_string(), Priority::Low);
        priorities
    }

    /// Default confidence thresholds for agents
    fn default_confidence_thresholds() -> HashMap<String, f64> {
        let mut thresholds = HashMap::new();
        thresholds.insert("security_analyzer".to_string(), 0.8);
        thresholds.insert("vulnerability_analyzer".to_string(), 0.85);
        thresholds.insert("performance_analyzer".to_string(), 0.7);
        thresholds.insert("quality_analyzer".to_string(), 0.75);
        thresholds.insert("dependency_analyzer".to_string(), 0.6);
        thresholds
    }
}

/// Status of the orchestrator
#[derive(Debug, Clone)]
pub struct OrchestratorStatus {
    pub active_tasks: usize,
    pub pending_tasks: usize,
    pub completed_tasks: usize,
    pub total_agents: usize,
    pub resource_usage: crate::core::resource_manager::ResourceStats,
}

/// Mock swarm agent for testing (would be replaced with real agents)
struct MockSwarmAgent {
    agent_type: String,
}

impl MockSwarmAgent {
    fn new(agent_type: &str) -> Self {
        Self {
            agent_type: agent_type.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl SwarmAgent for MockSwarmAgent {
    fn id(&self) -> &str {
        &self.agent_type
    }

    fn agent_type(&self) -> &str {
        &self.agent_type
    }

    fn priority(&self) -> Priority {
        Priority::Medium
    }

    fn can_handle(&self, task: &SwarmTask) -> bool {
        task.agent_type == self.agent_type
    }

    async fn execute_task(&self, task: SwarmTask) -> Result<TaskResult, SwarmError> {
        // Simulate task execution
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(TaskResult {
            task_id: task.id,
            agent_id: self.agent_type.clone(),
            status: TaskStatus::Completed,
            findings: Vec::new(), // Would contain actual findings
            metrics: crate::core::swarm_types::TaskMetrics {
                cpu_usage_percent: 10.0,
                memory_usage_mb: 50.0,
                io_operations: 100,
                network_requests: 5,
                processed_files: task.target_files.len() as u64,
                processed_lines: 1000,
            },
            error_message: None,
            execution_time: Duration::from_millis(100),
        })
    }

    fn resource_requirements(&self) -> ResourceRequirements {
        ResourceRequirements {
            cpu_cores: 1,
            memory_mb: 256,
            io_priority: Priority::Medium,
            network_bandwidth_mbps: 10,
        }
    }
}
