//! # utqc-linear
//!
//! Linear type resource rules for quantum circuits.
//! enforces: lin consumed once, aff at most once, un unlimited.

use utqc_core::Circuit;
use thiserror::Error;

/// Linear type resource error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum LinearError {
    /// Qubit already consumed (linear violation).
    #[error("qubit {0} already consumed (linear violation)")]
    QubitAlreadyConsumed(usize),

    /// Qubit not available.
    #[error("qubit {0} not available in scope")]
    QubitNotAvailable(usize),

    /// Resource leak (affine type not consumed).
    #[error("resource leak: qubit {0} (affine) not consumed")]
    ResourceLeak(usize),
}

/// Resource usage kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    /// Consumed exactly once.
    Linear,
    /// Consumed at most once.
    Affine,
    /// Unrestricted reuse.
    Unrestricted,
}

/// Track resource usage per qubit.
#[derive(Debug, Clone)]
pub struct ResourceTracker {
    /// Map from qubit index to (kind, usage_count).
    resources: Vec<(ResourceKind, usize)>,
}

impl ResourceTracker {
    /// Create a new tracker for the given number of qubits.
    pub fn new(num_qubits: usize) -> Self {
        Self {
            resources: vec![(ResourceKind::Linear, 0); num_qubits],
        }
    }

    /// Mark a qubit as used.
    pub fn use_qubit(&mut self, qubit: usize) -> Result<(), LinearError> {
        if qubit >= self.resources.len() {
            return Err(LinearError::QubitNotAvailable(qubit));
        }
        let (kind, count) = &mut self.resources[qubit];
        match kind {
            ResourceKind::Linear => {
                if *count > 0 {
                    return Err(LinearError::QubitAlreadyConsumed(qubit));
                }
                *count += 1;
            }
            ResourceKind::Affine => {
                if *count > 0 {
                    return Err(LinearError::QubitAlreadyConsumed(qubit));
                }
                *count += 1;
            }
            ResourceKind::Unrestricted => {
                *count += 1;
            }
        }
        Ok(())
    }

    /// Verify no affine resources leaked.
    pub fn verify_no_leaks(&self) -> Result<(), LinearError> {
        for (i, (kind, count)) in self.resources.iter().enumerate() {
            if *kind == ResourceKind::Affine && *count == 0 {
                return Err(LinearError::ResourceLeak(i));
            }
        }
        Ok(())
    }

    /// Check linearity of a circuit.
    /// Control qubits are read-only (not consumed). Only target qubits are consumed.
    pub fn check_circuit(circuit: &Circuit) -> Result<Self, LinearError> {
        let mut tracker = Self::new(circuit.num_qubits);
        for gate in &circuit.gates {
            match gate {
                utqc_core::Gate::Single { target, .. } => {
                    tracker.use_qubit(target.0)?;
                }
                utqc_core::Gate::Double { target, .. } => {
                    // Control qubit is read-only — not consumed
                    tracker.use_qubit(target.0)?;
                }
                utqc_core::Gate::Rotation { target, .. } => {
                    tracker.use_qubit(target.0)?;
                }
            }
        }
        Ok(tracker)
    }
}

/// The linear resource check pass.
pub struct LinearCheck;

impl utqc_core::Pass for LinearCheck {
    type Input = Circuit;
    type Output = Circuit;

    fn name(&self) -> &'static str {
        "linear-resource-check"
    }

    fn run(&self, input: Circuit) -> Result<Circuit, utqc_core::CircuitError> {
        ResourceTracker::check_circuit(&input).map_err(|e| {
            utqc_core::CircuitError::QubitOutOfBounds(0, 0) // Reuse error type
        })?;
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utqc_core::{Circuit, Gate, Qubit, SingleGate};

    #[test]
    fn test_linear_ok() {
        let mut circuit = Circuit::new(2, 2);
        circuit.add_gate(Gate::Single { gate: SingleGate::Hadamard, target: Qubit(0) }).unwrap();
        circuit.add_gate(Gate::Double { gate: utqc_core::DoubleGate::CNOT, control: Qubit(0), target: Qubit(1) }).unwrap();
        assert!(ResourceTracker::check_circuit(&circuit).is_ok());
    }

    #[test]
    fn test_linear_violation() {
        let mut circuit = Circuit::new(1, 1);
        circuit.add_gate(Gate::Single { gate: SingleGate::Hadamard, target: Qubit(0) }).unwrap();
        circuit.add_gate(Gate::Single { gate: SingleGate::PauliX, target: Qubit(0) }).unwrap();
        assert!(ResourceTracker::check_circuit(&circuit).is_err());
    }
}
