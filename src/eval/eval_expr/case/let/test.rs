use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::namely_type;
use crate::eval::Expr;
use crate::infra::WrapRc;
use crate::infra::WrapResult;

// let a = 10 in a
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Let(
        "a".to_string(),
        false,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 10).wrap_rc(),
        Expr::EnvRef(namely_type!("Int"), "a".to_string()).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.wrap_ok());
}

// let a = 20, b = 10 in a
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Let(
        "a".to_string(),
        false,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 20).wrap_rc(),
        Expr::Let(
            "b".to_string(),
            false,
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 10).wrap_rc(),
            Expr::EnvRef(namely_type!("Int"), "a".to_string())
                .wrap_rc()
        )
        .wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.wrap_ok());
}

// let a = 20, b = 10, a = 5 in a
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Let(
        "a".to_string(),
        false,
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 20).wrap_rc(),
        Expr::Let(
            "b".to_string(),
            false,
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 10).wrap_rc(),
            Expr::Let(
                "a".to_string(),
                false,
                namely_type!("Int"),
                Expr::Int(namely_type!("Int"), 5).wrap_rc(),
                Expr::EnvRef(namely_type!("Int"), "a".to_string())
                    .wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_eq!(evaluated, r.wrap_ok());
}
