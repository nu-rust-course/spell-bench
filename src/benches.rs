use crate::{
    corpus::{self, Corpus},
    Corrector,
    Tokenizer,
    Edit
};
use criterion::{BenchmarkId, Criterion};

use std::fmt::Display;

/// Extension trait that implements several benchmark possibilities for
/// any `Corrector`.
pub trait CorrectorBenches: Corrector {
    fn from_corpus_bench(corpus: Corpus, c: &mut Criterion) {
        let name = &format!("build_corpus_{}", corpus.name);
        Self::from_bytes_bench(name, corpus.as_bytes(), c);
    }

    fn from_bytes_bench(name: &str, bytes: &[u8], c: &mut Criterion) {
        let id = BenchmarkId::new(name, format!("{}B", bytes.len()));
        c.bench_with_input(id, &bytes, |b, bytes|
            b.iter(|| Self::from_corpus(*bytes)));
    }

    fn corpus_check_bench(n: usize, corpus: Corpus, c: &mut Criterion) {
        Self::corpus_check_with_edit_bench(n, corpus, "identity", Edit::I, c);
    }

    fn corpus_check_with_edit_bench(
        n: usize,
        corpus: Corpus,
        arg: impl Display,
        edit: Edit,
        c: &mut Criterion) {

        let name = corpus.name;
        let bytes = corpus.as_bytes();
        Self::bytes_check_with_edit_bench(name, arg, n, bytes, edit, c);
    }

    fn bytes_check_with_edit_bench(
        name: &str,
        arg: impl Display,
        n: usize,
        bytes: &[u8],
        e: Edit,
        c: &mut Criterion) {

        let corrector = Self::from_corpus(bytes);
        let words = Self::Tokens::tokenize(bytes);
        let problem: Vec<String> = words
            .iter()
            .map(String::as_str)
            .cycle()
            .skip(words.len() / 2)
            .map(|word| e.apply(word).collect())
            .take(n)
            .collect();

        let id = BenchmarkId::new(name, arg);
        c.bench_with_input(id, &problem, |b, problem|
            b.iter(||
                problem
                .iter()
                .filter(|word| corrector.suggest(word).is_suggestion())
                .count()));
    }

    fn build_dict_bench(c: &mut Criterion) {
        Self::from_corpus_bench(corpus::DICT, c);
    }

    fn check_dict_bench(n: usize, arg: impl Display, e: Edit, c: &mut Criterion) {
        Self::corpus_check_with_edit_bench(n, corpus::DICT, arg, e, c);
    }

    fn build_hamlet_bench(c: &mut Criterion) {
        Self::from_corpus_bench(corpus::HAMLET, c);
    }

    fn check_hamlet_bench(n: usize, arg: impl Display, e: Edit, c: &mut Criterion) {
        Self::corpus_check_with_edit_bench(n, corpus::HAMLET, arg, e, c);
    }

    fn build_small_bench(c: &mut Criterion) {
        Self::from_corpus_bench(corpus::SMALL, c);
    }

    fn check_small_bench(n: usize, arg: impl Display, e: Edit, c: &mut Criterion) {
        Self::corpus_check_with_edit_bench(n, corpus::SMALL, arg, e, c);
    }
}

impl<C: Corrector> CorrectorBenches for C { }

#[cfg(test)]
mod tests {
    use super::CorrectorBenches;
    use crate::{Corrector, Correction, DefaultTokenizer, Edit};

    use criterion::Criterion;

    use std::{
        io::BufRead,
        time::Duration,
    };

    struct Always(Correction<String>);

    impl Corrector for Always {
        fn from_corpus<R: BufRead>(_corpus: R) -> Self {
            Always(Correction::Correct)
        }

        type String = String;

        fn suggest(&self, _word: &str) -> Correction<String> {
            self.0.clone()
        }

        type Tokens = DefaultTokenizer;
    }

    fn crit() -> Criterion {
        Criterion::default()
            .nresamples(10)
            .without_plots()
            .measurement_time(Duration::from_millis(10))
            .warm_up_time(Duration::from_millis(1))
    }

    #[test]
    fn always_build_small_bench() {
        Always::build_small_bench(&mut crit());
    }

    #[test]
    fn always_check_small_bench() {
        Always::check_small_bench(10, "identity", Edit::I, &mut crit());
    }

    #[test]
    fn always_build_hamlet_bench() {
        Always::build_hamlet_bench(&mut crit());
    }
}
