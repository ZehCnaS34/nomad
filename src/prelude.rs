pub use std::collections::HashMap;
pub use std::fmt;
pub use std::sync::Arc;
pub use std::sync::LockResult;
pub use std::sync::Mutex;
pub use std::sync::MutexGuard;

pub use crate::ast::parser::parse;
pub use crate::ast::scanner::token::Kind as TokenKind;
pub use crate::ast::scanner::token::Token;
pub use crate::ast::scanner::Scanner;
pub use crate::ast::Node;
pub use crate::ast::Tag;
pub use crate::interpreter::Interpreter;
pub use crate::interpreter::Value;
pub use crate::result::runtime::ErrorKind;
pub use crate::result::runtime::ErrorKind::*;
pub use crate::result::*;

pub use std::convert::TryFrom;

pub mod node {
    pub use crate::ast::node::*;
}

#[macro_export]
macro_rules! defnode {
    ($enum:path : $struct:ident :: $var:ident => $body:expr ) => {
        impl TryFrom<Vec<Node>> for $struct {
            type Error = ErrorKind;

            fn try_from($var: Vec<Node>) -> Result<$struct> {
                $body
            }
        }
        impl From<$struct> for Node {
            fn from($var: $struct) -> Node {
                $enum($var)
            }
        }
        impl ToNode for $struct {
            fn make_node($var: Vec<Node>) -> Result<Node> {
                let node = $struct::try_from($var)?;
                Ok(node.into())
            }
        }
    };
}
