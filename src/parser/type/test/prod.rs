use crate::infer::env::r#macro::int_type;
use crate::infer::env::r#macro::namely_type;
use crate::infer::env::r#macro::prod_type;
use crate::infer::env::r#macro::unit_type;
use crate::parser::r#type::test::f;

#[test]
fn test_part1() {
    let r = prod_type![("a".to_string(), int_type!())];
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
fn test_part2() {
    let r = prod_type![
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), unit_type!()),
        ("intList".to_string(), namely_type!("IntList")),
    ];
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
fn test_part3() {
    let r = prod_type![
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), prod_type![
            ("x".to_string(), namely_type!("X")),
            ("y".to_string(), namely_type!("Y")),
        ]),
        ("intList".to_string(), namely_type!("IntList")),
    ];
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
fn test_part4() {
    let r = prod_type![
        ("abc".to_string(), prod_type![
            ("x".to_string(), namely_type!("X")),
            ("y".to_string(), namely_type!("Y")),
        ]),
        ("uuu".to_string(), namely_type!("A")),
        ("intList".to_string(), namely_type!("IntList")),
    ];
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
fn test_part5() {
    let r = prod_type![
        ("abc".to_string(), namely_type!("A")),
        ("uuu".to_string(), namely_type!("IntList")),
        ("s".to_string(), prod_type![
            ("x".to_string(), namely_type!("X")),
            ("y".to_string(), namely_type!("Y")),
        ]),
    ];
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
