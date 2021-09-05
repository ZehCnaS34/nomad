mod analyzer;
mod ast;
mod context;
mod parser;
mod result;
mod rt;
mod scanner;
mod token;
mod util;
mod view;
use crate::result::{issue, runtime_issue, NResult};
use crate::rt::Runtime;
use context::Context;
use std::fs::read_to_string;

fn run_file(file_name: String) -> NResult<()> {
    let context = Context::new();
    let source = read_to_string(file_name.as_str()).or(runtime_issue(format!(
        "Failed to load file: {}",
        file_name.as_str()
    )))?;

    let tokens = scanner::scan(&context, source)?;
    let expr = parser::parse(tokens)?;
    let runtime = Runtime::new();
    let result = runtime.interpret(expr)?;
    println!("result = {}", result);
    Ok(())
}

fn run() {}

fn main() -> NResult<()> {
    use std::env::args;
    let mut args = args();
    let _ = args.next().expect("How did this happen?");
    let filename = args.next().ok_or(issue("A filename is required."))?;
    run_file(filename)?;
    Ok(())
}
