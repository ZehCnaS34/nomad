use crate::ast::Tag;
use crate::interpreter::Interpreter;
use crate::prelude::*;

mod boolean_node;
mod decorator_node;
mod def_node;
mod do_node;
mod function_node;
mod if_node;
mod keyword_node;
mod let_node;
mod list_node;
mod loop_node;
mod macro_node;
mod meta_node;
mod number_node;
mod program_node;
mod quasi_quote_node;
mod quote_node;
mod recur_node;
mod string_node;
mod symbol_node;
mod vector_node;
mod while_node;

use crate::result::runtime::ErrorKind;
pub use boolean_node::BooleanNode;
pub use decorator_node::DecoratorNode;
pub use def_node::DefinitionNode;
pub use do_node::DoNode;
pub use function_node::FunctionCallNode;
pub use function_node::FunctionNode;
pub use if_node::IfNode;
pub use keyword_node::KeywordNode;
pub use let_node::LetNode;
pub use list_node::ListNode;
pub use loop_node::LoopNode;
pub use macro_node::MacroNode;
pub use meta_node::MetaNode;
pub use number_node::NumberNode;
pub use program_node::ProgramNode;
pub use quasi_quote_node::QuasiQuoteNode;
pub use quote_node::QuoteNode;
pub use recur_node::RecurNode;
pub use string_node::StringNode;
pub use symbol_node::SymbolNode;
pub use vector_node::VectorNode;
pub use while_node::WhileNode;

#[derive(Debug, Clone)]
pub enum Node {
    Nil,
    Boolean(BooleanNode),
    Number(NumberNode),
    String(StringNode),
    Symbol(SymbolNode),
    Keyword(KeywordNode),
    Function(FunctionNode),
    FunctionCall(FunctionCallNode),
    Definition(DefinitionNode),
    If(IfNode),
    While(WhileNode),
    Let(LetNode),
    List(ListNode),
    Do(DoNode),
    Program(ProgramNode),
    Vector(VectorNode),
    Recur(RecurNode),
    Loop(LoopNode),
    Quote(QuoteNode),
    QuasiQuote(QuasiQuoteNode),
    Meta(MetaNode),
    Macro(MacroNode),
    Decorator(DecoratorNode),
}

macro_rules! boilerplate {
    ($node:path , $sub:ident, $take:ident, $as:ident) => {
        impl Node {
            pub fn $as(&self) -> Option<&$sub> {
                match self {
                    $node(var) => Some(var),
                    _ => None,
                }
            }

            pub fn $take(self) -> Option<$sub> {
                match self {
                    $node(var) => Some(var),
                    _ => None,
                }
            }
        }
    };
}

boilerplate! { Node::Boolean, BooleanNode, take_boolean, as_boolean }
boilerplate! { Node::Decorator, DecoratorNode, take_dectorator, as_decorator }
boilerplate! { Node::Definition, DefinitionNode, take_definition, as_definition }
boilerplate! { Node::Do, DoNode, take_do, as_do }
boilerplate! { Node::Function, FunctionNode, take_function, as_function }
boilerplate! { Node::FunctionCall, FunctionCallNode, take_function_call, as_function_call }
boilerplate! { Node::If, IfNode, take_if, as_if }
boilerplate! { Node::Keyword, KeywordNode, take_keyword, as_keyword }
boilerplate! { Node::Let, LetNode, take_let, as_let }
boilerplate! { Node::List, ListNode, take_list, as_list }
boilerplate! { Node::Loop, LoopNode, take_loop, as_loop }
boilerplate! { Node::Macro, MacroNode, take_macro, as_macro }
boilerplate! { Node::Meta, MetaNode, take_meta, as_meta }
boilerplate! { Node::Number, NumberNode, take_number, as_number }
boilerplate! { Node::Program, ProgramNode, take_program, as_program }
boilerplate! { Node::QuasiQuote, QuasiQuoteNode, take_quasi_quote, as_quasi_quote }
boilerplate! { Node::Quote, QuoteNode, take_quote, as_quote }
boilerplate! { Node::Recur, RecurNode, take_recur, as_recur }
boilerplate! { Node::String, StringNode, take_string, as_string }
boilerplate! { Node::Symbol, SymbolNode, take_symbol, as_symbol }
boilerplate! { Node::Vector, VectorNode, take_vector, as_vector }
boilerplate! { Node::While, WhileNode, take_while, as_while }

pub trait ToNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, crate::result::runtime::ErrorKind>;
}
