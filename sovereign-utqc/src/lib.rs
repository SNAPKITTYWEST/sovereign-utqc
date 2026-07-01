//! # Sovereign UTQC
//!
//! Universal Topological Quantum Computer — topological structure from the UOR Atlas.
//! BDD-driven, V&V-gated. Every quantum gate WORM-sealed.

pub mod qubit;
pub mod gate;
pub mod topology;
pub mod seal;

pub use qubit::Qubit;
pub use gate::{QuantumGate, GateType};
pub use topology::TopologicalSurface;
pub use seal::WormSeal;
