//! Test corpora.

#[derive(Clone, Copy)]
pub struct Corpus<'a, 'b> {
    pub name: &'a str,
    pub text: &'b str,
}

pub const HAMLET: Corpus = Corpus::from_str("hamlet",
    include_str!("../resources/hamlet.txt"));

pub const DICT: Corpus = Corpus::from_str("dict",
    include_str!("../resources/dict.txt"));

pub const SMALL: Corpus = Corpus::from_str("small",
    include_str!("../resources/small.txt"));

pub const TINY: Corpus = Corpus::from_str("tiny",
    "hello rust goodbye rust");

impl<'a, 'b> Corpus<'a, 'b> {
    const fn from_str(name: &'a str, text: &'b str) -> Self {
        Self { name, text, }
    }

    pub const fn as_bytes(self) -> &'b [u8] {
        self.text.as_bytes()
    }
}
