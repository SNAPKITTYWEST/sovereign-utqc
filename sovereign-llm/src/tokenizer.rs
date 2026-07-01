//! Tokenizer

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenizer {
    pub vocab_size: usize,
    pub pad_token: usize,
    pub eos_token: usize,
}

impl Tokenizer {
    pub fn new(vocab_size: usize) -> Self {
        Self { vocab_size, pad_token: 0, eos_token: 1 }
    }

    pub fn encode(&self, text: &str) -> Vec<usize> {
        text.bytes().map(|b| b as usize % self.vocab_size).collect()
    }

    pub fn decode(&self, tokens: &[usize]) -> String {
        tokens.iter().map(|&t| (t % 128) as u8 as char).collect()
    }
}
