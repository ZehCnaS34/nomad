use super::ast::parser::AST;
use crate::ast::node::atom_node::{AtomNode, Symbol, Var};
use crate::ast::parser::NodeKind::Vector;
use crate::ast::parser::{Node, Tag};
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::sync::Mutex;

trait Introspection {
    fn is_truthy(&self) -> bool;
}

impl Introspection for AtomNode {
    fn is_truthy(&self) -> bool {
        match self {
            AtomNode::Boolean(b) => *b,
            _ => false,
        }
    }
}

trait Operation {
    fn add(&self, lhs: &Self) -> Self;
    fn sub(&self, lhs: &Self) -> Self;
    fn mul(&self, lhs: &Self) -> Self;
    fn div(&self, lhs: &Self) -> Self;
    fn eq(&self, lhs: &Self) -> Self;
    fn lt(&self, lhs: &Self) -> Self;
    fn gt(&self, lhs: &Self) -> Self;
}

impl Operation for AtomNode {
    fn add(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Integer(a + b),
            (Rational(a), Rational(b)) => Rational(a + b),
            (Str(a), Str(b)) => Str(format!("{}{}", a, b)),
            _ => panic!("fuck"),
        }
    }

    fn sub(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Integer(a - b),
            (Rational(a), Rational(b)) => Rational(a - b),
            _ => panic!("fuck"),
        }
    }

    fn mul(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Integer(a * b),
            (Rational(a), Rational(b)) => Rational(a * b),
            _ => panic!("fuck"),
        }
    }

    fn div(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Integer(a / b),
            (Rational(a), Rational(b)) => Rational(a / b),
            _ => panic!("fuck"),
        }
    }

    fn eq(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational, Boolean};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Boolean(a == b),
            (Rational(a), Rational(b)) => Boolean(a == b),
            _ => panic!("fuck"),
        }
    }

    fn gt(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational, Boolean};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Boolean(a > b),
            (Rational(a), Rational(b)) => Boolean(a > b),
            _ => panic!("fuck"),
        }
    }

    fn lt(&self, lhs: &Self) -> Self {
        use AtomNode::String as Str;
        use AtomNode::{Integer, Rational, Boolean};
        match (self, lhs) {
            (Integer(a), Integer(b)) => Boolean(a < b),
            (Rational(a), Rational(b)) => Boolean(a < b),
            _ => panic!("fuck"),
        }
    }
}

#[derive(Debug)]
struct Env {
    ast: AST,
    values: Mutex<HashMap<Symbol, AtomNode>>,
}

impl Env {
    fn new(ast: AST) -> Env {
        let mut queue = VecDeque::new();
        queue.push_back(ast.root.unwrap());
        Env {
            ast,
            values: Mutex::new(HashMap::new()),
        }
    }

    // fn resolve(&self, symbol: &Symbol) -> Option<&AtomNode> {
    //     let values = self.values.lock().unwrap();
    //     values.get(symbol).map(|value| {
    //         value
    //     })
    // }

    #[inline]
    fn put(&self, symbol: Symbol, atom: AtomNode) {
        let mut values = self.values.lock().unwrap();
        values.insert(symbol, atom);
    }

    #[inline]
    fn eval(&self, tag: Tag) -> AtomNode {
        let node = self.ast.get(&tag).unwrap();
        // println!("eval {:?}", tag);

        match node {
            Node::Atom(atom) => atom.clone(),
            Node::While { condition, body } => {
                while self.eval(*condition).is_truthy() {
                    for tag in body {
                        self.eval(*tag);
                    }
                }
                AtomNode::Nil
            }
            Node::Definition { ident, value } => {
                let ident = self.eval(*ident).take_symbol().unwrap();
                let value = self.eval(*value);
                self.put(ident, value);
                AtomNode::Nil
            }
            Node::Call {
                function,
                arguments,
            } => {
                let function = self.eval(*function);
                let function = function.take_symbol().unwrap();
                // TODO: Move this closer to the resolution stage. 
                // This allows short circuiting of expressions
                let arguments: Vec<_> = arguments
                    .iter()
                    .map(|tag| {
                        let atom = self.eval(*tag);
                        atom.as_symbol()
                            .map(|symbol| {
                                let values = self.values.lock().unwrap();
                                values.get(symbol).unwrap().clone()
                            })
                            .unwrap_or(atom)
                    })
                    .collect();
                match function.name() {
                    "println" => {
                        for (i, arg) in arguments.iter().enumerate() {
                            print!("{:?}", arg);
                            if i + 1 < arguments.len() {
                                print!(", ");
                            }
                        }
                        print!("\n");
                        AtomNode::Nil
                    }
                    "-" => {
                        let mut items = arguments.into_iter();
                        let mut difference = items.next().unwrap();
                        for current in items {
                            difference = difference.sub(&current);

                        }
                        difference
                    }
                    "+" => {
                        let mut items = arguments.into_iter();
                        let mut sum = items.next().unwrap();
                        for current in items {
                            sum = sum.add(&current);

                        }
                        sum
                    }
                    "=" => {
                        let mut flag = true;
                        let mut items = arguments.into_iter();
                        let mut last = items.next().unwrap();
                        for current in items {
                            if last.eq(&current).is_truthy() {
                                last = current;
                            } else {
                                flag = false;
                                break;
                            }

                        }
                        AtomNode::Boolean(flag)
                    }
                    ">" => {
                        let mut flag = true;
                        let mut items = arguments.into_iter();
                        let mut last = items.next().unwrap();
                        for current in items {
                            if last.gt(&current).is_truthy() {
                                last = current;
                            } else {
                                flag = false;
                                break;
                            }

                        }
                        AtomNode::Boolean(flag)
                    }
                    "<" => {
                        let mut flag = true;
                        let mut items = arguments.into_iter();
                        let mut last = items.next().unwrap();
                        for current in items {
                            if last.lt(&current).is_truthy() {
                                last = current;
                            } else {
                                flag = false;
                                break;
                            }

                        }
                        AtomNode::Boolean(flag)
                    }
                    atom => {
                        panic!("Fuck {:?}", atom);
                    }
                }
            }
            Node::Vector { expressions } => {
                let nodes: Vec<_> = expressions.iter().map(|tag| self.eval(*tag)).collect();
                AtomNode::Vector(nodes)
            }
            Node::Do { expressions } => {
                let mut value = AtomNode::Nil;
                for tag in expressions {
                    value = self.eval(*tag);
                }
                value
            }
            Node::Let { bindings, body } => {
                let bindings = self.eval(*bindings);
                println!("bindings {:?}", bindings);
                println!("body {:?}", body);
                panic!("bindings");
            }
            Node::If { condition, then, otherwise } => {
                if self.eval(*condition).is_truthy() {
                    self.eval(*then)
                } else {
                    self.eval(*otherwise)
                }
            }
            Node::Program { expressions } => {
                let mut value = AtomNode::Nil;
                for tag in expressions {
                    value = self.eval(*tag);
                }
                value
            }
        }
    }

    fn run(&self) {
        if let Some(tag) = self.ast.root {
            println!("result = {:?}", self.eval(tag));
        }
    }
}

pub fn interpret(ast: AST) {
    let env = Env::new(ast);
    env.run();
}
