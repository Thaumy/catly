use crate::btree_set;
use crate::infer::env::namely_type;
use crate::infer::env::sum_type;
use crate::infer::env::TypeEnv;
use crate::infra::WrapOption;
use crate::parser::r#type::Type;
use crate::unify::sum::lift_sum;

fn env<'t>() -> TypeEnv<'t> {
    /* env:
    type AB = A | B
    type ABC = A | B | C
    type S = AB | C
    type S2 = B | C
    */
    let vec = vec![
        ("AB".to_string(), sum_type![
            namely_type!("A"),
            namely_type!("B"),
        ]),
        ("ABC".to_string(), sum_type![
            namely_type!("A"),
            namely_type!("B"),
            namely_type!("C"),
        ]),
        ("S".to_string(), sum_type![
            namely_type!("AB"),
            namely_type!("C"),
        ]),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_part1() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B")];
    let derive = &namely_type!("AB");
    assert!(lift_sum(env, s, derive).is_some());

    let base = &Type::SumType(s.clone());
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part2() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B")];
    let derive = &namely_type!("ABC");
    assert!(lift_sum(env, s, derive).is_some());

    let base = &Type::SumType(s.clone());
    assert!(base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), derive.clone().wrap_some());
}

#[test]
fn test_part3() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B")];
    let derive = &namely_type!("S");
    assert!(lift_sum(env, s, derive).is_none());

    let base = &Type::SumType(s.clone());
    assert!(base
        .lift_to(env, derive)
        .is_none());
    assert_eq!(base.unify(env, derive), None);
}

#[test]
fn test_part4() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B")];
    let derive = &namely_type!("S2");
    assert!(lift_sum(env, s, derive).is_none());

    let base = &Type::SumType(s.clone());
    assert!(!base
        .lift_to(env, derive)
        .is_some());
    assert_eq!(base.unify(env, derive), None);
}
