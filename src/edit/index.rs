use std::iter;

#[derive(Debug, Copy, Clone)]
pub struct Index(isize);

impl From<isize> for Index {
    fn from(value: isize) -> Self {
        Index(value)
    }
}

impl From<Index> for isize {
    fn from(ix: Index) -> Self {
        ix.0
    }
}

impl Index {
    pub fn split_string(self, s: &str) -> Option<(&str, &str)> {
        self.byte_offset(s).map(|i| s.split_at(i))
    }

    pub fn byte_offset(self, s: &str) -> Option<usize> {
        let i = self.0;
        if i < 0 {
            char_offsets(s).rev().nth(-(i + 1) as usize)
        } else {
            char_offsets(s).nth(i as usize)
        }
    }
}

fn char_offsets<'a>(s: &'a str) -> impl DoubleEndedIterator<Item = usize> + 'a {
    s.char_indices().map(|p| p.0).chain(iter::once(s.len()))
}
