mod span;
pub mod token;

use span::Span;
use std::cell::Cell;
use token::{Kind, Token};

const EOF: char = '\0';
const NL: char = '\n';
const TAB: char = '\t';
const R: char = '\r';

fn is_newline(c: char) -> bool {
    NL == c
}
fn is_quote(c: char) -> bool {
    '"' == c
}

fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || "!$%^&*_-+=|<>?".contains(c)
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || "!$%^&*_-+=|<>?".contains(c)
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
    source: Vec<char>,
    span: Span,
}

impl SymbolMatcher for Scanner {
    fn is_symbol_start(&self) -> bool {
        self.check(is_symbol_start)
    }

    fn is_symbol_char(&self) -> bool {
        self.check(is_symbol_char)
    }

    fn is_obj_delim(&self) -> bool {
        self.peek_2()
            .map(|(dot, start)| dot == '.' && is_symbol_start(start))
            .unwrap_or(false)
    }

    fn is_ns_delim(&self) -> bool {
        self.peek_2()
            .map(|(slash, start)| slash == '/' && is_symbol_start(start))
            .unwrap_or(false)
    }
}

impl NumberMatcher for Scanner {
    fn is_digit(&self) -> bool {
        self.check(|char| char.is_digit(10))
    }

    fn is_dot_digit(&self) -> bool {
        self.peek_2()
            .map(|(dot, digit)| dot == '.' && digit.is_digit(10))
            .unwrap_or(false)
    }
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            span: Span::new(),
        }
    }

    fn ignore(&self) {
        self.eat();
        self.span.view(&self.source[..]);
    }

    fn make_token(&self, kind: Kind) -> Option<Token> {
        let span = self.span.clone();
        self.span.view(&self.source[..]).map(|lexeme| Token {
            kind,
            lexeme: String::from(lexeme),
            span,
        })
    }

    fn eat(&self) {
        self.span.advance(&self.source[..]);
    }

    fn peek_2(&self) -> Option<(char, char)> {
        let ref source = &self.source[..];
        let one = self.span.peek_n(source, 0)?;
        let two = self.span.peek_n(source, 1)?;
        Some((one, two))
    }

    fn check<F>(&self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        self.span.peek(&self.source[..]).map(f).unwrap_or(false)
    }

    fn check_not<F>(&self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        self.span
            .peek(&self.source[..])
            .map(|c| !f(c))
            .unwrap_or(false)
    }

    fn scan_comment(&self) -> Option<Token> {
        while self.check_not(is_newline) {
            self.eat();
        }
        self.make_token(Kind::Comment)
    }

    fn scan_delimiter(&self, kind: Kind) -> Option<Token> {
        self.eat();
        self.make_token(kind)
    }

    fn scan_symbol(&self) -> Option<Token> {
        let name = || {
            while self.is_symbol_char() {
                self.eat();
                if self.is_obj_delim() {
                    self.eat();
                }
            }
        };
        name();
        if self.is_ns_delim() {
            self.eat();
            name();
        }
        self.make_token(Kind::Symbol)
    }

    fn scan_number(&self) -> Option<Token> {
        println!("scanning number");
        let digits = || {
            while self.is_digit() {
                self.eat();
            }
        };
        digits();
        if self.is_dot_digit() {
            self.eat();
            digits();
        }
        self.make_token(Kind::Number)
    }

    fn peek(&self) -> Option<char> {
        let source = &self.source[..];
        self.span.peek(source)
    }

    fn scan_string(&self) -> Option<Token> {
        self.eat();
        while self.check_not(is_quote) {
            self.eat();
        }
        self.eat();
        self.make_token(Kind::String)
    }

    pub fn scan(source: String) -> Option<Vec<Token>> {
        use Kind::*;
        let mut tokens = vec![];
        let scanner = Scanner::new(source);

        while let Some(c) = scanner.peek() {
            match c {
                EOF => break,
                '/' => {
                    scanner.eat();
                    tokens.push(scanner.make_token(Kind::Symbol)?);
                }
                ',' => {
                    scanner.ignore();
                }
                ' ' => {
                    scanner.ignore();
                }
                NL => {
                    scanner.ignore();
                }
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
                c => {
                    panic!("we should handle every character {:?} {:?}", c, c as u32);
                }
            }
        }

        tokens.push(Token {
            kind: Kind::Eof,
            lexeme: std::string::String::new(),
            span: scanner.span.clone(),
        });

        Some(tokens)
    }
}
