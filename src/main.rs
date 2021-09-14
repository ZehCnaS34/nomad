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

// const SOURCE_FILE: &'static str = "./collatz.nd";
const SOURCE_FILE: &'static str = "./core.nd";
// const SOURCE_FILE: &'static str = "./init.el";

struct Environment {}

#[derive(Debug)]
struct MainResult;

fn run_repl() {}

fn run_file() {}

fn main() {
    let file = cli::start();
    let source = read_to_string(file).expect("Failed to read source file");
    let tokens = Scanner::scan(source).expect("Failed to tokenize file");
    let ast = parse(tokens).expect("Failed to parse AST");
    interpreter::interpret(ast);
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
