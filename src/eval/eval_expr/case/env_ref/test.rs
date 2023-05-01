use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::result::ResultAnyExt;

// def a = 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10),
        expr_env
    )]);

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.ok());
}

// def a = 10
// def a = 5
#[test]
fn test_part5() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10),
        expr_env
    )]);
    let expr_env = ExprEnv::new(vec![(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 5),
        expr_env
    )]);

    let expr = Expr::EnvRef(namely_type!("Int"), "a".to_string());
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_eq!(evaluated, r.ok());
}
