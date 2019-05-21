use std::borrow::Cow;
use std::io::BufRead;

pub type BoxIterator<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Correction<'a> {
    Correct,
    Uncorrectable,
    Suggestion(Cow<'a, str>)
}

macro_rules! matches {

    ( $e:expr, $p:pat ) => {
        match $e {
            $p => true,
            _  => false,
        }
    };

}

impl<'a> Correction<'a> {
    pub fn is_correct(&self) -> bool {
        matches!(self, Correction::Correct)
    }

    pub fn is_uncorrectable(&self) -> bool {
        matches!(self, Correction::Uncorrectable)
    }

    pub fn is_suggestion(&self) -> bool {
        matches!(self, Correction::Suggestion(_))
    }

    pub fn into_option(self) -> Option<Cow<'a, str>> {
        if let Correction::Suggestion(word) = self {
            Some(word)
        } else {
            None
        }
    }

    pub fn as_option(&'a self) -> Option<&'a str> {
        if let Correction::Suggestion(word) = self {
            Some(&word)
        } else {
            None
        }
    }

    pub fn map<'b, F>(self, f: F) -> Correction<'b>
    where
        F: FnOnce(Cow<'a, str>) -> Cow<'b, str> {

        use Correction::*;

        match self {
            Correct => Correct,
            Uncorrectable => Uncorrectable,
            Suggestion(word) => Suggestion(f(word)),
        }
    }
}

/// Minimal API for a spelling corrector.
pub trait Corrector: Sized {
    /// Builds a new corrector by reading and parsing words from the
    /// given corpus.
    fn from_corpus<R: BufRead>(corpus: R) -> Self;

    /// Checks `word` and attempts to offer a suggestion.
    fn suggest(&self, word: &str) -> Correction;

    /// Used for testing.
    type Tokens: Tokenizer;
}

/// This is for testing. Override its methods as needed if you tokenize
/// differently than I do.
pub trait Tokenizer {
    /// Reads from `corpus` and returns a vector of all words parsed
    /// therefrom. You can override this to change how
    fn tokenize<R: BufRead>(corpus: R) -> Vec<String> {
        corpus.lines()
              .fold(Vec::new(), |mut result, line| {
                  result.extend(Self::split_tokens(&line.unwrap())
                      .filter(|w| !Self::is_bad_word(w))
                      .map(Self::finish_word));
                  result
              })
    }

    /// Splits a line of text into tokens.
    ///
    /// The default implementation just calls `str::split_whitespace`.
    fn split_tokens(line: &str) -> BoxIterator<&str> {
        Box::new(line.split_whitespace())
    }

    /// Used to filter out words that should not be included in the
    /// result.
    ///
    /// Default implementation returns `true` for the empty string.
    fn is_bad_word(word: &str) -> bool {
        word.is_empty()
    }

    /// Constructs an owned string for the word while performing any
    /// final transformations that cannot be performed on a slice, for
    /// example changing the case. You can also filter characters from
    /// the word here.
    ///
    /// Default implementation calls `str::to_lowercase`.
    fn finish_word(word: &str) -> String {
        word.to_lowercase()
    }
}

#[derive(Debug, Clone)]
pub struct DefaultTokenizer;

impl Tokenizer for DefaultTokenizer { }

/// The code in this module demonstrates how to implement the `Corrector`
/// trait for a `suggest` method that borrows from the model.
pub mod example {
    use std::io::BufRead;
    use crate::{DefaultTokenizer, Corrector, Correction};

    impl Corrector for String {
        fn from_corpus<R: BufRead>(corpus: R) -> Self {
            for line in corpus.lines() {
                for token in line.unwrap().split_whitespace() {
                    return token.to_owned()
                }
            }

            panic!("OneWordCorrector: no tokens");
        }

        fn suggest(&self, word: &str) -> Correction {
            use Correction::*;

            if word == self {
                Correct
            } else if word.chars().next() == self.chars().next() {
                Suggestion(self.as_str().into())
            } else {
                Uncorrectable
            }
        }

        type Tokens = DefaultTokenizer;
    }
}

#[cfg(test)]
mod tests {
    use crate::Corrector;
    use crate::Correction::*;

    #[test]
    fn one_word_by_ref() {
        let the_word = "bees".to_owned();
        assert_eq!( the_word.suggest("bees"), Correct );
        assert_eq!( the_word.suggest("bee"),  Suggestion("bees".into()) );
        assert_eq!( the_word.suggest("eels"), Uncorrectable );
    }
}
