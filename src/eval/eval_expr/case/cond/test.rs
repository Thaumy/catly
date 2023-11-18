use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::namely_type;
use crate::eval::r#type::Expr;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::WrapResult;

// if false then 10 else 20
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Cond(
        Expr::Int(namely_type!("False"), 0).rc(),
        Expr::Int(namely_type!("Int"), 10).rc(),
        Expr::Int(namely_type!("Int"), 20).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.wrap_ok());
}

// if true then 10 else 20
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Cond(
        Expr::Int(namely_type!("True"), 1).rc(),
        Expr::Int(namely_type!("Int"), 10).rc(),
        Expr::Int(namely_type!("Int"), 20).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 10);

    assert_eq!(evaluated, r.wrap_ok());
}
