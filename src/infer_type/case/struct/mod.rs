mod r#fn;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::case::r#struct::r#fn::is_struct_vec_of_type_then_get_prod_vec;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::{AnyExt, Quad};
use crate::infra::result::AnyExt as ResAnyExt;
use crate::infra::vec::Ext;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;

// TODO: 外部环境约束同层传播完备性
pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    vec: &Vec<(String, MaybeType, Expr)>
) -> InferTypeRet {
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
                        .with_optional_fallback_type(v_t)
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
                        .with_optional_fallback_type(mt)
                )
            })
            .collect()
    };

    let mut constraint_acc = EnvRefConstraint::empty();

    // 收集约束
    let vec = vec
        .iter()
        .map(|(n, _, e)| {
            (n.to_string(), e.infer_type(type_env, &expr_env))
        })
        .map(|(n, x)| match x {
            Quad::L(t) => (n, t).ok(),
            Quad::ML(rc) => match constraint_acc
                .extend_new(rc.constraint.clone())
            {
                Some(new_constraint) => {
                    constraint_acc = new_constraint;
                    (n, rc.r#type).ok()
                }
                None => TypeMissMatch::of_constraint(
                    &constraint_acc.clone(),
                    &rc.constraint
                )
                .quad_r()
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

    InferTypeRet::from_auto_lift(
        type_env,
        &prod_type,
        expect_type,
        constraint_acc.some()
    )
}
