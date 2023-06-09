use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;

// def a = 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10)
            .rc()
            .some(),
        None
    )
    .rc();

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.ok());
}

// def a = 10
// def a = 5
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10)
            .rc()
            .some(),
        None
    );
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 5)
            .rc()
            .some(),
        expr_env.rc().some()
    )
    .rc();

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_eq!(evaluated, r.ok());
}

// def b = 10
// def a = b
// def a = a
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(
        "b".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10)
            .rc()
            .some(),
        None
    )
    .rc();
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::EnvRef(namely_type!("Int"), "b".to_string())
            .rc()
            .some(),
        expr_env.some()
    )
    .rc();
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::EnvRef(namely_type!("Int"), "a".to_string())
            .rc()
            .some(),
        expr_env.some()
    )
    .rc();

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.ok());
}
