use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::parser::r#type::Type;
use crate::unifier::lift;
use crate::unifier::prod::lift as lift_prod;
use crate::unifier::unify;

fn env() -> Vec<(String, Type)> {
    /* env:
    type S = { a: A }
    type SA = S | A
    type BA = B | A
    */
    vec![
        (
            "S".to_string(),
            Type::ProdType(vec![(
                "a".to_string(),
                Type::TypeEnvRef("A".to_string())
            )])
        ),
        (
            "SA".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("S".to_string()),
                Type::TypeEnvRef("A".to_string()),
            ])
        ),
        (
            "BA".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("B".to_string()),
                Type::TypeEnvRef("A".to_string()),
            ])
        ),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let v =
        &vec![("a".to_string(), Type::TypeEnvRef("A".to_string()))];
    let derive = &Type::ProdType(vec![(
        "a".to_string(),
        Type::TypeEnvRef("A".to_string())
    )]);
    assert!(lift_prod(env, v, derive));

    let base = &Type::ProdType(v.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let v =
        &vec![("a".to_string(), Type::TypeEnvRef("A".to_string()))];
    let derive = &Type::TypeEnvRef("S".to_string());
    assert!(lift_prod(env, v, derive));

    let base = &Type::ProdType(v.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let v =
        &vec![("a".to_string(), Type::TypeEnvRef("A".to_string()))];
    let derive = &Type::TypeEnvRef("SA".to_string());
    assert!(lift_prod(env, v, derive));

    let base = &Type::ProdType(v.clone());
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let v =
        &vec![("a".to_string(), Type::TypeEnvRef("A".to_string()))];
    let derive = &Type::TypeEnvRef("BA".to_string());
    assert!(!lift_prod(env, v, derive));

    let base = &Type::ProdType(v.clone());
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
