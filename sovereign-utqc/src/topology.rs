//! Topological surface codes

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologicalSurface {
    pub rows: usize,
    pub cols: usize,
    pub stabilizers: Vec<(usize, usize)>,
}

impl TopologicalSurface {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut stabilizers = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                stabilizers.push((r, c));
            }
        }
        Self { rows, cols, stabilizers }
    }

    pub fn distance(&self) -> usize {
        self.rows.min(self.cols)
    }

    pub fn logical_error_rate(&self, physical_error_rate: f64) -> f64 {
        let d = self.distance() as f64;
        physical_error_rate.powf(d / 2.0)
    }
}
