#[derive(Debug, Clone)]
pub struct SymbolNode {
    name: String,
    namespace: Option<String>,
}

impl SymbolNode {
    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(|namespace| &namespace[..])
    }

    pub fn is_qualified(&self) -> bool {
        self.namespace.is_some()
    }

    pub fn from(value: &str) -> SymbolNode {
        if value.len() == 1 {
            SymbolNode {
                name: String::from(value),
                namespace: None,
            }
        } else if let Some(index) = value.find('/') {
            SymbolNode {
                name: String::from(&value[index + 1..]),
                namespace: Some(String::from(&value[..index])),
            }
        } else {
            SymbolNode {
                name: String::from(value),
                namespace: None,
            }
        }
    }
}
