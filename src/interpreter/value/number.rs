use super::super::{Compare, Introspection, Math};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number {
            value: value as f64,
        }
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Number {
            value: value as f64,
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number { value }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
