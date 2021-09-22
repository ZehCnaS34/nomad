use crate::ast::Tag;
use crate::ast::node::SymbolNode;
use crate::ast::parser::Parser;

#[derive(Debug, Clone)]
pub struct MacroNode {
    name: Tag,
    parameters: Tag,
    body: Vec<Tag>,
}

impl MacroNode {
    pub fn from_tags(parser: &Parser, tags: Vec<Tag>) -> MacroNode {
        let (name, parameters, body) = {
            let mut tags = tags.into_iter();
            tags.next().expect("Dropping macro");
            let name = tags.next().expect("Not enough arguments").take_symbol().expect("Name should be a symbol");
            let arguments = tags.next().expect("Not enough arguments").take_vector().expect("Arguments should be a vector");
            let body = tags.collect::<Vec<_>>();
            (name, arguments, body)
        };
        MacroNode {
            name,
            parameters,
            body
        }
    }
}