use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;
use crate::{int_type, namely_type, unit_type};

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
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), unit_type!()),
        ("intList".to_string(), namely_type!("IntList")),
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
        ("abc".to_string(), namely_type!("A")),
        (
            "uuu".to_string(),
            Type::ProdType(vec![
                ("x".to_string(), namely_type!("X")),
                ("y".to_string(), namely_type!("Y")),
            ])
        ),
        ("intList".to_string(), namely_type!("IntList")),
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
                ("x".to_string(), namely_type!("X")),
                ("y".to_string(), namely_type!("Y")),
            ])
        ),
        ("uuu".to_string(), namely_type!("A")),
        ("intList".to_string(), namely_type!("IntList")),
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
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), namely_type!("IntList")),
        (
            "s".to_string(),
            Type::ProdType(vec![
                ("x".to_string(), namely_type!("X")),
                ("y".to_string(), namely_type!("Y")),
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
