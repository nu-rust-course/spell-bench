//! Helpers for timing the right parts.

use criterion::{black_box, Bencher};
use std::time::Instant;

pub trait FnBench<A = ()> {
    type State;
    fn ready(self) -> Self::State;
    fn go(state: &mut Self::State) -> A;

    fn bench_ready(self, timer: impl FnTimer)
    where
        Self: Sized
    {
        let ready = &mut self.ready();
        {timer}.iter(|| Self::go(black_box(ready)));
    }

    fn bench_go(&self, timer: impl FnTimer)
    where
        Self: Clone
    {
        {timer}.iter(|| black_box(self.clone()).ready());
    }

    fn bench_both(&self, timer: impl FnTimer)
    where
        Self: Clone
    {
        {timer}.iter(|| {
            let ready = &mut black_box(self.clone()).ready();
            Self::go(black_box(ready));
        })
    }
}


impl<F, S, A> FnBench<A> for F
where
    F: FnOnce() -> S,
    S: FnMut() -> A,
{
    type State = S;

    fn ready(self) -> S {
        self()
    }

    fn go(state: &mut S) -> A {
        state()
    }
}


pub trait FnTimer1<R> {
    fn iter(&mut self, routine: impl FnMut() -> R);
}

pub trait FnTimer {
    fn iter<R>(&mut self, routine: impl FnMut() -> R);
}

impl<T: FnTimer> FnTimer for &'_ mut T {
    fn iter<R>(&mut self, routine: impl FnMut() -> R) {
        T::iter(*self, routine);
    }
}

impl FnTimer for Bencher<'_> {
    fn iter<R>(&mut self, routine: impl FnMut() -> R) {
        Bencher::iter(self, routine)
    }
}

impl FnTimer for &'_ str {
    fn iter<R>(&mut self, mut routine: impl FnMut() -> R) {
        let start = Instant::now();
        routine();
        let stop = Instant::now();
        let dur = stop - start;
        eprintln!("{}: {} ns", self, dur.as_nanos());
    }
}
