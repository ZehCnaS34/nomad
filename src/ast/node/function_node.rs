use std::any::Any;
use std::fmt;

use crate::ast::node::{Node, ToNode};
use crate::ast::tag::Partition;
use crate::result::parser;
use crate::result::parser::ErrorKind::General;
use crate::{ast, ast::Tag};

trait Show {
    fn show(self) -> Self
    where
        Self: Sized + fmt::Debug;
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    function: Tag,
    arguments: Vec<Tag>,
}

impl ToNode for FunctionCallNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, parser::ErrorKind> {
        let mut tags = tags.into_iter();
        Ok(Node::FunctionCall(FunctionCallNode {
            function: tags.next().ok_or(parser::ErrorKind::CouldNotParseAtom)?,
            arguments: tags.collect(),
        }))
    }
}

impl FunctionCallNode {
    pub fn function(&self) -> Tag {
        self.function
    }
    pub fn arguments(&self) -> Vec<Tag> {
        self.arguments.iter().map(Clone::clone).collect()
    }
}

#[derive(Debug, Clone)]
pub enum FunctionNode {
    Named {
        name: Tag,
        parameters: Tag,
        body: Vec<Tag>,
    },
    Anonymous {
        parameters: Tag,
        body: Vec<Tag>,
    },
}

impl ToNode for FunctionNode {
    fn make_node(tags: Vec<Tag>) -> Result<Node, parser::ErrorKind> {
        let (_, name_or_args, args_or_first, mut body) = tags.take_3().ok_or(General("Failed"))?;
        let function = if name_or_args.is_symbol() {
            FunctionNode::Named {
                name: name_or_args,
                parameters: args_or_first,
                body,
            }
        } else {
            FunctionNode::Anonymous {
                parameters: name_or_args,
                body: {
                    body.insert(0, args_or_first);
                    body
                },
            }
        };
        Ok(Node::Function(function))
    }
}

impl FunctionNode {
    pub fn parameters(&self) -> Tag {
        match self {
            FunctionNode::Anonymous { parameters, .. } => *parameters,
            FunctionNode::Named { parameters, .. } => *parameters,
        }
    }

    pub fn body(&self) -> Vec<Tag> {
        match self {
            FunctionNode::Anonymous { body, .. } => body,
            FunctionNode::Named { body, .. } => body,
        }
        .iter()
        .map(Clone::clone)
        .collect()
    }

    pub fn name(&self) -> Option<Tag> {
        match self {
            FunctionNode::Anonymous { .. } => None,
            FunctionNode::Named { name, .. } => Some(*name),
        }
    }
}
