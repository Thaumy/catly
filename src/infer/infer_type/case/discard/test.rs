use std::assert_matches::assert_matches;
use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::namely_type;
use crate::infer::env::parse_to_env;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::ReqInfo;
use crate::infra::option::WrapOption;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = "
        type A = Int
    ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let infer_result = Expr::Discard(namely_type!("A").wrap_some())
        .infer_type(&type_env, &expr_env);

    let r = InferTypeRet::has_type(Expr::Discard(
        namely_type!("A").wrap_some()
    ));

    assert_eq!(infer_result, r)
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type =
        Expr::Discard(None).infer_type(&type_env, &expr_env);

    assert_eq!(
        expr_type,
        ReqInfo::of("_", EnvRefConstraint::empty()).into()
    )
}

#[test]
fn test_part3() {
    let (type_env, expr_env) = gen_env();

    let infer_result = Expr::Discard(namely_type!("B").wrap_some())
        .infer_type(&type_env, &expr_env);

    assert_matches!(infer_result, Quad::R(..))
}
