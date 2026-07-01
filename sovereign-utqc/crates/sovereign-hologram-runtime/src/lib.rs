//! # sovereign-hologram-runtime
//!
//! Hologram runtime — bridges compute substrate to verification substrate.

use serde::{Deserialize, Serialize};

/// Hologram configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HologramConfig {
    /// Enable WORM sealing.
    pub worm_seal: bool,
    /// Enable agent governance.
    pub agent_governance: bool,
    /// Max computation steps.
    pub max_steps: u64,
}

impl Default for HologramConfig {
    fn default() -> Self {
        Self { worm_seal: true, agent_governance: true, max_steps: 1_000_000 }
    }
}

/// Hologram runtime.
pub struct HologramRuntime {
    config: HologramConfig,
}

impl HologramRuntime {
    /// Create new runtime.
    pub fn new(config: HologramConfig) -> Self {
        Self { config }
    }

    /// Get config.
    pub fn config(&self) -> &HologramConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HologramConfig::default();
        assert!(config.worm_seal);
        assert!(config.agent_governance);
        assert_eq!(config.max_steps, 1_000_000);
    }
}
