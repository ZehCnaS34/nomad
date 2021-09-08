use crate::ast::node::Node;
use crate::runtime::{Execution, Runtime};
use crate::value::NValue;
use std::fmt;

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition: Box<Node>,
    pub body: Vec<Node>,
}

impl fmt::Display for WhileNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @", self.condition,)?;
        for expression in &self.body {
            write!(f, "{}", expression)?;
        }
        Ok(())
    }
}

impl Execution for WhileNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        let condition = self.condition.as_ref();
        let body = &self.body;
        while runtime.execute(condition).is_truthy() {
            for node in body {
                runtime.execute(node);
            }
        }
        NValue::Nil
    }
}
