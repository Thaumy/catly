use crate::infer::env::closure_type;
use crate::infer::env::int_type;
use crate::infra::RcAnyExt;
use crate::infra::WrapOption;
use crate::parser::define::test::f;
use crate::parser::define::Define;
use crate::parser::expr::r#type::Expr;

#[test]
fn test_part1() {
    let r = Define::ExprDef(
        "a".to_string(),
        None,
        Expr::EnvRef(None, "b".to_string())
    );
    let r = Some(r);

    let seq = "def a = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_part2() {
    let r = Define::ExprDef(
        "a".to_string(),
        int_type!().wrap_some(),
        Expr::EnvRef(None, "b".to_string())
    );
    let r = Some(r);

    let seq = "def a: Int = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_part3() {
    let r = Define::ExprDef(
        "a".to_string(),
        closure_type!(int_type!(), int_type!()).wrap_some(),
        Expr::EnvRef(None, "b".to_string())
    );
    let r = Some(r);

    let seq = "def a: Int -> Int = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_part4() {
    let e = Expr::Let(
        None,
        false,
        "a".to_string(),
        None,
        Expr::Int(None, 123).rc(),
        Expr::Let(
            None,
            false,
            "b".to_string(),
            None,
            Expr::Let(
                None,
                false,
                "x".to_string(),
                None,
                Expr::Closure(
                    None,
                    "i".to_string().wrap_some(),
                    None,
                    Expr::Closure(
                        None,
                        "j".to_string().wrap_some(),
                        None,
                        Expr::EnvRef(None, "k".to_string()).rc()
                    )
                    .rc()
                )
                .rc(),
                Expr::Let(
                    None,
                    false,
                    "y".to_string(),
                    None,
                    Expr::EnvRef(None, "a".to_string()).rc(),
                    Expr::Let(
                        None,
                        false,
                        "z".to_string(),
                        None,
                        Expr::Unit(None).rc(),
                        Expr::EnvRef(None, "a".to_string()).rc()
                    )
                    .rc()
                )
                .rc()
            )
            .rc(),
            Expr::Let(
                None,
                false,
                "d".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).rc(),
                    Expr::Int(None, 1).rc()
                )
                .rc(),
                Expr::Let(
                    None,
                    false,
                    "e".to_string(),
                    None,
                    Expr::Int(None, 6).rc(),
                    Expr::Let(
                        None,
                        false,
                        "k".to_string(),
                        None,
                        Expr::Unit(None).rc(),
                        Expr::Let(
                            None,
                            false,
                            "m".to_string(),
                            None,
                            Expr::Unit(None).rc(),
                            Expr::Let(
                                None,
                                false,
                                "n".to_string(),
                                None,
                                Expr::Int(None, 4).rc(),
                                Expr::Apply(
                                    None,
                                    Expr::Apply(
                                        None,
                                        Expr::EnvRef(
                                            None,
                                            "add".to_string()
                                        )
                                        .rc(),
                                        Expr::Unit(None).rc()
                                    )
                                    .rc(),
                                    Expr::Int(None, 456).rc()
                                )
                                .rc()
                            )
                            .rc()
                        )
                        .rc()
                    )
                    .rc()
                )
                .rc()
            )
            .rc()
        )
        .rc()
    );

    let r = Define::ExprDef("a".to_string(), None, e);
    let r = Some(r);

    let seq = "def a = \
            let a = 123, \
                b = \
                let x = i -> j -> k, \
                    y = a \
                in let z = () in a, \
                d = neg 1 \
            in \
            let e = 6, k = () in \
            let m = (), n = 4 in \
            add () 456";

    assert_eq!(f(seq), r)
}
