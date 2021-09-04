use std::cell::RefCell;
use crate::view::Cursor;
use crate::token::TokenType;

#[derive(Debug, PartialEq)]
pub enum IssueType {
    Ok,
    ScanError,
    ParseError,
    AnalyzeError,
    RuntimeError,
}

#[derive(Debug)]
pub struct Issue {
    pub issue: IssueType,
    pub position: Option<Cursor>,
    pub context: String,
}

impl Issue {
    pub fn parse_error<T: Into<String>>(context: T, cursor: &RefCell<Cursor>) -> Issue {
        let position = cursor.borrow();
        Issue {
            issue: IssueType::ParseError,
            position: Some(position.clone()),
            context: context.into(),
        }
    }
}

pub fn runtime_issue<T>(context: &'static str) -> NResult<T> {
    Err(Issue {
        issue: IssueType::RuntimeError,
        position: None,
        context: context.into(),
    })
}

pub type NResult<Type> = Result<Type, Issue>;

pub trait Skimmer {
    type Item;
    fn skim_token(&self, token_type: TokenType) -> NResult<Self::Item>;
    fn skim_issue(&self, token_type: IssueType) -> NResult<Self::Item>;
}

