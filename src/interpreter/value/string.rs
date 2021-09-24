use super::super::Concat;
use super::super::Introspection;
use super::super::Length;
use std::fmt;
use std::fmt::Formatter;

type Str = std::string::String;

#[derive(Debug, Clone)]
pub struct String {
    pub value: std::string::String,
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
