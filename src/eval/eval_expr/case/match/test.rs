use std::assert_matches::assert_matches;

use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::expr::Expr;
use crate::infra::r#box::Ext;
use crate::infra::result::AnyExt;

// match 5 with
// | 10 -> 1
// | 20 -> 2
// | _ -> 0
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Match(
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 5).boxed(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10),
                Expr::Int(namely_type!("Int"), 1)
            ),
            (
                Expr::Int(namely_type!("Int"), 20),
                Expr::Int(namely_type!("Int"), 2)
            ),
            (
                Expr::Discard(namely_type!("Int")),
                Expr::Int(namely_type!("Int"), 0)
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 0);

    assert_eq!(evaluated, r.ok());
}

// match 5 with
// | 10 -> 1
// | 5 -> 2
// | _ -> 0
#[test]
fn test_part2() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Match(
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 5).boxed(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10),
                Expr::Int(namely_type!("Int"), 1)
            ),
            (
                Expr::Int(namely_type!("Int"), 5),
                Expr::Int(namely_type!("Int"), 2)
            ),
            (
                Expr::Discard(namely_type!("Int")),
                Expr::Int(namely_type!("Int"), 0)
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 2);

    assert_eq!(evaluated, r.ok());
}

// match 15 with
// | 10 -> 1
// | a -> a
// | _ -> 0
#[test]
fn test_part3() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Match(
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 15).boxed(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10),
                Expr::Int(namely_type!("Int"), 1)
            ),
            (
                Expr::EnvRef(namely_type!("Int"), "a".to_string()),
                Expr::EnvRef(namely_type!("Int"), "a".to_string())
            ),
            (
                Expr::Discard(namely_type!("Int")),
                Expr::Int(namely_type!("Int"), 0)
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 15);

    assert_eq!(evaluated, r.ok());
}

// match 5 with
// | 10 -> 1
// | 20 -> 2
#[test]
fn test_part4() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]);

    let expr = Expr::Match(
        namely_type!("Int"),
        Expr::Int(namely_type!("Int"), 5).boxed(),
        vec![
            (
                Expr::Int(namely_type!("Int"), 10),
                Expr::Int(namely_type!("Int"), 1)
            ),
            (
                Expr::Int(namely_type!("Int"), 20),
                Expr::Int(namely_type!("Int"), 2)
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, &expr_env, &expr);

    let r = Expr::Int(namely_type!("Int"), 5);

    assert_matches!(evaluated, Result::Err(EvalErr { .. }));
}
