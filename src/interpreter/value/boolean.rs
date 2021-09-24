use std::fmt;
use std::fmt::Formatter;
use super::operation::Operation;

#[derive(Debug, Clone)]
pub struct Boolean {
    value: bool,
}

impl Operation for Boolean {
    fn is_truthy(&self) -> bool {
        self.value
    }

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
