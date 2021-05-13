//! Adaptor API to implement for your corrector model so that the
//! benchmarks can construct and exercise it.

pub use crate::util::optional_method::{
    OptionalMethod,
    Implemented,
    Unimplemented,
};
use super::tokenization::Tokenizer;

declare_optional_method! { FromOwnedWords<'a>(Vec<String>) }
declare_optional_method! { FromWords<'a>(impl Iterator<Item = &'a str>) }
declare_optional_method! { FromText<'a>(&'a str) }

pub trait CorrectorResult {
    fn is_correct(&self) -> bool;
    fn is_incorrect(&self) -> bool;
    fn suggestion(&self) -> Option<&str>;
}

pub trait Corrector<'a>: Sized {
    type FromOwnedWords: for<'b> FromOwnedWords<'b, Self>;
    type FromWords: for<'b> FromWords<'b, Self>;
    type FromText: for<'b> FromText<'b, Self>;
    type Tokenizer: Tokenizer;

    type Result: CorrectorResult;

    fn suggest(&'a self, word: &str) -> Self::Result;
}
