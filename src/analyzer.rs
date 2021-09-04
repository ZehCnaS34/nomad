use crate::ast::Expr;
use crate::result::*;
use crate::rt::Runtime;

pub fn analyze(expr: Expr, rt: Runtime) -> NResult<Expr> {
    Ok(expr)
}
