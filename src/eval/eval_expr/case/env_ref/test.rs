use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::namely_type;
use crate::eval::Expr;
use crate::infra::RcAnyExt;
use crate::infra::WrapOption;
use crate::infra::WrapResult;

// def a = 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10)
            .rc()
            .wrap_some(),
        None
    )
    .rc();

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.wrap_ok());
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
            .wrap_some(),
        None
    );
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 5)
            .rc()
            .wrap_some(),
        expr_env.rc().wrap_some()
    )
    .rc();

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_eq!(evaluated, r.wrap_ok());
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
            .wrap_some(),
        None
    )
    .rc();
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::EnvRef(namely_type!("Int"), "b".to_string())
            .rc()
            .wrap_some(),
        expr_env.wrap_some()
    )
    .rc();
    let expr_env = ExprEnv::new(
        "a".to_string(),
        namely_type!("Int"),
        Expr::EnvRef(namely_type!("Int"), "a".to_string())
            .rc()
            .wrap_some(),
        expr_env.wrap_some()
    )
    .rc();

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.wrap_ok());
}
