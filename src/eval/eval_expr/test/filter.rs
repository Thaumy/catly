use std::rc::Rc;

use crate::eval::env::parse_to_env;
use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::std::std_code;
use crate::infra::WrapRc;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
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
        def r1 = intCons 3 (intCons 4 emptyList)
        def evalFilter2 = filter2 (gt 3) intList
        def r2 = intCons 1 (intCons 2 emptyList)
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFilter1")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r1")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFilter2")
        .unwrap();
    let evaluated =
        eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r2")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.wrap_rc());

    assert_eq!(evaluated, r);
}
