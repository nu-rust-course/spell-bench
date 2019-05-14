use std::borrow::Cow;
use std::iter;

pub trait Edit {
    fn apply<'a>(&mut self, word: Cow<'a, str>) -> Option<Cow<'a, str>>;

    fn and<Next: Edit>(self, next: Next) -> (Self, Next)
    where
        Self: Sized {

        (self, next)
    }
}

pub fn fun<'a, T: Edit>(edit: &'a mut T)
    -> impl for<'b> FnMut(Cow<'b, str>) -> Option<Cow<'b, str>> + 'a {

    move |word| edit.apply(word)
}

impl<T: Edit, U: Edit> Edit for (T, U) {
    fn apply<'a>(&mut self, word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        let (f, g) = self;
        f.apply(word).and_then(fun(g))
    }
}

pub struct Identity;

impl Edit for Identity {
    fn apply<'a>(&mut self, word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        Some(word)
    }
}

pub struct LSkip(pub usize);

impl Edit for LSkip {
    fn apply<'a>(&mut self, mut word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        let oi = char_offsets(&word).nth(self.0);
        oi.map(|i| {
            match &mut word {
                Cow::Owned(s) => s.replace_range(..i, ""),
                Cow::Borrowed(s) => *s = &s[i..],
            }
            word
        })
    }
}

pub struct RSkip(pub usize);

impl Edit for RSkip {
    fn apply<'a>(&mut self, mut word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        let oi = char_offsets(&word).rev().nth(self.0);
        oi.map(|i| {
            match &mut word {
                Cow::Owned(s) => s.replace_range(i.., ""),
                Cow::Borrowed(s) => *s = &s[..i],
            }
            word
        })
    }
}

pub struct Pre(pub &'static str);

impl Edit for Pre {
    fn apply<'a>(&mut self, word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        Some(format!("{}{}", self.0, word).into())
    }
}

pub struct Post(pub &'static str);

impl Edit for Post {
    fn apply<'a>(&mut self, word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        Some(format!("{}{}", word, self.0).into())
    }
}

pub fn char_offsets<'a>(s: &'a str) -> impl DoubleEndedIterator<Item = usize> + 'a {
    s.char_indices().map(|p| p.0).chain(iter::once(s.len()))
}

pub struct Transpose(pub usize);

impl Edit for Transpose {
    fn apply<'a>(&mut self, word: Cow<'a, str>) -> Option<Cow<'a, str>> {
        let mut result = String::with_capacity(word.len());
        let mut chars = word.chars();

        result.extend(chars.by_ref().take(self.0));
        let (i, j) = (chars.next()?, chars.next()?);
        result.push(j);
        result.push(i);
        result.extend(chars);

        Some(result.into())
    }
}

