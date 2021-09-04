use crate::view::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    Symbol,
    Keyword,
    String,
    Eof,
    Indent,
    Slash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Quote,
    Comment,
    HashLeftBrace,
    HashLeftBracket,
    HashLeftParen,
    Hash,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: Cursor,
}
