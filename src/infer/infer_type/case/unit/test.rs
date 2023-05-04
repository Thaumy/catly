use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = "
        type A = Unit
        type B = Int
        def u1 = (): A
        def u2 = 10: A
        def u3 = ()
    ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = expr_env
        .get_expr("u1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let r =
        InferTypeRet::has_type(Expr::Unit(namely_type!("A").some()));

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
        namely_type!("Unit").some()
    ));

    assert_eq!(infer_result, r)
}
