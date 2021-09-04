use crate::ast::{Expr, Value};
use crate::result::{Issue, IssueType, NResult, Skimmer};
use crate::token::{Token, TokenType};
use std::str::FromStr;
use crate::view::View;
use std::f32;

type Tokens = View<Token>;
pub type ParseResult = NResult<Expr>;

fn is_eof(token: &Token) -> bool {
    token.token_type == TokenType::Eof
}

fn parse_vector(view: &Tokens) -> ParseResult {
    let mut expressions: Vec<Expr> = vec![];
    while view.peek_test(|token| token.token_type != TokenType::RightBracket) {
        let expr = parse_expr(&view)?;
        expressions.push(expr);
    }
    view.advance();
    Ok(Expr::Vector(expressions))
}

fn parse_list(view: &Tokens) -> ParseResult {
    let mut expressions: Vec<Expr> = vec![];
    while view.peek_test(|token| token.token_type != TokenType::RightParen) {
        let expr = parse_expr(&view)?;
        expressions.push(expr);
    }
    view.advance();
    match expressions.len() {
        0 => Ok(Expr::List(expressions)),
        1 => Ok(Expr::Application(
            Box::new(expressions.first().unwrap().clone()),
            vec![],
        )),
        _ => {
            let f = expressions.first().unwrap().clone();
            let args = expressions.get(1..expressions.len()).unwrap().clone();
            Ok(Expr::Application(Box::new(f), args.into()))
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
    if let Some(token) = view.advance() {
        match token.token_type {
            TokenType::String | TokenType::Keyword | TokenType::Symbol | TokenType::Number => {
                if let Ok(value) = Value::from_str(token.lexeme.as_str()) {
                    Ok(Expr::Atom(value))
                } else {
                    Err(Issue::parse_error("Failed", &view.cursor))
                }
            }
            TokenType::HashLeftBrace => parse_set(&view),
            TokenType::LeftParen => parse_list(&view),
            TokenType::LeftBracket => parse_vector(&view),
            _ => Err(Issue::parse_error("What the fuck", &view.cursor)),
        }
    } else {
        Err(Issue::parse_error("What the fuck", &view.cursor))
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
