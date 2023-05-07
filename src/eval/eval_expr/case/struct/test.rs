use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::r#macro::{namely_type, prod_type};
use crate::eval::r#type::expr::Expr;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;

// { a: Int = 10, b: Bool = true, c: Unit = ()}
#[test]
fn test_part1() {
    let type_env = TypeEnv::new(vec![]);
    let expr_env = ExprEnv::new(vec![]).rc();

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
                Expr::Int(namely_type!("Int"), 10)
            ),
            (
                "b".to_string(),
                namely_type!("Int"),
                Expr::Int(namely_type!("True"), 1)
            ),
            (
                "c".to_string(),
                namely_type!("Int"),
                Expr::Unit(namely_type!("Unit"))
            ),
        ]
    );
    let evaluated = eval_expr(&type_env, expr_env, &expr);

    assert_eq!(evaluated, expr.ok());
}
