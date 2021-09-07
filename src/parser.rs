extern crate im;
use crate::list::List;
use crate::parser::ParseError::Eof;
use crate::scanner::TokenKind::Comment;
use crate::scanner::{Token, TokenKind};
use im::Vector;
use std::cell::Cell;
use std::str::FromStr;

pub(crate) enum ParseError {
    Eof,
}

pub(crate) type ParseResult = Result<Expr, ParseError>;

#[derive(Debug, Clone)]
pub(crate) enum Atom {
    String(String),
    Number(f32),
    Symbol(String),
    Nil,
    Bool(bool),
}

impl Atom {
    pub fn take_symbol(self) -> Option<String> {
        match self {
            Atom::Symbol(value) => Some(value),
            _ => None,
        }
    }
}

impl FromStr for Atom {
    type Err = ParseError;

    fn from_str(atom: &str) -> Result<Self, Self::Err> {
        if atom == "nil" {
            Ok(Atom::Nil)
        } else if atom == "true" {
            Ok(Atom::Bool(false))
        } else if atom == "false" {
            Ok(Atom::Bool(false))
        } else if atom.starts_with('"') {
            Ok(Atom::String(String::from(&atom[1..atom.len() - 1])))
        } else if let Ok(value) = atom.parse() {
            Ok(Atom::Number(value))
        } else {
            Ok(Atom::Symbol(String::from(atom)))
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Atom(Atom),
    List(List<Expr>),
}

impl Expr {
    pub fn take_atom(self) -> Option<Atom> {
        match self {
            Expr::Atom(value) => Some(value),
            _ => None,
        }
    }
}

enum Action {}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn init() -> Range {
        Range { start: 0, end: 0 }
    }

    fn expand(self) -> Range {
        Range {
            end: self.end + 1,
            ..self
        }
    }
}

#[derive(Debug)]
struct Context {
    tokens: Vec<Token>,
    range: Cell<Range>,
}

impl Context {
    fn new(tokens: Vec<Token>) -> Context {
        Context {
            tokens,
            range: Cell::new(Range::init()),
        }
    }

    fn get(&self) -> Option<&Token> {
        let range = self.range.get();
        if range.end < self.tokens.len() {
            Some(&self.tokens[range.end])
        } else {
            None
        }
    }

    fn is_not(&self, kind: TokenKind) -> bool {
        let range = self.range.get();
        range.end >= self.tokens.len() || self.tokens[range.end].kind == kind
    }

    fn expand(&self) {
        self.range.set(self.range.get().expand());
    }
}

fn parse_list(context: &mut Context) -> ParseResult {
    use TokenKind::*;
    let mut list = List::new();
    context.expand();
    loop {
        let token = context.get().ok_or(ParseError::Eof)?;
        if token.kind != RightParen {
            list = list.prepend(parse_expr(context)?);
        } else {
            context.expand();
            break;
        }
    }
    Ok(Expr::List(list.reverse()))
}

fn parse_expr(context: &mut Context) -> ParseResult {
    use TokenKind::*;
    let token = context.get().ok_or(ParseError::Eof)?;
    match token.kind {
        Symbol | Number | String => {
            let atom = Atom::from_str(&token.lexeme[..])?;
            context.expand();
            Ok(Expr::Atom(atom))
        }
        LeftParen => parse_list(context),
        token => panic!("token not handled {:?}", token),
    }
}

pub(crate) fn parse(tokens: Vec<Token>) -> Option<Vec<Expr>> {
    use TokenKind::*;
    let tokens = tokens
        .into_iter()
        .filter(|token| token.kind != Comment)
        .collect();
    let mut context = Context::new(tokens);
    let mut expressions = vec![];
    loop {
        let token = context.get()?;
        if token.kind == Eof {
            break;
        } else {
            expressions.push(parse_expr(&mut context).ok()?);
        }
    }
    Some(expressions)
}
