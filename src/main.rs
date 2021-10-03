// #![allow(warnings, unused)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;

use std::fs::read_to_string;

#[macro_use]
pub mod ast;
pub mod interpreter;
pub mod result;
#[macro_use]
pub mod prelude;
mod emitter;

use prelude::*;

fn run_file(file: String) -> Result<()> {
    let source = read_to_string(file).ok().ok_or(General("Fuck"))?;
    let tokens = Scanner::scan(source).ok_or(General("Fuck"))?;
    let ast = parse(tokens)?;
    println!("{:#?}", ast);
    // let mut interpreter = interpreter::Interpreter::new();
    // let result = interpreter.eval(ast)?;
    Ok(())
}

fn main() {
    // win::main();
    let file = cli::start();
    match run_file(file) {
        Ok(..) => {}
        Err(err) => {
            println!("error {:?}", err)
        }
    }
}

mod cli {
    extern crate clap;

    use clap::{App, Arg};

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
        String::from(config)
    }
}
