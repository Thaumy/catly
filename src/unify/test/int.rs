use crate::btree_set;
use crate::infer::env::int_type;
use crate::infer::env::namely_type;
use crate::infer::env::sum_type;
use crate::infer::env::TypeEnv;
use crate::infra::WrapOption;
use crate::parser::r#type::Type;
use crate::unify::namely::lift_namely;

fn env<'t>() -> TypeEnv<'t> {
    /* env:
    type A = Int
    type B = A
    type C = B

    type S0 = D | Int
    type S1 = D | A
    type S2 = D | C
    type S3 = D | E
    */
    let vec = vec![
        ("A".to_string(), int_type!()),
        ("B".to_string(), namely_type!("A")),
        ("C".to_string(), namely_type!("B")),
        (
            "S0".to_string(),
            Type::SumType(btree_set![namely_type!("D"), int_type!()])
        ),
        ("S1".to_string(), sum_type![
            namely_type!("D"),
            namely_type!("A"),
        ]),
        ("S2".to_string(), sum_type![
            namely_type!("D"),
            namely_type!("C"),
        ]),
        ("S3".to_string(), sum_type![
            namely_type!("D"),
            namely_type!("E"),
        ]),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_part1() {
    let env = &env();
    let derive = &int_type!();
    assert!(lift_namely(env, "Int", derive).is_some());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part2() {
    let env = &env();
    let derive = &namely_type!("A");
    assert!(lift_namely(env, "Int", derive).is_some());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part3() {
    let env = &env();
    let derive = &namely_type!("B");
    assert!(lift_namely(env, "Int", derive).is_some());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part4() {
    let env = &env();
    let derive = &namely_type!("C");
    assert!(lift_namely(env, "Int", derive).is_some());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part5() {
    let env = &env();
    let derive = &namely_type!("D");
    assert!(lift_namely(env, "Int", derive).is_none());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_none());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part6() {
    let env = &env();
    let derive = &namely_type!("S0");
    assert!(lift_namely(env, "Int", derive).is_some());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part7() {
    let env = &env();
    let derive = &namely_type!("S1");
    assert!(lift_namely(env, "A", derive).is_some());

    let base = &namely_type!("A");
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part8() {
    let env = &env();
    let derive = &namely_type!("S2");
    assert!(lift_namely(env, "C", derive).is_some());

    let base = &namely_type!("C");
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part9() {
    let env = &env();
    let derive = &namely_type!("S3");
    assert!(lift_namely(env, "Int", derive).is_none());

    let base = &int_type!();
    assert!(base
        .lift_to(env, derive)
        .is_none());
    assert_eq!(base.unify(env, derive), None);
}
