use crate::interpreter::value::{Symbol, Value, Var};
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
        pub fn name(&self) -> &str {
            self.name.name()
        }
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

        pub fn get(&self, name: &Symbol) -> Value {
            self.bindings
                .get(name)
                .cloned()
                .expect("Value does not exist")
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

mod scope {
    use crate::interpreter::value::{Symbol, Value};
    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Mutex;

    #[derive(Debug)]
    pub struct Scope {
        root: Link,
    }

    type Link = Option<Rc<Node>>;
    type Storage = HashMap<Symbol, Value>;

    #[derive(Debug)]
    struct Node {
        storage: Mutex<Storage>,
        parent: Link,
    }

    impl Scope {
        pub fn new() -> Scope {
            Scope {
                root: Some(Rc::new(Node {
                    storage: Mutex::new(Storage::new()),
                    parent: None,
                })),
            }
        }

        pub fn push(&mut self) {
            let new_scope = Rc::new(Node {
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

        pub fn define(&mut self, name: Symbol, value: Value) {
            let values = self
                .root
                .as_ref()
                .map(|root| &root.storage)
                .expect("No root scope found");
            let mut values = values.lock().expect("Could not lock values");
            values.insert(name, value);
        }

        pub fn resolve(&self, name: &Symbol) -> Option<Value> {
            for scope in self.iter() {
                let storage = scope.lock().expect("Could not lock storage");
                if let Some(value) = storage.get(name) {
                    return Some(value.clone());
                }
            }
            None
        }

        fn iter(&self) -> Iter<'_> {
            Iter {
                next: self.root.as_deref(),
            }
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
}

use namespace::Namespace;
use pointers::Pointers;
use scope::Scope;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Context {
    namespaces: Mutex<HashMap<Symbol, Namespace>>,
    pointers: Mutex<Pointers>,
    scope: Mutex<Scope>,
}

impl Context {
    pub fn dump(&self) {
        let nss = self.namespaces.lock().expect("Could not namespaces");
        let ptrs = self.pointers.lock().expect("Could not lock pointers");
        let scope = self.scope.lock().expect("could not lock scope");
        println!("{:#?}", nss);
        println!("{:#?}", ptrs);
        println!("{:#?}", scope);
    }

    fn using_namespace<F, Return>(&self, f: F) -> Return
    where
        F: Fn(&mut Namespace) -> Return,
    {
        let mut namespaces = self
            .namespaces
            .lock()
            .expect("Could not lock namespace map");
        let mut pointers = self.pointers.lock().expect("Could not lock pointers");
        let namespace = namespaces
            .get_mut(&pointers.namespace)
            .expect("Namespace does not exist");
        f(namespace)
    }

    pub fn new_namespace(&self, name: Symbol) {
        let mut namespaces = self
            .namespaces
            .lock()
            .expect("Could not lock namespace map");
        namespaces.insert(name.clone(), Namespace::new(name));
    }

    pub fn new() -> Context {
        let mut context = Context {
            namespaces: Mutex::new(HashMap::new()),
            pointers: Mutex::new(Pointers::new()),
            scope: Mutex::new(Scope::new()),
        };
        context
    }

    pub fn define(&self, name: Symbol, atom: Value) -> Var {
        self.using_namespace(|namespace| {
            let mut name = name.clone();
            namespace.bind(name.clone(), atom.clone());
            name.qualify(namespace.name());
            name.to_var()
        })
    }

    pub fn push_scope(&self) {
        let mut scope = self.scope.lock().expect("Failed to lock scope");
        scope.push();
    }

    pub fn pop_scope(&self) {
        let mut scope = self.scope.lock().expect("Failed to lock scope");
        scope.pop();
    }

    pub fn set(&self, name: Symbol, value: Value) {
        let mut scope = self.scope.lock().expect("Failed to lock scope");
        scope.define(name, value)
    }

    pub fn get(&self, name: &Symbol) -> Option<Value> {
        let scope = self.scope.lock().expect("Failed to lock scope");
        scope.resolve(name)
    }

    pub fn resolve(&self, name: &Symbol) -> Value {
        self.using_namespace(|namespace| namespace.get(name))
    }
}
