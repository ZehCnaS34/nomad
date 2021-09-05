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
use crate::result::NResult;
use context::Context;
use crate::rt::Runtime;
use std::fs::read_to_string;
use crate::ast::print_ast;

fn main() -> NResult<()> {
    // use Value::*;
    // let rt = Runtime::new();

    // rt.define("age".into(), Some(Number(26.0)))?;

    // if let Ok(value) = rt.add(&Symbol("age".into()), &Number(40.0)) {
    //     println!("value = {:?}", value);
    // }

    let context = Context::new();
    if let Ok(source) = read_to_string("simple.nd") {
        let tokens = scanner::scan(&context, source)?;
        let expr = parser::parse(tokens)?;
        // print_ast(&expr);
        let runtime = Runtime::new();
        let result = runtime.interpret(expr)?;
        println!("{}", result);
        println!("runtime {:#?}", runtime);
        // let expr = analyzer::analyze(expr, runtime)?;
        // println!("analyze {:#?}", expr);
    }

    Ok(())
}
