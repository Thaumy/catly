mod r#fn;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::case::r#struct::r#fn::is_struct_vec_of_type_then_get_prod_vec;
use crate::get_type::get_type;
use crate::get_type::r#fn::with_constraint_lift_or_left;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::infra::result::AnyExt;
use crate::infra::vec::Ext;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;
use crate::{empty_constraint, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    vec: &Vec<(String, MaybeType, Expr)>
) -> GetTypeReturn {
    // 解构 expect_type 并判断与 vec 的相容性
    let prod_vec = match is_struct_vec_of_type_then_get_prod_vec(
        type_env,
        expect_type,
        vec
    ) {
        Ok(x) => x,
        Err(e) => return e
    };

    // 进行类型提示
    let vec: Vec<_> = match prod_vec {
        // expect_type 存在且可被解构, 对于其每一个类型, 都用作对应表达式的次要类型提示
        Some(t_vec) => t_vec
            .iter()
            .zip(vec.iter())
            .map(|((_, t), (v_n, v_t, v_e))| {
                (
                    v_n.to_string(),
                    v_t.clone(),
                    v_e
                        .try_with_fallback_type(v_t)
                        .with_fallback_type(t)
                )
            })
            .collect()
        ,
        // expect_type 不存在, 仅使用 vec 自身的类型对表达式进行提示
        None => vec
            .iter()
            .map(|(n, mt, e)| {
                (
                    n.to_string(),
                    mt.clone(),
                    e
                        .try_with_fallback_type(mt)
                )
            })
            .collect()
    };

    let mut constraint = empty_constraint!();

    // 收集约束
    let vec = vec
        .iter()
        .map(|(n, _, e)| {
            (n.to_string(), get_type(type_env, &expr_env, e))
        })
        .map(|(n, x)| match x {
            Quad::L(t) => (n, t).ok(),
            Quad::ML(rc) =>
                match constraint.extend_new(rc.constraint.clone()) {
                    Some(new_constraint) => {
                        constraint = new_constraint;
                        (n, rc.r#type).ok()
                    }
                    None => type_miss_match!(format!(
                        "Constraint conflict {constraint:?} <> {:?}",
                        rc.constraint
                    ))
                    .err()
                },
            err => err.clone().err()
        })
        .fold(vec![].ok(), |acc, x| match (acc, x) {
            (Ok(acc), Ok(x)) => acc.chain_push(x).ok(),
            (Ok(_), Err(e)) => Err(e),
            (err, _) => err
        });

    let prod_type = match vec {
        Ok(vec) => Type::ProdType(vec),
        Err(e) => return e
    };

    with_constraint_lift_or_left(
        constraint,
        type_env,
        &prod_type,
        expect_type
    )
}
