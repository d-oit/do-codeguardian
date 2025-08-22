use anyhow::Result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;

/// System load metrics
#[derive(Debug, Clone)]
pub struct SystemLoad {
    pub cpu_usage: f64,    // 0.0 to 1.0
    pub memory_usage: f64, // 0.0 to 1.0
    pub io_wait: f64,      // 0.0 to 1.0
    pub load_average: f64, // system load average
    pub timestamp: Instant,
}

impl Default for SystemLoad {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemLoad {
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            io_wait: 0.0,
            load_average: 0.0,
            timestamp: Instant::now(),
        }
    }

    /// Calculate overall system load score (0.0 to 1.0, higher = more loaded)
    pub fn load_score(&self) -> f64 {
        // Weighted combination of different load metrics
        let cpu_weight = 0.4;
        let memory_weight = 0.3;
        let io_weight = 0.2;
        let load_avg_weight = 0.1;

        (self.cpu_usage * cpu_weight)
            + (self.memory_usage * memory_weight)
            + (self.io_wait * io_weight)
            + ((self.load_average / num_cpus::get() as f64).min(1.0) * load_avg_weight)
    }

    /// Check if system is under high load
    pub fn is_high_load(&self) -> bool {
        self.load_score() > 0.7
    }

    /// Check if system is under moderate load
    pub fn is_moderate_load(&self) -> bool {
        self.load_score() > 0.4
    }
}

/// Adaptive parallelism controller
pub struct AdaptiveParallelismController {
    current_workers: AtomicUsize,
    min_workers: usize,
    max_workers: usize,
    adjustment_interval: Duration,
    last_adjustment: Instant,
    load_history: Vec<SystemLoad>,
    max_history_size: usize,
    system_load: Arc<Mutex<SystemLoad>>,
}

impl AdaptiveParallelismController {
    pub fn new(min_workers: usize, max_workers: usize, initial_workers: usize) -> Self {
        Self {
            current_workers: AtomicUsize::new(initial_workers),
            min_workers,
            max_workers,
            adjustment_interval: Duration::from_secs(5),
            last_adjustment: Instant::now(),
            load_history: Vec::with_capacity(10),
            max_history_size: 10,
            system_load: Arc::new(Mutex::new(SystemLoad::new())),
        }
    }

    /// Get current recommended number of workers
    pub fn current_workers(&self) -> usize {
        self.current_workers.load(Ordering::Relaxed)
    }

    /// Update system load and potentially adjust worker count
    pub async fn update_load(&self, new_load: SystemLoad) -> Result<()> {
        // Update system load
        {
            let mut load = self.system_load.lock().unwrap();
            *load = new_load.clone();
        }

        // Add to history
        let mut history = self.load_history.clone();
        history.push(new_load);
        if history.len() > self.max_history_size {
            history.remove(0);
        }

        // Check if we should adjust workers
        if self.last_adjustment.elapsed() >= self.adjustment_interval {
            self.adjust_workers(&history).await;
        }

        Ok(())
    }

    /// Manually adjust worker count
    pub async fn adjust_workers(&self, load_history: &[SystemLoad]) {
        if load_history.is_empty() {
            return;
        }

        // Calculate average load over recent history
        let avg_load = load_history
            .iter()
            .map(|load| load.load_score())
            .sum::<f64>()
            / load_history.len() as f64;

        let current = self.current_workers.load(Ordering::Relaxed);
        let mut new_workers = current;

        // Adjust based on load
        if avg_load > 0.8 {
            // High load - reduce workers significantly
            new_workers = (current * 3 / 4).max(self.min_workers);
        } else if avg_load > 0.6 {
            // Moderate-high load - reduce workers
            new_workers = (current * 4 / 5).max(self.min_workers);
        } else if avg_load < 0.2 {
            // Low load - increase workers
            new_workers = (current * 5 / 4).min(self.max_workers);
        } else if avg_load < 0.4 {
            // Moderate load - slight increase
            new_workers = (current * 6 / 5).min(self.max_workers);
        }

        if new_workers != current {
            self.current_workers.store(new_workers, Ordering::Relaxed);
            println!(
                "Adaptive parallelism: adjusted workers from {} to {} (avg load: {:.2})",
                current, new_workers, avg_load
            );
        }
    }

