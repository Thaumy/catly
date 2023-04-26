use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#fn::has_type;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::test::integration::get_std_code;
use crate::infer::infer_type::test::parse_env;
use crate::{bool_type, closure_type, int_type};

fn gen_env<'t>() -> (TypeEnv<'t>, ExprEnv<'t>) {
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
        ";
    parse_env(&seq)
}

fn target_type() -> InferTypeRet {
    has_type(closure_type!(
        int_type!(),
        closure_type!(
            Type::NamelyType("IntList".to_string()),
            bool_type!()
        )
    ))
}

#[test]
fn test_part1() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("find1")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}

#[test]
fn test_part2() {
    let (type_env, expr_env) = gen_env();

    let expr_type = expr_env
        .get_ref("find2")
        .unwrap()
        .infer_type(&type_env, &expr_env);

    assert_eq!(expr_type, target_type())
}
