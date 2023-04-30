use std::assert_matches::assert_matches;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::{eval_expr, EvalRet};
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::result::AnyExt;

pub fn case_int(type_annot: Type, int_value: i64) -> EvalRet {
    Expr::Int(type_annot, int_value).ok()
}

#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Int(namely_type!("Unit"), 10);
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    assert_eq!(evaluated, expr.ok());
}
