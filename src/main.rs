mod scanner;
mod rt;
mod view;
mod parser;
mod context;
mod util;
mod analyzer;
mod ast;
mod token;
mod result;
use crate::result::NResult;
use crate::rt::Runtime;
use crate::ast::Value;
use context::Context;
use std::fs::read_to_string;


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
        println!("tokens {:#?}", tokens);
        let expr = parser::parse(tokens)?;
        println!("parse {:#?}", expr);
        // let runtime = Runtime::new();
        // let expr = analyzer::analyze(expr, runtime)?;
        // println!("analyze {:#?}", expr);
    }
    Ok(())
}
