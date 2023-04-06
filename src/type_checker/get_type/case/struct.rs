use crate::infra::alias::MaybeType;
use crate::infra::either::Either;
use crate::infra::option::AnyExt as OptExt;
use crate::infra::quad::Quad;
use crate::infra::r#fn::id;
use crate::infra::result::AnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::r#fn::{
    destruct_type_env_ref,
    lift_or_left,
    with_constraint_lift_or_left
};
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    RequireInfo,
    TypeEnv,
    TypeMissMatch
};
use crate::type_checker::get_type::{get_type, get_type_with_hint};
use crate::unifier::can_lift;
use crate::{has_type, require_constraint, type_miss_match};

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    t: &MaybeType,
    vec: &Vec<(String, MaybeType, Expr)>
) -> GetTypeReturn {
    // 解构 t 并判断与 vec 的相容性
    let t_vec = match t {
        Some(t) => match destruct_type_env_ref(type_env, t) {
            // 解构合法, 当且仅当由 t 解构出的 ProdType 的字段数和 vec 相等
            // 且对于二者 zip 后的每一对字段, 其名称相同
            // 且 vec 字段的类型可以被提升到 ProdType 字段的类型(如果 vec 字段类型存在的话)
            Some(Type::ProdType(t_vec))
                if t_vec.len() == vec.len() &&
                    t_vec
                        .iter()
                        .zip(vec.iter())
                        .map(|((n, t), (v_n, v_t, v_e))| {
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
    let mut constraint = vec![];

    // 收集约束
    let vec = vec
        .iter()
        .map(|(n, _, e)| {
            (n.to_string(), get_type(type_env, expr_env, e))
        })
        .map(|(n, x)| match x {
            Quad::L(t) => (n, t).ok(),
            Quad::ML(rc) => {
                constraint.append(&mut rc.constraint.clone());
                (n, rc.r#type).ok()
            }
            err => err.clone().err()
        })
        .fold(Ok(vec![]), |acc, x| match (acc, x) {
            (Ok(mut acc), Ok(x)) => {
                acc.push(x);
                acc.ok()
            }
            (Ok(_), Err(e)) => Err(e),
            (err, _) => err
        });

    let prod_type = match vec {
        Ok(vec) => Type::ProdType(vec),
        Err(e) => return e
    };

    with_constraint_lift_or_left(constraint, type_env, &prod_type, t)
}
