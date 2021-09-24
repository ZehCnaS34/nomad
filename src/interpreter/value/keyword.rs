use crate::interpreter::Value;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Keyword {
    pub name: String,
    pub namespace: Option<String>,
}
