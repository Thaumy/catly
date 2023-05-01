use crate::eval::env::expr_env::{EnvEntry, ExprEnv};
use crate::eval::r#type::expr::{Expr, StructField};
use crate::infra::option::OptionAnyExt;

fn is_struct_match_pattern_then_env_vec<'t>(
    expr_env: &'t ExprEnv<'t>,
    struct_vec: &Vec<StructField>,
    pattern_vec: &Vec<StructField>
) -> Option<Vec<EnvEntry<'t>>> {
    if struct_vec.len() != pattern_vec.len() {
        return None;
    }

    let collected: Option<Vec<Vec<EnvEntry<'t>>>> = struct_vec
        .iter()
        .zip(pattern_vec.iter())
        .map(|((s_n, s_t, s_e), (p_n, p_t, p_e))| {
            if (s_n, s_t) == (p_n, p_t) {
                is_expr_match_pattern_then_env_vec(expr_env, s_e, p_e)
            } else {
                None
            }
        })
        .try_collect();

    (collected?).concat().some()
}

fn is_expr_match_pattern_then_env_vec<'t>(
    expr_env: &'t ExprEnv<'t>,
    evaluated_expr: &Expr,
    pattern: &Expr
) -> Option<Vec<EnvEntry<'t>>> {
    if evaluated_expr.get_type_annot() != pattern.get_type_annot() {
        return None;
    }

    match evaluated_expr {
        Expr::Unit(_) => match pattern {
            Expr::Unit(_) => vec![].some(),
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr.clone(),
                expr_env.clone()
            )]
            .some(),
            Expr::Discard(_) => vec![].some(),
            _ => panic!("Impossible case pattern: {pattern:?}")
        },
        Expr::Int(_, e_i) => match pattern {
            Expr::Int(_, p_i) =>
                if e_i == p_i {
                    vec![].some()
                } else {
                    None
                },
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr.clone(),
                expr_env.clone()
            )]
            .some(),
            Expr::Discard(_) => vec![].some(),
            _ => panic!("Impossible case pattern: {pattern:?}")
        },
        Expr::Closure(..) => match pattern {
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr.clone(),
                expr_env.clone()
            )]
            .some(),
            Expr::Discard(_) => vec![].some(),
            _ => panic!("Impossible case pattern: {pattern:?}")
        },
        Expr::Struct(_, e_s_v) => match pattern {
            Expr::Struct(_, p_s_v) =>
                is_struct_match_pattern_then_env_vec(
                    &expr_env, e_s_v, p_s_v
                ),
            Expr::EnvRef(p_t, ref_name) => vec![(
                ref_name.clone(),
                p_t.clone(),
                evaluated_expr.clone(),
                expr_env.clone()
            )]
            .some(),
            Expr::Discard(_) => vec![].some(),
            _ => panic!("Impossible case pattern: {pattern:?}")
        },
        _ => panic!("Impossible match target: {evaluated_expr:?}")
    }
}

// 如果 expr 匹配 pattern, 则返回经由(按需)扩展的表达式环境
pub fn is_expr_match_pattern_then_env<'t>(
    expr_env: &'t ExprEnv<'t>,
    evaluated_expr: &Expr,
    pattern: &Expr
) -> Option<ExprEnv<'t>> {
    is_expr_match_pattern_then_env_vec(
        expr_env,
        evaluated_expr,
        pattern
    )
    .map(|env_vec| expr_env.extend_vec_new(env_vec))
}
