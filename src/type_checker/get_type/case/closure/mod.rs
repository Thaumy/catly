mod case_rc;
mod case_t;

use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::case::closure::case_rc::case_rc;
use crate::type_checker::get_type::case::closure::case_t::case_type;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::{
    destruct_type_env_ref,
    inject_to_new_expr_env
};
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeEnv
};
use crate::type_miss_match;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    input_name: &Option<String>,
    input_type: &MaybeType,
    output_expr: &Expr
) -> GetTypeReturn {
    // Destruct t to ClosureType
    let (expect_i_t, expect_o_t) = match expect_type {
        Some(expect_type) =>
            match destruct_type_env_ref(type_env, expect_type) {
                Some(Type::ClosureType(expect_i_t, expect_o_t)) => (
                    expect_i_t.clone().map(|x| *x),
                    expect_o_t.clone().map(|x| *x)
                ),
                _ => return type_miss_match!()
            },
        _ => (None, None)
    };

    // Hint input_type
    let input_type = match input_type {
        None => expect_i_t,
        _ => input_type.clone()
    };

    // Inject parameter to env
    let expr_env = match input_name {
        Some(input_name) =>
            inject_to_new_expr_env(expr_env, input_name, &input_type),
        _ => expr_env.clone()
    };

    // Hint and get output_expr_type
    let output_expr_type = get_type_with_hint(
        type_env,
        &expr_env,
        output_expr,
        &expect_o_t
    );

    // 此处并不将 output_expr_type 与 hint 进行相容性判断
    // 因为这与 Closure 的类型提升规则相同, 稍后的类型提升会进行该工作
    // 而且提前返回带来的性能提升并不显著

    match output_expr_type {
        Quad::L(output_expr_type) => case_type(
            type_env,
            expect_type,
            input_type,
            output_expr_type
        ),
        Quad::ML(rc) =>
            case_rc(rc, type_env, expect_type, input_name, input_type),

        // get_type 不能推导出输出类型(即便进行了类型提示), 或推导错误
        // 推导错误是由类型不匹配导致的, 这种错误无法解决
        // 不能推导出输出类型是由缺乏类型信息导致的
        // 因为 Closure 不存在可以推导输出类型的第二个表达式, 所以不适用于旁路类型推导
        mr_r => mr_r.clone()
    }
}
