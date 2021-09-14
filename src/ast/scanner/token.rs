use super::span::Span;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Comment,
    Symbol,
    String,
    Number,
    Eof,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Carrot,
    Quote,
    Hash,
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub lexeme: String,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\t\t{}\t\t{:?}", self.lexeme, self.span, self.kind)
    }
}
