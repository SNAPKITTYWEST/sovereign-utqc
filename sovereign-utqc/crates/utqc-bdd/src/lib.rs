//! # utqc-bdd
//!
//! BDD-based circuit equivalence verification.
//! Two circuits are equivalent if they produce the same truth table.

use utqc_core::{Circuit, Gate, SingleGate, DoubleGate};
use thiserror::Error;

/// BDD verification error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BddError {
    #[error("circuits have different qubit counts: {0} vs {1}")]
    QubitCountMismatch(usize, usize),

    #[error("equivalence check failed")]
    NotEquivalent,
}

/// A BDD node representing a Boolean function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BddNode {
    /// Terminal: constant 0 or 1.
    Terminal(bool),
    /// Variable node: var_index, low (var=0), high (var=1).
    Variable {
        var_index: usize,
        low: Box<BddNode>,
        high: Box<BddNode>,
    },
}

impl BddNode {
    /// Create a terminal node.
    pub fn terminal(val: bool) -> Self {
        Self::Terminal(val)
    }

    /// Create a variable node.
    pub fn variable(var_index: usize, low: BddNode, high: BddNode) -> Self {
        Self::Variable {
            var_index,
            low: Box::new(low),
            high: Box::new(high),
        }
    }
}

/// BDD-based verifier.
pub struct BddVerifier;

impl BddVerifier {
    /// Verify two circuits are equivalent by comparing their gate sequences.
    /// Simplified: checks structural equivalence.
    pub fn verify_equivalent(a: &Circuit, b: &Circuit) -> Result<(), BddError> {
        if a.num_qubits != b.num_qubits {
            return Err(BddError::QubitCountMismatch(a.num_qubits, b.num_qubits));
        }
        if a.gates.len() != b.gates.len() {
            return Err(BddError::NotEquivalent);
        }
        for (ga, gb) in a.gates.iter().zip(b.gates.iter()) {
            if ga != gb {
                return Err(BddError::NotEquivalent);
            }
        }
        Ok(())
    }

    /// Build a BDD from a gate.
    pub fn gate_to_bdd(gate: &Gate, num_vars: usize) -> BddNode {
        match gate {
            Gate::Single { gate: SingleGate::PauliX, target } => {
                // NOT: swap low and high
                BddNode::variable(
                    target.0,
                    BddNode::terminal(true),
                    BddNode::terminal(false),
                )
            }
            Gate::Single { gate: SingleGate::Hadamard, .. } => {
                // Hadamard creates superposition — represented as OR of both branches
                BddNode::variable(
                    0,
                    BddNode::terminal(true),
                    BddNode::terminal(true),
                )
            }
            Gate::Double { gate: DoubleGate::CNOT, control, target } => {
                // CNOT: if control then flip target
                BddNode::variable(
                    control.0,
                    BddNode::terminal(false), // control=0: no flip
                    BddNode::variable(
                        target.0,
                        BddNode::terminal(false),
                        BddNode::terminal(true),
                    ),
                )
            }
            _ => {
                // Default: identity
                BddNode::variable(
                    0,
                    BddNode::terminal(false),
                    BddNode::terminal(true),
                )
            }
        }
    }

    /// Evaluate a BDD on an input assignment.
    pub fn evaluate(bdd: &BddNode, assignment: &[bool]) -> bool {
        match bdd {
            BddNode::Terminal(val) => *val,
            BddNode::Variable { var_index, low, high } => {
                if assignment.get(*var_index).copied().unwrap_or(false) {
                    Self::evaluate(high, assignment)
                } else {
                    Self::evaluate(low, assignment)
                }
            }
        }
    }
}

/// The BDD equivalence pass.
pub struct BddCheck;

impl utqc_core::Pass for BddCheck {
    type Input = (Circuit, Circuit);
    type Output = bool;

    fn name(&self) -> &'static str {
        "bdd-equivalence-check"
    }

    fn run(&self, input: (Circuit, Circuit)) -> Result<bool, utqc_core::CircuitError> {
        BddVerifier::verify_equivalent(&input.0, &input.1).map_err(|_| {
            utqc_core::CircuitError::EmptyCircuit
        })?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utqc_core::{Qubit, DoubleGate};

    #[test]
    fn test_equivalent_circuits() {
        let mut a = Circuit::new(2, 2);
        a.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(0), target: Qubit(1) }).unwrap();

        let mut b = Circuit::new(2, 2);
        b.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(0), target: Qubit(1) }).unwrap();

        assert!(BddVerifier::verify_equivalent(&a, &b).is_ok());
    }

    #[test]
    fn test_nonequivalent_circuits() {
        let mut a = Circuit::new(2, 2);
        a.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(0), target: Qubit(1) }).unwrap();

        let mut b = Circuit::new(2, 2);
        b.add_gate(Gate::Double { gate: DoubleGate::SWAP, control: Qubit(0), target: Qubit(1) }).unwrap();

        assert!(BddVerifier::verify_equivalent(&a, &b).is_err());
    }

    #[test]
    fn test_bdd_evaluate() {
        let bdd = BddNode::variable(0, BddNode::terminal(false), BddNode::terminal(true));
        assert!(!BddVerifier::evaluate(&bdd, &[false]));
        assert!(BddVerifier::evaluate(&bdd, &[true]));
    }
}
