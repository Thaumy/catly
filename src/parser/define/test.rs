use crate::parser::define::{Define, parse_define};
use crate::parser::expr::Expr;
use crate::parser::infra::r#box::Ext;
use crate::parser::preprocess::preprocess;
use crate::parser::r#type::Type;

fn f(seq: &str) -> Option<Define> {
    let seq = preprocess(&seq)?;
    parse_define(seq)
}

#[test]
fn test_parse_type_def_part1() {
    let r = Define::TypeDef(
        "A".to_string(),
        Type::TypeEnvRef("B".to_string()),
    );
    let r = Some(r);

    let seq = "type A = B";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_type_def_part2() {
    let t = Type::ProductType(vec![
        ("abc".to_string(), Type::TypeEnvRef("A".to_string())),
        ("uuu".to_string(), Type::TypeEnvRef("Unit".to_string())),
        ("intList".to_string(), Type::TypeEnvRef("IntList".to_string())),
    ]);

    let r = Define::TypeDef("A".to_string(), t);
    let r = Some(r);

    let seq = "type A = { abc: A, uuu: Unit, intList: IntList }";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_expr_def_part1() {
    let r = Define::ExprDef(
        "a".to_string(),
        Expr::EnvRef("b".to_string()),
    );
    let r = Some(r);

    let seq = "def a = b";
    assert_eq!(f(seq), r)
}

#[test]
fn test_parse_expr_def_part2() {
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

    let r = Define::ExprDef("a".to_string(), e);
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
