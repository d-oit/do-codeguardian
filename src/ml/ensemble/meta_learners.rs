use crate::ml::cross_validation::Classifier;
use anyhow::{anyhow, Result};

/// Simple linear regression meta-learner
#[derive(Clone)]
pub struct SimpleLinearRegression {
    weights: Vec<f64>,
    bias: f64,
    trained: bool,
}

impl SimpleLinearRegression {
    pub fn new() -> Self {
        Self {
            weights: Vec::new(),
            bias: 0.0,
            trained: false,
        }
    }
}

impl Classifier for SimpleLinearRegression {
    async fn train(&mut self, data: &[(Vec<f32>, f32)]) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow!("No training data provided"));
        }

        let n_features = data[0].0.len();
        let n_samples = data.len();

        // Initialize weights
        self.weights = vec![0.0; n_features];
        self.bias = 0.0;

        // Simple gradient descent
        let learning_rate = 0.01;
        let epochs = 1000;

        for _epoch in 0..epochs {
            let mut weight_gradients = vec![0.0; n_features];
            let mut bias_gradient = 0.0;

            for (features, target) in data {
                // Forward pass
                let prediction = self.predict_internal(features);
                let error = prediction - *target as f64;

                // Backward pass
                for (i, &feature) in features.iter().enumerate() {
                    weight_gradients[i] += error * feature as f64;
                }
                bias_gradient += error;
            }

            // Update weights
            for (i, gradient) in weight_gradients.iter().enumerate() {
                self.weights[i] -= learning_rate * gradient / n_samples as f64;
            }
            self.bias -= learning_rate * bias_gradient / n_samples as f64;
        }

        self.trained = true;
        Ok(())
    }

    async fn predict(&self, features: &[f32]) -> Result<f32> {
        if !self.trained {
            return Err(anyhow!("Model not trained"));
        }

        Ok(self.predict_internal(features) as f32)
    }
}

impl SimpleLinearRegression {
    fn predict_internal(&self, features: &[f32]) -> f64 {
        let prediction = features
            .iter()
            .zip(self.weights.iter())
            .map(|(f, w)| *f as f64 * w)
            .sum::<f64>()
            + self.bias;

        prediction.clamp(0.0, 1.0)
    }
}

/// Simple logistic regression meta-learner
#[derive(Clone)]
pub struct SimpleLogisticRegression {
    weights: Vec<f64>,
    bias: f64,
    trained: bool,
}

impl SimpleLogisticRegression {
    pub fn new() -> Self {
        Self {
            weights: Vec::new(),
            bias: 0.0,
            trained: false,
        }
    }

    fn sigmoid(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
}

impl Classifier for SimpleLogisticRegression {
    async fn train(&mut self, data: &[(Vec<f32>, f32)]) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow!("No training data provided"));
        }

        let n_features = data[0].0.len();
        let n_samples = data.len();

        // Initialize weights
        self.weights = vec![0.0; n_features];
        self.bias = 0.0;

        // Logistic regression with gradient descent
        let learning_rate = 0.1;
        let epochs = 1000;

        for _epoch in 0..epochs {
            let mut weight_gradients = vec![0.0; n_features];
            let mut bias_gradient = 0.0;

            for (features, target) in data {
                // Forward pass
                let prediction = self.predict_internal(features);
                let error = prediction - *target as f64;

                // Backward pass
                for (i, &feature) in features.iter().enumerate() {
                    weight_gradients[i] += error * feature as f64;
                }
                bias_gradient += error;
            }

            // Update weights
            for (i, gradient) in weight_gradients.iter().enumerate() {
                self.weights[i] -= learning_rate * gradient / n_samples as f64;
            }
            self.bias -= learning_rate * bias_gradient / n_samples as f64;
        }

        self.trained = true;
        Ok(())
    }

    async fn predict(&self, features: &[f32]) -> Result<f32> {
        if !self.trained {
            return Err(anyhow!("Model not trained"));
        }

        Ok(self.predict_internal(features) as f32)
    }
}

impl SimpleLogisticRegression {
    fn predict_internal(&self, features: &[f32]) -> f64 {
        let linear_combination = features
            .iter()
            .zip(self.weights.iter())
            .map(|(f, w)| *f as f64 * w)
            .sum::<f64>()
            + self.bias;

        self.sigmoid(linear_combination)
    }
}