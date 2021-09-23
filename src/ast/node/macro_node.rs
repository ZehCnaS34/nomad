use crate::ast::node::SymbolNode;
use crate::ast::parser::Parser;
use crate::ast::Tag;

#[derive(Debug, Clone)]
pub struct MacroNode {
    name: Tag,
    parameters: Tag,
    body: Vec<Tag>,
}

impl MacroNode {}
