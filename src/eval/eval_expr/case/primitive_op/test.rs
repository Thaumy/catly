use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::{closure_type, namely_type};
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;

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
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    assert_eq!(evaluated, expr.ok());
}

// add 1
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(Expr::Int(namely_type!("True"), 1).some())
            .rc(),
        None
    );
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    assert_eq!(evaluated, expr.ok());
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
    let evaluated = eval_expr(&type_env, expr_env.clone(), &expr);

    let r = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(None).rc(),
        expr_env.some()
    );

    assert_ne!(evaluated, r.ok());
}

// add 1
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(Expr::Int(namely_type!("True"), 1).some())
            .rc(),
        None
    );
    let evaluated = eval_expr(&type_env, expr_env.clone(), &expr);

    let r = Expr::PrimitiveOp(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        PrimitiveOp::Add(Expr::Int(namely_type!("True"), 1).some())
            .rc(),
        expr_env.some()
    );

    assert_ne!(evaluated, r.ok());
}
