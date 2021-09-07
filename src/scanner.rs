use crate::util::Matcher;
use std::cell::Cell;

const EOF: char = '\0';
const NL: char = '\n';
const TAB: char = '\t';
const R: char = '\r';

impl Matcher for &Vec<char> {
    type Item = char;
    fn check_not(&self, offset: usize, value: char) -> bool {
        offset < self.len() && self[offset] != value
    }
    fn check(&self, offset: usize, value: char) -> bool {
        offset < self.len() && self[offset] == value
    }
    fn test<F>(&self, offset: usize, tester: F) -> bool
        where
            F: Fn(char) -> bool,
    {
        offset < self.len() && tester(self[offset])
    }
    fn check_next(&self, offset: usize, value: char) -> bool {
        offset + 1 < self.len() && self[offset + 1] == value
    }
    fn test_next<F>(&self, offset: usize, tester: F) -> bool
        where
            F: Fn(char) -> bool,
    {
        offset + 1 < self.len() && tester(self[offset + 1])
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
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
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub position: Position,
}

fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || "!$%^&*_-+=|<>?".contains(c)
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || "!$%^&*_-+=|<>?".contains(c)
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

trait SymbolMatcher {
    fn is_symbol_start(&self) -> bool;
    fn is_symbol_char(&self) -> bool;
    fn is_obj_delim(&self) -> bool;
    fn is_ns_delim(&self) -> bool;
}

trait NumberMatcher {
    fn is_digit(&self) -> bool;
    fn is_dot_digit(&self) -> bool;
}

pub struct Scanner {
    source: String,
    range: Cell<Position>,
}

impl SymbolMatcher for Scanner {
    fn is_symbol_start(&self) -> bool {
        is_symbol_start(self.peek())
    }

    fn is_symbol_char(&self) -> bool {
        is_symbol_char(self.peek())
    }

    fn is_obj_delim(&self) -> bool {
        let (dot, start) = self.peek_2();
        dot == '.' && is_symbol_start(start)
    }

    fn is_ns_delim(&self) -> bool {
        let (slash, start) = self.peek_2();
        slash == '/' && is_symbol_start(start)
    }
}

impl NumberMatcher for Scanner {
    fn is_digit(&self) -> bool {
        self.peek().is_digit(10)
    }

    fn is_dot_digit(&self) -> bool {
        let (dot, digit) = self.peek_2();
        dot == '.' && digit.is_digit(10)
    }
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            range: Cell::new(Position {
                start: 0,
                end: 0,
                line: 1,
                column: 1,
            }),
        }
    }

    fn shift(&self) {
        let mut range = self.range.get();
        range.end += 1;
        range.start = range.end;
        self.range.set(range);
    }

    fn newline(&self) {
        self.shift();
        let mut range = self.range.get();
        range.line += 1;
        range.column = 1;
        self.range.set(range);
    }

    fn expand(&self) {
        let mut range = self.range.get();
        range.end += 1;
        range.column += 1;
        self.range.set(range);
    }

    fn peek_n(&self, n: usize) -> char {
        let range = self.range.get();
        self.source
            .get(range.end + n..range.end + n + 1)
            .and_then(|slice| slice.chars().next()).unwrap_or(EOF)
    }

    fn peek(&self) -> char {
        self.peek_n(0)
    }

    fn peek_2(&self) -> (char, char) {
        (self.peek_n(0), self.peek_n(1))
    }

    fn pop(&self) -> char {
        let value = self.peek();
        self.expand();
        value
    }

    fn make_token(&self, kind: TokenKind, lexeme: String) -> Token {
        let mut range = self.range.get();
        range.column -= (range.end - range.start - 1);
        println!("lexeme {:}\ncol {:?} new {:?}", lexeme, self.range.get(), range);
        Token {
            kind,
            lexeme,
            position: range,
        }
    }

    fn scan_comment(&self) -> Option<Token> {
        let mut lexeme = String::new();
        while NL != self.peek() && EOF != self.peek() {
            let c = self.pop();
            lexeme.push(c);
        }
        Some(self.make_token(TokenKind::Comment, lexeme))
    }

    fn scan_delimiter(&self, kind: TokenKind) -> Option<Token> {
        let mut lexeme = String::new();
        lexeme.push(self.pop());
        Some(self.make_token(kind, lexeme))
    }

    fn scan_symbol(&self) -> Option<Token> {
        let mut lexeme = String::new();
        let name = |lexeme: &mut String| {
            while self.is_symbol_char() {
                lexeme.push(self.pop());
                if self.is_obj_delim() {
                    lexeme.push(self.pop());
                }
            }
        };
        name(&mut lexeme);
        if self.is_ns_delim() {
            lexeme.push(self.pop());
            name(&mut lexeme);
        }
        Some(self.make_token(TokenKind::Symbol, lexeme))
    }

    fn scan_number(&self) -> Option<Token> {
        let mut lexeme = String::new();
        let digits = |lexeme: &mut String| {
            while self.is_digit() {
                lexeme.push(self.pop());
            }
        };
        digits(&mut lexeme);
        if self.is_dot_digit() {
            lexeme.push(self.pop());
            digits(&mut lexeme);
        }
        Some(self.make_token(TokenKind::Symbol, lexeme))
    }

    fn scan_string(&self) -> Option<Token> {
        let mut lexeme = String::new();
        lexeme.push(self.pop());
        while let c = self.peek() {
            match c {
                EOF => return None,
                '"' => break,
                _ => lexeme.push(self.pop()),
            }
        }
        lexeme.push(self.pop());
        Some(self.make_token(TokenKind::String, lexeme))
    }

    pub fn scan(source: String) -> Option<Vec<Token>> {
        use TokenKind::*;
        let mut tokens = vec![];
        let scanner = Scanner::new(source);

        while let c = scanner.peek() {
            match c {
                EOF => break,
                NL => scanner.newline(),
                ';' => tokens.push(scanner.scan_comment()?),
                '(' => tokens.push(scanner.scan_delimiter(LeftParen)?),
                ')' => tokens.push(scanner.scan_delimiter(RightParen)?),
                '[' => tokens.push(scanner.scan_delimiter(LeftBracket)?),
                ']' => tokens.push(scanner.scan_delimiter(RightBracket)?),
                '{' => tokens.push(scanner.scan_delimiter(LeftBrace)?),
                '}' => tokens.push(scanner.scan_delimiter(RightBracket)?),
                '"' => tokens.push(scanner.scan_string()?),
                c if is_symbol_start(c) => tokens.push(scanner.scan_symbol()?),
                c if c.is_digit(10) => tokens.push(scanner.scan_number()?),
                _ => scanner.shift(),
            }
        }

        Some(tokens)
    }
}

