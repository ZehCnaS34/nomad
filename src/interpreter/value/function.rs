use crate::ast::Tag;
use super::symbol::Symbol;

pub struct Function {
    name: Symbol,
    parameters: Vec<Symbol>,
    body: Vec<Tag>
}

#[derive(Debug, Clone)]
pub enum NativeFunction {
    Plus,
    Minus,
    Multiply,
    DumpContext,
    Divide,
    Or,
    Eq,
    Print,
    Println,
    LessThan,
    Mod,
    GreaterThan,
    Now,
}
