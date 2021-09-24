use std::cell::Cell;
use std::fmt;

#[derive(Debug, Copy, Clone)]
struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "o{}-l{}-c{}", self.offset, self.line, self.column)
    }
}

impl Position {
    fn new() -> Position {
        Position {
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    fn newline(self) -> Position {
        Position {
            line: self.line + 1,
            column: 1,
            ..self
        }
    }

    fn right(self) -> Position {
        Position {
            offset: self.offset + 1,
            column: self.column + 1,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    start: Cell<Position>,
    end: Cell<Position>,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = self.start.get();
        let end = self.end.get();
        write!(f, "[{}:{}]", start, end)
    }
}

impl Span {
    pub fn new() -> Span {
        Span {
            start: Cell::new(Position::new()),
            end: Cell::new(Position::new()),
        }
    }

    fn get(&self) -> (usize, usize) {
        (self.start.get().offset, self.end.get().offset)
    }

    fn set(&self, (start, end): (usize, usize)) {
        let mut pstart = self.start.get();
        pstart.offset = start;
        self.start.set(pstart);
        let mut pend = self.end.get();
        pend.offset = end;
        self.start.set(pend);
    }

    pub fn left_grow(&self) -> &Self {
        self.end.set(self.end.get().right());
        self
    }

    pub fn view<'s>(&self, source: &'s [char]) -> Option<String> {
        let (start, end) = self.get();
        if start != end && start >= 0 && end <= source.len() {
            self.set((end, end));
            let mut result = String::new();
            for c in &source[start..end] {
                result.push(*c);
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn peek_n(&self, source: &'_ [char], n: usize) -> Option<char> {
        let (_, end) = self.get();
        let end = end + n;
        if end < source.len() {
            source.get(end..end + 1).map(|view| view[0])
        } else {
            None
        }
    }

    pub fn peek(&self, source: &'_ [char]) -> Option<char> {
        self.peek_n(source, 0)
    }

    pub fn advance(&self, source: &'_ [char]) -> Option<char> {
        self.peek(source).map(|char| {
            self.left_grow();
            if char == '\n' {
                self.end.set(self.end.get().newline())
            }
            char
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_should_start_at_zero_zero() {
        let span = Span::new();
        assert_eq!(span.get(), (0, 0));
    }

    #[test]
    fn is_should_grow_towards_the_end() {
        let span = Span::new();
        assert_eq!(span.get(), (0, 0));
        span.left_grow();
        assert_eq!(span.get(), (0, 1));
    }

    #[test]
    fn it_should_look_view_into_source() {
        let span = Span::new();
        span.left_grow().left_grow().left_grow();
        let source: Vec<_> = "alexander".chars().collect();
        assert_eq!(span.view(&source[..]), Some("ale".to_string()))
    }

    #[test]
    fn it_should_shift_the_span_when_source_is_viewed() {
        let span = Span::new();
        span.left_grow().left_grow().left_grow();
        let source: Vec<_> = "alexander".chars().collect();
        assert_eq!(span.view(&source[..]), Some("ale".to_string()));
        assert_eq!(span.get(), (3, 3));
    }

    #[test]
    fn it_should_not_shift_the_span_if_view_is_empty() {
        let span = Span::new();
        let source: Vec<_> = "alexander".chars().collect();
        assert_eq!(span.view(&source[..]), None);
        assert_eq!(span.get(), (0, 0));
        assert_eq!(span.advance(&source[..]), Some('a'));
        assert_eq!(span.get(), (0, 1));
        assert_eq!(span.view(&source[..]), Some("a".to_string()));
        assert_eq!(span.get(), (1, 1));
    }

    #[test]
    fn is_should_peek_with_offset() {
        let span = Span::new();
        let source: Vec<_> = "alexander".chars().collect();
        assert_eq!(span.peek_n(&source[..], 1), Some('l'));
        assert_eq!(span.peek_n(&source[..], 20), None);
    }

    #[test]
    fn is_should_peek_then_end() {
        let span = Span::new();
        let source: Vec<_> = "alexander".chars().collect();
        assert_eq!(span.peek(&source[..]), Some('a'));
    }

    #[test]
    fn it_should_expand_the_span_when_advanced() {
        let span = Span::new();
        let source: Vec<_> = "alexander".chars().collect();
        assert_eq!(span.advance(&source[..]), Some('a'));
        assert_eq!(span.get(), (0, 1));
    }

    #[test]
    fn is_should_handle_bounds() {
        let span = Span::new();
        let source: Vec<_> = "alex".chars().collect();
        span.left_grow().left_grow().left_grow().left_grow();
        assert_eq!(span.view(&source[..]), Some(String::from("alex")));
    }

    #[test]
    fn is_should_handle_clonding() {
        let span = Span::new();
        let p2 = span.clone();
        p2.left_grow();
        println!("{} {}", span, p2)
    }
}
