//! Benchmarks for the Homework 3 spelling corrector.

pub mod prelude {
    pub use super::{
        benches::CorrectorBench,
        edit::Edit,
        traits::{
            model_adapter::{
                Corrector,
                CorrectorResult,
                FromWords,
                FromOwnedWords,
                FromText,
                OptionalMethod,
                Implemented,
                Unimplemented,
            },
            tokenization::{
                Tokenizer,
                DefaultTokenizer,
            },
        },
    };

    // Re-export the dependency.
    pub use criterion::{self, black_box};
}

mod macros;

#[macro_use]
mod util;

pub mod benches;
pub mod corpus;
pub mod edit;
pub mod traits;
