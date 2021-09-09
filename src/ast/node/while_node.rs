use crate::ast::node::Node;
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
