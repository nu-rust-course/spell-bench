//! Benchmarks for the Homework 3 spelling corrector.

pub const N: usize = 10;

pub use self::{
    edit::Edit,
    traits::{
        tokenizer::{Tokenizer, DefaultTokenizer, BoxIterator},
        corrector::{Corrector, Correction},
    },
};
pub use criterion;

/// A few texts.
pub mod corpus;

/// A datatype for representing and applying edits.
pub mod edit;

mod traits {
    /// Tokenization code for building the model from the corpus.
    pub mod tokenizer;

    /// Adaptor API to implement for your corrector model so that the
    /// benchmarks can construct and exercise it.
    pub mod corrector;
}

mod benches;
pub use benches::CorrectorBenches;

mod macros;
