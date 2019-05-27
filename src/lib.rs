//! Benchmarks for the Homework 3 spelling corrector.

#![cfg_attr(feature = "nightly", feature(test))]

pub const N: usize = 10;

#[cfg(feature = "nightly")]
extern crate test;

#[cfg(feature = "nightly")]
pub use test::Bencher;

#[cfg(not(feature = "nightly"))]
pub use benches::MockBencher as Bencher;

mod traits;
pub use traits::{BoxIterator, Correction, Corrector, Tokenizer, DefaultTokenizer};

mod benches;
pub use benches::CorrectorBenches;

mod edits;
pub use edits::Edit;

mod macros;
