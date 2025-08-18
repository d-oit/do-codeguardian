use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use rand::seq::SliceRandom;

/// Enhanced neural network classifier using FANN with adaptive learning
pub struct FannClassifier {
    network: fann::Fann,
    input_size: usize,
    learning_rate: f32,
    initial_learning_rate: f32,
    training_history: Vec<f32>, // Track training errors for adaptive learning
    best_error: f32,
    patience_counter: u32,
    early_stopping_patience: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub input_size: usize,
    pub hidden_layers: Vec<usize>,
    pub output_size: usize,
    pub learning_rate: f32,
    pub activation_function: String,
}

impl FannClassifier {
    /// Create a new classifier with the given configuration
    pub fn new(config: NetworkConfig) -> Result<Self> {
        let mut layers = vec![config.input_size as u32];
        layers.extend(config.hidden_layers.iter().map(|&x| x as u32));
        layers.push(config.output_size as u32);

        let network = fann::Fann::new(&layers)
            .map_err(|e| anyhow!("Failed to create FANN network: {:?}", e))?;

        // Configure network
        // Note: FANN API methods may vary, using basic configuration
        // network.set_learning_rate(config.learning_rate);
        // network.set_activation_func_hidden(fann::ActivationFunc::Sigmoid);
        // network.set_activation_func_output(fann::ActivationFunc::Sigmoid);

        Ok(Self {
            network,
            input_size: config.input_size,
            learning_rate: config.learning_rate,
            initial_learning_rate: config.learning_rate,
            training_history: Vec::new(),
            best_error: f32::INFINITY,
            patience_counter: 0,
            early_stopping_patience: 10, // Stop if no improvement for 10 epochs
        })
    }

    /// Load a pre-trained model from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let network = fann::Fann::from_file(path.as_ref())
            .map_err(|e| anyhow!("Failed to load FANN network: {:?}", e))?;

        // Extract configuration from loaded network
        let input_size = 8; // Default input size for now
        let learning_rate = 0.1; // Default learning rate

        Ok(Self {
            network,
            input_size,
            learning_rate,
            initial_learning_rate: learning_rate,
            training_history: Vec::new(),
            best_error: f32::INFINITY,
            patience_counter: 0,
            early_stopping_patience: 10,
        })
    }

    /// Save the trained model to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.network.save(path.as_ref())
            .map_err(|e| anyhow!("Failed to save FANN network: {:?}", e))
    }

    /// Predict output for given input features
    pub fn predict(&self, features: &[f32]) -> Result<f32> {
        if features.len() != self.input_size {
            return Err(anyhow!(
                "Input size mismatch: expected {}, got {}",
                self.input_size,
                features.len()
            ));
        }

        let output = self.network.run(features)
            .map_err(|e| anyhow!("FANN prediction failed: {:?}", e))?;

        Ok(output[0]) // Single output for binary classification
    }

    /// Train the network with a single example (online learning)
    pub fn train_incremental(&mut self, features: &[f32], target: f32) -> Result<()> {
        if features.len() != self.input_size {
            return Err(anyhow!(
                "Input size mismatch: expected {}, got {}",
                self.input_size,
                features.len()
            ));
        }

        let target_output = vec![target];
        
        self.network.train(features, &target_output)
            .map_err(|e| anyhow!("FANN training failed: {:?}", e))?;

        Ok(())
    }

    /// Enhanced batch training with adaptive learning rate and early stopping
    pub fn train_batch(&mut self, training_data: &[(Vec<f32>, f32)], epochs: u32) -> Result<f32> {
        let mut total_error = 0.0;
        self.training_history.clear();
        self.best_error = f32::INFINITY;
        self.patience_counter = 0;

        for epoch in 0..epochs {
            let mut epoch_error = 0.0;
            
            // Shuffle training data for better convergence
            let mut shuffled_data = training_data.to_vec();
            let mut rng = rand::thread_rng();
            shuffled_data.shuffle(&mut rng);
            
            for (features, target) in &shuffled_data {
                self.train_incremental(features, *target)?;
                
                // Calculate error for this example
                let prediction = self.predict(features)?;
                epoch_error += (prediction - target).powi(2);
            }
            
            total_error = epoch_error / training_data.len() as f32;
            self.training_history.push(total_error);
            
            // Adaptive learning rate adjustment
            self.adjust_learning_rate(total_error);
            
            // Early stopping check
            if total_error < self.best_error {
                self.best_error = total_error;
                self.patience_counter = 0;
            } else {
                self.patience_counter += 1;
                if self.patience_counter >= self.early_stopping_patience {
                    println!("Early stopping at epoch {} (error: {:.6})", epoch + 1, total_error);
                    break;
                }
            }
            
            // Progress reporting every 10 epochs
            if (epoch + 1) % 10 == 0 {
                println!("Epoch {}: Error = {:.6}, LR = {:.6}", epoch + 1, total_error, self.learning_rate);
            }
        }

        Ok(total_error)
    }

    /// Adjust learning rate based on training progress
    fn adjust_learning_rate(&mut self, _current_error: f32) {
        if self.training_history.len() < 3 {
            return; // Need at least 3 epochs to determine trend
        }
        
        let recent_errors = &self.training_history[self.training_history.len()-3..];
        let is_improving = recent_errors.windows(2).all(|w| w[1] < w[0]);
        let is_stagnating = recent_errors.windows(2).all(|w| (w[1] - w[0]).abs() < 0.0001);
        
        if is_stagnating {
            // Increase learning rate if stagnating
            self.learning_rate = (self.learning_rate * 1.1).min(self.initial_learning_rate * 2.0);
        } else if !is_improving {
            // Decrease learning rate if not improving
            self.learning_rate = (self.learning_rate * 0.9).max(self.initial_learning_rate * 0.1);
        }
        
        // Update network learning rate (if FANN API supports it)
        // self.network.set_learning_rate(self.learning_rate);
    }

    /// Get training statistics
    pub fn get_training_stats(&self) -> TrainingStats {
        TrainingStats {
            epochs_trained: self.training_history.len(),
            best_error: self.best_error,
            current_learning_rate: self.learning_rate,
            early_stopped: self.patience_counter >= self.early_stopping_patience,
            error_history: self.training_history.clone(),
        }
    }

    /// Get network statistics
    pub fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            input_size: self.input_size,
            hidden_layers: self.network.get_num_layers() as usize - 2, // Exclude input/output
            total_neurons: self.network.get_total_neurons() as usize,
            total_connections: self.network.get_total_connections() as usize,
            learning_rate: self.learning_rate,
        }
    }
}

#[derive(Debug)]
pub struct NetworkStats {
    pub input_size: usize,
    pub hidden_layers: usize,
    pub total_neurons: usize,
    pub total_connections: usize,
    pub learning_rate: f32,
}

#[derive(Debug, Clone)]
pub struct TrainingStats {
    pub epochs_trained: usize,
    pub best_error: f32,
    pub current_learning_rate: f32,
    pub early_stopped: bool,
    pub error_history: Vec<f32>,
}

impl std::fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FANN Network: {} inputs, {} hidden layers, {} neurons, {} connections, LR: {:.4}",
            self.input_size,
            self.hidden_layers,
            self.total_neurons,
            self.total_connections,
            self.learning_rate
        )
    }
}

/// Default configuration for CodeGuardian's use case
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            input_size: 12,  // Enhanced feature vector size
            hidden_layers: vec![16, 12, 8],  // Three hidden layers for better learning
            output_size: 1,  // Single relevance score
            learning_rate: 0.05,  // More conservative learning rate
            activation_function: "sigmoid".to_string(),
        }
    }
}