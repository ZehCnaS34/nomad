use super::Node;
use crate::runtime::{Execution, Runtime};
use crate::value::NValue;

#[derive(Debug, Clone)]
pub struct ListNode {
    items: Vec<Node>,
}

impl Execution for ListNode {
    fn execute(&self, runtime: &mut Runtime) -> NValue {
        println!("running {:?}", runtime);
        NValue::Nil
    }
}
