use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::r#box::Ext;

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
                Expr::EnvRef("add".to_string()).boxed(),
                Expr::EnvRef("a".to_string()).boxed(),
            ).boxed(),
            Expr::Int(None, 456).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq = "let a = 123 in add a 456";
    assert_eq!(f(seq), r);
    let seq = "let a = 123,in add a 456";
    assert_eq!(f(seq), r);
    let seq = "(((let (((a))) = (((123))) in (((add a (((456)))))))))";
    assert_eq!(f(seq), r);
    let seq = "(((let (((a))) = (((123))),in (((add a (((456)))))))))";
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
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::EnvRef("c".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef("d".to_string()).boxed(),
            ).boxed(),
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("add".to_string()).boxed(),
                    Expr::Unit(None).boxed(),
                ).boxed(),
                Expr::Int(None, 456).boxed(),
            ).boxed(),
        ).boxed(),
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
                    "i".to_string(),
                    None,
                    Expr::Closure(
                        None,
                        "j".to_string(),
                        None,
                        Expr::EnvRef("k".to_string()).boxed(),
                    ).boxed(),
                ).boxed(),
                Expr::Let(
                    None,
                    "y".to_string(),
                    None,
                    Expr::EnvRef("a".to_string()).boxed(),
                    Expr::Let(
                        None,
                        "z".to_string(),
                        None,
                        Expr::Unit(None).boxed(),
                        Expr::EnvRef("a".to_string()).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
            Expr::Let(
                None,
                "d".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef("neg".to_string()).boxed(),
                    Expr::Int(None, 1).boxed(),
                ).boxed(),
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
                                        Expr::EnvRef("add".to_string()).boxed(),
                                        Expr::Unit(None).boxed(),
                                    ).boxed(),
                                    Expr::Int(None, 456).boxed(),
                                ).boxed(),
                            ).boxed(),
                        ).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Some(r);

    let seq =
        "let a = 123, \
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
    let seq =
        "let a = (((123))), \
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
