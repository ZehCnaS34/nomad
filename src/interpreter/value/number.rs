use std::fmt;
use std::fmt::Formatter;
use super::operation::Operation;

#[derive(Debug, Clone)]
pub struct Number {
    value: f64,
}

impl Operation for Number {
    fn is_truthy(&self) -> bool {
        return true;
    }

    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Number(v as f64)
    }
}

impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number(v)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
