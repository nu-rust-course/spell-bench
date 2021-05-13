//! The code in this module demonstrates how to implement the `Corrector`
//! trait for a `suggest` method that borrows from the model.

/// A simple spelling corrector based on a corpus of only one word.
mod implementation {
    use std::io::BufRead;

    /// A simple corrector that knows only one word.
    #[derive(Clone, Debug)]
    pub struct TwoStringModel<S>(S, S);

    /// A borrowed correction.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Correction<'a> {
        Correct,
        Incorrect,
        Suggestion(&'a str),
    }

    impl TwoStringModel<String> {
        pub fn new(corpus: impl BufRead) -> Result<Self, &'static str> {
            Self::from_iter(corpus.lines().flat_map(Result::ok))
        }
    }

    impl<'a> TwoStringModel<&'a str> {
        pub fn new(corpus: &'a str) -> Result<Self, &'static str> {
            Self::from_iter(corpus.split_whitespace())
        }
    }

    impl<T> TwoStringModel<T> {
        pub fn from_iter<I>(words: I) -> Result<Self, &'static str>
        where
            I: IntoIterator<Item = T>,
        {
            let mut words = words.into_iter();
            let a = words.next().ok_or("TwoStringModel: no words")?;
            let b = words.next().ok_or("TwoStringModel: only one word")?;
            Ok(Self(a, b))
        }
    }

    impl<S: AsRef<str>> TwoStringModel<S> {
        pub fn correct(&self, word: &str) -> Correction {
            use Correction as C;

            let a = self.0.as_ref();
            let b = self.1.as_ref();

            if word == a || word == b {
                C::Correct
            } else if word.chars().next() == a.chars().next() {
                C::Suggestion(a)
            } else if word.chars().next() == b.chars().next() {
                C::Suggestion(b)
            } else if word.len() == a.len() {
                C::Suggestion(a)
            } else if word.len() == b.len() {
                C::Suggestion(b)
            } else {
                C::Incorrect
            }
        }
    }

    // mod tests {
    //     use super::TwoStringModel as Model;

    //     #[test]
    //     fn one_word_by_ref() {
    //         let model: Model<String> = Model("bees".into(), "wasps".into());
    //         assert_eq!(model.correct("bees"), Correct);
    //         assert_eq!(model.correct("bee"), Suggestion("bees"));
    //         assert_eq!(model.correct("eels"), Incorrect);
    //     }
    // }
}

mod integration_common {
    use super::implementation::Correction;
    use spell_bench::SuggestResult;

    impl SuggestResult for Correction<'_> {
        fn is_correct(&self) -> bool {
            matches!(self, Correction::Correct)
        }

        fn is_incorrect(&self) -> bool {
            matches!(self, Correction::Incorrect)
        }

        fn suggestion(&self) -> Option<&str> {
            if let Correction::Suggestion(word) = self {
                Some(word)
            } else {
                None
            }
        }
    }

}

mod integration_owned {
    use super::implementation::{Correction, TwoStringModel};
    use spell_bench::{
        // spell_bench,
        Corrector,
        FromOwnedWords,
        Implemented,
        Unimplemented,
        CorrectorBench,
        DefaultTokenizer,
    };

    type Model = TwoStringModel<String>;

    impl<'a> FromOwnedWords<'a, Model> for Implemented {
        fn build(words: Vec<String>) -> Model {
            Model::from_iter(words).unwrap()
        }
    }

    impl<'a> Corrector<'a> for Model {
        type FromOwnedWords = Implemented;
        type FromWords = Unimplemented;
        type FromText = Unimplemented;
        type Tokenizer = DefaultTokenizer;

        type Result = Correction<'a>;

        fn suggest(&'a self, word: &str) -> Self::Result {
            self.correct(word)
        }
    }

    fn meow() {
        Model::train("cat dog", |_| {});
    }

    // spell_bench! {
    //     for Model
    //     where mod build_model {
    //         fn build_cat_dog() {
    //             Model::train("cat dog", |model| { });
    //         }
    //     }
    // }
}

// mod integration_borrowed {
//     use super::implementation::{Correction, TwoStringModel};
//     use spell_bench::{
//         Corrector,
//         FromWords,
//         Implemented,
//         Unimplemented,
//         DefaultTokenizer,
//     };

//     type Model<'a> = TwoStringModel<&'a str>;

//     impl<'a, 'b: 'a> FromWords<'b, Model<'a>> for Implemented {
//         fn build(words: &[&'b str]) -> Model<'a> {
//             Model::from_iter(words.into_iter().copied()).unwrap()
//         }
//     }

//     impl<'a> Corrector<'a> for Model<'a> {
//         type FromWords = Implemented;
//         type FromOwnedWords = Unimplemented;
//         type FromText = Unimplemented;
//         type Tokenizer = DefaultTokenizer;

//         type Result = Correction<'a>;

//         fn suggest(&'a self, word: &str) -> Self::Result {
//             self.correct(word)
//         }
//     }
// }

fn main() {
}
