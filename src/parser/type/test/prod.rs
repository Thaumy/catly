use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;
use crate::{int_type, unit_type};

#[test]
fn test_parse_prod_type_part1() {
    let r = Type::ProdType(vec![("a".to_string(), int_type!())]);
    let r = Some(r);

    let seq = "{ a: Int }";
    assert_eq!(f(seq), r);
    let seq = "{ a: Int,}";
    assert_eq!(f(seq), r);
    let seq = "((({ a: (((Int))) })))";
    assert_eq!(f(seq), r);
    let seq = "((({ a: (((Int))),})))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_prod_type_part2() {
    let r = Type::ProdType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(), unit_type!()),
        (
            "intList".to_string(),
            Type::TypeEnvRef("IntList".to_string())
        ),
    ]);
    let r = Some(r);

    let seq = "{ abc: A, uuu: Unit, intList: IntList }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: Unit, intList: IntList,}";
    assert_eq!(f(seq), r);
    let seq = "((({ abc: (((A))), uuu: (((Unit))), intList: (((IntList))) })))";
    assert_eq!(f(seq), r);
    let seq = "((({ abc: (((A))), uuu: (((Unit))), intList: (((IntList))),})))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_prod_type_part3() {
    let r = Type::ProdType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        (
            "uuu".to_string(),
            Type::ProdType(vec![
                ("x".to_string(), Type::TypeEnvRef("X".to_string())),
                ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
            ])
        ),
        (
            "intList".to_string(),
            Type::TypeEnvRef("IntList".to_string())
        ),
    ]);
    let r = Some(r);

    let seq = "{ abc: A, uuu: { x: X, y: Y }, intList: IntList }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: { x: X, y: Y }, intList: IntList,}";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: ((({ x: (((X))), y: (((Y))) }))), intList: IntList }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: ((({ x: (((X))), y: (((Y))) }))), intList: IntList,}";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_prod_type_part4() {
    let r = Type::ProdType(vec![
        (
            "abc".to_string(),
            Type::ProdType(vec![
                ("x".to_string(), Type::TypeEnvRef("X".to_string())),
                ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
            ])
        ),
        ("uuu".to_string(), Type::TypeEnvRef("A".to_string())),
        (
            "intList".to_string(),
            Type::TypeEnvRef("IntList".to_string())
        ),
    ]);
    let r = Some(r);

    let seq = "{ abc: { x: X, y: Y }, uuu: A, intList: IntList }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: { x: X, y: Y }, uuu: A, intList: IntList,}";
    assert_eq!(f(seq), r);
    let seq = "{ abc: ((({ x: (((X))), y: (((Y))) }))), uuu: A, intList: IntList }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: ((({ x: (((X))), y: (((Y))) }))), uuu: A, intList: IntList,}";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_prod_type_part5() {
    let r = Type::ProdType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(), Type::TypeEnvRef("IntList".to_string())),
        (
            "s".to_string(),
            Type::ProdType(vec![
                ("x".to_string(), Type::TypeEnvRef("X".to_string())),
                ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
            ])
        ),
    ]);
    let r = Some(r);

    let seq = "{ abc: A, uuu: IntList, s: { x: X, y: Y } }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: IntList, s: { x: X, y: Y },}";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: IntList, s: ((({ x: (((X))), y: (((Y))) }))) }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: IntList, s: ((({ x: (((X))), y: (((Y))) }))),}";
    assert_eq!(f(seq), r);
}
