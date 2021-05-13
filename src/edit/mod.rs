//! A datatype for representing and applying edits.

use crate::util::StringCursor;

use std::rc::Rc;

/// An edit to a string.
#[derive(Debug, Clone)]
pub struct Edit(Option<Rc<EditNode>>);

#[derive(Debug, Clone)]
enum EditNode {
    Lit(char),
    Copy(usize),
    SeekRel(isize),
    Seq {
        a: Rc<EditNode>,
        b: Rc<EditNode>,
        a_len: usize,
    }
}

impl Default for Edit {
    fn default() -> Self {
        Self::new()
    }
}

impl From<EditNode> for Edit {
    fn from(node: EditNode) -> Self {
        Edit::from(Rc::from(node))
    }
}

impl From<&'_ EditNode> for Edit {
    fn from(node: &EditNode) -> Self {
        Edit::from(EditNode::clone(node))
    }
}

impl From<&'_ Rc<EditNode>> for Edit {
    fn from(rc: &Rc<EditNode>) -> Self {
        Edit::from(Rc::clone(rc))
    }
}

impl From<Rc<EditNode>> for Edit {
    fn from(rc: Rc<EditNode>) -> Self {
        Self(Some(rc))
    }
}

impl Edit {
    pub const I: Edit = Self::new();

    pub const fn new() -> Self {
        Edit(None)
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.len())
    }

    pub fn empty(&self) -> bool {
        self.0.as_ref().is_none()
    }

    pub fn insert(&self, i: usize, c: char) -> Self {
        let (a, b) = self.split_at(i);
        a.lit(c).then(b)
    }

    pub fn delete(&self, i: usize) -> Self {
        let (a, b) = self.split_at(i);
        a.seek_rel(1).then(b)
    }

    pub fn change(&self, i: usize, c: char) -> Self {
        let (a, b) = self.split_at(i);
        a.seek_rel(1).lit(c).then(b)
    }

    pub fn transpose(&self, i: usize) -> Self {
        let (a, r) = self.split_at(i);
        let (c1, r) = r.split_at(1);
        let (c2, b) = r.split_at(1);
        a.seek_rel(1).then(c2)
            .seek_rel(-2).then(c1)
            .seek_rel(1).then(b)
    }

    pub fn lit(self, c: char) -> Self {
        self.then(Self::new_lit(c))
    }

    pub fn copy(self, n: usize) -> Self {
        self.then(Self::new_copy(n))
    }

    pub fn seek_rel(self, z: isize) -> Self {
        self.then(Self::new_seek_rel(z))
    }

    pub fn then(self, other: Edit) -> Self {
        Self::seq(self, other)
    }

    pub fn new_lit(c: char) -> Self {
        EditNode::Lit(c).into()
    }

    pub fn new_copy(n: usize) -> Self {
        if n == 0 {
            Self::new()
        } else {
            EditNode::Copy(n).into()
        }
    }

    pub fn new_seek_rel(z: isize) -> Self {
        if z == 0 {
            Self::new()
        } else {
            EditNode::SeekRel(z).into()
        }
    }

    pub fn seq(a: Self, b: Self) -> Self {
        let node = match (a.0, b.0) {
            (Some(a), Some(b)) => Some(EditNode::seq(a, b).into()),
            (None, b) => b,
            (a, None) => a,
        };
        Self(node)
    }

    pub fn apply(&self, original: &str) -> String {
        self.iter(original).collect()
    }

    pub fn iter<'a>(&self, original: &'a str) -> Iter<'a> {
        let edit = self.0.as_ref().map(Rc::clone)
            .unwrap_or(Rc::new(EditNode::Copy(0)));
        let cursor = StringCursor::from(original);
        Iter { edit, cursor }
    }

    fn split_at(&self, i: usize) -> (Self, Self) {
        self.0.as_ref().map(Rc::as_ref)
            .unwrap_or_else(|| &EditNode::Copy(0))
            .split_at(i)
    }
}

impl EditNode {
    fn seq(a: Rc<Self>, b: Rc<Self>) -> Self {
        let a_len = a.len();
        Self::Seq { a, b, a_len }
    }

    fn len(&self) -> usize {
        use EditNode::*;
        match *self {
            Lit(_) => 1,
            Copy(n) => n,
            SeekRel(_) => 0,
            Seq { ref b, a_len, .. } => a_len + b.len(),
        }
    }

