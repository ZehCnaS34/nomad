pub mod atom_node;
pub(crate) mod def_node;
pub mod function_node;
pub mod if_node;
pub mod list_node;
pub mod while_node;

use crate::ast::node::atom_node::Symbol;
use crate::ast::node::def_node::DefinitionNode;
use crate::ast::node::while_node::WhileNode;
use atom_node::AtomNode;
use function_node::{FunctionCallNode, FunctionNode};
use if_node::IfNode;
use list_node::ListNode;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    /// Represents a constant value in nomad.
    /// things like number, string, symbol
    Atom(AtomNode),
    Function(FunctionNode),
    FunctionCall(FunctionCallNode),
    Definition(DefinitionNode),
    If(IfNode),
    While(WhileNode),
    List(ListNode),
}

impl Node {
    pub(crate) fn unwrap_symbol(self) -> Option<Symbol> {
        match self {
            Node::Atom(atom) => match atom {
                AtomNode::Symbol(symbol) => Some(symbol),
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Atom(atom) => write!(f, "{}", atom),
            Node::FunctionCall(function_call) => write!(f, "{}", function_call),
            Node::Definition(definition) => write!(f, "{}", definition),
            Node::If(if_form) => write!(f, "{}", if_form),
            Node::While(while_node) => write!(f, "{}", while_node),
            _ => {
                todo!("function literals are not defined");
            }
        }
    }
}
