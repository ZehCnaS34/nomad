use crate::interpreter::value::{Symbol, Value};
use std::collections::HashMap;
use std::sync::Mutex;

mod namespace {
    use crate::interpreter::value::{Symbol, Value};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Namespace {
        name: Symbol,
        bindings: HashMap<Symbol, Value>,
    }

    impl Namespace {
        pub fn new(name: Symbol) -> Namespace {
            if name.is_qualified() {
                panic!("namespaces should never have a qualified name");
            }
            Namespace {
                name,
                bindings: HashMap::new(),
            }
        }

        pub fn bind(&mut self, name: Symbol, atom: Value) {
            self.bindings.insert(name, atom);
        }
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

        pub fn new() -> Pointers {
            Pointers {
                namespace: Symbol::from("nomad.core"),
            }
        }
    }
}

use namespace::Namespace;
use pointers::Pointers;

#[derive(Debug)]
pub struct Context {
    namespaces: Mutex<HashMap<Symbol, Namespace>>,
    pointers: Mutex<Pointers>,
}

impl Context {
    fn using_namespace<F>(&self, f: F)
    where F: Fn(&mut Namespace) {
        let mut namespaces = self.namespaces.lock().expect("Could not lock namespace map");
        let mut pointers = self.pointers.lock().expect("Could not lock pointers");
        println!("{:?} {:?}", namespaces, pointers);
        let namespace = namespaces.get_mut(&pointers.namespace).expect("Namespace does not exist");
        f(namespace);
    }

    pub fn new_namespace(&self, name: Symbol) {
        let mut namespaces = self.namespaces.lock().expect("Could not lock namespace map");
        namespaces.insert(name.clone(), Namespace::new(name));
    }

    pub fn new() -> Context {
        let mut context = Context {
            namespaces: Mutex::new(HashMap::new()),
            pointers: Mutex::new(Pointers::new()),
        };
        context
    }

    pub fn define(&self, name: Symbol, atom: Value) {
        self.using_namespace(|namespace| {
            namespace.bind(name.clone(), atom.clone());
        });
    }
}
