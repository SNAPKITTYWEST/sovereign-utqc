//! # sovereign-covenant
//!
//! SOVEREIGN.md covenant: local-first, non-surveillance, user-controlled.

use serde::{Deserialize, Serialize};

/// Sovereignty covenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyCovenant {
    /// Local-first default.
    pub local_first: bool,
    /// Non-surveillance guarantee.
    pub non_surveillance: bool,
    /// Data minimization.
    pub data_minimization: bool,
    /// User control.
    pub user_control: bool,
    /// Transparency.
    pub transparency: bool,
}

impl Default for SovereigntyCovenant {
    fn default() -> Self {
        Self {
            local_first: true,
            non_surveillance: true,
            data_minimization: true,
            user_control: true,
            transparency: true,
        }
    }
}

impl SovereigntyCovenant {
    /// Verify all commitments are met.
    pub fn verify(&self) -> bool {
        self.local_first
            && self.non_surveillance
            && self.data_minimization
            && self.user_control
            && self.transparency
    }
}

/// The full covenant text.
pub const COVENANT_TEXT: &str = r#"# SOVEREIGN BY DESIGN

## Core Commitments

### Local First
Local computation, storage, and verification are the default.
Remote services are optional adapters, not the source of truth.

### Non-Surveillance
No behavioral telemetry by default.
No hidden analytics pipelines.
No silent identifiers or undeclared data exports.

### Data Minimization
Collect only what is strictly necessary.
Retention limited to shortest period compatible with recovery and audit.

### User Control
The user controls their data, state, and execution environment.
Defaults preserve privacy and local autonomy.

### Transparency
Behavior must be inspectable.
Configuration must be visible.
Data flows must be explainable.

## Enforcement

This covenant is enforced by architecture, not by aspiration.
If an implementation diverges from this covenant, the implementation is wrong."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covenant_default() {
        let covenant = SovereigntyCovenant::default();
        assert!(covenant.verify());
    }

    #[test]
    fn test_covenant_text() {
        assert!(COVENANT_TEXT.contains("Local First"));
        assert!(COVENANT_TEXT.contains("Non-Surveillance"));
    }
}
