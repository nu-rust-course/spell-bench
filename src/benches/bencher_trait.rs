pub trait Bencher {
    fn iter<T, F>(&mut self, f: F)
    where
        F: FnMut() -> T;
}

impl Bencher for test::Bencher {
    fn iter<T, F>(&mut self, f: F)
    where
        F: FnMut() -> T {

        test::Bencher::iter(self, f)
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

