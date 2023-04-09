mod case_t_rc;
mod r#fn;

use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
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
        // 无法获取 match_expr 类型信息, 启用旁路推导
        Quad::MR(_) => {
            // 当 case_expr_type 能够合一为某个类型时, 这个类型与 match_expr 将直接相关
            // 此时以该类型为 hint 求 match 表达式类型

            // 当 case_expr_type 不能合一时, 如果 match_expr 是 EnvRef
            // 那么在求 then_expr 时可能对产生针对 match_expr 的类型约束
            // 以合一后的约束目标为 hint 求 match 表达式类型
            todo!()
        }
        // 类型不相容
        r => r.clone()
    }
}
