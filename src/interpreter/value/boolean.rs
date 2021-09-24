use super::super::operation::Introspection;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean { value }
    }
}

impl Boolean {
    fn as_int(&self) -> i32 {
        if self.value {
            1
        } else {
            0
        }
    }
}

impl Introspection for Boolean {
    fn truthy(&self) -> bool {
        self.value
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
