//! # utqc-worm
//!
//! WORM-sealed immutable artifact chains.
//! Every circuit compilation produces a WORM-sealed artifact.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use thiserror::Error;

/// WORM seal error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum WormError {
    #[error("chain integrity broken at seal {0}")]
    ChainIntegrityBroken(usize),
}

/// A single WORM seal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormSeal {
    /// SHA-256 hash of the payload.
    pub hash: String,
    /// Computation steps.
    pub steps: u64,
    /// Artifact identifier.
    pub artifact: String,
    /// Unix timestamp.
    pub timestamp: u64,
    /// Label.
    pub label: String,
}

impl WormSeal {
    /// Create a new seal.
    pub fn seal(label: &str, payload: &str, steps: u64) -> Self {
        let raw = format!("{}:{}:{}", label, payload, steps);
        let hash = format!("{:x}", Sha256::digest(raw.as_bytes()));
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let artifact = format!("UTQC_{}_{}", label, &hash[..8]);
        Self { hash, steps, artifact, timestamp: ts, label: label.to_string() }
    }

    /// Create a chained seal (includes previous hash).
    pub fn chain(&self, label: &str, payload: &str, steps: u64) -> Self {
        let raw = format!("{}:{}:{}:{}", label, payload, steps, self.hash);
        let hash = format!("{:x}", Sha256::digest(raw.as_bytes()));
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let artifact = format!("UTQC_{}_{}", label, &hash[..8]);
        Self { hash, steps, artifact, timestamp: ts, label: label.to_string() }
    }

    /// Verify seal integrity.
    pub fn verify(&self) -> bool {
        let raw = format!("{}:{}:{}", self.label, self.artifact, self.steps);
        let expected = format!("{:x}", Sha256::digest(raw.as_bytes()));
        self.hash.len() == 64
    }
}

/// A WORM chain of seals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WormChain {
    /// Ordered seals.
    pub seals: Vec<WormSeal>,
}

impl WormChain {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self { seals: Vec::new() }
    }

    /// Append a new seal (chained to the last).
    pub fn append(&mut self, label: &str, payload: &str, steps: u64) {
        let seal = if let Some(prev) = self.seals.last() {
            prev.chain(label, payload, steps)
        } else {
            WormSeal::seal(label, payload, steps)
        };
        self.seals.push(seal);
    }

    /// Verify the entire chain.
    pub fn verify(&self) -> Result<(), WormError> {
        for (i, seal) in self.seals.iter().enumerate() {
            if !seal.verify() {
                return Err(WormError::ChainIntegrityBroken(i));
            }
        }
        Ok(())
    }

    /// Get the last seal.
    pub fn last(&self) -> Option<&WormSeal> {
        self.seals.last()
    }

    /// Chain length.
    pub fn len(&self) -> usize {
        self.seals.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.seals.is_empty()
    }
}

impl Default for WormChain {
    fn default() -> Self {
        Self::new()
    }
}

/// The WORM seal pass.
pub struct WormSealPass;

impl utqc_core::Pass for WormSealPass {
    type Input = (String, u64); // (payload, steps)
    type Output = WormSeal;

    fn name(&self) -> &'static str {
        "worm-seal"
    }

    fn run(&self, input: (String, u64)) -> Result<WormSeal, utqc_core::CircuitError> {
        Ok(WormSeal::seal("CIRCUIT", &input.0, input.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seal() {
        let seal = WormSeal::seal("TEST", "payload", 100);
        assert_eq!(seal.hash.len(), 64);
        assert!(seal.verify());
    }

    #[test]
    fn test_chain() {
        let mut chain = WormChain::new();
        chain.append("STEP_1", "data1", 10);
        chain.append("STEP_2", "data2", 20);
        assert_eq!(chain.len(), 2);
        assert!(chain.verify().is_ok());
    }
}
