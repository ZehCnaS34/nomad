use crate::defnode;
use crate::prelude::*;
use std::any::Any;

use crate::ast::node::{Node, SymbolNode, ToNode, VectorNode};
use crate::ast::tag::Partition;
use crate::result::runtime;
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

defnode! {
    Node::FunctionCall : FunctionCallNode :: nodes => {
        let (function, arguments) = nodes.take_1().unwrap();
        Ok(FunctionCallNode {
            function: Box::new(function),
            arguments,
        })
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

defnode! {
    Node::Function : FunctionNode :: nodes => {
        let (form, name_or_params, params_or_first_body, body) = nodes.take_3().unwrap();
        Ok(
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
                (_, _) => return Err(General("fuck")),
            },
        )
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
