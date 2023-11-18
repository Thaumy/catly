mod r#fn;
#[cfg(test)]
mod test;

use std::rc::Rc;

use r#fn::is_struct_vec_of_type_then_get_prod_vec;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::EnvRefConstraint;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::ReqInfo;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::Quad;
use crate::infra::WrapOption;
use crate::infra::WrapResult;
use crate::parser::expr::r#type::{Expr, StructField};
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
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
    let sf_n_and_sf_t: Vec<_> = match prod_vec {
        // expect_type 存在且可被解构, 对于其每一个字段类型, 都用作对应表达式的次要类型提示
        Some(prod_vec) => prod_vec
            .iter()
            .zip(struct_vec.iter())
            // pf: Prod field
            // sf: Struct field
            .map(|((_, pf_t), (sf_n, sf_t, sf_e))| {
                (
                    sf_n.clone(),
                    // 提示后推导
                    sf_e.with_opt_fallback_type(sf_t)
                        .with_fallback_type(pf_t)
                        .infer_type(type_env, expr_env)
                )
            })
            .collect(),
        // expect_type 不存在, 仅使用 vec 自身的类型对表达式进行提示
        None => struct_vec
            .iter()
            .map(|(sf_n, sf_t, sf_e)| {
                (
                    sf_n.clone(),
                    // 提示后推导
                    sf_e.with_opt_fallback_type(sf_t)
                        .infer_type(type_env, expr_env)
                )
            })
            .collect()
    };
    let sf_n_and_sf_t = sf_n_and_sf_t.into_iter();

    // 一旦发现类型不匹配(of struct field expr), 立即返回
    if let Some((_, type_miss_match)) = sf_n_and_sf_t
        .clone()
        // 任选一个错误即可(渐进式错误提示)
        .find(|(_, x)| matches!(x, Quad::R(_)))
    {
        return type_miss_match;
    } // 排除了 infer_type 的结果 R

    let sf_n_and_sf_t_with_constraint_and_expr =
        sf_n_and_sf_t.map(|(sf_n, sf_t)| match sf_t {
            Quad::L(_) | Quad::ML(_) => {
                let (typed_sf_e, constraint) =
                    sf_t.unwrap_expr_constraint();
                let sf_t = typed_sf_e
                    .unwrap_type_annot()
                    .clone();
                (sf_n, sf_t, constraint, typed_sf_e).wrap_ok()
            }
            mr => mr.wrap_err()
        });

    let outer_constraint = sf_n_and_sf_t_with_constraint_and_expr
        .clone()
        .try_fold(EnvRefConstraint::empty(), |acc, x| match x {
            Ok((.., c, _)) => match acc.extend_new(c.clone()) {
                Some(acc) => acc.wrap_ok(),
                None =>
                    TypeMissMatch::of_constraint(&acc, &c).wrap_err(),
            },
            Err(Quad::MR(ri)) => match acc
                .extend_new(ri.constraint.clone())
            {
                Some(acc) => acc.wrap_ok(),
                None =>
                    TypeMissMatch::of_constraint(&acc, &ri.constraint)
                        .wrap_err(),
            },
            _ => acc.wrap_ok()
        });

    // 如果合并约束时发生冲突, 立即返回
    let outer_constraint = match outer_constraint {
        Ok(c) => c,
        Err(type_miss_match) => return type_miss_match.into()
    };

    // 如果出现缺乏类型信息(of struct field expr), 则将收集到的外部约束传播出去
    if let Some(Err(Quad::MR(ri))) =
        sf_n_and_sf_t_with_constraint_and_expr
            .clone()
            .find(|x| matches!(x, Err(Quad::MR(_))))
    {
        return ReqInfo::of(ri.ref_name, outer_constraint).into();
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
        outer_constraint.wrap_some(),
        |t| {
            let typed_struct_vec =
                sf_n_and_sf_t_with_constraint_and_expr
                    .clone()
                    .filter_map(|x| x.ok())
                    .map(|(sf_n, sf_t, _, typed_sf_e)| {
                        (sf_n, sf_t.wrap_some(), typed_sf_e)
                    })
                    .collect();

            Expr::Struct(t.wrap_some(), typed_struct_vec)
        }
    )
}
