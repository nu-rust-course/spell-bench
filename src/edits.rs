use std::iter;

#[derive(Debug, Clone)]
pub struct Edit(EditInner);

#[derive(Debug, Clone)]
enum EditInner {
    Identity,
    Delete(Index),
    Insert(Index, char),
    Replace(Index, char),
    Transpose(Index),
    Sequence(Box<EditInner>, Box<EditInner>),
}

use self::EditInner::*;

impl EditInner {
    pub fn apply<S: AsRef<str>>(&self, word: S) -> Option<String> {
        let word = word.as_ref();

        match self {
            Identity => Some(word.to_owned()),

            Delete(i) => {
                let (before, after) = i.split_string(word)?;
                let (_, after) = str_uncons(after)?;
                Some(format!("{}{}", before, after))
            }

            Insert(i, c) => {
                let (before, after) = i.split_string(word)?;
                Some(format!("{}{}{}", before, c, after))
            }

            Replace(i, c) => {
                let (before, after) = i.split_string(word)?;
                let (_, after) = str_uncons(after)?;
                Some(format!("{}{}{}", before, c, after))
            }

            Transpose(i) => {
                let (before, after) = i.split_string(word)?;
                let (c1, before) = str_unsnoc(before)?;
                let (c2, after) = str_uncons(after)?;
                Some(format!("{}{}{}{}", before, c2, c1, after))
            }

            Sequence(ref e1, ref e2) =>
                e1.apply(word).and_then(|s| e2.apply(s)),
        }
    }
}

impl Edit {
    pub fn apply<S: AsRef<str>>(&self, word: S) -> Option<String> {
        self.0.apply(word)
    }

    pub fn identity() -> Self {
        Edit(Identity)
    }

    pub fn delete(index: isize) -> Self {
        Edit(Delete(Index(index)))
    }

    pub fn delete_rev(index: isize) -> Self {
        Self::delete(-index)
    }


    pub fn insert(index: isize, c: char) -> Self {
        Edit(Insert(Index(index), c))
    }

    pub fn insert_rev(index: isize, c: char) -> Self {
        Self::insert(-index, c)
    }

    pub fn replace(index: isize, c: char) -> Self {
        Edit(Replace(Index(index), c))
    }

    pub fn replace_rev(index: isize, c: char) -> Self {
        Self::replace(-index, c)
    }

    pub fn transpose(index: isize) -> Self {
        Edit(Transpose(Index(index)))
    }

    pub fn transpose_rev(index: isize) -> Self {
        Self::transpose(-index)
    }

    pub fn then(self, other: Self) -> Self {
        Edit(Sequence(Box::new(self.0), Box::new(other.0)))
    }
}

fn char_offsets<'a>(s: &'a str) -> impl DoubleEndedIterator<Item = usize> + 'a {
    s.char_indices().map(|p| p.0).chain(iter::once(s.len()))
}

fn str_uncons(s: &str) -> Option<(char, &str)> {
    let mut iter = s.chars();
    iter.next().map(|c| (c, iter.as_str()))
}

fn str_unsnoc(s: &str) -> Option<(char, &str)> {
    let mut iter = s.chars();
    iter.next_back().map(|c| (c, iter.as_str()))
}

#[derive(Debug, Copy, Clone)]
struct Index(isize);

impl Index {
    fn split_string(self, s: &str) -> Option<(&str, &str)> {
        self.byte_offset(s).map(|i| (&s[..i], &s[i..]))
    }

    fn byte_offset(self, s: &str) -> Option<usize> {
        match self.0 {
            -1 => Some(s.len()),
            0  => Some(0),

            i  => if i > 0 {
                char_offsets(s).nth(i as usize)
            } else {
                char_offsets(s).rev().nth(-i as usize - 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Edit;

    fn some(s: &str) -> Option<String> {
        Some(s.to_owned())
    }

    #[test]
    fn identity() {
        assert_eq!( Edit::identity().apply("hello"), some("hello") );
    }

    #[test]
    fn deletes() {
        assert_eq!( Edit::delete(0).apply("hello"), some("ello") );
        assert_eq!( Edit::delete(1).apply("hello"), some("hllo") );
        assert_eq!( Edit::delete(8).apply("hello"), None );
        assert_eq!( Edit::delete(-1).apply("hello"), None );
        assert_eq!( Edit::delete(-2).apply("hello"), some("hell") );
    }

    #[test]
    fn inserts() {
        assert_eq!( Edit::insert(0, 'x').apply("hello"), some("xhello") );
        assert_eq!( Edit::insert(1, 'x').apply("hello"), some("hxello") );
        assert_eq!( Edit::insert(8, 'x').apply("hello"), None );
        assert_eq!( Edit::insert(-1, 'x').apply("hello"), some("hellox") );
        assert_eq!( Edit::insert(-2, 'x').apply("hello"), some("hellxo") );
    }

    #[test]
    fn replaces() {
        assert_eq!( Edit::replace(0, 'x').apply("hello"), some("xello") );
        assert_eq!( Edit::replace(1, 'x').apply("hello"), some("hxllo") );
        assert_eq!( Edit::replace(8, 'x').apply("hello"), None );
        assert_eq!( Edit::replace(-1, 'x').apply("hello"), None );
        assert_eq!( Edit::replace(-2, 'x').apply("hello"), some("hellx") );
    }

    #[test]
    fn transpose() {
        assert_eq!( Edit::transpose(0).apply("hello"), None );
        assert_eq!( Edit::transpose(1).apply("hello"), some("ehllo") );
        assert_eq!( Edit::transpose(2).apply("hello"), some("hlelo") );
        assert_eq!( Edit::transpose(8).apply("hello"), None );
        assert_eq!( Edit::transpose(-1).apply("hello"), None );
        assert_eq!( Edit::transpose(-2).apply("hello"), some("helol") );
        assert_eq!( Edit::transpose(-3).apply("hello"), some("hello") );
        assert_eq!( Edit::transpose(-4).apply("hello"), some("hlelo") );
    }

    #[test]
    fn sequence() {
        assert_eq!( Edit::transpose(1).then(Edit::transpose(1)).apply("abcdef"),
                    some("abcdef") );
        assert_eq!( Edit::transpose(1).then(Edit::transpose(2)).apply("abcdef"),
                    some("bcadef") );
        assert_eq!( Edit::transpose(1).then(Edit::transpose(3)).apply("abcdef"),
                    some("badcef") );
        assert_eq!( Edit::delete(1).then(Edit::transpose(1)).apply("abcdef"),
                    some("cadef") );
        assert_eq!( Edit::transpose(1).then(Edit::delete(1)).apply("abcdef"),
                    some("bcdef") );
    }
}

