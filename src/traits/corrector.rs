use std::io::BufRead;

mod correction;
pub use correction::Correction;

/// Minimal API for a spelling corrector. Implement this for your own
/// corrector model struct.
pub trait Corrector: Sized {
    /// Builds a new corrector by reading and parsing words from the
    /// given corpus.
    fn from_corpus<R: BufRead>(corpus: R) -> Self;

    /// The form of string returned by the [`Corrector::suggest`]
    /// method.
    type String: AsRef<str>;

    /// Checks `word` and attempts to offer a suggestion.
    fn suggest(&self, word: &str) -> Correction<Self::String>;

    /// The [`Tokenizer`] used by the [`Corrector::from_corpus`] method.
    type Tokens: crate::Tokenizer;
}
