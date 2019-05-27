//! The code in this module demonstrates how to implement the `Corrector`
//! trait for a `suggest` method that borrows from the model.

#![cfg_attr(feature = "nightly", feature(test))]

/// A simple spelling corrector based on a corpus of only one word.
mod implementation {
    use std::io::BufRead;

    /// A simple corrector that knows only one word.
    #[derive(Clone, Debug)]
    pub struct SingleStringModel(String);

    /// A borrowed correction.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Correction<'a> {
        Correct,
        Incorrect,
        Suggestion(&'a str),
    }

    impl SingleStringModel {
        pub fn new(corpus: impl BufRead) -> Result<Self, &'static str> {
            for line in corpus.lines() {
                for token in line.unwrap().split_whitespace() {
                    return Ok(SingleStringModel(token.to_owned()))
                }
            }

            Err("Corrector: no tokens")
        }

        pub fn correct(&self, word: &str) -> Correction {
            if self.0 == word {
                Correction::Correct
            } else if self.0.chars().next() == word.chars().next() {
                Correction::Suggestion(self.0.as_str())
            } else {
                Correction::Incorrect
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::SingleStringModel as Model;
        use super::Correction::*;

        #[test]
        fn one_word_by_ref() {
            let model = Model("bees".to_owned());
            assert_eq!( model.correct("bees"), Correct );
            assert_eq!( model.correct("bee"),  Suggestion("bees") );
            assert_eq!( model.correct("eels"), Incorrect );
        }

    }
}

mod integration {
    use std::io::BufRead;
    use super::implementation::{SingleStringModel, Correction};

    impl spell_bench::Corrector for SingleStringModel {
        fn from_corpus<R: BufRead>(corpus: R) -> Self {
            SingleStringModel::new(corpus).unwrap()
        }

        fn suggest(&self, word: &str) -> spell_bench::Correction {
            use spell_bench::Correction::*;

            match self.correct(word) {
                Correction::Correct => Correct,
                Correction::Incorrect => Uncorrectable,
                Correction::Suggestion(s) => Suggestion(s.into()),
            }
        }

        type Tokens = spell_bench::DefaultTokenizer;
    }
}

#[cfg(feature = "nightly")]

spell_bench::spell_bench! {
    mod single_string_benches {
        use super::implementation::SingleStringModel as Corrector;
        bench_corrector!();
    }
}