    fn split_at(&self, i: usize) -> (Edit, Edit) {
        use EditNode::*;

        match *self {
            Lit(_) => {
                let me = Edit::from(self);
                if i == 0 {
                    (Edit::I, me)
                } else {
                    (me.copy(i - 1), Edit::I)
                }
            }

            Copy(n) => {
                (Edit::I.copy(i), Edit::I.copy(n.saturating_sub(i)))
            }

            SeekRel(n) => {
                (Edit::I.seek_rel(n).copy(i), Edit::I)
            }

            Seq { ref a, ref b, a_len } =>
                if let Some(d) = i.checked_sub(a_len) {
                    let (b1, b2) = b.split_at(d);
                    (Edit(a.clone().into()).then(b1), b2)
                } else {
                    let (a1, a2) = a.split_at(i);
                    (a1, a2.then(Edit(b.clone().into())))
                }
        }
    }
}

/// An iterator over the result of applying an [`Edit`] to a string.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    edit:   Rc<EditNode>,
    cursor: StringCursor<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.next_rec()
            .or_else(|| self.cursor.next())
    }
}

impl<'a> Iter<'a> {
    fn next_rec(&mut self) -> Option<char> {
        use EditNode::*;

        loop {
            match *self.edit {
                Lit(c) => {
                    self.edit = Copy(0).into();
                    return Some(c)
                }

                Copy(0) => return None,

                Copy(n) => {
                    self.edit = Copy(n - 1).into();
                    return self.cursor.next()
                }

                SeekRel(n) => {
                    self.edit = Copy(0).into();
                    if n < 0 {
                        self.cursor.retreat(-n as usize)?;
                    } else {
                        self.cursor.advance(n as usize)?;
                    }
                    return None;
                }

                Seq { ref a, ref b, .. } => {
                    let a = a.clone();
                    let b = b.clone();

                    let mut iter = Iter {
                        edit:   a,
                        cursor: self.cursor,
                    };

                    let opt_c = iter.next_rec();
                    self.cursor = iter.cursor;

                    if let Some(c) = opt_c {
                        self.edit = EditNode::seq(iter.edit, b).into();
                        return Some(c)
                    }

                    self.edit = b;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! apply_test {
        ($name:ident, $word:expr, $expected:expr, $edit:expr) => {
            #[test]
            fn $name() {
                use ::std::iter::FromIterator;
                use ::std::string::String;
                assert_eq!( String::from_iter($edit.apply($word)), $expected );
            }
        };
    }

    macro_rules! iota_test {
        ($name:ident, $word:expr, $edit:expr, $seq:expr) => {
            #[test]
            fn $name() {
                let seq = &$seq;
                assert_eq!( n_edits(seq.len(), $word, $edit), seq );
            }
        };
    }

    const I: Edit = Edit::I;

    apply_test!(hello_hello, "hello", "hello", I);
    apply_test!(hello_hullo, "hello", "hullo", I.copy(1).lit('u').seek_rel(1));
    apply_test!(hello_hulo,  "hello", "hulo",  I.copy(1).lit('u').seek_rel(2));

    iota_test!(hello_inserts, "hello", |i| I.insert(i, 'u'),
        ["uhello", "huello", "heullo", "helulo", "helluo", "hellou"]);

    iota_test!(hello_changes, "hello", |i| I.change(i, 'u'),
        ["uello", "hullo", "heulo", "heluo", "hellu"]);

    iota_test!(hello_deletes, "hello", |i| I.delete(i),
        ["ello", "hllo", "helo", "helo", "hell"]);

    iota_test!(hello_transposes, "hello", |i| I.transpose(i),
        ["ehllo", "hlelo", "hello", "helol"]);

    apply_test!(hello_change_delete, "hello", "hulo",
        I.change(1, 'u').delete(2));

    apply_test!(hello_insert_delete_transpose, "hello", "hueol",
        I.insert(1, 'u').delete(3).transpose(3));

    apply_test!(hello_insert_transpose, "hello", "huelol",
        I.insert(1, 'u').transpose(4));

    apply_test!(hello_transpose_change, "hello", "hlexo",
        I.transpose(1).change(3, 'x'));

    fn n_edits(n: usize, word: &str, mut f: impl FnMut(usize) -> Edit) -> Vec<String> {
        (0..n as _)
            .map(|i| f(i).iter(word).collect())
            .collect()
    }
}
