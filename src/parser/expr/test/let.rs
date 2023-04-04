use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::expr::test::f;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

#[test]
fn test_parse_let_part1() {
    let r = Expr::Let(
        None,
        "a".to_string(),
        None,
        Expr::Int(None, 123).boxed(),
        Expr::Apply(
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "add".to_string()).boxed(),
                Expr::EnvRef(None, "a".to_string()).boxed()
            )
            .boxed(),
            Expr::Int(None, 456).boxed()
        )
        .boxed()
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
fn test_parse_let_part2() {
    let r = Expr::Let(
        None,
        "a".to_string(),
        None,
        Expr::Int(None, 123).boxed(),
        Expr::Let(
            None,
            "b".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::EnvRef(None, "c".to_string()).boxed()
                )
                .boxed(),
                Expr::EnvRef(None, "d".to_string()).boxed()
            )
            .boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::Unit(None).boxed()
                )
                .boxed(),
                Expr::Int(None, 456).boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Some(r);

    let seq = "let a = 123, b = add c d in add () 456";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_let_part3() {
    let r = Expr::Let(
        None,
        "a".to_string(),
        None,
        Expr::Int(None, 123).boxed(),
        Expr::Let(
            None,
            "b".to_string(),
            None,
            Expr::Let(
                None,
                "x".to_string(),
                None,
                Expr::Closure(
                    None,
                    "i".to_string().some(),
                    None,
                    Expr::Closure(
                        None,
                        "j".to_string().some(),
                        None,
                        Expr::EnvRef(None, "k".to_string()).boxed()
                    )
                    .boxed()
                )
                .boxed(),
                Expr::Let(
                    None,
                    "y".to_string(),
                    None,
                    Expr::EnvRef(None, "a".to_string()).boxed(),
                    Expr::Let(
                        None,
                        "z".to_string(),
                        None,
                        Expr::Unit(None).boxed(),
                        Expr::EnvRef(None, "a".to_string()).boxed()
                    )
                    .boxed()
                )
                .boxed()
            )
            .boxed(),
            Expr::Let(
                None,
                "d".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).boxed(),
                    Expr::Int(None, 1).boxed()
                )
                .boxed(),
                Expr::Let(
                    None,
                    "e".to_string(),
                    None,
                    Expr::Int(None, 6).boxed(),
                    Expr::Let(
                        None,
                        "k".to_string(),
                        None,
                        Expr::Unit(None).boxed(),
                        Expr::Let(
                            None,
                            "m".to_string(),
                            None,
                            Expr::Unit(None).boxed(),
                            Expr::Let(
                                None,
                                "n".to_string(),
                                None,
                                Expr::Int(None, 4).boxed(),
                                Expr::Apply(
                                    None,
                                    Expr::Apply(
                                        None,
                                        Expr::EnvRef(
                                            None,
                                            "add".to_string()
                                        )
                                        .boxed(),
                                        Expr::Unit(None).boxed()
                                    )
                                    .boxed(),
                                    Expr::Int(None, 456).boxed()
                                )
                                .boxed()
                            )
                            .boxed()
                        )
                        .boxed()
                    )
                    .boxed()
                )
                .boxed()
            )
            .boxed()
        )
        .boxed()
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
fn test_parse_let_part4() {
    let r = Expr::Let(
        Type::TypeEnvRef("Int".to_string()).some(),
        "a".to_string(),
        Type::TypeEnvRef("Int".to_string()).some(),
        Expr::Int(None, 123).boxed(),
        Expr::Let(
            None,
            "b".to_string(),
            Type::TypeEnvRef("Int".to_string()).some(),
            Expr::Let(
                Type::TypeEnvRef("Int".to_string()).some(),
                "x".to_string(),
                None,
                Expr::Closure(
                    None,
                    "i".to_string().some(),
                    None,
                    Expr::Closure(
                        None,
                        "j".to_string().some(),
                        None,
                        Expr::EnvRef(None, "k".to_string()).boxed()
                    )
                    .boxed()
                )
                .boxed(),
                Expr::Let(
                    None,
                    "y".to_string(),
                    None,
                    Expr::EnvRef(None, "a".to_string()).boxed(),
                    Expr::Let(
                        None,
                        "z".to_string(),
                        None,
                        Expr::Unit(None).boxed(),
                        Expr::EnvRef(None, "a".to_string()).boxed()
                    )
                    .boxed()
                )
                .boxed()
            )
            .boxed(),
            Expr::Let(
                None,
                "d".to_string(),
                Type::TypeEnvRef("Int".to_string()).some(),
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).boxed(),
                    Expr::Int(None, 1).boxed()
                )
                .boxed(),
                Expr::Let(
                    Type::TypeEnvRef("Int".to_string()).some(),
                    "e".to_string(),
                    None,
                    Expr::Int(None, 6).boxed(),
                    Expr::Let(
                        None,
                        "k".to_string(),
                        None,
                        Expr::Unit(
                            Type::TypeEnvRef("Unit".to_string())
                                .some()
                        )
                        .boxed(),
                        Expr::Let(
                            None,
                            "m".to_string(),
                            Type::TypeEnvRef("Unit".to_string())
                                .some(),
                            Expr::Unit(
                                Type::TypeEnvRef("Unit".to_string())
                                    .some()
                            )
                            .boxed(),
                            Expr::Let(
                                None,
                                "n".to_string(),
                                None,
                                Expr::Int(None, 4).boxed(),
                                Expr::Apply(
                                    None,
                                    Expr::Apply(
                                        None,
                                        Expr::EnvRef(
                                            None,
                                            "add".to_string()
                                        )
                                        .boxed(),
                                        Expr::Unit(None).boxed()
                                    )
                                    .boxed(),
                                    Expr::Int(None, 456).boxed()
                                )
                                .boxed()
                            )
                            .boxed()
                        )
                        .boxed()
                    )
                    .boxed()
                )
                .boxed()
            )
            .boxed()
        )
        .boxed()
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
