#![allow(warnings, unused)]

#[macro_use]
pub mod ast;
pub mod interpreter;
pub mod result;

use crate::ast::parser;
use crate::ast::parser::parse;
use crate::ast::scanner::Scanner;
use std::cell::Cell;
use std::fs::read_to_string;
use std::io;
use std::str::FromStr;

const SOURCE_FILE: &'static str = "./collatz.nd";
// const SOURCE_FILE: &'static str = "./core.nd";
// const SOURCE_FILE: &'static str = "./init.el";

struct Environment {}

#[derive(Debug)]
struct MainResult;

fn main() -> Result<(), MainResult> {
    let source = read_to_string(SOURCE_FILE).expect("Failed to read source file");
    let tokens = Scanner::scan(source).ok_or(MainResult)?;
    let ast = parse(tokens).unwrap();
    // println!("{:#?}", ast);
    interpreter::interpret(ast);
    Ok(())
}
