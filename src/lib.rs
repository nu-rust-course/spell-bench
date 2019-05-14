//! Benchmarks for the Homework 3 spelling corrector.

use std::io::BufRead;

pub trait Corrector<'a> {
    type Word: AsRef<str>;

    fn from_corpus<R: BufRead>(corpus: R) -> Self;

    fn suggest(&'a self, word: &str) -> Result<(), Option<Self::Word>>;
}

/// Adapts a `Corrector` that returns suggestions by reference into
/// a `Corrector` that returns suggestions by value (owned `String`).
/// You don’t need to use this, but the `impl` of the `Corrector` trait
/// for this type shows how to `impl Corrector` for a model that returns
/// suggestions by value.
#[derive(Clone, Debug)]
pub struct AdaptToOwned<S>(S);

impl<'a, S: Corrector<'a>> Corrector<'a> for AdaptToOwned<S> {
    type Word = String;

    fn from_corpus<R: BufRead>(corpus: R) -> Self {
        AdaptToOwned(S::from_corpus(corpus))
    }

    fn suggest(&'a self, word: &str) -> Result<(), Option<Self::Word>> {
        self.0.suggest(word)
            .map_err(|ow| ow.map(|w| w.as_ref().to_owned()))
    }
}

/// The code in this module demonstrates how to implement the `Corrector`
/// trait for a `suggest` method that borrows from the model.
pub mod example {
    use std::io::BufRead;
    use super::Corrector;

    impl<'a> Corrector<'a> for String {
        type Word = &'a str;

        fn from_corpus<R: BufRead>(corpus: R) -> Self {
            for line in corpus.lines() {
                for token in line.unwrap().split_whitespace() {
                    return token.to_owned()
                }
            }

            panic!("OneWordCorrector: no tokens");
        }

        fn suggest(&'a self, word: &str) -> Result<(), Option<Self::Word>> {
            if word == self {
                Ok(())
            } else if word.chars().next() == self.chars().next() {
                Err(Some(self.as_str()))
            } else {
                Err(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Corrector, AdaptToOwned};

    #[test]
    fn one_word_by_ref() {
        let the_word = "bees".to_owned();
        assert_eq!( the_word.suggest("bees"), Ok(()) );
        assert_eq!( the_word.suggest("bee"),  Err(Some("bees")) );
        assert_eq!( the_word.suggest("eels"), Err(None) );
    }

    #[test]
    fn one_word_by_val() {
        let corrector = AdaptToOwned("bees".to_owned());
        assert_eq!( corrector.suggest("bees"), Ok(()) );
        assert_eq!( corrector.suggest("bee"),  Err(Some("bees".to_owned())) );
        assert_eq!( corrector.suggest("eels"), Err(None) );
    }
}
