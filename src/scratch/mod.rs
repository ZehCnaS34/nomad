use crate::interpreter::{Symbol, Value, Var};
use crate::result::runtime::ErrorKind::General;
use crate::result::RuntimeResult;

type Result<T> = RuntimeResult<T>;

fn gen<T>(msg: &'static str) -> Result<T> {
    Err(General(msg))
}

struct Context {}

impl Context {
    fn new() -> Context {
        Context {}
    }
    fn resolve<S>(&self, symbol: S) -> Result<Value>
    where
        S: Into<Symbol>
    {
        todo!()
    }

    fn define<S, V>(&self, symbol: S, value: V) -> Result<Var>
    where
        S: Into<Symbol>,
        V: Into<Value>,
    {
        todo!()
    }
}

fn simple() {
    let mut ctx = Context::new();
    ctx.define(("awesome"), "value");
    ctx.resolve(("awesome"));

    ctx.define(("awesome", "awesome"), "awesome");

}