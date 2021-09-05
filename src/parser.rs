use crate::ast;
use crate::ast::{Expr, Value};
use crate::result::{Issue, NResult, parse_issue};
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
        1 => {
            let function = Box::new(expressions.iter().next().unwrap().clone());
            Ok(Expr::Invoke{
                function,
                parameters: vec![],
            })
        },
        _ => {
            let mut args = expressions.iter_mut();
            let function = Box::new(args.next().unwrap().clone());
            let parameters: Vec<ast::Expr> = args.map(|expr| expr.clone()).collect();
            Ok(Expr::Invoke{
                function,
                parameters
            })
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

fn parse_decorator(view: &Tokens) -> ParseResult {
    let mutator = Box::new(parse_expr(view)?);
    let mut parameters = vec![];
    while !view.peek_kind_test(TokenType::RightBracket) && !view.peek_kind_test(TokenType::Eof) {
        parameters.push(parse_expr(view)?);
    }
    if view.peek_kind_test(TokenType::Eof) {
        return parse_issue("Unexpected EOF");
    }
    view.advance().unwrap();
    let target = Box::new(parse_expr(view)?);
    Ok(Expr::Decorator{
        mutator,
        parameters,
        target,
    })
}

fn parse_block(view: &Tokens) -> ParseResult {
    use Expr::Block;
    use TokenType::*;
    let mut expressions = vec![];
    while !view.peek_kind_test(RightBrace) && !view.peek_kind_test(Eof) {
        expressions.push(parse_expr(view)?);
    }
    if view.peek_kind_test(Eof) {
        return parse_issue("Expected }, given [eof]");
    }
    view.advance().unwrap();
    Ok(Block{expressions})
}

fn parse_expr(view: &Tokens) -> ParseResult {
    match view.advance() {
        Some(token) => match token.token_type {
            TokenType::Eof => parse_issue("Unexpected eof"),
            TokenType::String | TokenType::Keyword | TokenType::Symbol | TokenType::Number | TokenType::Slash => {
                if let Ok(value) = Value::from_str(token.lexeme.as_str()) {
                    Ok(Expr::Atom(value))
                } else {
                    Err(Issue::parse_error("Failed to parse atom.", &view.cursor))
                }
            }
            TokenType::HashLeftBrace => parse_set(view),
            TokenType::LeftBrace => parse_block(view),
            TokenType::LeftParen => parse_list(view),
            TokenType::LeftBracket => parse_vector(view),
            TokenType::HashLeftBracket => parse_decorator(view),
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
