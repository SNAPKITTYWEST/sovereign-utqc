//! # Sovereign PRISM
//!
//! Bitcoin proof-of-work as UOR-ADDR realization.
//! Every hash WORM-sealed.

pub mod pow;
pub mod block;
pub mod seal;

pub use pow::ProofOfWork;
pub use block::Block;
pub use seal::WormSeal;
