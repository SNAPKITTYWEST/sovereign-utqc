//! # sovereign-pirtm
//!
//! PIRTM compiler IR — Prime-Indexed Recursive Tensor Mathematics.
//! Non-recursive circuit lowering.

use utqc_core::{Circuit, Gate, Pass, CircuitError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// PIRTM error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PirtnError {
    #[error("invalid tensor shape")]
    InvalidShape,
}

/// Tensor operation IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TensorOp {
    /// Matrix multiply.
    MatMul { a: usize, b: usize, c: usize },
    /// Element-wise add.
    Add { a: usize, b: usize, out: usize },
    /// Reshape.
    Reshape { input: usize, shape: Vec<usize> },
}

/// PIRTM program: a sequence of tensor operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PirtnProgram {
    /// Operations.
    pub ops: Vec<TensorOp>,
    /// Number of tensors.
    pub num_tensors: usize,
}

impl PirtnProgram {
    /// Create new empty program.
    pub fn new(num_tensors: usize) -> Self {
        Self { ops: Vec::new(), num_tensors }
    }

    /// Add an operation.
    pub fn add_op(&mut self, op: TensorOp) {
        self.ops.push(op);
    }

    /// Lower to quantum circuit.
    pub fn lower_to_circuit(&self) -> Result<Circuit, PirtnError> {
        use utqc_core::{Gate, Qubit, SingleGate, DoubleGate};
        let n = self.num_tensors.max(2);
        let mut circuit = Circuit::new(n, n);
        for op in &self.ops {
            match op {
                TensorOp::MatMul { a, b, c } => {
                    // MatMul → Hadamard + CNOT sequence
                    let _ = c;
                    if *a < n && *b < n {
                        let _ = circuit.add_gate(Gate::Single { gate: SingleGate::Hadamard, target: Qubit(*a) });
                        let _ = circuit.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(*a), target: Qubit(*b) });
                    }
                }
                TensorOp::Add { a, b, .. } => {
                    if *a < n {
                        let _ = circuit.add_gate(Gate::Single { gate: SingleGate::PauliX, target: Qubit(*a) });
                    }
                    if *b < n {
                        let _ = circuit.add_gate(Gate::Single { gate: SingleGate::PauliX, target: Qubit(*b) });
                    }
                }
                TensorOp::Reshape { .. } => {}
            }
        }
        Ok(circuit)
    }
}

/// The PIRTM lowering pass.
pub struct PirtnLower;

impl Pass for PirtnLower {
    type Input = PirtnProgram;
    type Output = Circuit;

    fn name(&self) -> &'static str {
        "pirtm-lower"
    }

    fn run(&self, input: PirtnProgram) -> Result<Circuit, CircuitError> {
        input.lower_to_circuit().map_err(|_| CircuitError::EmptyCircuit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pirtm_lower() {
        let mut prog = PirtnProgram::new(4);
        prog.add_op(TensorOp::MatMul { a: 0, b: 1, c: 2 });
        let circuit = prog.lower_to_circuit().unwrap();
        assert!(circuit.validate().is_ok());
    }
}
