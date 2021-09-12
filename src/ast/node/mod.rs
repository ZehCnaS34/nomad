use crate::ast::Tag;
use crate::interpreter::{Execute, Interpreter};

mod boolean_node;
mod def_node;
mod do_node;
mod function_node;
mod if_node;
mod keyword_node;
mod list_node;
mod loop_node;
mod number_node;
mod program_node;
mod recur_node;
mod string_node;
mod symbol_node;
mod vector_node;
mod while_node;

pub use boolean_node::BooleanNode;
pub use def_node::DefinitionNode;
pub use do_node::DoNode;
pub use function_node::FunctionCallNode;
pub use function_node::FunctionNode;
pub use if_node::IfNode;
pub use keyword_node::KeywordNode;
pub use list_node::ListNode;
pub use loop_node::LoopNode;
pub use number_node::NumberNode;
pub use program_node::ProgramNode;
pub use recur_node::RecurNode;
pub use string_node::StringNode;
pub use symbol_node::SymbolNode;
pub use vector_node::VectorNode;
pub use while_node::WhileNode;

#[derive(Debug, Clone)]
pub enum Node {
    Nil,
    Boolean(boolean_node::BooleanNode),
    Number(number_node::NumberNode),
    String(string_node::StringNode),
    Symbol(symbol_node::SymbolNode),
    Keyword(keyword_node::KeywordNode),
    Function(function_node::FunctionNode),
    FunctionCall(function_node::FunctionCallNode),
    Definition(def_node::DefinitionNode),
    If(if_node::IfNode),
    While(while_node::WhileNode),
    List(list_node::ListNode),
    Do(do_node::DoNode),
    Program(program_node::ProgramNode),
    Vector(vector_node::VectorNode),
    Recur(recur_node::RecurNode),
    Loop(loop_node::LoopNode),
}

impl Node {
    pub fn as_symbol(&self) -> Option<&SymbolNode> {
        match self {
            Node::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }
}
