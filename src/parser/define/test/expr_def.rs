use crate::parser::define::Define;
use crate::parser::define::test::f;
use crate::parser::expr::Expr;
use crate::parser::infra::option::AnyExt;
use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::Type;

#[test]
fn test_parse_expr_def_part1() {
    let r = Define::ExprDef(
        "a".to_string(),
        None,
        Expr::EnvRef(None, "b".to_string()),
    );
    let r = Some(r);

    let seq = "def a = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_expr_def_part2() {
    let r = Define::ExprDef(
        "a".to_string(),
        Type::TypeEnvRef("Int".to_string()).some(),
        Expr::EnvRef(None, "b".to_string()),
    );
    let r = Some(r);

    let seq = "def a: Int = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_expr_def_part3() {
    let r = Define::ExprDef(
        "a".to_string(),
        Type::ClosureType(
            Type::TypeEnvRef("Int".to_string()).boxed(),
            Type::TypeEnvRef("Int".to_string()).boxed(),
        ).some(),
        Expr::EnvRef(None, "b".to_string()),
    );
    let r = Some(r);

    let seq = "def a: Int -> Int = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_expr_def_part4() {
    let e = Expr::Let(
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
                        Expr::EnvRef(None, "k".to_string()).boxed(),
                    ).boxed(),
                ).boxed(),
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
                        Expr::EnvRef(None, "a".to_string()).boxed(),
                    ).boxed(),
                ).boxed(),
            ).boxed(),
            Expr::Let(
                None,
                "d".to_string(),
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "neg".to_string()).boxed(),
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
                                        Expr::EnvRef(None, "add".to_string()).boxed(),
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

    let r = Define::ExprDef("a".to_string(), None, e);
    let r = Some(r);


    let seq =
        "def a = \
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
