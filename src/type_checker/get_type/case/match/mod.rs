mod case_ri;
mod case_t_rc;
mod r#fn;

use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::type_checker::get_type::case::r#match::case_ri::case_ri;
use crate::type_checker::get_type::case::r#match::case_t_rc::case_t_rc;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::GetTypeReturn;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    target_expr: &Expr,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    let target_expr_type = get_type(type_env, expr_env, target_expr);

    match target_expr_type {
        // L 与 ML 同样只有是否需要传播对外界环境的约束的区别
        Quad::L(_) | Quad::ML(_) => case_t_rc(
            type_env,
            expr_env,
            target_expr_type,
            expect_type,
            vec
        ),

        // 无法获取 target_expr 类型信息, 启用旁路类型推导
        // 同样, 为了防止内层环境对外层环境造成跨越优先级的约束, 仅当 target_expr 没有类型标注时才能启用旁路推导
        // 相关讨论参见 let case
        Quad::MR(require_info) if target_expr.is_no_type_annot() =>
            case_ri(
                type_env,
                expr_env,
                require_info,
                expect_type,
                target_expr,
                vec
            ),

        mr_r => mr_r
    }
}
