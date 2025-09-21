//! Resource management system for optimizing CPU, memory, and I/O usage across parallel agents

use crate::core::swarm_types::{Priority, ResourceRequirements, SwarmConfig, SwarmError};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{timeout, Duration};

/// Resource manager for coordinating resource usage across swarm agents
pub struct ResourceManager {
    config: SwarmConfig,
    system_limits: SystemLimits,
    active_resources: Arc<Mutex<ResourceUsage>>,
    resource_semaphores: HashMap<String, Arc<Semaphore>>,
    task_resource_tracking: Arc<Mutex<HashMap<String, ResourceAllocation>>>,
}

impl ResourceManager {
    pub fn new(config: SwarmConfig) -> Self {
        let system_limits = SystemLimits::detect();
        let mut resource_semaphores = HashMap::new();

        // Create semaphores for different resource types
        resource_semaphores.insert(
            "cpu".to_string(),
            Arc::new(Semaphore::new(system_limits.max_cpu_cores)),
        );
        resource_semaphores.insert(
            "memory".to_string(),
            Arc::new(Semaphore::new((system_limits.max_memory_mb / 100) as usize)), // 100MB per permit
        );
        resource_semaphores.insert(
            "io".to_string(),
            Arc::new(Semaphore::new(system_limits.max_io_operations as usize)),
        );

        Self {
            config,
            system_limits,
            active_resources: Arc::new(Mutex::new(ResourceUsage::default())),
            resource_semaphores,
            task_resource_tracking: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Allocate resources for a task
    pub async fn allocate_resources(
        &self,
        task_id: &str,
        requirements: &ResourceRequirements,
    ) -> Result<ResourceAllocation, SwarmError> {
        let allocation = ResourceAllocation {
            task_id: task_id.to_string(),
            cpu_cores: requirements.cpu_cores,
            memory_mb: requirements.memory_mb,
            io_priority: requirements.io_priority,
            network_bandwidth_mbps: requirements.network_bandwidth_mbps,
            allocated_at: std::time::Instant::now(),
        };

        // Check if allocation would exceed system limits
        {
            let active = self.active_resources.lock().await;
            if active.cpu_usage + requirements.cpu_cores > self.system_limits.max_cpu_cores {
                return Err(SwarmError::ResourceExhausted(
                    "CPU cores exhausted".to_string(),
                ));
            }
            if active.memory_usage_mb + requirements.memory_mb > self.system_limits.max_memory_mb {
                return Err(SwarmError::ResourceExhausted(
                    "Memory exhausted".to_string(),
                ));
            }
        }

        // Acquire resource permits
        let _cpu_permits = self.acquire_permits("cpu", requirements.cpu_cores).await?;
        let _memory_permits = self
            .acquire_permits("memory", (requirements.memory_mb / 100) as usize)
            .await?;

        // Update active resource tracking
        {
            let mut active = self.active_resources.lock().await;
            active.cpu_usage += requirements.cpu_cores;
            active.memory_usage_mb += requirements.memory_mb;
            active.active_tasks += 1;
        }

        // Track allocation for this task
        {
            let mut tracking = self.task_resource_tracking.lock().await;
            tracking.insert(task_id.to_string(), allocation.clone());
        }

        Ok(allocation)
    }

    /// Release resources allocated to a task
    pub async fn release_resources(&self, task_id: &str) -> Result<(), SwarmError> {
        let allocation = {
            let mut tracking = self.task_resource_tracking.lock().await;
            tracking.remove(task_id)
        };

        if let Some(allocation) = allocation {
            // Release permits
            self.release_permits("cpu", allocation.cpu_cores).await?;
            self.release_permits("memory", (allocation.memory_mb / 100) as usize)
                .await?;

            // Update active resource tracking
            let mut active = self.active_resources.lock().await;
            active.cpu_usage = active.cpu_usage.saturating_sub(allocation.cpu_cores);
            active.memory_usage_mb = active.memory_usage_mb.saturating_sub(allocation.memory_mb);
            active.active_tasks = active.active_tasks.saturating_sub(1);
        }

        Ok(())
    }

    /// Check if resources are available for a task
    pub async fn check_resource_availability(
        &self,
        requirements: &ResourceRequirements,
    ) -> ResourceAvailability {
        let active = self.active_resources.lock().await;

        let cpu_available =
            active.cpu_usage + requirements.cpu_cores <= self.system_limits.max_cpu_cores;
        let memory_available =
            active.memory_usage_mb + requirements.memory_mb <= self.system_limits.max_memory_mb;
        let io_available = active.io_operations < self.system_limits.max_io_operations;

        let overall_available = cpu_available && memory_available && io_available;

        ResourceAvailability {
            overall_available,
            cpu_available,
            memory_available,
            io_available,
            estimated_wait_time: if overall_available {
                Duration::from_secs(0)
            } else {
                Duration::from_secs(5) // Simple estimation
            },
        }
    }

    /// Get current resource usage statistics
    pub async fn get_resource_stats(&self) -> ResourceStats {
        let active = self.active_resources.lock().await;
        let tracking = self.task_resource_tracking.lock().await;

        ResourceStats {
            current_cpu_usage: active.cpu_usage,
            current_memory_usage_mb: active.memory_usage_mb,
            current_io_operations: active.io_operations,
            active_tasks: active.active_tasks,
            max_cpu_cores: self.system_limits.max_cpu_cores,
            max_memory_mb: self.system_limits.max_memory_mb,
            max_io_operations: self.system_limits.max_io_operations,
            tracked_allocations: tracking.len(),
        }
    }

    /// Optimize resource allocation based on task priorities
    pub async fn optimize_allocation(&self, pending_tasks: Vec<PendingTask>) -> Vec<PendingTask> {
        let mut prioritized_tasks = pending_tasks;
        let _active = self.active_resources.lock().await;

        // Sort tasks by priority (highest first)
        prioritized_tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut optimized = Vec::new();

        for task in prioritized_tasks {
            let availability = self.check_resource_availability(&task.requirements).await;

            if availability.overall_available {
                optimized.push(task);
            } else {
                // For high-priority tasks, we might want to preempt lower-priority tasks
                if matches!(task.priority, Priority::Critical) {
                    // Preemption logic would go here
                    optimized.push(task);
                }
                // Lower priority tasks wait
            }
        }

        optimized
    }

    /// Monitor resource usage and trigger scaling if needed
    pub async fn monitor_and_scale(&self) -> Result<(), SwarmError> {
        let stats = self.get_resource_stats().await;

        // Check if we need to scale up or down
        let cpu_utilization = stats.current_cpu_usage as f64 / stats.max_cpu_cores as f64;
        let memory_utilization = stats.current_memory_usage_mb as f64 / stats.max_memory_mb as f64;

        if cpu_utilization > 0.9 || memory_utilization > 0.9 {
            tracing::warn!(
                "High resource utilization detected - CPU: {:.2}%, Memory: {:.2}%",
                cpu_utilization * 100.0,
                memory_utilization * 100.0
            );
            // In a real implementation, this would trigger scaling actions
        }

        if cpu_utilization < 0.3 && memory_utilization < 0.3 && stats.active_tasks < 5 {
            tracing::info!("Low resource utilization - could scale down");
        }

        Ok(())
    }

    /// Acquire permits from a semaphore
    async fn acquire_permits(
        &self,
        resource_type: &str,
        count: usize,
    ) -> Result<Vec<tokio::sync::SemaphorePermit<'_>>, SwarmError> {
        if let Some(semaphore) = self.resource_semaphores.get(resource_type) {
            let mut permits = Vec::new();
            for _ in 0..count {
                match timeout(Duration::from_secs(30), semaphore.acquire()).await {
                    Ok(permit) => {
                        if let Ok(p) = permit {
                            permits.push(p);
                        }
                    }
                    Err(_) => {
                        // Timeout - release any permits we already acquired
                        drop(permits);
                        return Err(SwarmError::ResourceExhausted(format!(
                            "Timeout acquiring {} permits for {}",
                            count, resource_type
                        )));
                    }
                }
            }
            Ok(permits)
        } else {
            Err(SwarmError::ConfigurationError(format!(
                "Unknown resource type: {}",
                resource_type
            )))
        }
    }

    /// Release permits back to semaphore
    async fn release_permits(&self, _resource_type: &str, _count: usize) -> Result<(), SwarmError> {
        // In this simplified implementation, permits are automatically released when dropped
        // In a real implementation, you might want to track permits explicitly
        Ok(())
    }

    /// Handle resource exhaustion gracefully
    pub async fn handle_resource_exhaustion(&self, task_id: &str) -> Result<(), SwarmError> {
        tracing::warn!("Resource exhaustion detected for task {}", task_id);

        // Release resources for this task
        self.release_resources(task_id).await?;

        // Could implement backoff strategies, task queuing, etc.
        Ok(())
    }
}

/// System resource limits
#[derive(Debug, Clone)]
pub struct SystemLimits {
    pub max_cpu_cores: usize,
    pub max_memory_mb: u64,
    pub max_io_operations: u64,
    pub max_network_bandwidth_mbps: u64,
}

impl SystemLimits {
    /// Detect system limits (simplified implementation)
    pub fn detect() -> Self {
        Self {
            max_cpu_cores: num_cpus::get(),
            max_memory_mb: 8192, // 8GB default, in practice you'd detect actual system memory
            max_io_operations: 1000, // Arbitrary limit
            max_network_bandwidth_mbps: 100, // Arbitrary limit
        }
    }
}

/// Current resource usage
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    pub cpu_usage: usize,
    pub memory_usage_mb: u64,
    pub io_operations: u64,
    pub active_tasks: usize,
}

