use std::assert_matches::assert_matches;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::parse_env;
use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
    let seq = "
        type A = Int
        type B = Unit
        def i1 = 10: A
        def i2 = (): A
        def i3 = 20
    ";
    parse_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_expr("i1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let r = InferTypeRet::has_type(
        namely_type!("A"),
        Expr::Int(namely_type!("A").some(), 10)
    );

    assert_eq!(expr_type, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_expr("i2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_matches!(expr_type, Quad::R(..))
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_expr("i3")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    let r = InferTypeRet::has_type(
        int_type!(),
        Expr::Int(int_type!().some(), 20)
    );

    assert_eq!(expr_type, r)
}
