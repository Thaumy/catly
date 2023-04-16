use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::int_type;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    // Cond(EnvRef, Int, Unit)
    let r = Expr::Cond(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Int(None, 123).boxed(),
        Expr::Unit(None).boxed()
    );
    let r = Some(r);

    assert_eq!(f("if abc then 123 else ()"), r);
    assert_eq!(f("if ((abc)) then ((123)) else ((()))"), r);
    assert_eq!(f("(if (((abc))) then (((123))) else (((()))))"), r);
    assert_eq!(
        f("(((if (((abc))) then (((123))) else (((()))))))"),
        r
    );
}

#[test]
fn test_part2() {
    // Cond(a, a, a)
    // while: a = Cond(EnvRef, Apply(Int, Unit), Int)
    let e = Expr::Cond(
        None,
        Expr::EnvRef(None, "abc".to_string()).boxed(),
        Expr::Apply(
            None,
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed()
        )
        .boxed(),
        Expr::Int(None, 456).boxed()
    );
    let r = Some(Expr::Cond(
        None,
        e.clone().boxed(),
        e.clone().boxed(),
        e.clone().boxed()
    ));

    let e = "if abc then 123 () else 456";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(f(seq), r);
    let e = "if abc then (123 ()) else 456";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(f(seq), r);
    let e = "(((if ((abc)) then ((123 ())) else ((456)))))";
    let seq = &format!("if {} then {} else {}", e, e, e);
    assert_eq!(f(seq), r);
}

#[test]
fn test_part3() {
    // Cond(b, b, b)
    // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
    // while: b = Cond(a, a, a)
    let a = Expr::Cond(
        None,
        Expr::Apply(
            None,
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed()
        )
        .boxed(),
        Expr::Int(None, 123).boxed(),
        Expr::EnvRef(None, "abc".to_string()).boxed()
    );
    let b = Expr::Cond(
        None,
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed()
    );
    let r = Expr::Cond(
        None,
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed()
    );
    let r = Some(r);

    let a = "if 123 () then 123 else abc";
    let b = &format!("if {} then {} else {}", a, a, a);
    let seq = &format!("if {} then {} else {}", b, b, b);
    assert_eq!(f(seq), r);
    let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
    let b = &format!("(((if {} then {} else {})))", a, a, a);
    let seq = &format!("if {} then {} else {}", b, b, b);
    assert_eq!(f(seq), r);
}

#[test]
fn test_part4() {
    // Cond(b, b, b)
    // while: a = Cond(Apply(Int, Unit), Int, EnvRef)
    // while: b = Cond(a, a, a)
    let a = Expr::Cond(
        None,
        Expr::Apply(
            None,
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed()
        )
        .boxed(),
        Expr::Int(None, 123).boxed(),
        Expr::EnvRef(None, "abc".to_string()).boxed()
    );
    let b = Expr::Cond(
        None,
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed()
    );
    let r = Expr::Cond(
        None,
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed()
    );
    let r = Some(r);

    let a = "(((if (((123 ()))) then (((123))) else (((abc))))))";
    let b =
        &format!("(((if ((({}))) then ((({}))) else {})))", a, a, a);
    let seq =
        &format!("(((if ((({}))) then {} else ((({}))))))", b, b, b);

    assert_eq!(f(seq), r);
}

#[test]
fn test_part5() {
    let a = Expr::Cond(
        int_type!().some(),
        Expr::Apply(
            int_type!().some(),
            Expr::Int(None, 123).boxed(),
            Expr::Unit(None).boxed()
        )
        .boxed(),
        Expr::Int(int_type!().some(), 123).boxed(),
        Expr::EnvRef(None, "abc".to_string()).boxed()
    );
    let b = Expr::Cond(
        None,
        a.clone().boxed(),
        a.clone().boxed(),
        a.clone().boxed()
    );
    let r = Expr::Cond(
        int_type!().some(),
        b.clone().boxed(),
        b.clone().boxed(),
        b.clone().boxed()
    );
    let r = Some(r);

    let a = "(if ((123 ()): Int) then (123: Int) else abc): Int";
    let b =
        &format!("(((if ((({}))) then ((({}))) else {})))", a, a, a);
    let seq =
        &format!("(if ((({}))) then {} else ((({})))): Int", b, b, b);

    assert_eq!(f(seq), r);
}
