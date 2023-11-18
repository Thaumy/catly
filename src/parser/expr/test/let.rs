use crate::infer::env::int_type;
use crate::infer::env::unit_type;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::expr::test::f;

#[test]
fn test_part1() {
    let r = Expr::Let(
        None,
        false,
        "a".to_string(),
        None,
        Expr::Int(None, 123).rc(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).rc(),
                Expr::EnvRef(None, "a".to_string()).rc()
            )
            .rc(),
            Expr::Int(None, 456).rc()
        )
        .rc()
    );
    let r = Some(r);

    let seq = "let a = 123 in add a 456";
    assert_eq!(f(seq), r);
    let seq = "let a = 123,in add a 456";
    assert_eq!(f(seq), r);
    let seq =
        "(((let (((a))) = (((123))) in (((add a (((456)))))))))";
    assert_eq!(f(seq), r);
    let seq =
        "(((let (((a))) = (((123))),in (((add a (((456)))))))))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part2() {
    let r = Expr::Let(
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
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).rc(),
                    Expr::EnvRef(None, "c".to_string()).rc()
                )
                .rc(),
                Expr::EnvRef(None, "d".to_string()).rc()
            )
            .rc(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).rc(),
                    Expr::Unit(None).rc()
                )
                .rc(),
                Expr::Int(None, 456).rc()
            )
            .rc()
        )
        .rc()
    );
    let r = Some(r);

    let seq = "let a = 123, b = add c d in add () 456";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part3() {
    let r = Expr::Let(
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
    let r = Some(r);

    let seq = "let a = 123, \
             b = \
             let x = i -> j -> k, \
                 y = a \
             in let z = () in a, \
             d = neg 1 \
         in \
         let e = 6, k = () in \
         let m = (), n = 4 in \
         add () 456";
    assert_eq!(f(seq), r);
    let seq = "let a = (((123))), \
             b = \
             (((\
                 let x = ((((((i))) -> ((((((j))) -> (((k))))))))), \
                     y = (((a))) \
                 in (((\
                    let (((z))) = (((()))) in (((a)))\
                    )))\
             ))), \
             (((d))) = \
                 (((\
                     (((neg))) (((1)))\
                 ))) \
         in \
         (((\
         let (((e))) = (((6))), (((k))) = (((()))) in \
             (((\
             let (((m))) = (((()))), (((n))) = (((4))) in \
                 (((\
                 add () (((456)))\
                 )))\
             )))\
         )))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_part4() {
    let r = Expr::Let(
        int_type!().wrap_some(),
        false,
        "a".to_string(),
        int_type!().wrap_some(),
        Expr::Int(None, 123).rc(),
        Expr::Let(
            None,
            false,
            "b".to_string(),
            int_type!().wrap_some(),
            Expr::Let(
                int_type!().wrap_some(),
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
                int_type!().wrap_some(),
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).rc(),
                    Expr::Int(None, 1).rc()
                )
                .rc(),
                Expr::Let(
                    int_type!().wrap_some(),
                    false,
                    "e".to_string(),
                    None,
                    Expr::Int(None, 6).rc(),
                    Expr::Let(
                        None,
                        false,
                        "k".to_string(),
                        None,
                        Expr::Unit(unit_type!().wrap_some()).rc(),
                        Expr::Let(
                            None,
                            false,
                            "m".to_string(),
                            unit_type!().wrap_some(),
                            Expr::Unit(unit_type!().wrap_some()).rc(),
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
    let r = Some(r);

    let seq = "(let a: Int = 123, \
             b: Int = \
             (let x = i -> j -> k, \
                 y = a \
             in let z = () in a): Int, \
             d: Int = neg 1 \
         in \
         (let e = 6, k = (): Unit in \
         let m: Unit = (): Unit, n = 4 in \
         add () 456): Int): Int";
    assert_eq!(f(seq), r);
}
