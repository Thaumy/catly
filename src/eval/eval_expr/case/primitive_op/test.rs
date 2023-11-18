use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::Expr;
use crate::eval::PrimitiveOp;
use crate::eval::{closure_type, namely_type};
use crate::infra::RcAnyExt;
use crate::infra::WrapOption;
use crate::infra::WrapResult;

// add
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(None).rc(),
        None
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().rc());

    assert_eq!(evaluated, expr.wrap_ok());
}

// add 1
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(
            Expr::Int(namely_type!("True"), 1)
                .rc()
                .wrap_some()
        )
        .rc(),
        None
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().rc());

    assert_eq!(evaluated, expr.wrap_ok());
}

// add
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(None).rc(),
        None
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(None).rc(),
        expr_env.wrap_some()
    );

    assert_ne!(evaluated, r.wrap_ok());
}

// add 1
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(
            Expr::Int(namely_type!("True"), 1)
                .rc()
                .wrap_some()
        )
        .rc(),
        None
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(
            Expr::Int(namely_type!("True"), 1)
                .rc()
                .wrap_some()
        )
        .rc(),
        expr_env.wrap_some()
    );

    assert_ne!(evaluated, r.wrap_ok());
}
