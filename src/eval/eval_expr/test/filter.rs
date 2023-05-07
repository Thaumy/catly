use crate::eval::env::expr_env::ExprEnv;
use crate::eval::env::parse_to_env;
use crate::eval::env::type_env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::eval_expr::test::get_std_code;
use crate::eval::r#macro::namely_type;
use crate::eval::r#type::expr::Expr;
use crate::infra::result::ResultAnyExt;

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv) {
    let seq = get_std_code() +
        "
        # 1
        def filter1: (Int -> Bool) -> IntList -> IntList =
            p -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if p head then
                        intCons head (filter1 p tail)
                    else
                        filter1 p tail
                | (_: EmptyList) -> emptyList
        # 2
        def filter2 =
            # 对于 list 的类型标注是必须的, 因为 SumType 是不封闭的
            p -> list ->
                match (list: IntList) with
                | ({ head = head, tail = tail }: IntCons) ->
                    if p head then
                        (intCons head (filter2 p tail)): IntList
                    else
                        filter2 p tail
                | (_: EmptyList) -> emptyList

        def intList = intCons 1 (intCons 2 (intCons 3 (intCons 4 emptyList)))

        def evalFilter1 = filter1 (x -> gt x 2) intList
        def evalFilter2 = filter2 (lt 3) intList
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFilter1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Struct(namely_type!("IntCons"), vec![
        (
            "head".to_string(),
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 3)
        ),
        (
            "tail".to_string(),
            namely_type!("IntList"),
            Expr::Struct(namely_type!("IntCons"), vec![
                (
                    "head".to_string(),
                    namely_type!("Int"),
                    Expr::Int(namely_type!("Int"), 4)
                ),
                (
                    "tail".to_string(),
                    namely_type!("IntList"),
                    Expr::Unit(namely_type!("EmptyList"))
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
        .get_ref_expr_and_env("evalFilter2")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Struct(namely_type!("IntCons"), vec![
        (
            "head".to_string(),
            namely_type!("Int"),
            Expr::Int(namely_type!("Int"), 4)
        ),
        (
            "tail".to_string(),
            namely_type!("IntList"),
            Expr::Unit(namely_type!("EmptyList"))
        ),
    ]);

    assert_eq!(evaluated, r.ok());
}
