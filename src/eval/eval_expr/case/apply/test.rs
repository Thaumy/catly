use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::Expr;
use crate::eval::r#type::PrimitiveOp;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::WrapResult;

// neg 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Neg).rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), -10);

    assert_eq!(evaluated, r.wrap_ok());
}

// add 10
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Add(None)).rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::from(PrimitiveOp::Add(
        Expr::Int(namely_type!("Int"), 10)
            .rc()
            .wrap_some()
    ));

    assert_eq!(evaluated, r.wrap_ok());
}

// add 10 10
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Add(
            Expr::Int(namely_type!("Int"), 10)
                .rc()
                .wrap_some()
        ))
        .rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.wrap_ok());
}

// not false
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Add(None)).rc(),
        Expr::Int(namely_type!("Int"), 10).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::from(PrimitiveOp::Add(
        Expr::Int(namely_type!("Int"), 10)
            .rc()
            .wrap_some()
    ));

    assert_eq!(evaluated, r.wrap_ok());
}

// and false
#[test]
fn test_part5() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::And(None)).rc(),
        Expr::Int(namely_type!("False"), 0).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::from(PrimitiveOp::And(
        Expr::Int(namely_type!("False"), 0)
            .rc()
            .wrap_some()
    ));

    assert_eq!(evaluated, r.wrap_ok());
}

// and true false
#[test]
fn test_part6() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::And(
            Expr::Int(namely_type!("True"), 1)
                .rc()
                .wrap_some()
        ))
        .rc(),
        Expr::Int(namely_type!("False"), 0).rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

    let r = Expr::Int(namely_type!("False"), 0);

    assert_eq!(evaluated, r.wrap_ok());
}
