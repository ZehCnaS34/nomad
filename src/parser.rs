use crate::ast;
use crate::ast::{Expr, Value};
use crate::result::{Issue, NResult};
use crate::token::{Token, TokenType};
use crate::view::{KindMatcher, View};
use im::Vector;
use std::str::FromStr;

type Tokens = View<Token>;
type ParseResult = NResult<Expr>;

fn is_eof(token: &Token) -> bool {
    token.token_type == TokenType::Eof
}

impl KindMatcher for Tokens {
    type Kind = TokenType;

    fn peek_kind_test(&self, kind: Self::Kind) -> bool {
        self.peek_test(|token| token.token_type == kind)
    }
}

fn parse_vector(view: &Tokens) -> ParseResult {
    let mut expressions: Vector<Expr> = Vector::new();
    while view.peek_test(|token| token.token_type != TokenType::RightBracket) {
        let expr = parse_expr(&view)?;
        expressions.push_back(expr);
    }
    view.advance();
    Ok(Expr::Vector(expressions))
}

fn parse_list(view: &Tokens) -> ParseResult {
    let mut expressions: Vector<Expr> = Vector::new();
    while view.peek_test(|token| token.token_type != TokenType::RightParen) {
        let expr = parse_expr(&view)?;
        expressions.push_back(expr);
    }
    view.advance();
    match expressions.len() {
        0 => Ok(Expr::List(expressions)),
        1 => Ok(Expr::Invoke(
            Box::new(expressions.iter().next().unwrap().clone()),
            vec![],
        )),
        _ => {
            let mut args = expressions.iter_mut();
            let f = args.next().unwrap().clone();
            let args: Vec<ast::Expr> = args.map(|expr| expr.clone()).collect();
            Ok(Expr::Invoke(Box::new(f), args.into()))
        }
    }
}

fn parse_set(view: &Tokens) -> ParseResult {
    let mut expressions: Vec<Expr> = vec![];
    while view.peek_test(|token| token.token_type != TokenType::RightBrace) {
        let expr = parse_expr(&view)?;
        expressions.push(expr);
    }
    view.advance();
    Ok(Expr::HashSet(expressions))
}

fn parse_expr(view: &Tokens) -> ParseResult {
    match view.advance() {
        Some(token) => match token.token_type {
            TokenType::String | TokenType::Keyword | TokenType::Symbol | TokenType::Number | TokenType::Slash => {
                if let Ok(value) = Value::from_str(token.lexeme.as_str()) {
                    Ok(Expr::Atom(value))
                } else {
                    Err(Issue::parse_error("Failed to parse atom.", &view.cursor))
                }
            }
            TokenType::HashLeftBrace => parse_set(&view),
            TokenType::LeftParen => parse_list(&view),
            TokenType::LeftBracket => parse_vector(&view),
            v => {
                println!("{:?}", v);
                todo!();
            }
        },
        None => Err(Issue::parse_error("What the fuck", &view.cursor)),
    }
}

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    let view = View::<Token>::new(tokens);
    let mut expressions: Vec<Expr> = vec![];

    while !view.peek_test(is_eof) {
        match parse_expr(&view) {
            Ok(expr) => {
                expressions.push(expr);
                view.reset();
            }
            error => {
                return error;
            }
        }
    }

    Ok(Expr::Program(expressions))
}
