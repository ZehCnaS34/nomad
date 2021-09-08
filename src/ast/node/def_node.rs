use crate::ast::node::atom_node::Symbol;
use crate::ast::node::Node;
use crate::runtime::{Execution, Runtime};
use crate::value::NValue;
use std::fmt;
use std::vec::IntoIter;

pub enum Error {
    DefMissingIdent,
    DefInvalidSymbol,
    DefMissingValue,
}

pub type Result = std::result::Result<DefinitionNode, Error>;

#[derive(Debug, Clone)]
pub struct DefinitionNode {
    pub var: Symbol,
    pub value: Box<Node>,
}

impl DefinitionNode {
    pub fn from_into_iter(mut nodes: IntoIter<Node>) -> Result {
        let var = nodes.next().ok_or(Error::DefMissingIdent)?;
        let var = var.unwrap_symbol().ok_or(Error::DefInvalidSymbol)?;
        if var.is_qualified() {
            return Err(Error::DefInvalidSymbol);
        }
        let value = nodes.next().ok_or(Error::DefMissingValue)?;
        Ok(DefinitionNode {
            var,
            value: Box::new(value),
        })
    }
}

impl fmt::Display for DefinitionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.var, self.value)
    }
}

impl Execution for DefinitionNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        let value = runtime.execute(self.value.as_ref());
        runtime.define(self.var.clone(), value);
        NValue::Nil
    }
}
