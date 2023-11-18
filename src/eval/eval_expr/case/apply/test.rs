use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::namely_type;
use crate::eval::Expr;
use crate::eval::PrimitiveOp;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::infra::WrapResult;

// neg 10
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Neg).wrap_rc(),
        Expr::Int(namely_type!("Int"), 10).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), -10);

    assert_eq!(evaluated, r.wrap_ok());
}

// add 10
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Add(None)).wrap_rc(),
        Expr::Int(namely_type!("Int"), 10).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::from(PrimitiveOp::Add(
        Expr::Int(namely_type!("Int"), 10)
            .wrap_rc()
            .wrap_some()
    ));

    assert_eq!(evaluated, r.wrap_ok());
}

// add 10 10
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Add(
            Expr::Int(namely_type!("Int"), 10)
                .wrap_rc()
                .wrap_some()
        ))
        .wrap_rc(),
        Expr::Int(namely_type!("Int"), 10).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 20);

    assert_eq!(evaluated, r.wrap_ok());
}

// not false
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::Add(None)).wrap_rc(),
        Expr::Int(namely_type!("Int"), 10).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::from(PrimitiveOp::Add(
        Expr::Int(namely_type!("Int"), 10)
            .wrap_rc()
            .wrap_some()
    ));

    assert_eq!(evaluated, r.wrap_ok());
}

// and false
#[test]
fn test_part5() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::And(None)).wrap_rc(),
        Expr::Int(namely_type!("False"), 0).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::from(PrimitiveOp::And(
        Expr::Int(namely_type!("False"), 0)
            .wrap_rc()
            .wrap_some()
    ));

    assert_eq!(evaluated, r.wrap_ok());
}

// and true false
#[test]
fn test_part6() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Apply(
        Expr::from(PrimitiveOp::And(
            Expr::Int(namely_type!("True"), 1)
                .wrap_rc()
                .wrap_some()
        ))
        .wrap_rc(),
        Expr::Int(namely_type!("False"), 0).wrap_rc()
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("False"), 0);

    assert_eq!(evaluated, r.wrap_ok());
}
