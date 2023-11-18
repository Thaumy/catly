use std::rc::Rc;

use crate::eval::env::parse_to_env;
use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr::eval_expr;
use crate::eval::std::std_code;
use crate::infra::RcAnyExt;

fn gen_env<'t>() -> (TypeEnv<'t>, Rc<ExprEnv>) {
    let seq = std_code().to_owned() +
        "
        # 1
        def find1: Int -> IntList -> Bool =
            n -> list ->
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if eq head n then
                        true
                    else
                        find1 n tail
                | (_: EmptyList) -> false
        # 2
        def find2 =
            # 对于 list 的类型标注是必须的, 因为 SumType 是不封闭的
            # 分支中的 Bool 标注同理
            n -> (list: IntList) -> (
                match list with
                | ({ head = head, tail = tail }: IntCons) ->
                    if eq head n then
                        true
                    else
                        find2 n tail
                | (_: EmptyList) -> false
            ): Bool

        def intList = intCons 1 (intCons 2 (intCons 3 (intCons 4 emptyList)))

        def evalFind1 = find1 3 intList
        def r1 = true
        def evalFind2 = find2 5 intList
        def r2 = false
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFind1")
        .unwrap();
    let evaluated = eval_expr(&type_env, &eval_env, &ref_expr.rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r1")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.rc());

    assert_eq!(evaluated, r);
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFind2")
        .unwrap();
    let evaluated = eval_expr(&type_env, &eval_env, &ref_expr.rc());

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("r2")
        .unwrap();
    let r = eval_expr(&type_env, &eval_env, &ref_expr.rc());

    assert_eq!(evaluated, r);
}
