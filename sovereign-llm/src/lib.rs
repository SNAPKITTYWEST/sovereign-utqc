//! # Sovereign LLM
//!
//! 1.1B parameter model pretraining.
//! Every weight WORM-sealed.

pub mod model;
pub mod tokenizer;
pub mod train;
pub mod seal;

pub use model::Model;
pub use tokenizer::Tokenizer;
pub use train::Trainer;
pub use seal::WormSeal;
