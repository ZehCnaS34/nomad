use crate::ast::*;
use std::collections::VecDeque;

pub trait Emit {
    fn emit(&self, ast: &AST) -> Option<String>;
}

pub fn emit(ast: &AST) -> Option<String> {
    let root = ast.root?;
    let node = ast.get(&root)?;
    node.emit(ast)
}

impl Emit for Node {
    fn emit(&self, ast: &AST) -> Option<String> {
        match self {
            Node::Nil => Some(String::from("null")),
            Node::Boolean(node) => node.emit(ast),
            Node::Number(node) => node.emit(ast),
            Node::String(node) => node.emit(ast),
            Node::Symbol(node) => node.emit(ast),
            Node::Keyword(node) => node.emit(ast),
            Node::Function(node) => node.emit(ast),
            Node::FunctionCall(node) => node.emit(ast),
            Node::Definition(node) => node.emit(ast),
            Node::If(node) => node.emit(ast),
            Node::While(node) => node.emit(ast),
            Node::Let(node) => node.emit(ast),
            Node::List(node) => node.emit(ast),
            Node::Do(node) => node.emit(ast),
            Node::Program(node) => node.emit(ast),
            Node::Vector(node) => node.emit(ast),
            Node::Recur(node) => node.emit(ast),
            Node::Loop(node) => node.emit(ast),
            Node::Quote(node) => node.emit(ast),
            Node::QuasiQuote(node) => node.emit(ast),
            Node::Meta(node) => node.emit(ast),
            Node::Macro(node) => node.emit(ast),
            Node::Decorator(node) => node.emit(ast),
        }
    }
}

impl Emit for BooleanNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        Some(self.0.to_string())
    }
}
impl Emit for NumberNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        Some(self.0.to_string())
    }
}
impl Emit for StringNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        Some(self.value().to_string())
    }
}
impl Emit for SymbolNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        Some("symbol".into())
    }
}
impl Emit for KeywordNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        Some("keyword".into())
    }
}
impl Emit for FunctionNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        let mut output = String::new();
        output.push_str("(function ");

        let name = ast.get(&self.name()?)?.emit(ast);
        if let Some(name) = name {
            output.push_str(name.as_str());
        }

        let parameters = ast.get(&self.parameters())?;
        if let Node::Vector(vector) = parameters {
            output.push('(');
            let end = vector.items().len();
            for (i, tag) in vector.items().iter().enumerate() {
                let param = ast.get(tag)?;
                let result = param.emit(ast)?;
                output = output + &result;
                if i + 1 != end {
                    output.push_str(", ");
                }
            }
            output.push(')');
        } else {
            return None
        }

        let body = self.body();
        let end = body.len();
        output.push('{');
        for (i, tag) in body.iter().enumerate() {
            let node = ast.get(&tag)?;
            let fragment = node.emit(ast)?;
            if i + 1 != end {
                output = output + &fragment;
                output.push(';');
            } else {
                output.push_str("return ");
                output = output + &fragment;
                output.push(';');
            }
        }
        output.push('}');
        output.push(')');

        Some(output)
    }
}
impl Emit for FunctionCallNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for DefinitionNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for IfNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for WhileNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for LetNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for ListNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for DoNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for ProgramNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        let mut output = String::new();
        for tag in self.expressions() {
            let node = ast.get(&tag)?;
            output = output + &node.emit(ast)?;
        }
        Some(output)
    }
}
impl Emit for VectorNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for RecurNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for LoopNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for QuoteNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for QuasiQuoteNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for MetaNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for MacroNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
impl Emit for DecoratorNode {
    fn emit(&self, ast: &AST) -> Option<String> {
        todo!()
    }
}
