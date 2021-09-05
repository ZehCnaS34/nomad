use crate::result::{runtime_issue, Issue, NResult};
use std::fmt;
use std::str::FromStr;

pub type Symbol = String;

pub trait SymbolIntrospection {
    type Item;
    fn is_qualified(&self) -> bool;
    fn split_symbol(&self) -> NResult<(Self::Item, Self::Item)>;
    fn name(&self) -> Self::Item;
    fn namespace(&self) -> Option<Self::Item>;
}

impl SymbolIntrospection for Symbol {
    type Item = Symbol;

    fn is_qualified(&self) -> bool {
        self.contains('/')
    }
    fn split_symbol(&self) -> NResult<(Symbol, Symbol)> {
        if let Some(index) = self.find(|c| c == '/') {
            let (namespace, name) = self.split_at(index + 1);
            Ok((namespace.into(), name.into()))
        } else {
            runtime_issue("Invalid symbol")
        }
    }

    fn name(&self) -> Symbol {
        self.split_symbol().map(|(_, n)| n).unwrap_or(self.clone())
    }
    fn namespace(&self) -> Option<Symbol> {
        self.split_symbol().map(|(ns, _)| ns).ok()
    }
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

impl fmt::Display for Lookup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.namespace, self.name)
    }
}

struct Function {}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(Number),
    Symbol(Symbol),
    Lookup(Lookup),
    Keyword(Symbol),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;
        match self {
            String(value) => write!(f, "{}", value),
            Number(value) => write!(f, "{}", value),
            Symbol(value) => write!(f, "{}", value),
            Keyword(value) => write!(f, "{}", value),
            Lookup(value) => write!(f, "{}", value),
            Boolean(value) => write!(f, "{}", value),
            Nil => write!(f, "nil"),
        }
    }
}

impl FromStr for Value {
    type Err = ParseValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "true" {
            Ok(Value::Boolean(true))
        } else if s == "false" {
            Ok(Value::Boolean(false))
        } else if s.starts_with('"') {
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
    pub fn get_symbol(self) -> NResult<Symbol> {
        if let Value::Symbol(symbol) = self {
            Ok(symbol)
        } else {
            runtime_issue("Failed to resolve to symbol")
        }
    }
    pub fn get_number(self) -> NResult<Number> {
        if let Value::Number(number) = self {
            Ok(number)
        } else {
            runtime_issue("Failed to resolve to number")
        }
    }
    pub fn is_symbol(&self) -> bool {
        if let Value::Symbol(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(value) => *value,
            Value::Nil => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Value),
    Invoke(Box<Expr>, Vec<Expr>),
    List(Vec<Expr>),
    HashSet(Vec<Expr>),
    Vector(Vec<Expr>),
    Program(Vec<Expr>),
}

pub fn print_ast(expr: &Expr) {
    fn inner(expr: &Expr, offset: i32) {
        use Expr::*;
        match expr {
            Program(expressions) => {
                for expression in expressions {
                    inner(expression, offset + 1);
                }
            }
            Invoke(f, args) => {
                inner(f, offset);
                for arg in args {
                    inner(arg, offset + 1);
                }
            }
            Atom(value) => {
                for i in 0..offset {
                    print!("\t");
                }
                println!("{}", value);
            }
            _ => todo!(),
        }
    }
    inner(expr, -1);
}

impl Expr {
    pub fn get_atom(self) -> NResult<Value> {
        if let Expr::Atom(value) = self {
            Ok(value)
        } else {
            runtime_issue("Failed to resolve to atom")
        }
    }
}
