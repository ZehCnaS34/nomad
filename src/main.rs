#![allow(warnings, unused)]

mod list;
mod parser;
mod scanner;
mod tree;
mod util;

use crate::list::List;
use crate::parser::{parse, Atom, Expr};
use crate::scanner::{Scanner, Token, TokenKind};
use std::cell::Cell;
use std::fs::read_to_string;
use std::io;
use std::str::FromStr;

const SOURCE_FILE: &'static str = "fact.nd";

struct Environment {}

impl Environment {
    fn new() -> Environment {
        Environment {}
    }
}

fn interpret_expression(expression: &Expr, env: &mut Environment) -> Result<Atom, &'static str> {
    match expression {
        Expr::Atom(value) => Ok(value.clone()),
        Expr::List(list) => {
            let (head, tail) = (list.head(), list.tail());
            if head.is_none() {
                Ok(Atom::Nil)
            } else {
                let head = head.unwrap();
                match head {
                    Expr::Atom(atom) => match atom {
                        Atom::Symbol(symbol) => {
                            match &symbol[..] {
                                "def" => {
                                    let (ident, value) =
                                        (tail.head().unwrap().clone(), tail.tail());
                                    let ident = ident
                                        .take_atom()
                                        .and_then(|atom| atom.take_symbol())
                                        .ok_or("invalid ident")?;
                                    println!("symbol {:?}", ident);
                                    println!("value {:?}", value);
                                }
                                "while" => {
                                    let (ident, value) =
                                        (tail.head().unwrap().clone(), tail.tail());
                                    let ident = ident
                                        .take_atom()
                                        .and_then(|atom| atom.take_symbol())
                                        .ok_or("invalid ident")?;
                                    println!("symbol {:?}", ident);
                                    println!("value {:?}", value);
                                }
                                "if" => {
                                    let (ident, value) =
                                        (tail.head().unwrap().clone(), tail.tail());
                                    let ident = ident
                                        .take_atom()
                                        .and_then(|atom| atom.take_symbol())
                                        .ok_or("invalid ident")?;
                                    println!("symbol {:?}", ident);
                                    println!("value {:?}", value);
                                }
                                "cons" => {
                                    let (ident, value) =
                                        (tail.head().unwrap().clone(), tail.tail());
                                    let ident = ident
                                        .take_atom()
                                        .and_then(|atom| atom.take_symbol())
                                        .ok_or("invalid ident")?;
                                    println!("symbol {:?}", ident);
                                    println!("value {:?}", value);
                                }
                                value => {
                                    println!("value={:?}", value);
                                }
                            }
                            Ok(Atom::Symbol(symbol.clone()))
                        }
                        Atom::String(string) => Ok(Atom::String(string.clone())),
                        Atom::Number(number) => Ok(Atom::Number(*number)),
                        Atom::Bool(boolean) => Ok(Atom::Bool(*boolean)),
                        Atom::Nil => Ok(Atom::Nil),
                    },
                    expression => interpret_expression(expression, env),
                }
            }
        }
    }
}

fn interpret(expressions: Vec<Expr>, env: &mut Environment) {
    for e in &expressions {
        println!("value {:?}", interpret_expression(e, env));
    }
}

#[derive(Debug)]
struct MainResult;

fn main() -> Result<(), MainResult> {
    let source = read_to_string(SOURCE_FILE).expect("Failed to read source file");
    let tokens = Scanner::scan(source).ok_or(MainResult)?;
    println!("{:#?}", tokens);
    // let expressions = parse(tokens).ok_or(MainResult)?;
    // let mut environment = Environment::new();
    // let value = interpret(expressions, &mut environment);
    Ok(())
}
