#![allow(warnings, unused)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;
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

struct Environment {}

#[derive(Debug)]
struct MainResult;

fn run_repl() {}

macro_rules! take {
    ($value:expr) => {
        match $value {
            Some(value) => Some(value),
            None => return None,
        }
    };
}

fn run_file(file: String) {
    let source = read_to_string(file).expect("Failed to read source file");
    let tokens = Scanner::scan(source).expect("Failed to tokenize file");
    let ast = parse(tokens).expect("Failed to parse AST");
    let mut interpreter = interpreter::Interpreter::new();
    match interpreter.eval(ast) {
        Ok(result) => println!("{:?}", result),
        Err(err) => println!("failure {:?}", err),
    }
    println!("{:#?}", interpreter);
}

fn main() {
    // win::main();
    let file = cli::start();
    run_file(file);
}

mod cli {
    extern crate clap;
    use clap::{App, Arg, SubCommand};

    pub fn start() -> String {
        let matches = App::new("nomad")
            .version("0.0.0")
            .author("Alexander Sanchez <the@mild.one>")
            .about("A programming language with no home.")
            .arg(
                Arg::with_name("source")
                    .value_name("FILE")
                    .help("The file to run"),
            )
            .get_matches();
        let config = matches.value_of("source").unwrap_or("core.nd");
        return String::from(config);
    }
}