    /// Get current system load
    pub fn current_load(&self) -> SystemLoad {
        self.system_load.lock().unwrap().clone()
    }

    /// Force set worker count
    pub fn set_workers(&self, count: usize) {
        let clamped = count.clamp(self.min_workers, self.max_workers);
        self.current_workers.store(clamped, Ordering::Relaxed);
    }

    /// Get performance metrics
    pub fn metrics(&self) -> AdaptiveParallelismMetrics {
        let current_load = self.current_load();
        let avg_load = if self.load_history.is_empty() {
            0.0
        } else {
            self.load_history
                .iter()
                .map(|load| load.load_score())
                .sum::<f64>()
                / self.load_history.len() as f64
        };

        AdaptiveParallelismMetrics {
            current_workers: self.current_workers.load(Ordering::Relaxed),
            min_workers: self.min_workers,
            max_workers: self.max_workers,
            current_load_score: current_load.load_score(),
            average_load_score: avg_load,
            load_history_size: self.load_history.len(),
            last_adjustment: self.last_adjustment.elapsed(),
        }
    }
}

#[derive(Debug)]
pub struct AdaptiveParallelismMetrics {
    pub current_workers: usize,
    pub min_workers: usize,
    pub max_workers: usize,
    pub current_load_score: f64,
    pub average_load_score: f64,
    pub load_history_size: usize,
    pub last_adjustment: Duration,
}

/// System load monitor
pub struct SystemLoadMonitor {
    controller: Arc<AdaptiveParallelismController>,
    monitoring_interval: Duration,
    enabled: Arc<AtomicUsize>, // 0 = disabled, 1 = enabled
}

impl SystemLoadMonitor {
    pub fn new(controller: Arc<AdaptiveParallelismController>) -> Self {
        Self {
            controller,
            monitoring_interval: Duration::from_secs(2),
            enabled: Arc::new(AtomicUsize::new(1)),
        }
    }

    /// Start monitoring system load in background
    pub async fn start_monitoring(&self) -> Result<()> {
        let controller = Arc::clone(&self.controller);
        let enabled = Arc::clone(&self.enabled);
        let interval = self.monitoring_interval;

        tokio::spawn(async move {
            let mut interval_timer = time::interval(interval);

            loop {
                interval_timer.tick().await;

                if enabled.load(Ordering::Relaxed) == 0 {
                    break;
                }

                // Measure system load
                let load = Self::measure_system_load().await;

                if let Err(e) = controller.update_load(load).await {
                    eprintln!("Error updating system load: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Stop monitoring
    pub fn stop_monitoring(&self) {
        self.enabled.store(0, Ordering::Relaxed);
    }

    /// Measure current system load
    async fn measure_system_load() -> SystemLoad {
        let mut load = SystemLoad::new();

        // Get CPU usage (simplified - in real implementation would use system APIs)
        load.cpu_usage = Self::get_cpu_usage().await;

        // Get memory usage
        load.memory_usage = Self::get_memory_usage().await;

        // Get I/O wait (simplified)
        load.io_wait = Self::get_io_wait().await;

        // Get load average
        load.load_average = Self::get_load_average().await;

        load
    }

    async fn get_cpu_usage() -> f64 {
        // Simplified CPU usage measurement
        // In a real implementation, this would use system APIs like sysinfo or procfs
        // For now, return a simulated value based on system load
        let loadavg = Self::get_load_average().await;
        (loadavg / num_cpus::get() as f64).clamp(0.0, 1.0)
    }

    async fn get_memory_usage() -> f64 {
        // Simplified memory usage measurement
        // In a real implementation, this would use system APIs
        // For now, return a conservative estimate
        0.3 // Assume 30% memory usage
    }

    async fn get_io_wait() -> f64 {
        // Simplified I/O wait measurement
        // In a real implementation, this would use system APIs
        0.1 // Assume 10% I/O wait
    }

    async fn get_load_average() -> f64 {
        // Get system load average
        // In a real implementation, this would read from /proc/loadavg or similar
        // For now, return a simulated value
        let base_load = std::thread::available_parallelism()
            .map(|p| p.get() as f64 * 0.5)
            .unwrap_or(2.0);
        base_load.min(8.0) // Cap at reasonable value
    }
}
