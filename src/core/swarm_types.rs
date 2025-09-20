//! Common types and traits for the swarm orchestrator framework

use crate::core::performance_monitor::SwarmPerformanceMonitor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// Priority levels for swarm tasks and results
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Status of a swarm task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Result of a swarm task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub agent_id: String,
    pub status: TaskStatus,
    pub findings: Vec<Finding>,
    pub metrics: TaskMetrics,
    pub error_message: Option<String>,
    pub execution_time: Duration,
}

/// Metrics collected during task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub io_operations: u64,
    pub network_requests: u64,
    pub processed_files: u64,
    pub processed_lines: u64,
}

/// A subtask that can be executed by a swarm agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmTask {
    pub id: String,
    pub agent_type: String,
    pub priority: Priority,
    pub target_files: Vec<String>,
    pub parameters: HashMap<String, String>,
    pub timeout: Duration,
    pub dependencies: Vec<String>, // Task IDs this task depends on
}

/// Configuration for the swarm orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub max_concurrent_tasks: usize,
    pub max_memory_mb: u64,
    pub max_cpu_percent: f64,
    pub task_timeout: Duration,
    pub enable_resource_monitoring: bool,
    pub enable_performance_tracking: bool,
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
}

/// Strategy for resolving conflicts between agent results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// Use the result with highest priority agent
    PriorityBased,
    /// Use the result with highest confidence score
    ConfidenceBased,
    /// Combine results from all agents
    ConsensusBased,
    /// Require manual review for conflicts
    ManualReview,
}

/// Information about a conflict between agent results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub finding_id: String,
    pub conflicting_findings: Vec<Finding>,
    pub agent_ids: Vec<String>,
    pub resolution_strategy: ConflictResolutionStrategy,
}

/// Aggregated results from the entire swarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmResults {
    pub task_results: Vec<TaskResult>,
    pub aggregated_findings: Vec<Finding>,
    pub conflicts: Vec<ConflictInfo>,
    pub performance_metrics: SwarmPerformanceMetrics,
    pub execution_summary: ExecutionSummary,
}

/// Performance metrics for the entire swarm execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmPerformanceMetrics {
    pub total_execution_time: Duration,
    pub average_task_time: Duration,
    pub max_concurrent_tasks: usize,
    pub total_cpu_usage_percent: f64,
    pub peak_memory_usage_mb: f64,
    pub total_io_operations: u64,
    pub total_network_requests: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
}

/// Summary of swarm execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSummary {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub cancelled_tasks: usize,
    pub total_findings: usize,
    pub unique_findings: usize,
    pub conflicts_resolved: usize,
}

/// Trait for swarm agents that can execute tasks
#[async_trait::async_trait]
pub trait SwarmAgent: Send + Sync {
    /// Get the unique identifier for this agent
    fn id(&self) -> &str;

    /// Get the type of analysis this agent performs
    fn agent_type(&self) -> &str;

    /// Get the priority level of this agent
    fn priority(&self) -> Priority;

    /// Check if this agent can handle the given task
    fn can_handle(&self, task: &SwarmTask) -> bool;

    /// Execute the task and return results
    async fn execute_task(&self, task: SwarmTask) -> Result<TaskResult, SwarmError>;

    /// Get resource requirements for this agent
    fn resource_requirements(&self) -> ResourceRequirements;
}

/// Resource requirements for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores: usize,
    pub memory_mb: u64,
    pub io_priority: Priority,
    pub network_bandwidth_mbps: u64,
}

/// Error types for swarm operations
#[derive(Debug, Clone)]
pub enum SwarmError {
    TaskTimeout(String),
    ResourceExhausted(String),
    AgentFailure(String),
    DependencyFailure(String),
    ConfigurationError(String),
    InternalError(String),
}

impl fmt::Display for SwarmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwarmError::TaskTimeout(msg) => write!(f, "Task timeout: {}", msg),
            SwarmError::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
            SwarmError::AgentFailure(msg) => write!(f, "Agent failure: {}", msg),
            SwarmError::DependencyFailure(msg) => write!(f, "Dependency failure: {}", msg),
            SwarmError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            SwarmError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SwarmError {}

/// Channel types for swarm communication
pub type TaskSender = mpsc::UnboundedSender<SwarmTask>;
pub type TaskReceiver = mpsc::UnboundedReceiver<SwarmTask>;
pub type ResultSender = mpsc::UnboundedSender<TaskResult>;
pub type ResultReceiver = mpsc::UnboundedReceiver<TaskResult>;

/// Shared state for the swarm orchestrator
pub struct SwarmState {
    pub config: SwarmConfig,
    pub active_tasks: HashMap<String, SwarmTask>,
    pub completed_tasks: HashMap<String, TaskResult>,
    pub task_dependencies: HashMap<String, Vec<String>>,
    pub agent_registry: HashMap<String, Arc<dyn SwarmAgent>>,
    pub performance_monitor: Arc<SwarmPerformanceMonitor>,
}

impl SwarmState {
    pub fn new(config: SwarmConfig, performance_monitor: Arc<SwarmPerformanceMonitor>) -> Self {
        Self {
            config,
            active_tasks: HashMap::new(),
            completed_tasks: HashMap::new(),
            task_dependencies: HashMap::new(),
            agent_registry: HashMap::new(),
            performance_monitor,
        }
    }
}
