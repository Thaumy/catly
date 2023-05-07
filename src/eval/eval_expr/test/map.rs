use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::parse_to_env;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::eval_expr::test::get_std_code;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::result::ResultAnyExt;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv) {
    let seq = get_std_code() +
        "
        # 1
        def map1: (Int -> Int) -> IntList -> IntList =
            f -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map1 f tail)
                | (_: EmptyList) -> emptyList
        # 2
        def map2 =
            # 对于 list 的类型标注是必须的, 因为 SumType 是不封闭的
            # 分支中的 IntList 标注同理
            f -> (list: IntList) -> (
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    intCons (f head) (map2 f tail)
                | (_: EmptyList) -> emptyList
            ): IntList

        def intList = intCons 1 (intCons 2 (intCons 3 (intCons 4 emptyList)))

        def evalMap1 = map1 (add 1) intList
        def evalMap2 = map2 (add 1) intList
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalMap1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Struct(namely_type!("IntList"), vec![
        (
            "h".to_string(),
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 2)
        ),
        (
            "t".to_string(),
            namely_type!("IntList"),
            Expr::Struct(namely_type!("IntList"), vec![
                (
                    "h".to_string(),
                    namely_type!("Int"),
                    Expr::Int(namely_type!("Int"), 3)
                ),
                (
                    "t".to_string(),
                    namely_type!("IntList"),
                    Expr::Struct(namely_type!("IntList"), vec![
                        (
                            "h".to_string(),
                            namely_type!("Int"),
                            Expr::Int(namely_type!("Int"), 4)
                        ),
                        (
                            "t".to_string(),
                            namely_type!("Int"),
                            Expr::Struct(
                                namely_type!("IntList"),
                                vec![
                                    (
                                        "h".to_string(),
                                        namely_type!("Int"),
                                        Expr::Int(
                                            namely_type!("Int"),
                                            5
                                        )
                                    ),
                                    (
                                        "t".to_string(),
                                        namely_type!("IntList"),
                                        Expr::Unit(namely_type!(
                                            "EmptyList"
                                        ))
                                    ),
                                ]
                            )
                        ),
                    ])
                ),
            ])
        ),
    ]);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalMap2")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Struct(namely_type!("IntList"), vec![
        (
            "h".to_string(),
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 2)
        ),
        (
            "t".to_string(),
            namely_type!("IntList"),
            Expr::Struct(namely_type!("IntList"), vec![
                (
                    "h".to_string(),
                    namely_type!("Int"),
                    Expr::Int(namely_type!("Int"), 3)
                ),
                (
                    "t".to_string(),
                    namely_type!("IntList"),
                    Expr::Struct(namely_type!("IntList"), vec![
                        (
                            "h".to_string(),
                            namely_type!("Int"),
                            Expr::Int(namely_type!("Int"), 4)
                        ),
                        (
                            "t".to_string(),
                            namely_type!("Int"),
                            Expr::Struct(
                                namely_type!("IntList"),
                                vec![
                                    (
                                        "h".to_string(),
                                        namely_type!("Int"),
                                        Expr::Int(
                                            namely_type!("Int"),
                                            5
                                        )
                                    ),
                                    (
                                        "t".to_string(),
                                        namely_type!("IntList"),
                                        Expr::Unit(namely_type!(
                                            "EmptyList"
                                        ))
                                    ),
                                ]
                            )
                        ),
                    ])
                ),
            ])
        ),
    ]);

    assert_eq!(evaluated, r.ok());
}
