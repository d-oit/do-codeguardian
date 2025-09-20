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

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{timeout, Duration};
use tracing::{error, info, warn};

/// Main swarm orchestrator for coordinating parallel analysis tasks
pub struct SwarmOrchestrator {
    config: SwarmConfig,
    state: Arc<Mutex<SwarmState>>,
    task_decomposer: TaskDecomposer,
    result_aggregator: ResultAggregator,
    conflict_resolver: ConflictResolver,
    resource_manager: Arc<ResourceManager>,
    performance_monitor: Arc<SwarmPerformanceMonitor>,
    scheduler: ResourceAwareScheduler,
    task_sender: mpsc::UnboundedSender<SwarmTask>,
    task_receiver: mpsc::UnboundedReceiver<SwarmTask>,
    result_sender: mpsc::UnboundedSender<TaskResult>,
    result_receiver: mpsc::UnboundedReceiver<TaskResult>,
}

impl SwarmOrchestrator {
    /// Create a new swarm orchestrator
    pub fn new(config: SwarmConfig) -> Result<Self, SwarmError> {
        let performance_config = PerformanceConfig {
            cpu_threshold_percent: 80.0,
            memory_threshold_mb: 1024.0,
            io_threshold_operations: 1000,
            max_history_size: 1000,
            monitoring_interval_ms: 1000,
        };

        let performance_monitor = Arc::new(SwarmPerformanceMonitor::new(performance_config));
        let resource_manager = Arc::new(ResourceManager::new(config.clone()));
        let scheduler = ResourceAwareScheduler::new(resource_manager.clone());

        let (task_sender, task_receiver) = mpsc::unbounded_channel();
        let (result_sender, result_receiver) = mpsc::unbounded_channel();

        let state = Arc::new(Mutex::new(SwarmState::new(
            config.clone(),
            performance_monitor.clone(),
        )));

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
        let final_metrics = self
            .performance_monitor
            .generate_performance_report()
            .await?;

        info!(
            "Swarm analysis completed: {} findings, {} conflicts resolved",
            aggregated_results.aggregated_findings.len(),
            aggregated_results.conflicts.len()
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
            let availability = self
                .resource_manager
                .check_resource_availability(&pending.requirements)
                .await;

            if availability.overall_available {
                // Start task immediately
                let handle = self.spawn_task_execution(task);
                handles.push(handle);
            } else {
                // Schedule for later
                self.scheduler.schedule_task(pending).await?;
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
        self.scheduler.process_pending_tasks().await?;

        Ok(results)
    }

    /// Spawn a task for execution
    fn spawn_task_execution(&self, task: SwarmTask) -> tokio::task::JoinHandle<TaskResult> {
        let state = self.state.clone();
        let result_sender = self.result_sender.clone();
        let performance_monitor = self.performance_monitor.clone();
        let resource_manager = self.resource_manager.clone();

        tokio::spawn(async move {
            // Start performance monitoring
            let _ = performance_monitor.start_task_monitoring(&task.id).await;

            // Allocate resources
            let allocation = match resource_manager
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

            // Release resources
            let _ = resource_manager.release_resources(&result.task_id).await;

            // Complete performance monitoring
            let _ = performance_monitor
                .complete_task_monitoring(&result.task_id, &result)
                .await;

            // Send result
            let _ = result_sender.send(result.clone());

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
    }

    /// Get orchestrator status
    pub async fn get_status(&self) -> OrchestratorStatus {
        let state = self.state.lock().await;
        let resource_stats = self.resource_manager.get_resource_stats().await;

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

        // Close channels

        // Wait for any remaining tasks to complete
        while let Some(_) = self.task_receiver.recv().await {
            // Process any remaining tasks
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
