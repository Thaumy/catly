use crate::btree_set;
use crate::parser::expr::Expr;
use crate::parser::expr::test::f;
use crate::parser::infra::option::AnyExt;
use crate::parser::infra::r#box::Ext;
use crate::parser::r#type::Type;

#[test]
fn test_parse_struct_part1() {
    let r = Expr::Struct(
        None,
        vec![
            ("a".to_string(), None, Expr::Int(None, 123)),
            ("ab".to_string(), None, Expr::EnvRef(None, "ref".to_string())),
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
    let a = Expr::Struct(
        None,
        vec![
            ("abc".to_string(),
             None,
             Expr::Struct(
                 None,
                 vec![
                     ("efg".to_string(),
                      None,
                      Expr::Cond(
                          None,
                          Expr::Int(None, 123).boxed(),
                          Expr::Unit(None).boxed(),
                          Expr::Int(None, 0).boxed(),
                      ))
                 ])),
            ("x".to_string(), None, Expr::Int(None, 1)),
        ]);
    let fun = Expr::Closure(
        None,
        "x".to_string(),
        None,
        Expr::Closure(
            None,
            "y".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::EnvRef(None, "x".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef(None, "y".to_string()).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Expr::Struct(
        None,
        vec![
            ("a".to_string(), None, a),
            ("ab".to_string(), None, Expr::Apply(
                None,
                Expr::EnvRef(None, "neg".to_string()).boxed(),
                Expr::Int(None, 1).boxed(),
            )),
            ("fun".to_string(), None, fun),
        ]);
    let r = Some(r);

    let seq =
        "{ \
               a = { abc = { efg = if 123 then () else 0 }, x = 1 }, \
               ab = neg 1, \
               fun = x -> y -> add x y \
             }";
    assert_eq!(f(seq), r);
    let seq =
        "((({ \
                  a = ((({ abc = { efg = if 123 then ((())) else 0 }, x = 1 }))), \
                  ab = (((neg))) 1, \
                  fun = (x -> y -> add x y) \
            })))";
    assert_eq!(f(seq), r);
    let seq =
        "((({ \
                  (((a))) = ((({ abc = { efg = if (((123))) then ((())) else 0 }, x = (((1))) }))), \
                  (((ab))) = ((((((neg))) (((1)))))), \
                  (((fun))) = (x -> (((y -> add x y)))) \
            })))";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_struct_part3() {
    let a = Expr::Struct(
        None,
        vec![
            ("abc".to_string(),
             Type::TypeEnvRef("Int".to_string()).some(),
             Expr::Struct(
                 None,
                 vec![
                     ("efg".to_string(),
                      None,
                      Expr::Cond(
                          None,
                          Expr::Int(None, 123).boxed(),
                          Expr::Unit(None).boxed(),
                          Expr::Int(None, 0).boxed(),
                      ))
                 ])),
            ("x".to_string(), None, Expr::Int(None, 1)),
        ]);
    let fun = Expr::Closure(
        None,
        "x".to_string(),
        None,
        Expr::Closure(
            None,
            "y".to_string(),
            None,
            Expr::Apply(
                None,
                Expr::Apply(
                    None,
                    Expr::EnvRef(None, "add".to_string()).boxed(),
                    Expr::EnvRef(None, "x".to_string()).boxed(),
                ).boxed(),
                Expr::EnvRef(None, "y".to_string()).boxed(),
            ).boxed(),
        ).boxed(),
    );
    let r = Expr::Struct(
        Type::TypeEnvRef("Int".to_string()).some(),
        vec![
            ("a".to_string(),
             Type::TypeEnvRef("Int".to_string()).some(),
             a),
            ("ab".to_string(),
             Type::ClosureType(
                 Type::TypeEnvRef("Int".to_string()).boxed(),
                 Type::TypeEnvRef("Int".to_string()).boxed(),
             ).some(),
             Expr::Apply(
                 None,
                 Expr::EnvRef(None, "neg".to_string()).boxed(),
                 Expr::Int(None, 1).boxed(),
             )),
            ("fun".to_string(),
             Type::ProductType(vec![
                 ("a".to_string(),
                  Type::TypeEnvRef("Int".to_string())),
                 ("b".to_string(),
                  Type::TypeEnvRef("Unit".to_string())),
             ]).some(),
             fun),
            ("y".to_string(),
             Type::ProductType(vec![
                 ("a".to_string(),
                  Type::ProductType(vec![
                      ("a".to_string(),
                       Type::TypeEnvRef("Int".to_string())),
                      ("b".to_string(),
                       Type::TypeEnvRef("Unit".to_string())),
                  ])),
             ]).some(),
             Expr::Int(None, 0)),
        ]);
    let r = Some(r);

    let seq =
        "{ \
           a: Int = { abc: Int = { efg = if 123 then () else 0 }, x = 1 }, \
           ab: Int -> Int = neg 1, \
           fun: { a: Int, b: Unit } = x -> y -> add x y, \
           y: { a: { a: Int, b: Unit,} } = 0 \
         }: Int";
    assert_eq!(f(seq), r);
}

#[test]
fn test_parse_struct_part4() {
    let ab = Type::ProductType(vec![
        ("a".to_string(),
         Type::TypeEnvRef("Int".to_string())),
    ]).some();

    let cd = Type::ProductType(vec![
        ("a".to_string(),
         Type::TypeEnvRef("Int".to_string())),
        ("b".to_string(),
         Type::TypeEnvRef("Int".to_string())),
    ]).some();

    let ef = Type::ProductType(vec![
        ("a".to_string(),
         Type::TypeEnvRef("Int".to_string())),
        ("b".to_string(),
         Type::TypeEnvRef("Int".to_string())),
        ("c".to_string(),
         Type::TypeEnvRef("Int".to_string())),
    ]).some();

    let r = Expr::Struct(
        None,
        vec![
            ("a".to_string(),
             ab.clone(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
            ("b".to_string(),
             ab.clone(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
            ("c".to_string(),
             cd.clone(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
            ("d".to_string(),
             cd.clone(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
            ("e".to_string(),
             ef.clone(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
            ("f".to_string(),
             ef.clone(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
            ("g".to_string(),
             Type::ProductType(vec![
                 ("a".to_string(),
                  Type::TypeEnvRef("Int".to_string())),
                 ("b".to_string(),
                  Type::SumType(btree_set![
                    Type::TypeEnvRef("A".to_string()),
                    Type::TypeEnvRef("B".to_string()),
                  ])),
                 ("c".to_string(),
                  Type::SumType(btree_set![
                    Type::TypeEnvRef("A".to_string()),
                    Type::TypeEnvRef("B".to_string()),
                    Type::TypeEnvRef("C".to_string()),
                  ])),
             ]).some(),
             Expr::Unit(Type::TypeEnvRef("Unit".to_string()).some())),
        ]);
    let r = Some(r);

    let seq =
        "{ \
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
