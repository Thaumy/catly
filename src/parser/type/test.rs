use std::collections::BTreeSet;

use crate::parser::infra::{BoxExt, MaybeType};
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;
use crate::parser::r#type::{parse_type, Type};

fn f(seq: &str) -> MaybeType {
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
    parse_type(seq)
}

#[test]
fn test_parse_int_type() {
    let r = Type::TypeEnvRef("Int".to_string());
    let r = Some(r);

    let seq = "Int";
    assert_eq!(f(seq), r);
    let seq = "(((Int)))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_unit_type() {
    let r = Type::TypeEnvRef("Unit".to_string());
    let r = Some(r);

    let seq = "Unit";
    assert_eq!(f(seq), r);
    let seq = "(((Unit)))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_f_env_ref() {
    let r = Type::TypeEnvRef("A".to_string());
    let r = Some(r);

    let seq = "A";
    assert_eq!(f(seq), r);
    let seq = "(((A)))";
    assert_eq!(f(seq), r);

    assert_eq!(f("a"), None);
    assert_eq!(f("1"), None);
}

#[test]
fn test_f_env_ref_part2() {
    let r = Type::TypeEnvRef("Abc123".to_string());
    let r = Some(r);

    let seq = "Abc123";
    assert_eq!(f(seq), r);
    let seq = "(((Abc123)))";
    assert_eq!(f(seq), r);

    assert_eq!(f("abc"), None);
    assert_eq!(f("123abc"), None);
}

#[test]
fn test_parse_closure_type_part1() {
    let r = Type::ClosureType(
        Type::TypeEnvRef("T".to_string()).boxed(),
        Type::TypeEnvRef("TList".to_string()).boxed(),
    );
    let r = Some(r);

    let seq = "T -> TList";
    assert_eq!(f(seq), r);
    let seq = "((((((T))) -> (((TList))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_type_part2() {
    let r = Type::ClosureType(
        Type::TypeEnvRef("T".to_string()).boxed(),
        Type::ClosureType(
            Type::TypeEnvRef("U".to_string()).boxed(),
            Type::TypeEnvRef("TUEither".to_string()).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "T -> U ->  TUEither";
    assert_eq!(f(seq), r);
    let seq = "(((T -> (((U -> (((TUEither)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_closure_type_part3() {
    let r = Type::ClosureType(
        Type::ClosureType(
            Type::TypeEnvRef("T".to_string()).boxed(),
            Type::TypeEnvRef("U".to_string()).boxed(),
        ).boxed(),
        Type::TypeEnvRef("TUEither".to_string()).boxed(),
    );
    let r = Some(r);

    let seq = "(T -> U) -> TUEither";
    assert_eq!(f(seq), r);
    let seq = "((((((T -> U))) -> (((TUEither))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_sum_type() {
    let r = Type::SumType(BTreeSet::from([
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("Unit".to_string()),
        Type::TypeEnvRef("C".to_string()),
        Type::TypeEnvRef("Int".to_string()),
    ]));
    let r = Some(r);

    let seq = "A | Unit | C | Int";
    assert_eq!(f(seq), r);
    let seq = "(((A | Unit))) | (((C | Int)))";
    assert_eq!(f(seq), r);
    let seq = "A | (((Unit | C))) | Int";
    assert_eq!(f(seq), r);
    let seq = "A | (Unit | C | Int)";
    assert_eq!(f(seq), r);
    let seq = "A | (((Unit | C | Int)))";
    assert_eq!(f(seq), r);
}

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
