use super::debug::Dump;
use crate::interpreter::value::{Symbol, Value};
use crate::prelude::*;
use crate::result::runtime::ErrorKind as Error;

use prettytable::{Cell, Row, Table};

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Scope {
    root: Link,
}

type Link = Option<Arc<Node>>;
type Storage = HashMap<Symbol, Value>;

#[derive(Debug)]
struct Node {
    storage: Mutex<Storage>,
    parent: Link,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            root: Some(Arc::new(Node {
                storage: Mutex::new(Storage::new()),
                parent: None,
            })),
        }
    }

    pub fn len(&self) -> usize {
        let mut output = 0;
        for scope in self.iter() {
            output += 1;
        }
        output
    }

    pub fn push(&mut self) {
        let new_scope = Arc::new(Node {
            storage: Mutex::new(Storage::new()),
            parent: self.root.take(),
        });

        self.root = Some(new_scope);
    }

    pub fn pop(&mut self) {
        self.root.take().map(|node| {
            self.root = node.parent.clone();
        });
    }

    pub fn define<S, V>(&mut self, name: S, value: V) -> Result<()>
    where
        S: Into<Symbol>,
        V: Into<Value>,
    {
        let name = name.into();
        let value = value.into();
        let values = self
            .root
            .as_ref()
            .map(|root| &root.storage)
            .ok_or(Error::MissingNode)?;
        let mut values = values.lock()?;
        values.insert(name, value);
        Ok(())
    }

    pub fn resolve(&self, name: &Symbol) -> Result<Value> {
        for scope in self.iter() {
            let storage = scope.lock()?;
            if let Some(value) = storage.get(name) {
                return Ok(value.clone());
            }
        }
        Err(Error::BindingNotFound)
    }

    fn iter(&self) -> Iter<'_> {
        Iter {
            next: self.root.as_deref(),
        }
    }
}

impl Dump for Scope {
    fn dump(&self) -> Result<()> {
        for (i, storage) in self.iter().enumerate() {
            let storage = storage.lock()?;
            let mut table = Table::new();
            table.add_row(row![format!("[SCOPE::{}]", i), ""]);
            table.add_row(row!["key", "value"]);
            for (key, val) in storage.iter() {
                table.add_row(row![key.to_string(), val.to_string()]);
            }
            table.printstd();
        }
        Ok(())
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Mutex<Storage>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.parent.as_deref();
            &node.storage
        })
    }
}
