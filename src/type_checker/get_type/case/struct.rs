use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptExt;
use crate::infra::quad::Quad;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt;
use crate::infra::vec::Ext;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#fn::{
    destruct_namely_type,
    with_constraint_lift_or_left
};
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::unifier::can_lift;
use crate::{empty_constraint, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    vec: &Vec<(String, MaybeType, Expr)>
) -> GetTypeReturn {
    // 解构 t 并判断与 vec 的相容性
    let t_vec = match expect_type {
        Some(expect_type) =>
            match destruct_namely_type(type_env, expect_type) {
                // 解构合法, 当且仅当由 t 解构出的 ProdType 的字段数和 vec 相等
                // 且对于二者 zip 后的每一对字段, 其名称相同
                // 且 vec 字段的类型可以被提升到 ProdType 字段的类型(如果 vec 字段类型存在的话)
                Some(Type::ProdType(t_vec))
                    if t_vec.len() == vec.len() &&
                        t_vec
                            .iter()
                            .zip(vec.iter())
                            .map(|((n, t), (v_n, v_t, _))| {
                                // 名称相等判断
                                n == v_n &&
                                // 类型相容判断
                                v_t.clone()
                                    .map(|v_t| {
                                        can_lift(type_env, &v_t, t)
                                    })
                                    .unwrap_or(true)
                            })
                            .all(id) =>
                    t_vec.some(),
                _ => return type_miss_match!()
            },
        None => None
    };

    // 进行类型提示
    let vec: Vec<_> = match t_vec {
        // t 存在且可被解构, 对于其每一个类型, 都用作对应表达式的次要类型提示
        Some(t_vec) => t_vec
            .iter()
            .zip(vec.iter())
            .map(|((_, t), (v_n, v_t, v_e))| {
                (
                    v_n.to_string(),
                    v_t.clone(),
                    v_e.clone()
                        .try_with_fallback_type(v_t)
                        .with_fallback_type(t)
                )
            })
            .collect()
        ,
        // t 不存在, 仅使用 vec 自身的类型对表达式进行提示
        None => vec
            .iter()
            .map(|(n, mt, e)| {
                (
                    n.to_string(),
                    mt.clone(),
                    e.clone()
                        .try_with_fallback_type(mt)
                )
            })
            .collect()
    };

    // TODO: Lazy init
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
                match constraint.extend_new(rc.constraint) {
                    Some(c) => {
                        constraint = c;
                        (n, rc.r#type).ok()
                    }
                    None => Err(type_miss_match!())
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
