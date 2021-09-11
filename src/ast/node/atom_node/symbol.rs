use std::fmt;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Symbol {
    literal: String,
}

pub trait ToSymbol {
    fn to_symbol(&self) -> Symbol;
}

impl ToSymbol for Symbol {
    fn to_symbol(&self) -> Symbol {
        self.clone()
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Symbol {
        Symbol {
            literal: String::from(value),
        }
    }
}

impl Symbol {
    pub fn name(&self) -> &str {
        if self.literal.len() <= 1 {
            &self.literal[..]
        } else if let Some(index) = &self.literal[..].find('/') {
            &self.literal[index + 1..]
        } else {
            &self.literal[..]
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        if self.literal.len() <= 1 {
            None
        } else if let Some(index) = &self.literal[..].find('/') {
            Some(&self.literal[0..index + 0])
        } else {
            None
        }
    }

    pub fn get_namespace(&self) -> Option<Symbol> {
        self.namespace().map(|namespace| namespace.into())
    }

    pub fn is_qualified(&self) -> bool {
        self.literal.len() > 1 && self.literal[..].contains('/')
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
