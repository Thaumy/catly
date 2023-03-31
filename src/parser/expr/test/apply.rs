use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::option::AnyExt;
use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::Type;

#[test]
fn test_parse_apply_part1() {
    // Apply(Unit, Int)
    let r = Expr::Apply(
        None,
        Expr::Unit(None).boxed(),
        Expr::Int(None, 123).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("() 123"), r);
    assert_eq!(f("(()) (123)"), r);
    assert_eq!(f("((())) ((123))"), r);
    assert_eq!(f("(((())) ((123)))"), r);
    assert_eq!(f("((((())) ((123))))"), r);
}

#[test]
fn test_parse_apply_part2() {
    // Apply(EnvRef, Int)
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Int(None, 123).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc 123"), r);
    assert_eq!(f("(abc) (123)"), r);
    assert_eq!(f("((abc)) ((123))"), r);
    assert_eq!(f("(((abc)) ((123)))"), r);
    assert_eq!(f("((((abc)) ((123))))"), r);
}

#[test]
fn test_parse_apply_part3() {
    // Apply(EnvRef, Unit)
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Unit(None).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc ()"), r);
    assert_eq!(f("(abc) (())"), r);
    assert_eq!(f("((abc)) ((()))"), r);
    assert_eq!(f("(((abc)) ((())))"), r);
    assert_eq!(f("((((abc)) ((()))))"), r);
}

#[test]
fn test_parse_apply_part4() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Unit(None).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc (abc ())"), r);
    assert_eq!(f("(abc) ((abc ()))"), r);
    assert_eq!(f("((abc)) (((abc ())))"), r);
    assert_eq!(f("(((abc)) (((abc ()))))"), r);
    assert_eq!(f("((((abc)) (((abc ())))))"), r);
}

#[test]
fn test_parse_apply_part5() {
    // Apply(EnvRef, Apply(EnvRef, Apply(EnvRef, Unit)))
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::EnvRef(None, "abc".to_string()).boxed(),
                Expr::Unit(None).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc (abc (abc ()))"), r);
    assert_eq!(f("(abc) ((abc (abc ())))"), r);
    assert_eq!(f("((abc)) (((abc (abc ()))))"), r);
    assert_eq!(f("(((abc)) (((abc (abc ())))))"), r);
    assert_eq!(f("((((abc)) (((abc (abc ()))))))"), r);
}

#[test]
fn test_parse_apply_part6() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Int(None, 123).boxed(),
        ).boxed(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).boxed(),
                Expr::Int(None, 123).boxed(),
            ).boxed(),
            Expr::Int(None, 456).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc 123 (add 123 456)"), r);
    assert_eq!(f("abc ((123)) (((add 123 456)))"), r);
    assert_eq!(f("(((abc (((123))) (((add (((123))) (((456)))))))))"), r);
}

#[test]
fn test_parse_apply_part7() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::Int(None, 123).boxed(),
                ).boxed(),
                Expr::Int(None, 456).boxed(),
            ).boxed(),
        ).boxed(),
        Expr::Int(None, 123).boxed(),
    );
    let r = Some(r);

    assert_eq!(f("abc (add 123 456) 123"), r);
    assert_eq!(f("abc (((add 123 456))) ((123))"), r);
    assert_eq!(f("(((abc (((add (((123))) (((456)))))) (((123))))))"), r);
}

#[test]
fn test_parse_apply_part8() {
    let r = Expr::Apply(
        Type::TypeEnvRef("Int".to_string()).some(),
        Expr::Apply(
            Type::TypeEnvRef("Int".to_string()).some(),
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Apply(
                Type::TypeEnvRef("Int".to_string()).some(),
                Expr::Apply(
                    Type::TypeEnvRef("Int".to_string()).some(),
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::Int(
                        Type::TypeEnvRef("Int".to_string()).some(),
                        123,
                    ).boxed(),
                ).boxed(),
                Expr::Int(
                    Type::TypeEnvRef("Int".to_string()).some(),
                    456,
                ).boxed(),
            ).boxed(),
        ).boxed(),
        Expr::Int(
            Type::TypeEnvRef("Int".to_string()).some(),
            123,
        ).boxed(),
    );
    let r = Some(r);

    let seq = "(\
    (\
    (abc ((((add (123: Int)): Int) (456: Int)): Int)): Int\
    ) \
    (123: Int)\
    ): Int";
    assert_eq!(f(seq), r);
}


#[test]
fn test_parse_apply_part9() {
    let r = Expr::Apply(
        Type::TypeEnvRef("Int".to_string()).some(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::Int(
                        Type::TypeEnvRef("Int".to_string()).some(),
                        123,
                    ).boxed(),
                ).boxed(),
                Expr::Int(
                    Type::TypeEnvRef("Int".to_string()).some(),
                    456,
                ).boxed(),
            ).boxed(),
        ).boxed(),
        Expr::Int(
            Type::TypeEnvRef("Int".to_string()).some(),
            123,
        ).boxed(),
    );
    let r = Some(r);

    let seq = "(abc (add (123: Int) (456: Int)) (123: Int)): Int";
    assert_eq!(f(seq), r);
}
