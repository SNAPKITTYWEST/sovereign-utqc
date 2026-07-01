//! # utqc-quantum
//!
//! Quantum algorithms — QFT, Grover, Shor, QPE.
//! Every algorithm compiles to the circuit IR.

use utqc_core::{Circuit, Gate, Qubit, SingleGate, DoubleGate, CircuitError};
use utqc_goldilocks::Goldilocks;
use thiserror::Error;

/// Quantum algorithm error.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantumError {
    #[error("circuit error: {0}")]
    Circuit(#[from] CircuitError),
}

/// Quantum Fourier Transform on n qubits.
pub struct Qft;

impl Qft {
    /// Build a QFT circuit for `num_qubits` starting at qubit `start`.
    pub fn circuit(num_qubits: usize, start: usize) -> Result<Circuit, QuantumError> {
        let mut circ = Circuit::new(num_qubits, num_qubits);
        for i in 0..num_qubits {
            circ.add_gate(Gate::Single {
                gate: SingleGate::Hadamard,
                target: Qubit(start + i),
            })?;
            for j in (i + 1)..num_qubits {
                let angle = std::f64::consts::PI / (1u64 << (j - i)) as f64;
                circ.add_gate(Gate::Double {
                    gate: DoubleGate::CNOT,
                    control: Qubit(start + j),
                    target: Qubit(start + i),
                })?;
                circ.add_gate(Gate::Rotation {
                    target: Qubit(start + i),
                    angle,
                })?;
            }
        }
        // Swap qubits for standard ordering
        for i in 0..(num_qubits / 2) {
            circ.add_gate(Gate::Double {
                gate: DoubleGate::SWAP,
                control: Qubit(start + i),
                target: Qubit(start + num_qubits - 1 - i),
            })?;
        }
        Ok(circ)
    }
}

/// Grover's search algorithm.
pub struct Grover;

impl Grover {
    /// Build a Grover circuit for `num_qubits` with `num_solutions` marked states.
    pub fn circuit(num_qubits: usize, num_solutions: usize) -> Result<Circuit, QuantumError> {
        let mut circ = Circuit::new(num_qubits, num_qubits);

        // Initialize: Hadamard on all qubits
        for i in 0..num_qubits {
            circ.add_gate(Gate::Single {
                gate: SingleGate::Hadamard,
                target: Qubit(i),
            })?;
        }

        // Number of Grover iterations
        let iterations = Self::optimal_iterations(num_qubits, num_solutions);
        for _ in 0..iterations {
            // Oracle (mark solution states) — placeholder oracle
            Self::add_oracle(&mut circ, num_qubits)?;
            // Diffusion operator
            Self::add_diffusion(&mut circ, num_qubits)?;
        }

        // Measure all
        for i in 0..num_qubits {
            circ.add_measurement(Qubit(i), i)?;
        }

        Ok(circ)
    }

    /// Optimal number of Grover iterations.
    pub fn optimal_iterations(num_qubits: usize, num_solutions: usize) -> usize {
        let n = 1u64 << num_qubits;
        let m = num_solutions as f64;
        let ratio = n as f64 / m;
        let theta = (m / n as f64).sqrt().asin();
        let optimal = (std::f64::consts::FRAC_PI_4 / theta).round() as usize;
        optimal.max(1)
    }

    /// Add oracle gates (placeholder — marks first solution).
    fn add_oracle(circ: &mut Circuit, num_qubits: usize) -> Result<(), QuantumError> {
        // Placeholder: multi-controlled Z on all qubits
        if num_qubits >= 2 {
            circ.add_gate(Gate::Double {
                gate: DoubleGate::CZ,
                control: Qubit(0),
                target: Qubit(num_qubits - 1),
            })?;
        }
        Ok(())
    }

