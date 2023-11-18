use std::rc::Rc;

use crate::eval::env::expr_env::{ExprEnv, ExprEnvEntry};
use crate::eval::env::type_env::TypeEnv;
use crate::eval::r#type::{Expr, StructField};
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;

fn is_struct_match_pattern_then_env_vec(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    struct_vec: &Vec<StructField>,
    pattern_vec: &Vec<StructField>
) -> Option<Vec<ExprEnvEntry>> {
    if struct_vec.len() != pattern_vec.len() {
        return None;
    }

    let collected: Option<Vec<Vec<ExprEnvEntry>>> = struct_vec
        .iter()
        .zip(pattern_vec.iter())
        .map(|((s_n, s_t, s_e), (p_n, p_t, p_e))| {
            if (s_n, s_t) == (p_n, p_t) {
                is_expr_match_pattern_then_env_vec(
                    type_env,
                    expr_env,
                    &s_e.clone().rc(),
                    p_e
                )
            } else {
                None
            }
        })
        .try_collect();

    (collected?)
        .concat()
        .wrap_some()
}

fn is_expr_match_pattern_then_env_vec(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    evaluated_expr: &Rc<Expr>,
    pattern: &Expr
) -> Option<Vec<ExprEnvEntry>> {
    // 进行类型相容性测试
    if !type_env.can_lift_to(
        &evaluated_expr.get_type_annot()?,
        &pattern.get_type_annot()?
    ) {
        return None;
    }

    match evaluated_expr.as_ref() {
        Expr::Unit(_) => match pattern {
            Expr::Unit(_) => vec![].wrap_some(),
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr
                    .clone()
                    .wrap_some(),
                expr_env.clone().wrap_some()
            )]
            .wrap_some(),
            Expr::Discard(_) => vec![].wrap_some(),
            _ => unreachable!()
        },
        Expr::Int(_, e_i) => match pattern {
            Expr::Int(_, p_i) =>
                if e_i == p_i {
                    vec![].wrap_some()
                } else {
                    None
                },
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr
                    .clone()
                    .wrap_some(),
                expr_env.clone().wrap_some()
            )]
            .wrap_some(),
            Expr::Discard(_) => vec![].wrap_some(),
            _ => unreachable!()
        },
        Expr::Closure(..) => match pattern {
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr
                    .clone()
                    .wrap_some(),
                expr_env.clone().wrap_some()
            )]
            .wrap_some(),
            Expr::Discard(_) => vec![].wrap_some(),
            _ => unreachable!()
        },
        Expr::Struct(_, e_s_v) => match pattern {
            Expr::Struct(_, p_s_v) =>
                is_struct_match_pattern_then_env_vec(
                    type_env, expr_env, e_s_v, p_s_v
                ),
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr
                    .clone()
                    .wrap_some(),
                expr_env.clone().wrap_some()
            )]
            .wrap_some(),
            Expr::Discard(_) => vec![].wrap_some(),
            _ => unreachable!()
        },
        _ => unreachable!()
    }
}

// 如果 expr 匹配 pattern, 则返回经由(按需)扩展的表达式环境
pub fn is_expr_match_pattern_then_env(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    evaluated_expr: &Rc<Expr>,
    pattern: &Rc<Expr>
) -> Option<Rc<ExprEnv>> {
    let expr_env_vec = is_expr_match_pattern_then_env_vec(
        type_env,
        expr_env,
        evaluated_expr,
        pattern
    )?;

    expr_env
        .extend_vec_new(expr_env_vec)
        .wrap_some()
}
