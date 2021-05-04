use std::io::BufRead;

pub type BoxIterator<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

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

