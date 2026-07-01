//! Model

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub params: usize,
    pub layers: usize,
    pub hidden_dim: usize,
    pub weights: Vec<f32>,
}

impl Model {
    pub fn tiny() -> Self {
        Self {
            name: "sovereign-tiny".to_string(),
            params: 1_100_000_000,
            layers: 22,
            hidden_dim: 2048,
            weights: Vec::new(),
        }
    }

    pub fn forward(&self, _input: &[f32]) -> Vec<f32> {
        // Simplified: return zeros
        vec![0.0; self.hidden_dim]
    }
}
