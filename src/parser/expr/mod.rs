use std::vec;

use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::go;
use crate::parser::expr::r#type::MaybeExpr;

mod r#fn;
mod pat;
#[cfg(test)]
mod test;
pub mod r#type;

type In = crate::pp::Out;

pub fn parse_expr(seq: Vec<In>) -> MaybeExpr {
    let r = go(vec![Pat::Start], seq).into();

    if cfg!(feature = "parser_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[parsed]", "Expr");
        println!("{log}");
    }

    r
}
