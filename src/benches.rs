use crate::{Corrector, Tokenizer, Edit};

/// The text of hamlet. Should be entirely Roman letters and space.
pub const HAMLET: &[u8] = include_bytes!("../resources/hamlet.txt");

/// Extension trait that implements several benchmark possibilities for
/// any `Corrector`.
pub trait CorrectorBenches: Corrector {

    fn read_bytes(bytes: &[u8], bench: &mut impl Bencher) {
        bench.iter(move || Self::from_corpus(bytes))
    }

    fn check_bytes(n: usize, bytes: &[u8], bench: &mut impl Bencher) {
        Self::check_bytes_with_edits(n, bytes, &Edit::identity(), bench);
    }

    fn check_bytes_with_edits(n: usize,
                              bytes: &[u8],
                              e: &Edit,
                              bench: &mut impl Bencher) {

        let corrector = Self::from_corpus(bytes);
        let words = Self::Tokens::tokenize(bytes);
        let skip = words.len() / 2;
        let problem = words
            .into_iter()
            .cycle()
            .skip(skip)
            .filter_map(|word| e.apply(word))
            .take(n)
            .collect::<Vec<_>>();

        bench.iter(move ||
            problem.iter()
                .filter(|word| corrector.suggest(word).is_suggestion())
                .count())
    }

    fn read_hamlet(bench: &mut impl Bencher) {
        Self::read_bytes(HAMLET, bench);
    }

    fn check_hamlet_with_edits(n: usize, e: &Edit, bench: &mut impl Bencher) {
        Self::check_bytes_with_edits(n, HAMLET, e, bench);
    }

    fn check_hamlet(n: usize, bench: &mut impl Bencher) {
        Self::check_bytes(n, HAMLET, bench);
    }

}

/// Trait to let us mock `test::Bencher` for testing benchmarks.
pub trait Bencher {
    fn iter<T, F>(&mut self, f: F)
    where
        F: FnMut() -> T;
}

#[cfg(feature = "nightly")]
impl Bencher for test::Bencher {
    fn iter<T, F>(&mut self, f: F)
    where
        F: FnMut() -> T {

        test::Bencher::iter(self, f)
    }
}

#[derive(Clone, Debug)]
pub struct MockBencher(pub usize);

impl Bencher for MockBencher {
    fn iter<T, F>(&mut self, mut f: F)
    where
        F: FnMut() -> T {

        for _ in 0 .. self.0 {
            f();
        }
    }
}

impl<C: Corrector> CorrectorBenches for C { }

#[cfg(test)]
mod tests {
    use std::io::BufRead;
    use crate::{Corrector, Correction, DefaultTokenizer};
    use super::CorrectorBenches;
    use crate::benches::MockBencher;

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

        <Always<Correction>>::check_bytes(100, super::HAMLET, &mut MockBencher(1));
    }
}
