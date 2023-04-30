use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::{closure_type, namely_type};
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::r#box::Ext;
use crate::infra::result::AnyExt as ResAnyExt;

// neg 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        namely_type!("Int"),
        (PrimitiveOp::Neg.into(): Expr).boxed(),
        Expr::Int(namely_type!("Int"), 10).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), -10);

    assert_eq!(evaluated, r.ok());
}

// add 10
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        (PrimitiveOp::Add(None).into(): Expr).boxed(),
        Expr::Int(namely_type!("Int"), 10).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r =
        PrimitiveOp::Add(Expr::Int(namely_type!("Int"), 10).some())
            .into(): Expr;

    assert_eq!(evaluated, r.ok());
}

// add 10 10
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        namely_type!("Int"),
        (PrimitiveOp::Add(Expr::Int(namely_type!("Int"), 10).some())
            .into(): Expr)
            .boxed(),
        Expr::Int(namely_type!("Int"), 10).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.ok());
}

// not false
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        closure_type!(namely_type!("Int"), namely_type!("Int")),
        (PrimitiveOp::Add(None).into(): Expr).boxed(),
        Expr::Int(namely_type!("Int"), 10).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r =
        PrimitiveOp::Add(Expr::Int(namely_type!("Int"), 10).some())
            .into(): Expr;

    assert_eq!(evaluated, r.ok());
}

// and false
#[test]
fn test_part5() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        closure_type!(namely_type!("Bool"), namely_type!("Bool")),
        (PrimitiveOp::And(None).into(): Expr).boxed(),
        Expr::Int(namely_type!("False"), 0).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r =
        PrimitiveOp::And(Expr::Int(namely_type!("False"), 0).some())
            .into(): Expr;

    assert_eq!(evaluated, r.ok());
}

// and true false
#[test]
fn test_part6() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        namely_type!("Bool"),
        (PrimitiveOp::And(Expr::Int(namely_type!("True"), 1).some())
            .into(): Expr)
            .boxed(),
        Expr::Int(namely_type!("False"), 0).boxed()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("False"), 0);

    assert_eq!(evaluated, r.ok());
}
