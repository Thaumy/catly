use crate::env::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::lift;
use crate::unifier::sum::lift as lift_sum;
use crate::unifier::unify;
use crate::{btree_set, namely_type};

fn env() -> TypeEnv {
    /* env:
    type AB = A | B
    type ABC = A | B | C
    type S = AB | C
    type S2 = B | C
    */
    let vec = vec![
        (
            "AB".to_string(),
            Type::SumType(btree_set![
                namely_type!("A"),
                namely_type!("B"),
            ])
        ),
        (
            "ABC".to_string(),
            Type::SumType(btree_set![
                namely_type!("A"),
                namely_type!("B"),
                namely_type!("C"),
            ])
        ),
        (
            "S".to_string(),
            Type::SumType(btree_set![
                namely_type!("AB"),
                namely_type!("C"),
            ])
        ),
    ];

    TypeEnv::new(vec)
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B"),];
    let derive = &namely_type!("AB");
    assert!(lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B"),];
    let derive = &namely_type!("ABC");
    assert!(lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B"),];
    let derive = &namely_type!("S");
    assert!(lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let s = &btree_set![namely_type!("A"), namely_type!("B"),];
    let derive = &namely_type!("S2");
    assert!(!lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
