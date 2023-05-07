use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::primitive_op::PrimitiveOp;
use crate::eval::r#type::expr::Expr;
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;

// neg 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        (PrimitiveOp::Neg.into(): Expr).rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, expr_env.rc(), &expr);

    let r = Expr::Int(namely_type!("Int"), -10);

    assert_eq!(evaluated, r.ok());
}

// add 10
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Apply(
        (PrimitiveOp::Add(None).into(): Expr).rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, expr_env.rc(), &expr);

    let r =
        PrimitiveOp::Add(Expr::Int(namely_type!("Int"), 10).some())
            .into(): Expr;

    assert_eq!(evaluated, r.ok());
}

// add 10 10
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]).rc();

    let expr = Expr::Apply(
        (PrimitiveOp::Add(Expr::Int(namely_type!("Int"), 10).some())
            .into(): Expr)
            .rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.ok());
}

// not false
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]).rc();

    let expr = Expr::Apply(
        (PrimitiveOp::Add(None).into(): Expr).rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    let r =
        PrimitiveOp::Add(Expr::Int(namely_type!("Int"), 10).some())
            .into(): Expr;

    assert_eq!(evaluated, r.ok());
}

// and false
#[test]
fn test_part5() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]).rc();

    let expr = Expr::Apply(
        (PrimitiveOp::And(None).into(): Expr).rc(),
        Expr::Int(namely_type!("False"), 0).rc()
    );
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    let r =
        PrimitiveOp::And(Expr::Int(namely_type!("False"), 0).some())
            .into(): Expr;

    assert_eq!(evaluated, r.ok());
}

// and true false
#[test]
fn test_part6() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]).rc();

    let expr = Expr::Apply(
        (PrimitiveOp::And(Expr::Int(namely_type!("True"), 1).some())
            .into(): Expr)
            .rc(),
        Expr::Int(namely_type!("False"), 0).rc()
    );
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    let r = Expr::Int(namely_type!("False"), 0);

    assert_eq!(evaluated, r.ok());
}
