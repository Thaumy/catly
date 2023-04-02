use crate::btree_set;
use crate::parser::r#type::Type;
use crate::unifier::env_ref::lift;

fn env() -> Vec<(String, Type)> {
    /* env:
    type A = Unit
    type B = A
    type C = B
    type D = C
    type A_B = A | B
    type B_C_D = B | C | D
    */

    vec![
        ("A".to_string(),
         Type::TypeEnvRef("Unit".to_string())),
        ("B".to_string(),
         Type::TypeEnvRef("A".to_string())),
        ("C".to_string(),
         Type::TypeEnvRef("B".to_string())),
        ("D".to_string(),
         Type::TypeEnvRef("C".to_string())),
        ("AB".to_string(),
         Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
            ])),
        ("ABC".to_string(),
         Type::SumType(btree_set![
                Type::TypeEnvRef("A".to_string()),
                Type::TypeEnvRef("B".to_string()),
                Type::TypeEnvRef("C".to_string()),
            ])),
    ]
}

#[test]
fn test_lift_part1() {
    let env = &env();
    assert!(lift(env, "A", "B"));
}

#[test]
fn test_lift_part2() {
    let env = &env();
    assert!(lift(env, "B", "C"));
}

#[test]
fn test_lift_part3() {
    let env = &env();
    assert!(!lift(env, "B", "D"));
}

#[test]
fn test_lift_part4() {
    let env = &env();
    assert!(!lift(env, "B", "A"));
}

#[test]
fn test_lift_part5() {
    let env = &env();
    assert!(lift(env, "A", "AB"));
}

#[test]
fn test_lift_part6() {
    let env = &env();
    assert!(lift(env, "B", "ABC"));
}

#[test]
fn test_lift_part7() {
    let env = &env();
    assert!(!lift(env, "BC", "ABC"));
}
