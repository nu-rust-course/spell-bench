//! Benchmarks for the Homework 3 spelling corrector.

#![cfg_attr(feature = "nightly", feature(test))]

pub const N: usize = 10;

#[cfg(feature = "nightly")]
extern crate test;

#[cfg(feature = "nightly")]
pub use test::Bencher;

#[cfg(not(feature = "nightly"))]
pub use benches::MockBencher as Bencher;

pub mod corpus {
    use lazy_static::lazy_static;
    use std::str;

    const HAMLET_BYTES: &[u8] = include_bytes!("../resources/hamlet.txt");
    const DICT_BYTES: &[u8] = include_bytes!("../resources/dict.txt");

    lazy_static! {
        pub static ref HAMLET: &'static str = str::from_utf8(HAMLET_BYTES).unwrap();
        pub static ref DICT: &'static str = str::from_utf8(DICT_BYTES).unwrap();
    }

    pub const SMALL: &str = "hello rust goodbye rust";
}

mod traits;
pub use traits::{BoxIterator, Correction, Corrector, Tokenizer, DefaultTokenizer};

mod benches;
pub use benches::CorrectorBenches;

mod edits;
pub use edits::Edit;

mod macros;
