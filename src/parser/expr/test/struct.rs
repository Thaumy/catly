use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::expr::test::f;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::{btree_set, int_type, unit_type};

#[test]
fn test_parse_struct_part1() {
    let r = Expr::Struct(None, vec![
        ("a".to_string(), None, Expr::Int(None, 123)),
        (
            "ab".to_string(),
            None,
            Expr::EnvRef(None, "ref".to_string())
        ),
        ("abc".to_string(), None, Expr::Unit(None)),
    ]);
    let r = Some(r);

    let seq = "{ a = 123, ab = ref, abc = () }";
    assert_eq!(f(seq), r);
    let seq = "{ a = 123, ab = ref, abc = (),}";
    assert_eq!(f(seq), r);
    let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())) }))";
    assert_eq!(f(seq), r);
    let seq = "(({ a = (((123))), ab = (((ref))), abc = ((())),}))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_struct_part2() {
    let a = Expr::Struct(None, vec![
        (
            "abc".to_string(),
            None,
            Expr::Struct(None, vec![(
                "efg".to_string(),
                None,
                Expr::Cond(
                    None,
                    Expr::Int(None, 123).boxed(),
                    Expr::Unit(None).boxed(),
                    Expr::Int(None, 0).boxed()
                )
            )])
        ),
        ("x".to_string(), None, Expr::Int(None, 1)),
    ]);
    let fun = Expr::Closure(
        None,
        "x".to_string().some(),
        None,
        Expr::Closure(
            None,
            "y".to_string().some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::EnvRef(None, "x".to_string()).boxed()
                )
                .boxed(),
                Expr::EnvRef(None, "y".to_string()).boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Expr::Struct(None, vec![
        ("a".to_string(), None, a),
        (
            "ab".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).boxed(),
                Expr::Int(None, 1).boxed()
            )
        ),
        ("fun".to_string(), None, fun),
    ]);
    let r = Some(r);

    let seq = "{ \
               a = { abc = { efg = if 123 then () else 0 }, x = 1 }, \
               ab = neg 1, \
               fun = x -> y -> add x y \
             }";
    assert_eq!(f(seq), r);
    let seq = "((({ \
                  a = ((({ abc = { efg = if 123 then ((())) else 0 }, x = 1 }))), \
                  ab = (((neg))) 1, \
                  fun = (x -> y -> add x y) \
            })))";
    assert_eq!(f(seq), r);
    let seq = "((({ \
                  (((a))) = ((({ abc = { efg = if (((123))) then ((())) else 0 }, x = (((1))) }))), \
                  (((ab))) = ((((((neg))) (((1)))))), \
                  (((fun))) = (x -> (((y -> add x y)))) \
            })))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_struct_part3() {
    let a = Expr::Struct(None, vec![
        (
            "abc".to_string(),
            int_type!().some(),
            Expr::Struct(None, vec![(
                "efg".to_string(),
                None,
                Expr::Cond(
                    None,
                    Expr::Int(None, 123).boxed(),
                    Expr::Unit(None).boxed(),
                    Expr::Int(None, 0).boxed()
                )
            )])
        ),
        ("x".to_string(), None, Expr::Int(None, 1)),
    ]);
    let fun = Expr::Closure(
        None,
        "x".to_string().some(),
        None,
        Expr::Closure(
            None,
            "y".to_string().some(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::EnvRef(None, "x".to_string()).boxed()
                )
                .boxed(),
                Expr::EnvRef(None, "y".to_string()).boxed()
            )
            .boxed()
        )
        .boxed()
    );
    let r = Expr::Struct(int_type!().some(), vec![
        ("a".to_string(), int_type!().some(), a),
        (
            "ab".to_string(),
            Type::ClosureType(
                int_type!().boxed(),
                int_type!().boxed()
            )
            .some(),
            Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).boxed(),
                Expr::Int(None, 1).boxed()
            )
        ),
        (
            "fun".to_string(),
            Type::ProdType(vec![
                ("a".to_string(), int_type!()),
                ("b".to_string(), unit_type!()),
            ])
            .some(),
            fun
        ),
        (
            "y".to_string(),
            Type::ProdType(vec![(
                "a".to_string(),
                Type::ProdType(vec![
                    ("a".to_string(), int_type!()),
                    ("b".to_string(), unit_type!()),
                ])
            )])
            .some(),
            Expr::Int(None, 0)
        ),
    ]);
    let r = Some(r);

    let seq = "{ \
           a: Int = { abc: Int = { efg = if 123 then () else 0 }, x = 1 }, \
           ab: Int -> Int = neg 1, \
           fun: { a: Int, b: Unit } = x -> y -> add x y, \
           y: { a: { a: Int, b: Unit,} } = 0 \
         }: Int";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_struct_part4() {
    let ab =
        Type::ProdType(vec![("a".to_string(), int_type!())]).some();

    let cd = Type::ProdType(vec![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!()),
    ])
    .some();

    let ef = Type::ProdType(vec![
        ("a".to_string(), int_type!()),
        ("b".to_string(), int_type!()),
        ("c".to_string(), int_type!()),
    ])
    .some();

    let r = Expr::Struct(None, vec![
        (
            "a".to_string(),
            ab.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "b".to_string(),
            ab.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "c".to_string(),
            cd.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "d".to_string(),
            cd.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "e".to_string(),
            ef.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "f".to_string(),
            ef.clone(),
            Expr::Unit(unit_type!().some())
        ),
        (
            "g".to_string(),
            Type::ProdType(vec![
                ("a".to_string(), int_type!()),
                (
                    "b".to_string(),
                    Type::SumType(btree_set![
                        Type::TypeEnvRef("A".to_string()),
                        Type::TypeEnvRef("B".to_string()),
                    ])
                ),
                (
                    "c".to_string(),
                    Type::SumType(btree_set![
                        Type::TypeEnvRef("A".to_string()),
                        Type::TypeEnvRef("B".to_string()),
                        Type::TypeEnvRef("C".to_string()),
                    ])
                ),
            ])
            .some(),
            Expr::Unit(unit_type!().some())
        ),
    ]);
    let r = Some(r);

    let seq = "{ \
           a: { a: Int } = (): Unit, \
           b: { a: Int,} = (): Unit, \
           c: { a: Int, b: Int } = (): Unit, \
           d: { a: Int, b: Int,} = (): Unit, \
           e: { a: Int, b: Int, c: Int } = (): Unit, \
           f: { a: Int, b: Int, c: Int,} = (): Unit, \
           g: { a: Int, b: A | B, c: A | B | C } = (): Unit, \
        }";
    assert_eq!(f(seq), r);
}