/// Resource allocation for a specific task
#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub task_id: String,
    pub cpu_cores: usize,
    pub memory_mb: u64,
    pub io_priority: Priority,
    pub network_bandwidth_mbps: u64,
    pub allocated_at: std::time::Instant,
}

/// Resource availability check result
#[derive(Debug, Clone)]
pub struct ResourceAvailability {
    pub overall_available: bool,
    pub cpu_available: bool,
    pub memory_available: bool,
    pub io_available: bool,
    pub estimated_wait_time: Duration,
}

/// Resource usage statistics
#[derive(Debug, Clone)]
pub struct ResourceStats {
    pub current_cpu_usage: usize,
    pub current_memory_usage_mb: u64,
    pub current_io_operations: u64,
    pub active_tasks: usize,
    pub max_cpu_cores: usize,
    pub max_memory_mb: u64,
    pub max_io_operations: u64,
    pub tracked_allocations: usize,
}

/// Pending task with resource requirements
#[derive(Debug, Clone)]
pub struct PendingTask {
    pub task_id: String,
    pub priority: Priority,
    pub requirements: ResourceRequirements,
}

/// Resource-aware task scheduler
pub struct ResourceAwareScheduler {
    resource_manager: Arc<ResourceManager>,
    pending_queue: Arc<Mutex<Vec<PendingTask>>>,
    active_tasks: Arc<Mutex<HashSet<String>>>,
}

