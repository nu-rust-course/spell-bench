/// Generate the name of a benchmark from a sequence of parts.
pub fn bench_tag<I, T>(pieces: I) -> String
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let mut pieces = pieces.into_iter();
    let mut buf = StringBuilder::new();

    for s in pieces.next() {
        buf += s;
    }

    for s in pieces {
        buf += "::";
        buf += s;
    }

    return buf.into();
}

#[derive(Debug, Default)]
struct StringBuilder(String);

impl StringBuilder {
    fn new() -> Self {
        Self::default()
    }
}

impl<RHS: AsRef<str>> std::ops::AddAssign<RHS> for StringBuilder {
    fn add_assign(&mut self, rhs: RHS) {
        self.0.push_str(rhs.as_ref());
    }
}

impl From<StringBuilder> for String {
    fn from(sb: StringBuilder) -> Self {
        sb.0
    }
}

#[test]
fn try_bench_tag() {
    assert_eq!( bench_tag(&["foo", "bar"]), "foo::bar" );
}

