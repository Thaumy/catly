use std::collections::BTreeSet;

use crate::parser::BoxExt;
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;
use crate::parser::r#type::{parse_type, Type};

fn f(seq: &str) -> Option<Type> {
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
    parse_type(seq)
}

#[test]
fn test_parse_int_type() {
    let r = Type::IntType;
    let r = Some(r);

    let seq = "Int";
    assert_eq!(f(seq), r);
    let seq = "(((Int)))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_unit_type() {
    let r = Type::UnitType;
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
fn test_f_apply_part1() {
    let r = Type::TypeApply(
        Type::TypeEnvRef("Lhs".to_string()).boxed(),
        Type::TypeEnvRef("Rhs".to_string()).boxed(),
    );
    let r = Some(r);

    let seq = "Lhs Rhs";
    assert_eq!(f(seq), r);
    let seq = "(((Lhs Rhs)))";
    assert_eq!(f(seq), r);
    let seq = "((((((Lhs))) (((Rhs))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_f_apply_part2() {
    let r = Type::TypeApply(
        Type::TypeApply(
            Type::TypeApply(
                Type::TypeEnvRef("A".to_string()).boxed(),
                Type::TypeEnvRef("B".to_string()).boxed(),
            ).boxed(),
            Type::TypeEnvRef("C".to_string()).boxed(),
        ).boxed(),
        Type::TypeEnvRef("D".to_string()).boxed(),
    );
    let r = Some(r);

    let seq = "A B C D";
    assert_eq!(f(seq), r);
    let seq = "(((A B) C) D)";
    assert_eq!(f(seq), r);
    let seq = "((((((((((((A))) (((B)))))) (((C)))))) (((D))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_f_closure_part1() {
    let r = Type::TypeClosure(
        "T".to_string(),
        Type::TypeApply(
            Type::TypeEnvRef("List".to_string()).boxed(),
            Type::TypeEnvRef("T".to_string()).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "T -> List T";
    assert_eq!(f(seq), r);
    let seq = "(((T))) -> ((((((List))) (((T))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_f_closure_part2() {
    let r = Type::TypeClosure(
        "T".to_string(),
        Type::TypeClosure(
            "U".to_string(),
            Type::TypeApply(
                Type::TypeApply(
                    Type::TypeEnvRef("Either".to_string()).boxed(),
                    Type::TypeEnvRef("T".to_string()).boxed(),
                ).boxed(),
                Type::TypeEnvRef("U".to_string()).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "T -> U -> Either T U";
    assert_eq!(f(seq), r);
    let seq = "(((T -> (((U -> ((((((Either T))) U)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_sum_type() {
    let r = Type::SumType(BTreeSet::from([
        Type::TypeEnvRef("A".to_string()),
        Type::UnitType,
        Type::TypeEnvRef("C".to_string()),
        Type::IntType,
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
        ("a".to_string(), Type::IntType)
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
        ("uuu".to_string(), Type::UnitType),
        ("intList".to_string(), Type::TypeApply(
            Type::TypeEnvRef("List".to_string()).boxed(),
            Type::IntType.boxed(),
        )),
    ]);
    let r = Some(r);

    let seq = "{ abc: A, uuu: Unit, intList: List Int }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: Unit, intList: List Int,}";
    assert_eq!(f(seq), r);
    let seq = "((({ abc: (((A))), uuu: (((Unit))), intList: ((((((List))) Int))) })))";
    assert_eq!(f(seq), r);
    let seq = "((({ abc: (((A))), uuu: (((Unit))), intList: ((((((List))) Int))),})))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_product_type_part3() {
    let r = Type::ProductType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(), Type::ProductType(vec![
            ("x".to_string(), Type::TypeEnvRef("X".to_string())),
            ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
        ])),
        ("intList".to_string(), Type::TypeApply(
            Type::TypeEnvRef("List".to_string()).boxed(),
            Type::IntType.boxed(),
        )),
    ]);
    let r = Some(r);

    let seq = "{ abc: A, uuu: { x: X, y: Y }, intList: List Int }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: { x: X, y: Y }, intList: List Int,}";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: ((({ x: (((X))), y: (((Y))) }))), intList: List Int }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: ((({ x: (((X))), y: (((Y))) }))), intList: List Int,}";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_product_type_part4() {
    let r = Type::ProductType(vec![
        ("abc".to_string(), Type::ProductType(vec![
            ("x".to_string(), Type::TypeEnvRef("X".to_string())),
            ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
        ])),
        ("uuu".to_string(), Type::TypeEnvRef("A".to_string())),
        ("intList".to_string(), Type::TypeApply(
            Type::TypeEnvRef("List".to_string()).boxed(),
            Type::IntType.boxed(),
        )),
    ]);
    let r = Some(r);

    let seq = "{ abc: { x: X, y: Y }, uuu: A, intList: List Int }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: { x: X, y: Y }, uuu: A, intList: List Int,}";
    assert_eq!(f(seq), r);
    let seq = "{ abc: ((({ x: (((X))), y: (((Y))) }))), uuu: A, intList: List Int }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: ((({ x: (((X))), y: (((Y))) }))), uuu: A, intList: List Int,}";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_product_type_part5() {
    let r = Type::ProductType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(), Type::TypeApply(
            Type::TypeEnvRef("List".to_string()).boxed(),
            Type::IntType.boxed(),
        )),
        ("intList".to_string(), Type::ProductType(vec![
            ("x".to_string(), Type::TypeEnvRef("X".to_string())),
            ("y".to_string(), Type::TypeEnvRef("Y".to_string())),
        ])),
    ]);
    let r = Some(r);

    let seq = "{ abc: A, uuu: List Int, intList: { x: X, y: Y } }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: List Int, intList: { x: X, y: Y },}";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: List Int, intList: ((({ x: (((X))), y: (((Y))) }))) }";
    assert_eq!(f(seq), r);
    let seq = "{ abc: A, uuu: List Int, intList: ((({ x: (((X))), y: (((Y))) }))),}";
    assert_eq!(f(seq), r);
}
