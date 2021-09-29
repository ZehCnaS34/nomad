use std::any::Any;
use std::fmt;

use crate::ast::node::{Node, SymbolNode, ToNode, VectorNode};
use crate::ast::tag::Partition;
use crate::result::parser;
use crate::result::runtime::ErrorKind::General;
use crate::{ast, ast::Tag};

trait Show {
    fn show(self) -> Self
    where
        Self: Sized + fmt::Debug;
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    function: Box<Node>,
    arguments: Vec<Node>,
}

impl ToNode for FunctionCallNode {
    fn make_node(nodes: Vec<Node>) -> Result<Node, parser::ErrorKind> {
        let (function, arguments) = nodes.take_1().unwrap();
        Ok(Node::FunctionCall(FunctionCallNode{
            function: Box::new(function),
            arguments,
        }))
    }
}

impl FunctionCallNode {
    pub fn function(&self) -> Tag {
        todo!()
    }
    pub fn arguments(&self) -> Vec<Tag> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum FunctionNode {
    Named {
        name: SymbolNode,
        parameters: VectorNode,
        body: Vec<Node>,
    },
    Anonymous {
        parameters: VectorNode,
        body: Vec<Node>,
    },
}

impl ToNode for FunctionNode {
    fn make_node(tags: Vec<Node>) -> Result<Node, parser::ErrorKind> {
        let (form, name_or_params, params_or_first_body, body) = tags.take_3().unwrap();
        Ok(Node::Function(
            match (name_or_params, params_or_first_body) {
                (Node::Symbol(name), Node::Vector(parameters)) => FunctionNode::Named {
                    name,
                    parameters,
                    body,
                },
                (Node::Vector(parameters), node) => FunctionNode::Anonymous {
                    parameters,
                    body: vec![vec![node], body].concat(),
                },
                (_, _) => return Err(parser::ErrorKind::General("fuck")),
            },
        ))
    }
}

impl FunctionNode {
    pub fn parameters(&self) -> Tag {
        todo!()
    }

    pub fn body(&self) -> Vec<Tag> {
        todo!()
    }

    pub fn name(&self) -> Option<Tag> {
        todo!()
    }
}
