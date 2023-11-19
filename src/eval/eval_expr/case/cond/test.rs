use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::namely_type;
use crate::eval::Expr;
use crate::infra::WrapRc;
use crate::infra::WrapResult;

// if false then 10 else 20
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Cond(
        Expr::Int(namely_type!("False"), 0).wrap_rc(),
        Expr::Int(namely_type!("Int"), 10).wrap_rc(),
        Expr::Int(namely_type!("Int"), 20).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.wrap_ok());
}

// if true then 10 else 20
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Cond(
        Expr::Int(namely_type!("True"), 1).wrap_rc(),
        Expr::Int(namely_type!("Int"), 10).wrap_rc(),
        Expr::Int(namely_type!("Int"), 20).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.wrap_ok());
}
