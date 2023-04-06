use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::lift;
use crate::unifier::sum::lift as lift_sum;
use crate::unifier::unify;

fn env() -> Vec<(String, Type)> {
    /* env:
    type AB = A | B
    type ABC = A | B | C
    type S = AB | C
    type S2 = B | C
    */
    vec![
        (
            "AB".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
            ])
        ),
        (
            "ABC".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
                Type::TypeEnvRef("C".to_string()),
            ])
        ),
        (
            "S".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("AB".to_string()),
                Type::TypeEnvRef("C".to_string()),
            ])
        ),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let s = &btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("B".to_string()),
    ];
    let derive = &Type::TypeEnvRef("AB".to_string());
    assert!(lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let s = &btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("B".to_string()),
    ];
    let derive = &Type::TypeEnvRef("ABC".to_string());
    assert!(lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let s = &btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("B".to_string()),
    ];
    let derive = &Type::TypeEnvRef("S".to_string());
    assert!(lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let s = &btree_set![
        Type::TypeEnvRef("A".to_string()),
        Type::TypeEnvRef("B".to_string()),
    ];
    let derive = &Type::TypeEnvRef("S2".to_string());
    assert!(!lift_sum(env, s, derive));

    let base = &Type::SumType(s.clone());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
