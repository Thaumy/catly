use std::assert_matches::assert_matches;

use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::namely_type;
use crate::eval::EvalErr;
use crate::eval::Expr;
use crate::infra::WrapRc;
use crate::infra::WrapResult;

// match 5 with
// | 10 -> 1
// | 20 -> 2
// | _ -> 0
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Match(
        Expr::Int(namely_type!("Int"), 5).wrap_rc(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10).wrap_rc(),
                Expr::Int(namely_type!("Int"), 1).wrap_rc()
            ),
            (
                Expr::Int(namely_type!("Int"), 20).wrap_rc(),
                Expr::Int(namely_type!("Int"), 2).wrap_rc()
            ),
            (
                Expr::Discard(namely_type!("Int")).wrap_rc(),
                Expr::Int(namely_type!("Int"), 0).wrap_rc()
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 0);

    assert_eq!(evaluated, r.wrap_ok());
}

// match 5 with
// | 10 -> 1
// | 5 -> 2
// | _ -> 0
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Match(
        Expr::Int(namely_type!("Int"), 5).wrap_rc(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10).wrap_rc(),
                Expr::Int(namely_type!("Int"), 1).wrap_rc()
            ),
            (
                Expr::Int(namely_type!("Int"), 5).wrap_rc(),
                Expr::Int(namely_type!("Int"), 2).wrap_rc()
            ),
            (
                Expr::Discard(namely_type!("Int")).wrap_rc(),
                Expr::Int(namely_type!("Int"), 0).wrap_rc()
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 2);

    assert_eq!(evaluated, r.wrap_ok());
}

// match 15 with
// | 10 -> 1
// | a -> a
// | _ -> 0
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Match(
        Expr::Int(namely_type!("Int"), 15).wrap_rc(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10).wrap_rc(),
                Expr::Int(namely_type!("Int"), 1).wrap_rc()
            ),
            (
                Expr::EnvRef(namely_type!("Int"), "a".to_string())
                    .wrap_rc(),
                Expr::EnvRef(namely_type!("Int"), "a".to_string())
                    .wrap_rc()
            ),
            (
                Expr::Discard(namely_type!("Int")).wrap_rc(),
                Expr::Int(namely_type!("Int"), 0).wrap_rc()
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    let r = Expr::Int(namely_type!("Int"), 15);

    assert_eq!(evaluated, r.wrap_ok());
}

// match 5 with
// | 10 -> 1
// | 20 -> 2
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Match(
        Expr::Int(namely_type!("Int"), 5).wrap_rc(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10).wrap_rc(),
                Expr::Int(namely_type!("Int"), 1).wrap_rc()
            ),
            (
                Expr::Int(namely_type!("Int"), 20).wrap_rc(),
                Expr::Int(namely_type!("Int"), 2).wrap_rc()
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr.wrap_rc());

    assert_matches!(
        evaluated,
        Result::Err(EvalErr::NonExhaustiveMatch(..))
    );
}
