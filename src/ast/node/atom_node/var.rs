#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Var {
    literal: String,
}

impl Var {
    pub fn make((ns, n): (&str, &str)) -> Var {
        Var {
            literal: format!("{}/{}", ns, n),
        }
    }
}
