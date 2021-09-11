use crate::ast::node::atom_node::AtomNode;
use crate::ast::parser::Tag;
use crate::interpreter::{Execute, Interpreter};

pub mod atom_node;
pub mod def_node;
pub mod do_node;
pub mod function_node;
pub mod if_node;
pub mod list_node;
pub mod program_node;
pub mod while_node;

#[derive(Debug, Clone)]
pub enum Node {
    /// Represents a constant value in nomad.
    /// things like number, string, symbol
    Atom(atom_node::AtomNode),
    /// Represents an anonymous or named function
    Function(function_node::FunctionNode),
    FunctionCall(function_node::FunctionCallNode),
    Definition(def_node::DefinitionNode),
    If(if_node::IfNode),
    While(while_node::WhileNode),
    List(list_node::ListNode),
    Do(do_node::DoNode),
    Program(program_node::ProgramNode),
}

impl Node {
    pub fn as_atom(&self) -> Option<&atom_node::AtomNode> {
        match self {
            Node::Atom(node) => Some(node),
            _ => None,
        }
    }
}

impl Execute for Node {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) -> AtomNode {
        match self {
            Node::Atom(node) => node.execute(interpreter, own_tag),
            Node::Function(node) => node.execute(interpreter, own_tag),
            Node::FunctionCall(node) => node.execute(interpreter, own_tag),
            Node::Definition(node) => node.execute(interpreter, own_tag),
            Node::If(node) => node.execute(interpreter, own_tag),
            Node::While(node) => node.execute(interpreter, own_tag),
            Node::List(node) => node.execute(interpreter, own_tag),
            Node::Do(node) => node.execute(interpreter, own_tag),
            Node::Program(node) => node.execute(interpreter, own_tag),
        }
    }
}
