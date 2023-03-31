use crate::parser::ast::test::f;
use crate::parser::define::Define;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

#[test]
fn test_parse_ast_part1() {
    let t1 = Define::TypeDef(
        "A".to_string(),
        Type::TypeEnvRef("B".to_string()),
    );
    let d1 = Define::ExprDef(
        "a".to_string(),
        None,
        Expr::Int(None, 1),
    );
    let t2 = Define::TypeDef(
        "C".to_string(),
        Type::TypeEnvRef("D".to_string()),
    );
    let d2 = Define::ExprDef(
        "b".to_string(),
        None,
        Expr::Unit(None),
    );
    let r = vec![t1, d1, t2, d2];
    let r = Some(r);

    let seq =
        "type A = B
         def a = 1
         type C = D
         def b = ()";
    assert_eq!(f(seq), r);
}

