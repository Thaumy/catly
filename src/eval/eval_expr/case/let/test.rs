use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::result::ResultAnyExt;

// let a = 10 in a
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Let(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10).boxed(),
        Expr::EnvRef(namely_type!("Int"), "a".to_string()).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.ok());
}

// let a = 20, b = 10 in a
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Let(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 20).boxed(),
        Expr::Let(
            "b".to_string(),
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 10).boxed(),
            Expr::EnvRef(namely_type!("Int"), "a".to_string())
                .boxed()
        )
        .boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.ok());
}

// let a = 20, b = 10, a = 5 in a
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Let(
        "a".to_string(),
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 20).boxed(),
        Expr::Let(
            "b".to_string(),
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 10).boxed(),
            Expr::Let(
                "a".to_string(),
                namely_type!("Int"),
                Expr::Int(namely_type!("Int"), 5).boxed(),
                Expr::EnvRef(namely_type!("Int"), "a".to_string())
                    .boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_eq!(evaluated, r.ok());
}
