use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::Expr;
use crate::eval::{namely_type, prod_type};
use crate::infra::WrapRc;
use crate::infra::WrapResult;

// { a: Int = 10, b: Bool = true, c: Unit = ()}
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::empty().wrap_rc();

    let expr = Expr::Struct(
        prod_type![
            ("a".to_string(), namely_type!("Int")),
            ("b".to_string(), namely_type!("Bool")),
            ("c".to_string(), namely_type!("Unit"))
        ],
        vec![
            (
                "a".to_string(),
                namely_type!("Int"),
                Expr::Int(namely_type!("Int"), 10).wrap_rc()
            ),
            (
                "b".to_string(),
                namely_type!("Int"),
                Expr::Int(namely_type!("True"), 1).wrap_rc()
            ),
            (
                "c".to_string(),
                namely_type!("Int"),
                Expr::Unit(namely_type!("Unit")).wrap_rc()
            ),
        ]
    );
    let evaluated =
        eval_expr(&type_env, &expr_env, &expr.clone().wrap_rc());

    assert_eq!(evaluated, expr.wrap_ok());
}
