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

        def evalFind1 = find1 3
        def evalFind2 = find2 5
        ";
    parse_to_env(&seq).unwrap()
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFind1")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("True"), 1);

    assert_eq!(evaluated, r.ok());
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let (ref_expr, eval_env) = expr_env
        .get_ref_expr_and_env("evalFind2")
        .unwrap();
    let evaluated = eval_expr(&type_env, eval_env, &ref_expr);

    let r = Expr::Int(namely_type!("False"), 0);

    assert_eq!(evaluated, r.ok());
}
