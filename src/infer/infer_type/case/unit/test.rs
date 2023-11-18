use std::assert_matches::assert_matches;
use std::rc::Rc;

use crate::infer::env::namely_type;
use crate::infer::env::parse_to_env;
use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::Quad;
use crate::infra::WrapOption;
use crate::parser::expr::r#type::Expr;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = "
        type A = Unit
        type B = Int
        def u1 = (): A
        def u2 = 10: A
        def u3 = ()
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_expr("u1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let r = InferTypeRet::has_type(Expr::Unit(
        namely_type!("A").wrap_some()
    ));

    assert_eq!(infer_result, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_expr("u2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_expr("u3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let r = InferTypeRet::has_type(Expr::Unit(
        namely_type!("Unit").wrap_some()
    ));

    assert_eq!(infer_result, r)
}
