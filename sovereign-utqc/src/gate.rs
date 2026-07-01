//! Quantum gates

use serde::{Deserialize, Serialize};
use crate::qubit::Qubit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateType {
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    CNOT,
    TGate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumGate {
    pub gate_type: GateType,
    pub target: usize,
    pub control: Option<usize>,
}

impl QuantumGate {
    pub fn apply(&self, qubits: &mut [Qubit]) {
        match self.gate_type {
            GateType::PauliX => {
                qubits[self.target].alpha = qubits[self.target].beta;
                qubits[self.target].beta = 1.0 - qubits[self.target].alpha;
            }
            GateType::Hadamard => {
                let a = qubits[self.target].alpha;
                let b = qubits[self.target].beta;
                qubits[self.target].alpha = (a + b) / 2.0_f64.sqrt();
                qubits[self.target].beta = (a - b) / 2.0_f64.sqrt();
            }
            GateType::TGate => {
                qubits[self.target].apply_phase(std::f64::consts::FRAC_PI_4);
            }
            _ => {}
        }
    }
}
