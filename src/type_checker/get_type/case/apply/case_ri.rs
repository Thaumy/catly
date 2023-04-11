use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::{
    GetTypeReturn,
    RequireInfo
};

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    require_info: RequireInfo,
    expect_type: &MaybeType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> GetTypeReturn {
    match lhs_expr {
        // 完全没有附加类型信息的 Closure
        Expr::Closure(None, i_n, None, o_e) => match expect_type {
            // 可以确定输出类型, 输入输出都能被 hint
            Some(expect_output_type) =>
                match get_type(type_env, expr_env, rhs_expr) {
                    Quad::L(rhs_expr_type) => {
                        let closure_type = Type::ClosureType(
                            rhs_expr_type.clone().boxed(),
                            expect_output_type
                                .clone()
                                .boxed()
                        );
                        let lhs_expr = Expr::Closure(
                            closure_type.some(),
                            i_n.clone(),
                            rhs_expr_type.clone().some(),
                            o_e.clone()
                        );
                        get_type(type_env, expr_env, &lhs_expr)
                    }
                    // 约束将在调用 get_type 时被传播, 所以无需处理
                    Quad::ML(rc) => {
                        let closure_type = Type::ClosureType(
                            rc.r#type.clone().boxed(),
                            expect_output_type
                                .clone()
                                .boxed()
                        );
                        let lhs_expr = Expr::Closure(
                            closure_type.some(),
                            i_n.clone(),
                            rc.r#type.some(),
                            o_e.clone()
                        );
                        get_type(type_env, expr_env, &lhs_expr)
                    }
                    // 信息不足以获得 rhs_expr_type
                    mr_r => mr_r
                },
            // 无法确定输出类型, 仅对输入类型 hint
            None => match get_type(type_env, expr_env, rhs_expr) {
                Quad::L(rhs_expr_type) => {
                    let lhs_expr = Expr::Closure(
                        None,
                        i_n.clone(),
                        rhs_expr_type.clone().some(),
                        o_e.clone()
                    );
                    get_type(type_env, expr_env, &lhs_expr)
                }
                // 约束将在调用 get_type 时被传播, 所以无需处理
                Quad::ML(rc) => {
                    let lhs_expr = Expr::Closure(
                        None,
                        i_n.clone(),
                        rc.r#type.some(),
                        o_e.clone()
                    );
                    get_type(type_env, expr_env, &lhs_expr)
                }
                // 信息不足以获得 rhs_expr_type
                mr_r => mr_r
            }
        },
        // 附加有输入类型信息的 Closure
        Expr::Closure(None, i_n, Some(i_t), o_e) => match expect_type
        {
            // 可以 hint 输出类型
            Some(expect_output_type) => {
                let closure_type = Type::ClosureType(
                    i_t.clone().boxed(),
                    expect_output_type
                        .clone()
                        .boxed()
                );
                let lhs_expr = Expr::Closure(
                    closure_type.some(),
                    i_n.clone(),
                    i_t.clone().some(),
                    o_e.clone()
                );
                get_type(type_env, expr_env, &lhs_expr)
            }
            // 无法确定输出类型
            // 由于在输入类型已被确定的情况下仍不能获得 lhs_expr_type, 所以此时已经不能继续推导了
            None => Quad::MR(require_info) // 返回原错误
        },
        _ => Quad::MR(require_info)
    }
}
