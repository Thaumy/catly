use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::test::f;
use crate::parser::r#type::Type;

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
