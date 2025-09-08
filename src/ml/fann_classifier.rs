use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[cfg(feature = "ml")]
use fann::{ActivationFunc, Fann};

/// Lightweight neural network classifier using FANN
#[cfg(feature = "ml")]
pub struct FannClassifier {
    network: Fann,
    input_size: usize,
}

/// Stub implementation when ML feature is disabled
#[cfg(not(feature = "ml"))]
pub struct FannClassifier {
    input_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub input_size: usize,
    pub hidden_layers: Vec<usize>,
    pub output_size: usize,
    pub activation_function: String,
}

#[cfg(feature = "ml")]
impl FannClassifier {
    /// Create a new classifier with the given configuration
    pub fn new(config: NetworkConfig) -> Result<Self> {
        let mut layers: Vec<u32> = vec![config.input_size as u32];
        layers.extend(config.hidden_layers.iter().map(|&x| x as u32));
        layers.push(config.output_size as u32);

        let mut network =
            Fann::new(&layers).map_err(|e| anyhow!("Failed to create FANN network: {:?}", e))?;

        // Configure network
        // Note: FANN crate may not have set_learning_rate method
        // network.set_learning_rate(config.learning_rate);
        network.set_activation_func_hidden(ActivationFunc::Sigmoid);
        network.set_activation_func_output(ActivationFunc::Sigmoid);

        Ok(Self {
            network,
            input_size: config.input_size,
        })
    }

    /// Load a pre-trained model from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let network = Fann::from_file(path.as_ref().to_str().unwrap())
            .map_err(|e| anyhow!("Failed to load FANN network: {:?}", e))?;

        // Extract configuration from loaded network
        let input_size = network.get_num_input() as usize;

        Ok(Self {
            network,
            input_size,
        })
    }

    /// Save the trained model to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.network
            .save(path.as_ref().to_str().unwrap())
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

        let output = self
            .network
            .run(features)
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

        self.network
            .train(features, &target_output)
            .map_err(|e| anyhow!("FANN training failed: {:?}", e))?;

        Ok(())
    }

    /// Batch training with multiple examples
    pub fn train_batch(&mut self, training_data: &[(Vec<f32>, f32)], epochs: u32) -> Result<f32> {
        let mut total_error = 0.0;

        for _ in 0..epochs {
            let mut epoch_error = 0.0;

            for (features, target) in training_data {
                self.train_incremental(features, *target)?;

                // Calculate error for this example
                let prediction = self.predict(features)?;
                epoch_error += (prediction - target).powi(2);
            }

            total_error = epoch_error / training_data.len() as f32;
        }

        Ok(total_error)
    }

    /// Get network statistics
    pub fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            input_size: self.input_size,
            hidden_layers: self.network.get_num_layers() as usize - 2, // Exclude input/output
            total_neurons: self.network.get_total_neurons() as usize,
            total_connections: self.network.get_total_connections() as usize,
        }
    }
}

/// Stub implementation when ML feature is disabled
#[cfg(not(feature = "ml"))]
impl FannClassifier {
    /// Create a new classifier with the given configuration (stub)
    pub fn new(config: NetworkConfig) -> Result<Self> {
        Ok(Self {
            input_size: config.input_size,
        })
    }

    /// Load a pre-trained model from file (stub)
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self> {
        Err(anyhow!("ML feature is disabled. Enable with --features ml"))
    }

    /// Save the trained model to file (stub)
    pub fn save<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
        Err(anyhow!("ML feature is disabled. Enable with --features ml"))
    }

    /// Predict output for given input features (stub)
    pub fn predict(&self, _features: &[f32]) -> Result<f32> {
        Ok(0.5) // Neutral score when ML is disabled
    }

    /// Train the network with a single example (stub)
    pub fn train_incremental(&mut self, _features: &[f32], _target: f32) -> Result<()> {
        Ok(()) // No-op when ML is disabled
    }

    /// Batch training with multiple examples (stub)
    pub fn train_batch(&mut self, _training_data: &[(Vec<f32>, f32)], _epochs: u32) -> Result<f32> {
        Ok(0.0) // Return zero error for stub
    }

    /// Get network statistics (stub)
    pub fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            input_size: self.input_size,
            hidden_layers: 0,
            total_neurons: 0,
            total_connections: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub input_size: usize,
    pub hidden_layers: usize,
    pub total_neurons: usize,
    pub total_connections: usize,
}

impl std::fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FANN Network: {} inputs, {} hidden layers, {} neurons, {} connections",
            self.input_size, self.hidden_layers, self.total_neurons, self.total_connections
        )
    }
}

/// Default configuration for CodeGuardian's use case
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            input_size: if cfg!(feature = "ast") { 24 } else { 8 }, // Enhanced feature vector size
            hidden_layers: if cfg!(feature = "ast") {
                vec![32, 16, 8] // Deeper network for more features
            } else {
                vec![12, 8] // Original network for base features
            },
            output_size: 1, // Single relevance score
            activation_function: "sigmoid".to_string(),
        }
    }
}

impl NetworkConfig {
    /// Create configuration optimized for AST-enhanced features
    pub fn enhanced() -> Self {
        Self {
            input_size: 24,                  // 8 base + 16 AST features
            hidden_layers: vec![48, 24, 12], // Larger network for complex patterns
            output_size: 1,
            activation_function: "sigmoid".to_string(),
        }
    }

    /// Create configuration for base features only
    pub fn basic() -> Self {
        Self {
            input_size: 8,
            hidden_layers: vec![12, 8],
            output_size: 1,
            activation_function: "sigmoid".to_string(),
        }
    }
}
