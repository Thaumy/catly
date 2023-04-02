use crate::btree_set;
use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::Type;
use crate::unifier::product::lift;

fn env() -> Vec<(String, Type)> {
    /* env:
    type S = { a: A }
    type SA = S | A
    type BA = B | A
    */
    vec![
        ("S".to_string(),
         Type::ProductType(vec![
             ("a".to_string(), Type::TypeEnvRef("A".to_string())),
         ])),
        ("SA".to_string(),
         Type::SumType(btree_set![
             Type::TypeEnvRef("S".to_string()),
             Type::TypeEnvRef("A".to_string()),
         ])),
        ("BA".to_string(),
         Type::SumType(btree_set![
             Type::TypeEnvRef("B".to_string()),
             Type::TypeEnvRef("A".to_string()),
         ])),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    let v = &vec![
        ("a".to_string(), Type::TypeEnvRef("A".to_string())),
    ];
    let derive = &Type::ProductType(vec![
        ("a".to_string(), Type::TypeEnvRef("A".to_string())),
    ]);

    assert!(lift(env, v, derive));
}

#[test]
fn test_lift_part2() {
    let env = &env();
    let v = &vec![
        ("a".to_string(), Type::TypeEnvRef("A".to_string())),
    ];
    let derive = &Type::TypeEnvRef("S".to_string());

    assert!(lift(env, v, derive));
}

#[test]
fn test_lift_part3() {
    let env = &env();
    let v = &vec![
        ("a".to_string(), Type::TypeEnvRef("A".to_string())),
    ];
    let derive = &Type::TypeEnvRef("SA".to_string());

    assert!(lift(env, v, derive));
}

#[test]
fn test_lift_part4() {
    let env = &env();
    let v = &vec![
        ("a".to_string(), Type::TypeEnvRef("A".to_string())),
    ];
    let derive = &Type::TypeEnvRef("BA".to_string());

    assert!(!lift(env, v, derive));
}