impl ResourceAwareScheduler {
    pub fn new(resource_manager: Arc<ResourceManager>) -> Self {
        Self {
            resource_manager,
            pending_queue: Arc::new(Mutex::new(Vec::new())),
            active_tasks: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Schedule a task considering resource constraints
    pub async fn schedule_task(&self, task: PendingTask) -> Result<(), SwarmError> {
        let availability = self
            .resource_manager
            .check_resource_availability(&task.requirements)
            .await;

        if availability.overall_available {
            // Resources available - schedule immediately
            self.active_tasks.lock().await.insert(task.task_id.clone());
            // In practice, you'd start the task here
        } else {
            // Add to pending queue
            self.pending_queue.lock().await.push(task);
        }

        Ok(())
    }

    /// Process pending tasks when resources become available
    pub async fn process_pending_tasks(&self) -> Result<(), SwarmError> {
        let mut pending = self.pending_queue.lock().await;
        let mut active = self.active_tasks.lock().await;

        if pending.is_empty() {
            return Ok(());
        }

        // Sort pending tasks by priority
        pending.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut to_schedule = Vec::new();

        // Try to schedule high-priority tasks first
        for task in pending.iter() {
            let availability = self
                .resource_manager
                .check_resource_availability(&task.requirements)
                .await;
            if availability.overall_available {
                to_schedule.push(task.clone());
            }
        }

        // Remove scheduled tasks from pending queue
        pending.retain(|task| !to_schedule.iter().any(|t| t.task_id == task.task_id));

        // Mark as active
        for task in &to_schedule {
            active.insert(task.task_id.clone());
        }

        Ok(())
    }
}
