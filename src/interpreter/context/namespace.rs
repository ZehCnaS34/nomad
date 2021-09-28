use crate::interpreter::value::{Symbol, Value};
use crate::result::runtime::ErrorKind as Error;
use crate::result::RuntimeResult as Result;

use prettytable::{Cell, Row, Table};
use shared_map::SharedMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

mod shared_map {
    use std::cmp::Eq;
    use std::collections::hash_map::Iter;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::PoisonError;

    use crate::result::runtime::ErrorKind as Error;
    use crate::result::RuntimeResult as Result;

    #[derive(Debug)]
    pub struct SharedMap<Key: Eq + Hash, Value>(Arc<Mutex<HashMap<Key, Value>>>);

    impl<Key: Eq + Hash, Value> Default for SharedMap<Key, Value> {
        fn default() -> SharedMap<Key, Value> {
            SharedMap(Arc::new(Mutex::new(HashMap::new())))
        }
    }

    impl<Key: Eq + Hash, Value> SharedMap<Key, Value> {
        pub fn insert(&self, key: Key, value: Value) -> Result<()> {
            let root = self.0.clone();
            let mut root = root.lock()?;
            root.insert(key, value);
            Ok(())
        }

        pub fn get(&self, key: &Key) -> Result<Value>
        where
            Value: Clone,
        {
            let root = self.0.clone();
            let mut root = root.lock()?;
            match root.get(&key) {
                Some(value) => Ok(value.clone()),
                None => Err(Error::NotDefined),
            }
        }
    }
}

#[derive(Debug)]
pub struct Namespace {
    name: Symbol,
    aliases: SharedMap<Symbol, Namespace>,
    bindings: SharedMap<Symbol, Value>,
}

impl Namespace {
    pub fn name(&self) -> &str {
        self.name.name()
    }

    pub fn new(name: Symbol) -> Namespace {
        Namespace {
            name,
            aliases: SharedMap::default(),
            bindings: SharedMap::default(),
        }
    }

    pub fn define(&self, key: Symbol, value: Value) -> Result<()> {
        self.bindings.insert(key, value)
    }

    pub fn resolve(&self, key: Symbol) -> Result<Value> {
        self.bindings.get(&key)
    }

    pub fn dump(&self) {
        todo!()
        // let mut table = Table::new();
        // table.add_row(row![format!("[namespace::{}]", self.name), ""]);
        // table.add_row(row!["key", "value"]);
        // for (key, val) in self.bindings.iter() {
        //     table.add_row(row![key.to_string(), val.to_string()]);
        // }
        // table.printstd();
    }
}
