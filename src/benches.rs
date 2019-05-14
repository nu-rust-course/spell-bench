use bencher;
use crate::{Corrector, Tokenizer};

const HAMLET: &[u8] = include_bytes!("../resources/hamlet.txt");

pub trait Bencher {
    fn iter<T, F>(&mut self, f: F)
    where
        F: FnMut() -> T;
}

impl Bencher for bencher::Bencher {
    fn iter<T, F>(&mut self, f: F)
    where
        F: FnMut() -> T {

        bencher::Bencher::iter(self, f)
    }
}

impl Bencher for usize {
    fn iter<T, F>(&mut self, mut f: F)
    where
        F: FnMut() -> T {

        for _ in 0 .. *self {
            f();
        }
    }
}

pub trait CorrectorBenches: Corrector {
    fn read_bytes(bytes: &[u8], bench: &mut impl Bencher) {
        bench.iter(move || Self::from_corpus(bytes))
    }

    fn check_bytes(n: usize, bytes: &[u8], bench: &mut impl Bencher) {
        let hamlet_words = Self::Tokens::tokenize(bytes);
        let corrector = Self::from_corpus(bytes);

        bench.iter(move ||
            hamlet_words.iter()
                        .cycle()
                        .take(n)
                        .for_each(|word|
                            assert!(corrector.suggest(word).is_correct())));
    }

    fn read_hamlet(bench: &mut impl Bencher) {
        Self::read_bytes(HAMLET, bench);
    }

    fn check_hamlet(n: usize, bench: &mut impl Bencher) {
        Self::check_bytes(n, HAMLET, bench);
    }
}

impl<C: Corrector> CorrectorBenches for C { }

#[cfg(test)]
mod tests {
    use std::io::BufRead;
    use crate::{Corrector, Correction, DefaultTokenizer};
    use super::CorrectorBenches;

    #[test]
    fn allow_everything() {
        struct Always<T>(T);

        impl<'a> Corrector for Always<Correction<'a>> {
            fn from_corpus<R: BufRead>(_corpus: R) -> Self {
                Always(Correction::Correct)
            }

            fn suggest(&self, _word: &str) -> Correction {
                self.0.clone()
            }

            type Tokens = DefaultTokenizer;
        }

        <Always<Correction>>::check_bytes(100, super::HAMLET, &mut 1);
    }
}
