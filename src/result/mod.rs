use crate::prelude::*;

pub mod runtime {
    #[derive(Debug)]
    pub enum ErrorKind {
        BindingNotFound,
        CouldNotParseAtom,
        ExpectedClosingParen,
        IfMissingCondition,
        IfMissingTrueBranch,
        InvalidArgumentArity,
        InvalidDefForm,
        InvalidNamespace,
        InvalidNode,
        InvalidOperation,
        MissingNode,
        NodeNotFound,
        NotDefined,
        StorageIssue,
        TagNodeMissMatch,
        UnexpectedEof,
        General(&'static str),
    }
}

pub trait MakeError {
    type Item;
    fn info(message: &'static str) -> Self::Item;
}

pub type RuntimeResult<T> = std::result::Result<T, runtime::ErrorKind>;
impl<T> MakeError for RuntimeResult<T> {
    type Item = RuntimeResult<T>;
    fn info(message: &'static str) -> Self::Item {
        Err(runtime::ErrorKind::General(message))
    }
}

use std::io;

impl From<io::ErrorKind> for ErrorKind {
    fn from(_: io::ErrorKind) -> Self {
        General("io operation failure")
    }
}
