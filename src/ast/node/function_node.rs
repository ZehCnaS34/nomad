use std::any::Any;
use std::fmt;

use crate::{
    ast,
    ast::{Tag, CHILD_LIMIT},
    copy, take_tags,
};

trait Show {
    fn show(self) -> Self
    where
        Self: Sized + fmt::Debug;
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub function: Tag,
    pub arguments: [Tag; CHILD_LIMIT.function_call],
}

impl FunctionCallNode {
    pub fn from_tags(tags: &[Tag]) -> FunctionCallNode {
        FunctionCallNode {
            function: tags[0],
            arguments: copy! { tags, 1, CHILD_LIMIT.function_call },
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunctionNode {
    Named {
        name: Tag,
        parameters: Tag,
        body: [Tag; CHILD_LIMIT.while_body],
    },
    Anonymous {
        parameters: Tag,
        body: [Tag; CHILD_LIMIT.while_body],
    },
}

impl FunctionNode {
    pub fn from_tags(tags: &[Tag]) -> FunctionNode {
        if tags[0].is_atom() {
            FunctionNode::Named {
                name: tags[0],
                parameters: tags[1],
                body: copy! { tags, 2, CHILD_LIMIT.while_body },
            }
        } else if tags[0].is_vector() {
            FunctionNode::Anonymous {
                parameters: tags[0],
                body: copy! { tags, 1, CHILD_LIMIT.while_body },
            }
        } else {
            panic!("Function definitions must start is either a identity or")
        }
    }
}
