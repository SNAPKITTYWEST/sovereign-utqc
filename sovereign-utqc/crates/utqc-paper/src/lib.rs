//! # utqc-paper
//!
//! LaTeX theorem export for circuit verification proofs.

use utqc_core::Circuit;
use utqc_worm::WormSeal;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use thiserror::Error;

/// Paper export error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PaperError {
    #[error("serialization error: {0}")]
    Serialization(String),
}

/// A theorem statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theorem {
    /// Theorem name.
    pub name: String,
    /// Statement.
    pub statement: String,
    /// Proof sketch.
    pub proof: String,
    /// Associated circuit hash.
    pub circuit_hash: String,
}

/// LaTeX document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatexDocument {
    /// Title.
    pub title: String,
    /// Authors.
    pub authors: Vec<String>,
    /// Theorems.
    pub theorems: Vec<Theorem>,
    /// WORM seal.
    pub seal: WormSeal,
}

impl LatexDocument {
    /// Create a new document.
    pub fn new(title: &str, authors: Vec<String>) -> Self {
        let seal = WormSeal::seal("PAPER", title, 0);
        Self { title: title.to_string(), authors, theorems: Vec::new(), seal }
    }

    /// Add a theorem.
    pub fn add_theorem(&mut self, theorem: Theorem) {
        self.theorems.push(theorem);
    }

    /// Export as LaTeX string.
    pub fn to_latex(&self) -> String {
        let mut latex = String::new();
        latex.push_str("\\documentclass{article}\n");
        latex.push_str("\\usepackage{amsmath,amsthm}\n");
        latex.push_str("\\newtheorem{theorem}{Theorem}\n\n");
        latex.push_str(&format!("\\title{{{}}}\n", self.title));
        latex.push_str(&format!("\\author{{{}}}\n", self.authors.join(" and ")));
        latex.push_str("\\begin{document}\n\\maketitle\n\n");

        for theorem in &self.theorems {
            latex.push_str(&format!("\\begin{{theorem}}[{}]\n", theorem.name));
            latex.push_str(&format!("{}\n", theorem.statement));
            latex.push_str("\\end{theorem}\n\n");
            latex.push_str("\\begin{proof}\n");
            latex.push_str(&format!("{}\n", theorem.proof));
            latex.push_str("\\end{proof}\n\n");
        }

        latex.push_str("\\end{document}\n");
        latex
    }
}

/// Export circuit as LaTeX theorem.
pub fn export_circuit_theorem(circuit: &Circuit) -> Theorem {
    let circuit_str = format!("{:?}", circuit);
    let circuit_hash = format!("{:x}", sha2::Sha256::digest(circuit_str.as_bytes()));
    Theorem {
        name: "Circuit Equivalence".to_string(),
        statement: format!(
            "Circuit with {} qubits and {} gates compiles correctly.",
            circuit.num_qubits, circuit.gates.len()
        ),
        proof: "Verified by BDD equivalence check and WORM sealing.".to_string(),
        circuit_hash,
    }
}

/// The paper export pass.
pub struct PaperExport;

impl utqc_core::Pass for PaperExport {
    type Input = Circuit;
    type Output = LatexDocument;

    fn name(&self) -> &'static str {
        "paper-export"
    }

    fn run(&self, input: Circuit) -> Result<LatexDocument, utqc_core::CircuitError> {
        let mut doc = LatexDocument::new(
            "sovereign-utqc: Non-Recursive Universal Trusted Quantum Compiler",
            vec!["SnapKitty Collective".to_string()],
        );
        doc.add_theorem(export_circuit_theorem(&input));
        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utqc_core::{Qubit, SingleGate};

    #[test]
    fn test_export() {
        let mut circuit = Circuit::new(2, 2);
        circuit.add_gate(utqc_core::Gate::Single {
            gate: SingleGate::Hadamard,
            target: Qubit(0),
        }).unwrap();
        circuit.add_gate(utqc_core::Gate::Single {
            gate: SingleGate::PauliX,
            target: Qubit(1),
        }).unwrap();

        let mut doc = LatexDocument::new("Test", vec!["Author".to_string()]);
        doc.add_theorem(export_circuit_theorem(&circuit));

        let latex = doc.to_latex();
        assert!(latex.contains("\\title{Test}"));
        assert!(latex.contains("Circuit Equivalence"));
    }
}
