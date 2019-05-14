//! Benchmarks for the Homework 3 spelling corrector.

#![feature(test)]

pub const N: usize = 10;

extern crate test;
pub use test::Bencher;

mod traits;
pub use traits::{BoxIterator, Correction, Corrector, Tokenizer, DefaultTokenizer};

mod benches;
pub use benches::{CorrectorBenches, edits};

mod macros;
