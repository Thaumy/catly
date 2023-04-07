use crate::btree_set;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::unifier::closure::lift as lift_closure;
use crate::unifier::lift;
use crate::unifier::unify;

fn env() -> Vec<(String, Type)> {
    /* env:
    type A = Int
    type B = Int

    type F = A -> B
    type G = A -> B -> C
    type FG = F | G
    type AB = A | B
    */
    vec![
        ("A".to_string(), Type::TypeEnvRef("Int".to_string())),
        ("B".to_string(), Type::TypeEnvRef("Int".to_string())),
        (
            "F".to_string(),
            Type::ClosureType(
                Type::TypeEnvRef("A".to_string())
                    .boxed()
                    .some(),
                Type::TypeEnvRef("B".to_string())
                    .boxed()
                    .some()
            )
        ),
        (
            "G".to_string(),
            Type::ClosureType(
                Type::TypeEnvRef("A".to_string())
                    .boxed()
                    .some(),
                Type::ClosureType(
                    Type::TypeEnvRef("B".to_string())
                        .boxed()
                        .some(),
                    Type::TypeEnvRef("C".to_string())
                        .boxed()
                        .some()
                )
                .boxed()
                .some()
            )
        ),
        (
            "FG".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("F".to_string()),
                Type::TypeEnvRef("G".to_string()),
            ])
        ),
        (
            "AB".to_string(),
            Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
            ])
        ),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::ClosureType(
        Type::TypeEnvRef("A".to_string())
            .boxed()
            .some(),
        Type::TypeEnvRef("B".to_string())
            .boxed()
            .some()
    );
    assert!(lift_closure(
        env,
        &a.clone().some(),
        &b.clone().some(),
        derive
    ));

    let base = &Type::ClosureType(
        a.clone().boxed().some(),
        b.clone().boxed().some()
    );
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("F".to_string());
    assert!(lift_closure(
        env,
        &a.clone().some(),
        &b.clone().some(),
        derive
    ));

    let base = &Type::ClosureType(
        a.clone().boxed().some(),
        b.clone().boxed().some()
    );
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("G".to_string());
    assert!(!lift_closure(
        env,
        &a.clone().some(),
        &b.clone().some(),
        derive
    ));

    let base = &Type::ClosureType(
        a.clone().boxed().some(),
        b.clone().boxed().some()
    );
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("FG".to_string());
    assert!(lift_closure(
        env,
        &a.clone().some(),
        &b.clone().some(),
        derive
    ));

    let base = &Type::ClosureType(
        a.clone().boxed().some(),
        b.clone().boxed().some()
    );
    assert!(lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), derive.clone().some());
}

#[test]
fn test_lift_part5() {
    let env = &env();
    let a = &Type::TypeEnvRef("A".to_string());
    let b = &Type::TypeEnvRef("B".to_string());
    let derive = &Type::TypeEnvRef("AB".to_string());
    assert!(!lift_closure(
        env,
        &a.clone().some(),
        &b.clone().some(),
        derive
    ));

    let base = &Type::ClosureType(
        a.clone().boxed().some(),
        b.clone().boxed().some()
    );
    assert!(!lift(env, base, derive).is_some());
    assert_eq!(unify(env, base, derive), None);
}