    /// Add diffusion operator.
    fn add_diffusion(circ: &mut Circuit, num_qubits: usize) -> Result<(), QuantumError> {
        // H on all, X on all, multi-controlled Z, X on all, H on all
        for i in 0..num_qubits {
            circ.add_gate(Gate::Single {
                gate: SingleGate::Hadamard,
                target: Qubit(i),
            })?;
            circ.add_gate(Gate::Single {
                gate: SingleGate::PauliX,
                target: Qubit(i),
            })?;
        }
        if num_qubits >= 2 {
            circ.add_gate(Gate::Double {
                gate: DoubleGate::CZ,
                control: Qubit(0),
                target: Qubit(num_qubits - 1),
            })?;
        }
        for i in 0..num_qubits {
            circ.add_gate(Gate::Single {
                gate: SingleGate::PauliX,
                target: Qubit(i),
            })?;
            circ.add_gate(Gate::Single {
                gate: SingleGate::Hadamard,
                target: Qubit(i),
            })?;
        }
        Ok(())
    }
}

/// Quantum Phase Estimation.
pub struct Qpe;

impl Qpe {
    /// Build a QPE circuit.
    pub fn circuit(num_counting_qubits: usize, target_qubit: usize) -> Result<Circuit, QuantumError> {
        let total = num_counting_qubits + 1;
        let mut circ = Circuit::new(total, num_counting_qubits);

        // Hadamard on counting qubits
        for i in 0..num_counting_qubits {
            circ.add_gate(Gate::Single {
                gate: SingleGate::Hadamard,
                target: Qubit(i),
            })?;
        }

        // Controlled unitary powers
        for i in 0..num_counting_qubits {
            let power = 1u64 << i;
            for _ in 0..power {
                circ.add_gate(Gate::Double {
                    gate: DoubleGate::CNOT,
                    control: Qubit(i),
                    target: Qubit(target_qubit),
                })?;
            }
        }

        // Inverse QFT on counting qubits
        let qft_circ = Qft::circuit(num_counting_qubits, 0)?;
        // Append gates in reverse (simplified — just add H gates)
        for i in 0..num_counting_qubits {
            circ.add_gate(Gate::Single {
                gate: SingleGate::Hadamard,
                target: Qubit(i),
            })?;
        }

        // Measure counting qubits
        for i in 0..num_counting_qubits {
            circ.add_measurement(Qubit(i), i)?;
        }

        Ok(circ)
    }
}

/// Shor's algorithm scaffold.
pub struct Shor;

impl Shor {
    /// Build a Shor circuit for factoring N (simplified).
    pub fn circuit(num_qubits: usize) -> Result<Circuit, QuantumError> {
        // Shor = QPE + modular exponentiation
        // Simplified: just build QPE on half the qubits
        let counting = num_qubits / 2;
        Qpe::circuit(counting, counting)
    }
}

/// Compile a circuit to Goldilocks field representation.
pub fn compile_to_goldilocks(circuit: &Circuit) -> Vec<Goldilocks> {
    circuit.gates.iter().map(|g| {
        match g {
            Gate::Single { gate, .. } => Goldilocks::new(*gate as u8 as u64),
            Gate::Double { gate, .. } => Goldilocks::new(*gate as u8 as u64 + 100),
            Gate::Rotation { angle, .. } => Goldilocks::new((*angle * 1000.0) as u64),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qft() {
        let circ = Qft::circuit(3, 0).unwrap();
        assert!(circ.depth() > 0);
        assert!(circ.validate().is_ok());
    }

    #[test]
    fn test_grover() {
        let circ = Grover::circuit(3, 1).unwrap();
        assert!(circ.depth() > 0);
        assert!(circ.validate().is_ok());
    }

    #[test]
    fn test_qpe() {
        let circ = Qpe::circuit(2, 2).unwrap();
        assert!(circ.depth() > 0);
    }

    #[test]
    fn test_goldilocks_compile() {
        let circ = Qft::circuit(2, 0).unwrap();
        let field_elements = compile_to_goldilocks(&circ);
        assert!(!field_elements.is_empty());
    }
}
