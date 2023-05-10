use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::{closure_type, namely_type};
use crate::eval::r#type::expr::Expr;
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;

// (a: Int) -> 1
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        "a".to_string().some(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).rc(),
        None
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().rc());

    assert_ne!(evaluated, expr.ok());
}

// (_: Int) -> 1
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        None,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).rc(),
        None
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().rc());

    assert_ne!(evaluated, expr.ok());
}

// (a: Int) -> 1
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        "a".to_string().some(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).rc(),
        None
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        "a".to_string().some(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).rc(),
        expr_env.some()
    );

    assert_eq!(evaluated, r.ok());
}

// (_: Int) -> 1
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        None,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).rc(),
        None
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Closure(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        None,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 1).rc(),
        expr_env.some()
    );

    assert_eq!(evaluated, r.ok());
}
