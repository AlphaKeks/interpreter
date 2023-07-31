#![deny(clippy::correctness, clippy::perf)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![warn(rust_2018_idioms, missing_debug_implementations)]

#[cfg(test)]
mod test_setup;
pub mod tracing;

pub mod tokenizer;
pub use tokenizer::Tokenizer;
