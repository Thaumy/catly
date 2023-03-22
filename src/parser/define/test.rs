use crate::parser::define::{Define, parse_define};
use crate::parser::expr::Expr;
use crate::parser::infra::BoxExt;
use crate::parser::preprocess::blank::preprocess_blank;
use crate::parser::preprocess::comment::preprocess_comment;
use crate::parser::preprocess::keyword::preprocess_keyword;
use crate::parser::r#type::Type;

fn f(seq: &str) -> Option<Define> {
    let seq = preprocess_comment(seq);
    let seq = preprocess_blank(&seq);
    let seq = preprocess_keyword(&seq);
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
        ("uuu".to_string(), Type::UnitType),
        ("intList".to_string(), Type::TypeApply(
            Type::TypeEnvRef("List".to_string()).boxed(),
            Type::IntType.boxed(),
        ))]);

    let r = Define::TypeDef("A".to_string(), t);
    let r = Some(r);

    let seq = "type A = { abc: A, uuu: Unit, intList: List Int }";
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
        "a".to_string(),
        Expr::Int(123).boxed(),
        Expr::Let(
            "b".to_string(),
            Expr::Let("x".to_string(),
                      Expr::Closure(
                          "i".to_string(),
                          Expr::Closure(
                              "j".to_string(),
                              Expr::EnvRef("k".to_string()).boxed(),
                          ).boxed(),
                      ).boxed(),
                      Expr::Let(
                          "y".to_string(),
                          Expr::EnvRef("a".to_string()).boxed(),
                          Expr::Let(
                              "z".to_string(),
                              Expr::Unit.boxed(),
                              Expr::EnvRef("a".to_string()).boxed(),
                          ).boxed(),
                      ).boxed(),
            ).boxed(),
            Expr::Let(
                "d".to_string(),
                Expr::Apply(
                    Expr::EnvRef("neg".to_string()).boxed(),
                    Expr::Int(1).boxed(),
                ).boxed(),
                Expr::Let(
                    "e".to_string(),
                    Expr::Int(6).boxed(),
                    Expr::Let(
                        "k".to_string(),
                        Expr::Unit.boxed(),
                        Expr::Let(
                            "m".to_string(),
                            Expr::Unit.boxed(),
                            Expr::Let(
                                "n".to_string(),
                                Expr::Int(4).boxed(),
                                Expr::Apply(
                                    Expr::Apply(
                                        Expr::EnvRef("add".to_string()).boxed(),
                                        Expr::Unit.boxed(),
                                    ).boxed(),
                                    Expr::Int(456).boxed(),
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
