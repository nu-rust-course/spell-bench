use std::iter;

#[derive(Clone, Copy, Debug)]
pub struct StringCursor<'a> {
    data:     &'a str,
    byte_pos: usize,
}

impl<'a> From<&'a str> for StringCursor<'a> {
    fn from(data: &'a str) -> Self {
        let byte_pos = 0;
        StringCursor { data, byte_pos }
    }
}

impl<'a> StringCursor<'a> {
    pub fn from_end(data: &'a str) -> Self {
        let byte_pos = data.len();
        StringCursor { data, byte_pos }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_str(&self) -> &'a str {
        self.data
    }

    pub fn split(&self) -> (&'a str, &'a str) {
        self.data.split_at(self.byte_pos)
    }

    pub fn before(&self) -> &'a str {
        self.split().0
    }

    pub fn after(&self) -> &'a str {
        self.split().1
    }

    pub fn peek(&self) -> Option<char> {
        self.after().chars().next()
    }

    pub fn peek_back(&self) -> Option<char> {
        self.before().chars().next_back()
    }

    pub fn advance(&mut self, n: usize) -> Option<&'a str> {
        let after = self.after();
        let start = self.byte_pos;
        let len = char_boundaries(after).nth(n)?;
        self.byte_pos += len;
        Some(&self.data[start .. start + len])
    }

    pub fn retreat(&mut self, n: usize) -> Option<&'a str> {
        let before = self.before();
        let end = self.byte_pos;
        let start = char_boundaries(before).nth_back(n)?;
        self.byte_pos = start;
        Some(&self.data[start .. end])
    }
}

fn char_boundaries<'a>(s: &'a str) -> impl DoubleEndedIterator<Item = usize> + 'a {
    s.char_indices()
        .map(|(i, _)| i)
        .chain(iter::once(s.len()))
}

impl<'a> Iterator for StringCursor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let after = self.after();
        let mut chars = after.char_indices();
        let (_, c) = chars.next()?;
        self.byte_pos += chars.next()
            .map_or_else(|| after.len(), |(i, _)| i);
        Some(c)
    }
}

impl<'a> DoubleEndedIterator for StringCursor<'a> {
    fn next_back(&mut self) -> Option<char> {
        let before = self.before();
        let mut chars = before.char_indices().rev();
        let (i, c) = chars.next()?;
        self.byte_pos = i;
        Some(c)
    }
}

impl Default for StringCursor<'_> {
    fn default() -> Self {
        StringCursor {
            data: "",
            byte_pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringCursor as SC;

    #[test]
    fn walk_forward() {
        let s: String = SC::from("don’t worry…").collect();
        assert_eq!( s, "don’t worry…" );
    }

    #[test]
    fn walk_backward() {
        let s: String = SC::from_end("don’t worry…").rev().collect();
        assert_eq!( s, "…yrrow t’nod" );
    }

    #[test]
    fn walk_around() {
        let mut sc = SC::from("don’t worry…");
        assert_eq!( sc.next(), Some('d') );
        assert_eq!( sc.next(), Some('o') );
        assert_eq!( sc.next(), Some('n') );

        assert_eq!( sc.peek(), Some('’') );
        assert_eq!( sc.peek_back(), Some('n') );

        assert_eq!( sc.next_back(), Some('n') );
        assert_eq!( sc.next_back(), Some('o') );
        assert_eq!( sc.next_back(), Some('d') );
        assert_eq!( sc.next_back(), None );

        assert_eq!( sc.advance(6), Some("don’t ") );
        assert_eq!( sc.next(), Some('w') );
        assert_eq!( sc.advance(4), Some("orry") );
        assert_eq!( sc.next(), Some('…') );
        assert_eq!( sc.next(), None );

        assert_eq!( sc.retreat(5), Some("orry…") );
        assert_eq!( sc.next_back(), Some('w') );
        assert_eq!( sc.next_back(), Some(' ') );
    }
}
