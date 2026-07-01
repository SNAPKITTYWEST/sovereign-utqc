//! Trainer

use crate::model::Model;

pub struct Trainer {
    pub learning_rate: f32,
    pub batch_size: usize,
    pub steps: usize,
}

impl Trainer {
    pub fn new(learning_rate: f32, batch_size: usize) -> Self {
        Self { learning_rate, batch_size, steps: 0 }
    }

    pub fn train_step(&mut self, _model: &mut Model, _loss: f32) {
        self.steps += 1;
    }
}
