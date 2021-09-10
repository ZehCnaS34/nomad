use crate::ast::node::atom_node::AtomNode;
use crate::ast::node::atom_node::Symbol;
use crate::ast::node::Node;
use crate::ast::parser::Tag;
use crate::ast::CHILD_LIMIT;
use crate::copy;
use crate::interpreter::{Execute, Interpreter};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub function: Tag,
    pub arguments: [Tag; CHILD_LIMIT.function_call],
}

impl FunctionCallNode {
    pub fn from_tags(tags: &[Tag]) -> FunctionCallNode {
        FunctionCallNode {
            function: tags[0],
            arguments: copy! { tags, 1, CHILD_LIMIT.function_call },
        }
    }
}

macro_rules! short_circuit {
    ( $arguments:expr, $fn:expr, $inst:ident ) => {{
        use crate::interpreter::Interpreter;
        let mut flag = true;
        let mut last_tag: Option<Tag> = None;
        for current_tag in Tag::tags(&$arguments) {
            if let Some(last) = last_tag {
                if $fn(&$inst, last, current_tag) {
                    last_tag = Some(current_tag);
                } else {
                    flag = false;
                    break;
                }
            } else {
                last_tag = Some(current_tag);
            }
        }
        flag
    }};
}

macro_rules! reduction {
    ( $arguments:expr, $fn:expr, $default:expr, $inst:ident ) => {{
        use crate::interpreter::Interpreter;
        let mut last_tag: Option<Tag> = None;
        let mut reduction = Some($default);
        for current_tag in Tag::tags(&$arguments) {
            if let Some(r) = last_tag {
                reduction = Some($fn(&$inst, r, current_tag));
            } else {
                last_tag = Some(current_tag);
            }
        }
        reduction
    }};
}

impl Execute for FunctionCallNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        let function = interpreter.intern_tag(self.function);
        match function {
            AtomNode::Symbol(symbol) => {
                if !symbol.is_qualified() {
                    println!("running {:?}", symbol.name());
                    match symbol.name() {
                        "<" => {
                            let flag =
                                short_circuit! { self.arguments, Interpreter::lt, interpreter };
                            interpreter.set_tag_data(own_tag, AtomNode::Boolean(flag));
                        }
                        ">" => {
                            let flag =
                                short_circuit! { self.arguments, Interpreter::gt, interpreter };
                            interpreter.set_tag_data(own_tag, AtomNode::Boolean(flag));
                        }
                        "+" => {
                            let total = reduction! { self.arguments, Interpreter::add, AtomNode::Nil, interpreter };
                        }
                        "-" => {}
                        "=" => {}
                        "println" => {}
                        function => panic!("fuck {:?}", function),
                    }
                }
            }
            _ => panic!("fuck"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunctionNode {
    Named {
        name: Tag,
        parameters: [Tag; CHILD_LIMIT.function_call],
        body: [Tag; CHILD_LIMIT.while_body],
    },
    Anonymous {
        parameters: [Tag; CHILD_LIMIT.function_call],
        body: [Tag; CHILD_LIMIT.while_body],
    },
}

impl Execute for FunctionNode {
    fn execute(&self, interpreter: &Interpreter, own_tag: Tag) {
        todo!();
    }
}
