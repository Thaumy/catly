use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::Expr;
use crate::eval::{closure_type, namely_type};
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::infra::WrapResult;

// (a: Int) -> 1
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        "a".to_string().wrap_some(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).wrap_rc(),
        None
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().wrap_rc());

    assert_ne!(evaluated, expr.wrap_ok());
}

// (_: Int) -> 1
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        None,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).wrap_rc(),
        None
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().wrap_rc());

    assert_ne!(evaluated, expr.wrap_ok());
}

// (a: Int) -> 1
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        "a".to_string().wrap_some(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).wrap_rc(),
        None
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        "a".to_string().wrap_some(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).wrap_rc(),
        expr_env.wrap_some()
    );

    assert_eq!(evaluated, r.wrap_ok());
}

// (_: Int) -> 1
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        None,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).wrap_rc(),
        None
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        None,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).wrap_rc(),
        expr_env.wrap_some()
    );

    assert_eq!(evaluated, r.wrap_ok());
}
