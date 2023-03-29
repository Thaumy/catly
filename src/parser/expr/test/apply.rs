use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::r#box::Ext;

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
        Expr::EnvRef("abc".to_string()).boxed(),
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
        Expr::EnvRef("abc".to_string()).boxed(),
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
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef("abc".to_string()).boxed(),
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
        Expr::EnvRef("abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::EnvRef("abc".to_string()).boxed(),
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
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Int(None, 123).boxed(),
        ).boxed(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef("add".to_string()).boxed(),
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
            Expr::EnvRef("abc".to_string()).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("add".to_string()).boxed(),
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
