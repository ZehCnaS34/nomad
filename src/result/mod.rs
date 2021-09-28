pub mod scanner {
    #[derive(Debug)]
    pub enum ErrorKind {
        General(&'static str),
    }
}

pub mod runtime {
    #[derive(Debug)]
    pub enum ErrorKind {
        NotDefined,
        InvalidOperation,
        NodeNotFound,
        InvalidNode,
        InvalidArgumentArity,
        BindingNotFound,
        TagNodeMissMatch,
        MissingNode,
        General(&'static str),
    }
}

pub mod parser {
    #[derive(Debug)]
    pub enum ErrorKind {
        UnexpectedEof,
        ExpectedClosingParen,
        CouldNotParseAtom,
        IfMissingCondition,
        IfMissingTrueBranch,
        InvalidDefForm,
        InvalidOperation,
        General(&'static str),
    }
}

pub trait MakeError {
    type Item;
    fn info(message: &'static str) -> Self::Item;
}

pub type ParseResult<T> = std::result::Result<T, parser::ErrorKind>;
impl<T> MakeError for ParseResult<T> {
    type Item = ParseResult<T>;
    fn info(message: &'static str) -> Self::Item {
        Err(parser::ErrorKind::General(message))
    }
}
pub type RuntimeResult<T> = std::result::Result<T, runtime::ErrorKind>;
impl<T> MakeError for RuntimeResult<T> {
    type Item = RuntimeResult<T>;
    fn info(message: &'static str) -> Self::Item {
        Err(runtime::ErrorKind::General(message))
    }
}
pub type ScannerResult<T> = std::result::Result<T, scanner::ErrorKind>;
impl<T> MakeError for ScannerResult<T> {
    type Item = ScannerResult<T>;
    fn info(message: &'static str) -> Self::Item {
        Err(scanner::ErrorKind::General(message))
    }
}
