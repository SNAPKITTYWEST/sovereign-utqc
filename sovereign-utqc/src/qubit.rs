//! Qubit representation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qubit {
    pub id: usize,
    pub alpha: f64,
    pub beta: f64,
}

impl Qubit {
    pub fn new(id: usize) -> Self {
        Self { id, alpha: 1.0, beta: 0.0 }
    }

    pub fn measure(&self) -> bool {
        let prob_zero = self.alpha * self.alpha;
        rand::random::<f64>() < prob_zero
    }

    pub fn apply_phase(&mut self, phase: f64) {
        let cos = phase.cos();
        let sin = phase.sin();
        let new_alpha = self.alpha * cos - self.beta * sin;
        let new_beta = self.alpha * sin + self.beta * cos;
        self.alpha = new_alpha;
        self.beta = new_beta;
    }
}
