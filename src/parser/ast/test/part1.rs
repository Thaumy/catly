use crate::btree_set;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::prod_type;
use crate::infer::env::sum_type;
use crate::infer::env::unit_type;
use crate::infra::option::WrapOption;
use crate::parser::ast::test::f;
use crate::parser::define::Define;
use crate::parser::expr::r#type::Expr;

#[test]
fn test_part1() {
    let t1 = Define::TypeDef("A".to_string(), namely_type!("B"));
    let d1 =
        Define::ExprDef("a".to_string(), None, Expr::Int(None, 1));
    let d11 = Define::ExprDef(
        "a1".to_string(),
        sum_type![
            namely_type!("A"),
            namely_type!("B"),
            namely_type!("C"),
        ]
        .wrap_some(),
        Expr::Int(None, 1)
    );
    let t2 = Define::TypeDef("C".to_string(), namely_type!("D"));
    let d2 = Define::ExprDef("b".to_string(), None, Expr::Unit(None));
    let d22 = Define::ExprDef(
        "b1".to_string(),
        prod_type![
            ("x".to_string(), int_type!()),
            ("y".to_string(), unit_type!()),
        ]
        .wrap_some(),
        Expr::Unit(None)
    );
    let r = vec![t1, d1, d11, t2, d2, d22];
    let r = Some(r);

    let seq = "type A = B
         def a = 1
         def a1: A | B | C = 1
         type C = D
         def b = ()
         def b1: { x: Int, y: Unit } = ()";
    assert_eq!(f(seq), r);
}
