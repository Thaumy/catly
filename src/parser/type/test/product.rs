use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;

#[test]
fn test_parse_product_type_part1() {
    let r = Type::ProductType(vec![
        ("a".to_string(), Type::TypeEnvRef("Int".to_string()))
    ]);
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
fn test_parse_product_type_part2() {
    let r = Type::ProductType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(), Type::TypeEnvRef("Unit".to_string())),
        ("intList".to_string(), Type::TypeEnvRef("IntList".to_string())),
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
fn test_parse_product_type_part3() {
    let r = Type::ProductType(vec![
        ("abc".to_string(),
         Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(),
         Type::ProductType(vec![
             ("x".to_string(), Type::TypeEnvRef("X".to_string())),
             ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
         ])),
        ("intList".to_string(),
         Type::TypeEnvRef("IntList".to_string())),
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
fn test_parse_product_type_part4() {
    let r = Type::ProductType(vec![
        ("abc".to_string(),
         Type::ProductType(vec![
             ("x".to_string(), Type::TypeEnvRef("X".to_string())),
             ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
         ])),
        ("uuu".to_string(),
         Type::TypeEnvRef("A".to_string())),
        ("intList".to_string(),
         Type::TypeEnvRef("IntList".to_string())),
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
fn test_parse_product_type_part5() {
    let r = Type::ProductType(vec![
        ("abc".to_string(),
         Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(),
         Type::TypeEnvRef("IntList".to_string())),
        ("s".to_string(),
         Type::ProductType(vec![
             ("x".to_string(), Type::TypeEnvRef("X".to_string())),
             ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
         ])),
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
