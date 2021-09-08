use crate::ast::node::Node;
use crate::runtime::{Execution, Runtime};
use crate::value::NValue;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IfNode {
    pub condition: Box<Node>,
    pub true_branch: Box<Node>,
    pub false_branch: Box<Node>,
}

impl fmt::Display for IfNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ? {} : {}",
            self.condition, self.true_branch, self.false_branch
        )
    }
}

impl Execution for IfNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        if runtime.execute(self.condition.as_ref()).is_truthy() {
            runtime.execute(self.true_branch.as_ref())
        } else {
            runtime.execute(self.false_branch.as_ref())
        }
    }
}
