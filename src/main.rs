#![allow(warnings, unused)]

use std::fs::read_to_string;
use std::io;

const SOURCE_FILE: &'static str = "fib.nd";

trait Matcher {
    fn check_not(&self, offset: usize, value: char) -> bool;
    fn check(&self, offset: usize, value: char) -> bool;
    fn test<F>(&self, offset: usize, tester: F) -> bool
        where
            F: Fn(char) -> bool;
    fn check_next(&self, offset: usize, value: char) -> bool;
    fn test_next<F>(&self, offset: usize, tester: F) -> bool
        where
            F: Fn(char) -> bool;
}

impl Matcher for &Vec<char> {
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

#[derive(Debug)]
enum TokenKind {
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
struct Token {
    kind: TokenKind,
    lexeme: String,
    start: usize,
    end: usize,
    line: usize,
}

fn is_symbol_start(c: char) -> bool {
    c.is_alphabetic() || "!$%^&*_-+=|<>?".contains(c)
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || "!$%^&*_-+=|<>?".contains(c)
}

#[derive(Debug)]
enum Action {
    Skip,
    Newline,
    ScanNumber,
    ScanComment,
    ScanString,
    ScanSingle(TokenKind),
    ScanSymbol,
    Fatal(char),
}

fn infer_action(source: &Vec<char>, context: &Context) -> Action {
    use Action::*;
    use TokenKind::*;
    match source[context.end] {
        ',' => Skip,
        ' ' => Skip,
        '\r' => Skip,
        '\t' => Skip,
        '\n' => Newline,
        '(' => ScanSingle(LeftParen),
        ')' => ScanSingle(RightParen),
        '[' => ScanSingle(LeftBracket),
        ']' => ScanSingle(RightBracket),
        '{' => ScanSingle(LeftBrace),
        '}' => ScanSingle(RightBrace),
        '"' => ScanString,
        ';' => ScanComment,
        c if is_symbol_start(c) => ScanSymbol,
        c if c.is_digit(10) => ScanNumber,
        c => Fatal(c),
    }
}

fn act(context: &mut Context, source: &Vec<char>, action: Action) {
    use Action::*;
    match action {
        Skip => {
            context.start += 1;
            context.end += 1;
        }
        Newline => {
            context.start += 1;
            context.end += 1;
            context.line += 1;
        }
        ScanNumber => {
            let (start, mut end) = context.location();
            while source.test(end, |c| c.is_digit(10)) {
                end += 1;
            }
            if source.check(end, '.') && source.test_next(end, |c| c.is_digit(10)) {
                end += 1;
                while source.test(end, |c| c.is_digit(10)) {
                    end += 1;
                }
            }
            context.push(TokenKind::Number, source, start, end);
        }
        ScanSymbol => {
            let (start, mut end) = context.location();
            while source.test(end, is_symbol_char) {
                end += 1;
                while source.check(end, '.') && source.test_next(end, is_symbol_start) {
                    end += 1;
                }
            }
            while source.check(end, '/') && source.test_next(end, is_symbol_start) {
                end += 1;
                while source.test(end, is_symbol_char) {
                    end += 1;
                    while source.check(end, '.') && source.test_next(end, is_symbol_start) {
                        end += 1;
                    }
                }
            }
            context.push(TokenKind::Symbol, source, start, end);
        }
        ScanString => {
            let (start, mut end) = context.location();
            while source.check_not(end, '"') {
                end += 1;
            }
            if end == source.len() {
                panic!("Unmatched string delimiter");
            }
            end += 1;
            context.push(TokenKind::String, source, start, end);
        }
        ScanSingle(kind) => {
            let (start, mut end) = context.location();
            while end < source.len() && end - start < 1 {
                end += 1;
            }
            context.push(kind, source, start, end);
        }
        ScanComment => {
            let (start, mut end) = context.location();
            while source.check_not(end, '\n') {
                end += 1;
            }
            context.push(TokenKind::Comment, source, start, end);
        }
        Fatal(c) => {
            panic!("unhandled character. c = {}", c);
        }
    }
}

#[derive(Debug)]
struct Context {
    tokens: Vec<Token>,
    start: usize,
    end: usize,
    line: usize,
}

impl Context {
    fn new() -> Context {
        Context {
            tokens: vec![],
            start: 0,
            end: 0,
            line: 1,
        }
    }

    fn location(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    fn push(&mut self, kind: TokenKind, source: &Vec<char>, start: usize, end: usize) {
        let mut lexeme = String::new();
        for i in start..end {
            lexeme.push(source[i]);
        }
        self.tokens.push(Token {
            kind,
            lexeme,
            start,
            end,
            line: self.line,
        });
        self.start = end;
        self.end = end;
    }

    fn cap(&mut self) {
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            start: self.start,
            end: self.end,
            line: self.line,
        })
    }
}

fn scan(source: &String) -> Vec<Token> {
    let mut context = Context::new();
    let mut source: Vec<_> = source.chars().collect();
    while let Some(_) = source.get(context.end) {
        let action = infer_action(&source, &context);
        act(&mut context, &source, action);
    }
    context.cap();
    context.tokens
}

#[derive(Debug)]
enum Value {
    String(String),
    Number(f32),
    Symbol(String),
}

#[derive(Debug)]
enum List {
    Empty,
    Cons(Box<Expr>, Box<List>)
}

#[derive(Debug)]
enum Expr {
    Atom(Value),
    List(List)
}

fn main() -> Result<(), io::Error> {
    let source = read_to_string(SOURCE_FILE)?;
    let tokens = scan(&source);
    for token in tokens {
        println!("token = {:?}", token);
    }
    Ok(())
}
