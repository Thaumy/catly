mod r#fn;
/*#[cfg(test)]
mod test;
*/
use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::r#struct::r#fn::is_struct_vec_of_type_then_get_prod_vec;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::infra::result::ResultAnyExt;
use crate::parser::expr::r#type::{Expr, StructField};
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    struct_vec: &Vec<StructField>
) -> InferTypeRet {
    // 解构 expect_type 并判断与 vec 的相容性
    let prod_vec = match is_struct_vec_of_type_then_get_prod_vec(
        type_env,
        expect_type,
        struct_vec
    ) {
        // prod_vec 存在当且仅当 expect_type 存在
        Ok(prod_vec) => prod_vec,
        // 当 expect_type 解构异常或与 struct_vec 不匹配时, 产生错误
        Err(e) => return e
    };

    // 不进行层次约束共享的原因和 match case 相同
    let sf_n_and_sf_t = match prod_vec {
        // expect_type 存在且可被解构, 对于其每一个字段类型, 都用作对应表达式的次要类型提示
        Some(prod_vec) =>
            prod_vec
                .iter()
                .zip(struct_vec.iter())
                // pf: Prod field
                // sf: Struct field
                .map(|((_, pf_t), (sf_n, sf_t, sf_e))| {
                    (
                        sf_n.to_string(),
                        // 提示后推导
                        sf_e.with_opt_fallback_type(sf_t)
                            .with_fallback_type(pf_t)
                            .infer_type(type_env, &expr_env)
                    )
                })
                .collect(): Vec<_>,
        // expect_type 不存在, 仅使用 vec 自身的类型对表达式进行提示
        None =>
            struct_vec
                .iter()
                .map(|(sf_n, sf_t, sf_e)| {
                    (
                        sf_n.to_string(),
                        // 提示后推导
                        sf_e.with_opt_fallback_type(sf_t)
                            .infer_type(type_env, &expr_env)
                    )
                })
                .collect(): Vec<_>,
    }
    .into_iter();

    // 一旦发现类型不匹配(of struct field expr), 立即返回
    match sf_n_and_sf_t
        .clone()
        // 任选一个错误即可(渐进式错误提示)
        .find(|(_, x)| matches!(x, Quad::R(_)))
    {
        Some((_, type_miss_match)) => return type_miss_match,
        _ => {}
    } // 排除了 infer_type 的结果 R

    let sf_n_and_sf_t_with_constraint_and_expr = sf_n_and_sf_t
        .clone()
        .map(|(sf_n, sf_t)| match sf_t {
            Quad::L(_) | Quad::ML(_) => {
                let (sf_t, constraint, typed_sf_e) =
                    sf_t.unwrap_type_constraint_expr();
                (sf_n, sf_t, constraint, typed_sf_e).ok()
            }
            mr => mr.err()
        });

    let outer_constraint = sf_n_and_sf_t_with_constraint_and_expr
        .clone()
        .try_fold(EnvRefConstraint::empty(), |acc, x| match x {
            Ok((.., c, _)) => match acc.extend_new(c.clone()) {
                Some(acc) => acc.ok(),
                None => TypeMissMatch::of_constraint(&acc, &c).err()
            },
            Err(Quad::MR(ri)) => match acc
                .extend_new(ri.constraint.clone())
            {
                Some(acc) => acc.ok(),
                None =>
                    TypeMissMatch::of_constraint(&acc, &ri.constraint)
                        .err(),
            },
            _ => acc.ok()
        });

    // 如果合并约束时发生冲突, 立即返回
    let outer_constraint = match outer_constraint {
        Ok(c) => c,
        Err(type_miss_match) => return type_miss_match.into()
    };

    // 如果出现缺乏类型信息(of struct field expr), 则将收集到的外部约束传播出去
    match sf_n_and_sf_t_with_constraint_and_expr
        .clone()
        .find(|x| matches!(x, Err(Quad::MR(_))))
    {
        Some(Err(Quad::MR(ri))) =>
            return ReqInfo::of(ri.ref_name, outer_constraint).into(),
        _ => {}
    } // 排除了 infer_type 的结果 MR

    let prod_type = Type::ProdType(
        sf_n_and_sf_t_with_constraint_and_expr
            .clone()
            .filter_map(|x| x.ok())
            .map(|(sf_n, sf_t, ..)| (sf_n, sf_t))
            .collect()
    );

    InferTypeRet::from_auto_lift(
        type_env,
        &prod_type,
        expect_type,
        outer_constraint.some(),
        |t| {
            let typed_struct_vec =
                sf_n_and_sf_t_with_constraint_and_expr
                    .clone()
                    .filter_map(|x| x.ok())
                    .map(|(sf_n, sf_t, _, typed_sf_e)| {
                        (sf_n, sf_t.some(), typed_sf_e)
                    })
                    .collect(): Vec<_>;

            Expr::Struct(t.some(), typed_struct_vec)
        }
    )
}
