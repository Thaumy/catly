mod case_ri;
mod case_t_rc;
mod r#fn;

use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::case::r#match::case_ri::case_ri;
use crate::type_checker::get_type::case::r#match::case_t_rc::case_t_rc;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeEnv
};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    match_expr: &Expr,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    let match_expr_type = get_type(type_env, expr_env, match_expr);

    match match_expr_type {
        // L 与 ML 同样只有是否需要传播对外界环境的约束的区别
        Quad::L(_) | Quad::ML(_) => case_t_rc(
            type_env,
            expr_env,
            match_expr_type,
            expect_type,
            vec
        ),
        // 无法获取 match_expr 类型信息, 启用旁路类型推导
        Quad::MR(require_info) => case_ri(
            type_env,
            expr_env,
            require_info,
            expect_type,
            match_expr,
            vec
        ),
        // 类型不相容
        r => r.clone()
    }
}
