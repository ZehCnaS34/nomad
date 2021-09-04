use crate::result::{runtime_issue, Issue, NResult};
use std::str::FromStr;

pub type Symbol = String;

pub fn is_qualified(s: &Symbol) -> bool {
    s.contains('/')
}

pub fn split_symbol(s: &Symbol) -> NResult<(Symbol, Symbol)> {
    if let Some(index) = s.find(|c| c == '/') {
        let (namespace, name) = s.split_at(index);
        Ok((namespace.into(), name.into()))
    } else {
        runtime_issue("Invalid symbol")
    }
}

pub fn name(s: &Symbol) -> Symbol {
    split_symbol(s).map(|(_, n)| n).unwrap_or(s.clone())
}

pub fn namespace(s: &Symbol) -> Symbol {
    split_symbol(s).map(|(ns, _)| ns).unwrap_or(s.clone())
}

pub type Number = f32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseValueError {
    pub lexeme: String,
}

#[derive(Debug, Clone)]
enum LookupKind {
    Symbol,
    Keyword,
}

#[derive(Debug, Clone)]
struct Lookup {
    kind: LookupKind,
    namespace: Symbol,
    name: Symbol,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(Number),
    Symbol(Symbol),
    Lookup(Lookup),
    Keyword(Symbol),
}

impl FromStr for Value {
    type Err = ParseValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            Ok(Value::String(s.get(1..s.len() - 1).unwrap().to_string()))
        } else if s.starts_with(":") {
            let s = String::from(s.strip_prefix(":").unwrap());
            if let Some(index) = s.find('/') {
                let (namespace, name) = s.split_at(index);
                Ok(Value::Lookup(Lookup {
                    kind: LookupKind::Keyword,
                    name: name[1..].into(),
                    namespace: namespace.into(),
                }))
            } else {
                Ok(Value::Keyword(String::from(s)))
            }
        } else if let Ok(value) = s.parse() {
            Ok(Value::Number(value))
        } else {
            let s = String::from(s);
            if let Some(index) = s.find('/') {
                let (namespace, name) = s.split_at(index);
                Ok(Value::Lookup(Lookup {
                    kind: LookupKind::Symbol,
                    name: name[1..].into(),
                    namespace: namespace.into(),
                }))
            } else {
                Ok(Value::Symbol(String::from(s)))
            }
        }
    }
}

impl Value {
    pub fn is_symbol(&self) -> bool {
        if let Value::Symbol(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Value),
    Def(String, Option<Box<Expr>>),
    Application(Box<Expr>, Vec<Expr>),
    List(Vec<Expr>),
    HashSet(Vec<Expr>),
    Vector(Vec<Expr>),
    Program(Vec<Expr>),
}
