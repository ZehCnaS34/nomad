use std::fmt;

///! Symbols are the
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Symbol {
    Qualified { name: String, namespace: String },
    UnQualified { name: String },
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Symbol::*;
        match self {
            Qualified {name, namespace} => write!(f, "{}/{}", name, namespace),
            UnQualified { name } => write!(f, "{}", name),
        }
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Symbol {
        if value.len() == 1 {
            Symbol::UnQualified { name: String::from(value) }
        } else if let Some(index) = value.find('/') {
            Symbol::Qualified {
                name: String::from(&value[index+1..]),
                namespace: String::from(&value[..index]),
            }
        } else {
            Symbol::UnQualified { name: String::from(value) }
        }
    }
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Symbol::UnQualified { name } => &name[..],
            Symbol::Qualified { name, namespace: _ } => &name[..],
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        match self {
            Symbol::UnQualified { name } => None,
            Symbol::Qualified { name: _, namespace } => Some(&namespace[..]),
        }
    }

    pub fn get_namespace(&self) -> Option<Symbol> {
        self.namespace().map(|namespace| namespace.into())
    }

    pub fn is_qualified(&self) -> bool {
        match self {
            Symbol::UnQualified { .. } => false,
            Symbol::Qualified { .. } => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_inspect_symbol_name() {
        let sym: Symbol = "awesome".into();
        assert_eq!(sym.name(), "awesome");
        let sym: Symbol = "nomad.core/add".into();
        assert_eq!(sym.name(), "add");
    }

    #[test]
    fn properly_inspect_symbol_namespace() {
        let sym: Symbol = "nomad.core/add".into();
        assert_eq!(sym.namespace(), Some("nomad.core"));
    }

    #[test]
    fn inspect_divide_symbol() {
        let sym: Symbol = "/".into();
        assert_eq!(sym.name(), "/");
    }

    #[test]
    fn meta_information() {
        let sym: Symbol = "nomad.core/information".into();
        assert_eq!(sym.is_qualified(), true);
    }

    #[test]
    fn create_namespace_symbol_from_symbol() {
        let full_sym: Symbol = "nomad.core/information".into();
        let ns_sym = full_sym.get_namespace().unwrap();
        assert_eq!(ns_sym.name(), "nomad.core");
    }
}
