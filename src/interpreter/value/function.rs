use super::symbol::Symbol;
use crate::ast::Tag;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Option<Symbol>,
    pub parameters: Vec<Tag>,
    pub body: Vec<Tag>,
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
