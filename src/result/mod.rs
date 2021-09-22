pub mod scanner {
    #[derive(Debug)]
    pub enum ErrorKind {}
}

pub mod runtime {
    #[derive(Debug)]
    pub enum ErrorKind {
        NotDefined,
        InvalidOperation,
        NodeNotFound,
        InvalidNode,
        InvalidArgumentArrity,
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
    }
}

pub type ParseResult<T> = std::result::Result<T, parser::ErrorKind>;
pub type RuntimeResult<T> = std::result::Result<T, runtime::ErrorKind>;
pub type ScannerResult<T> = std::result::Result<T, scanner::ErrorKind>;
