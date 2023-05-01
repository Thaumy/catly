use crate::infer::env::r#macro::int_type;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    // Apply(Unit, Int)
    let r = Expr::Apply(
        None,
        Expr::Unit(None).boxed(),
        Expr::Int(None, 123).boxed()
    );
    let r = Some(r);

    assert_eq!(f("() 123"), r);
    assert_eq!(f("(()) (123)"), r);
    assert_eq!(f("((())) ((123))"), r);
    assert_eq!(f("(((())) ((123)))"), r);
    assert_eq!(f("((((())) ((123))))"), r);
}

#[test]
fn test_part2() {
    // Apply(EnvRef, Int)
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Int(None, 123).boxed()
    );
    let r = Some(r);

    assert_eq!(f("abc 123"), r);
    assert_eq!(f("(abc) (123)"), r);
    assert_eq!(f("((abc)) ((123))"), r);
    assert_eq!(f("(((abc)) ((123)))"), r);
    assert_eq!(f("((((abc)) ((123))))"), r);
}

#[test]
fn test_part3() {
    // Apply(EnvRef, Unit)
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Unit(None).boxed()
    );
    let r = Some(r);

    assert_eq!(f("abc ()"), r);
    assert_eq!(f("(abc) (())"), r);
    assert_eq!(f("((abc)) ((()))"), r);
    assert_eq!(f("(((abc)) ((())))"), r);
    assert_eq!(f("((((abc)) ((()))))"), r);
}

#[test]
fn test_part4() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Unit(None).boxed()
        )
        .boxed()
    );
    let r = Some(r);

    assert_eq!(f("abc (abc ())"), r);
    assert_eq!(f("(abc) ((abc ()))"), r);
    assert_eq!(f("((abc)) (((abc ())))"), r);
    assert_eq!(f("(((abc)) (((abc ()))))"), r);
    assert_eq!(f("((((abc)) (((abc ())))))"), r);
}

#[test]
fn test_part5() {
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
                Expr::Unit(None).boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Some(r);

    assert_eq!(f("abc (abc (abc ()))"), r);
    assert_eq!(f("(abc) ((abc (abc ())))"), r);
    assert_eq!(f("((abc)) (((abc (abc ()))))"), r);
    assert_eq!(f("(((abc)) (((abc (abc ())))))"), r);
    assert_eq!(f("((((abc)) (((abc (abc ()))))))"), r);
}

#[test]
fn test_part6() {
    // Apply(EnvRef, Apply(EnvRef, Unit))
    let r = Expr::Apply(
        None,
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Int(None, 123).boxed()
        )
        .boxed(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).boxed(),
                Expr::Int(None, 123).boxed()
            )
            .boxed(),
            Expr::Int(None, 456).boxed()
        )
        .boxed()
    );
    let r = Some(r);

    assert_eq!(f("abc 123 (add 123 456)"), r);
    assert_eq!(f("abc ((123)) (((add 123 456)))"), r);
    assert_eq!(
        f("(((abc (((123))) (((add (((123))) (((456)))))))))"),
        r
    );
}

#[test]
fn test_part7() {
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
                    Expr::Int(None, 123).boxed()
                )
                .boxed(),
                Expr::Int(None, 456).boxed()
            )
            .boxed()
        )
        .boxed(),
        Expr::Int(None, 123).boxed()
    );
    let r = Some(r);

    assert_eq!(f("abc (add 123 456) 123"), r);
    assert_eq!(f("abc (((add 123 456))) ((123))"), r);
    assert_eq!(
        f("(((abc (((add (((123))) (((456)))))) (((123))))))"),
        r
    );
}

#[test]
fn test_part8() {
    let r = Expr::Apply(
        int_type!().some(),
        Expr::Apply(
            int_type!().some(),
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Apply(
                int_type!().some(),
                Expr::Apply(
                    int_type!().some(),
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::Int(int_type!().some(), 123).boxed()
                )
                .boxed(),
                Expr::Int(int_type!().some(), 456).boxed()
            )
            .boxed()
        )
        .boxed(),
        Expr::Int(int_type!().some(), 123).boxed()
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
fn test_part9() {
    let r = Expr::Apply(
        int_type!().some(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::Int(int_type!().some(), 123).boxed()
                )
                .boxed(),
                Expr::Int(int_type!().some(), 456).boxed()
            )
            .boxed()
        )
        .boxed(),
        Expr::Int(int_type!().some(), 123).boxed()
    );
    let r = Some(r);

    let seq = "(abc (add (123: Int) (456: Int)) (123: Int)): Int";
    assert_eq!(f(seq), r);
}
