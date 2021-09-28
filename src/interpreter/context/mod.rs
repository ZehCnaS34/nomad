use crate::interpreter::value::{Symbol, Value, Var};
use crate::result::runtime::ErrorKind as Error;
use crate::result::RuntimeResult;
use crate::result::RuntimeResult as Result;

use std::collections::HashMap;
use std::error::Error as E;
use std::sync::Arc;
use std::sync::LockResult;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::PoisonError;

mod debug;
mod namespace;
mod scope;

pub use debug::Dump;
use namespace::Namespace;
use pointers::Pointers;
use scope::Scope;

impl<Guard> From<PoisonError<Guard>> for Error {
    fn from(error: PoisonError<Guard>) -> Self {
        Error::StorageIssue
    }
}

mod pointers {
    use crate::interpreter::value::{Symbol, Value};

    #[derive(Debug)]
    pub struct Pointers {
        pub namespace: Symbol,
    }

    impl Pointers {
        pub fn set_namespace(&mut self, symbol: Symbol) {
            if symbol.is_qualified() {
                panic!("Namespaces should not be qualified.");
            }
            self.namespace = symbol;
        }

        pub fn dump(&self) {}

        pub fn new() -> Pointers {
            Pointers {
                namespace: Symbol::from("nomad.core"),
            }
        }
    }
}

#[derive(Debug)]
pub struct Context {
    namespaces: Mutex<HashMap<Symbol, Arc<Namespace>>>,
    pointers: Mutex<Pointers>,
    scope: Mutex<Scope>,
}

impl Context {
    fn current_namespace(&self) -> RuntimeResult<Arc<Namespace>> {
        let mut namespaces = self.namespaces.lock()?;
        let mut pointers = self.pointers.lock()?;
        let namespace = namespaces
            .get(&pointers.namespace)
            .ok_or(Error::InvalidNamespace)?;
        Ok(namespace.clone())
    }

    pub fn new_namespace(&self, name: Symbol) -> RuntimeResult<()> {
        let mut namespaces = self.namespaces.lock()?;
        namespaces.insert(name.clone(), Arc::new(Namespace::new(name)));
        Ok(())
    }

    pub fn new() -> Context {
        let mut context = Context {
            namespaces: Mutex::new(HashMap::new()),
            pointers: Mutex::new(Pointers::new()),
            scope: Mutex::new(Scope::new()),
        };
        context
    }

    pub fn define<S, V>(&self, symbol: S, value: V) -> RuntimeResult<Var>
    where
        S: Into<Symbol>,
        V: Into<Value>,
    {
        let symbol = symbol.into();
        let value = value.into();
        let namespace = self.current_namespace()?;
        let mut name = symbol.clone();
        namespace.define(name.clone(), value.clone())?;
        name.qualify(namespace.name());
        Ok(Var {
            name: name.name().to_string(),
            namespace: namespace.name().to_string(),
        })
    }

    pub fn push_scope(&self) -> Result<()> {
        let mut scope = self.scope.lock()?;
        scope.push();
        Ok(())
    }

    pub fn scope_depth(&self) -> Result<usize> {
        let mut scope = self.scope.lock()?;
        Ok(scope.len())
    }

    pub fn pop_scope(&self) -> Result<()> {
        let mut scope = self.scope.lock()?;
        scope.pop();
        Ok(())
    }

    pub fn set(&self, name: Symbol, value: Value) -> Result<()> {
        let mut scope = self.scope.lock()?;
        scope.define(name, value);
        Ok(())
    }

    pub fn get(&self, name: &Symbol) -> Result<Value> {
        let scope = self.scope.lock()?;
        scope.resolve(name)
    }

    pub fn resolve(&self, name: &Symbol) -> Result<Value> {
        let namespace = self.current_namespace()?;
        namespace.resolve(name.clone())
    }
}

impl Dump for Context {
    fn dump(&self) -> Result<()> {
        println!("dumping context");
        Ok(())
    }
}
