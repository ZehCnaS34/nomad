use crate::ast::node::{Symbol, ToSymbol};
use std::collections::HashMap;
use std::sync::Mutex;

mod namespace {
    use crate::ast::node::{AtomNode, Symbol, ToSymbol};
    use std::collections::HashMap;

    pub struct Namespace {
        name: Symbol,
        bindings: HashMap<Symbol, AtomNode>,
    }

    impl Namespace {
        pub fn new<S>(name: S) -> Namespace
        where
            S: ToSymbol,
        {
            Namespace {
                name: name.to_symbol(),
                bindings: HashMap::new(),
            }
        }

        pub fn bind(&mut self, name: Symbol, atom: AtomNode) {
            self.bindings.insert(name, atom);
        }

    }
}

mod pointers {
    use crate::ast::node::{Symbol, ToSymbol};

    pub struct Pointers {
        namespace: Symbol,
    }

    impl Pointers {
        pub fn set_namespace<S: ToSymbol>(&mut self, symbol: S)  {
            let namespace_name = symbol.to_symbol();
            if namespace_name.is_qualified() {
                panic!("Namespaces should not be qualified.");
            }
            self.namespace = namespace_name;
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

pub struct Context {
    namespaces: Mutex<HashMap<Symbol, Namespace>>,
    pointers: Mutex<Pointers>,
}

impl Context {
    pub fn new() -> Context {
        let mut context = Context {
            namespaces: Mutex::new(HashMap::new()),
            pointers: Mutex::new(Pointers::new()),
        };

        let plus: Symbol = Symbol::from("nomad.core/+");
        let minus: Symbol = Symbol::from("nomad.core/-");
        let div: Symbol = Symbol::from("nomad.core//");
        let mult: Symbol = Symbol::from("nomad.core/*");

        context
    }
}
