use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::parser::ast::test::f;
use crate::parser::define::Define;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

#[test]
fn test_parse_ast_part1() {
    let t1 = Define::TypeDef(
        "A".to_string(),
        Type::TypeEnvRef("B".to_string())
    );
    let d1 =
        Define::ExprDef("a".to_string(), None, Expr::Int(None, 1));
    let d11 = Define::ExprDef(
        "a1".to_string(),
        Type::SumType(btree_set![
            Type::TypeEnvRef("A".to_string()),
            Type::TypeEnvRef("B".to_string()),
            Type::TypeEnvRef("C".to_string()),
        ])
        .some(),
        Expr::Int(None, 1)
    );
    let t2 = Define::TypeDef(
        "C".to_string(),
        Type::TypeEnvRef("D".to_string())
    );
    let d2 = Define::ExprDef("b".to_string(), None, Expr::Unit(None));
    let d22 = Define::ExprDef(
        "b1".to_string(),
        Type::ProdType(vec![
            ("x".to_string(), Type::TypeEnvRef("Int".to_string())),
            ("y".to_string(), Type::TypeEnvRef("Unit".to_string())),
        ])
        .some(),
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
