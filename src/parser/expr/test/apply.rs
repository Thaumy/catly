use crate::infer::env::int_type;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    // Apply(Unit, Int)
    let r = Expr::Apply(
        None,
        Expr::Unit(None).wrap_rc(),
        Expr::Int(None, 123).wrap_rc()
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
        Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
        Expr::Int(None, 123).wrap_rc()
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
        Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
        Expr::Unit(None).wrap_rc()
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
        Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
            Expr::Unit(None).wrap_rc()
        )
        .wrap_rc()
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
        Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
            Expr::Apply(
                None,
                Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
                Expr::Unit(None).wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc()
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
            Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
            Expr::Int(None, 123).wrap_rc()
        )
        .wrap_rc(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).wrap_rc(),
                Expr::Int(None, 123).wrap_rc()
            )
            .wrap_rc(),
            Expr::Int(None, 456).wrap_rc()
        )
        .wrap_rc()
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
            Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).wrap_rc(),
                    Expr::Int(None, 123).wrap_rc()
                )
                .wrap_rc(),
                Expr::Int(None, 456).wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc(),
        Expr::Int(None, 123).wrap_rc()
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
        int_type!().wrap_some(),
        Expr::Apply(
            int_type!().wrap_some(),
            Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
            Expr::Apply(
                int_type!().wrap_some(),
                Expr::Apply(
                    int_type!().wrap_some(),
                    Expr::EnvRef(None, "add".to_string()).wrap_rc(),
                    Expr::Int(int_type!().wrap_some(), 123).wrap_rc()
                )
                .wrap_rc(),
                Expr::Int(int_type!().wrap_some(), 456).wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc(),
        Expr::Int(int_type!().wrap_some(), 123).wrap_rc()
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
        int_type!().wrap_some(),
        Expr::Apply(
            None,
            Expr::EnvRef(None, "abc".to_string()).wrap_rc(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).wrap_rc(),
                    Expr::Int(int_type!().wrap_some(), 123).wrap_rc()
                )
                .wrap_rc(),
                Expr::Int(int_type!().wrap_some(), 456).wrap_rc()
            )
            .wrap_rc()
        )
        .wrap_rc(),
        Expr::Int(int_type!().wrap_some(), 123).wrap_rc()
    );
    let r = Some(r);

    let seq = "(abc (add (123: Int) (456: Int)) (123: Int)): Int";
    assert_eq!(f(seq), r);
}
