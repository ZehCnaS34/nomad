use super::value::*;
use crate::result::runtime::ErrorKind;

pub trait Introspection {
    fn truthy(&self) -> bool;
    fn falsy(&self) -> bool {
        !self.truthy()
    }
}

pub trait Compare {
    fn eq(&self, other: &Self) -> bool;
    fn neq(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    fn lt(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;
}

pub trait Lookup {
    type Item;
    type Key;
    type Err;
    fn lookup(&self, key: Self::Key) -> Result<&Self::Item, Self::Err>;
}

pub trait Math {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn modulus(&self, other: &Self) -> Self;
}

pub trait Concat {
    fn concat(&self, other: &Self) -> Self;
}

pub trait Length {
    fn length(&self) -> usize;
}

type Str = std::string::String;
type STR = &'static str;

impl Introspection for Value {
    fn truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(boolean) => boolean.truthy(),
            _ => true,
        }
    }
}

impl Introspection for Number {
    fn truthy(&self) -> bool {
        return true;
    }
}

impl Math for Number {
    fn add(&self, other: &Self) -> Self {
        Number {
            value: self.value + other.value,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Number {
            value: self.value - other.value,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Number {
            value: self.value * other.value,
        }
    }

    fn div(&self, other: &Self) -> Self {
        Number {
            value: self.value / other.value,
        }
    }

    fn modulus(&self, other: &Self) -> Self {
        Number {
            value: self.value % other.value,
        }
    }
}

impl Compare for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }
}

impl Concat for String {
    fn concat(&self, other: &Self) -> Self {
        let mut value = Str::new();
        value.push_str(&self.value[..]);
        value.push_str(&other.value[..]);
        String { value }
    }
}

impl Introspection for String {
    fn truthy(&self) -> bool {
        return true;
    }
}

impl Length for String {
    fn length(&self) -> usize {
        self.value.len()
    }
}

impl Lookup for Vector<Value> {
    type Item = Value;
    type Key = Number;
    type Err = ErrorKind;

    fn lookup(&self, key: Self::Key) -> Result<&Self::Item, Self::Err> {
        let key = key.value as usize;
        self.get(key).ok_or(ErrorKind::BindingNotFound)
    }
}