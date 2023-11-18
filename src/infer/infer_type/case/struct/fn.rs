use crate::infer::env::TypeEnv;
use crate::infer::infer_type::r#fn::destruct_namely_type;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::id;
use crate::infra::WrapOption;
use crate::infra::WrapQuad;
use crate::infra::WrapResult;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::Type;
use crate::parser::r#type::{OptType, ProdField};

type StructVec = Vec<(String, OptType, Expr)>;
type ProdVec = Vec<ProdField>;

// 仅从 Struct 结构上进行类型判断, 不对其字段表达式求类型
// 对于 SumType 中存在多个类型均和 Struct 结构兼容的情况, 仅取第一个兼容类型
// 因为此时如果对 Struct 字段表达式求精确类型可能产生一系列约束, 而无法确定哪种约束是最符合要求的
// 因而无法确定 SumType 中最优的匹配类型, 同时也会极大增加 struct case 的实现难度
// 所以对 Struct 字段表达式求类型并判断其相容性的工作由 struct case 的剩余部分解决
pub fn is_struct_vec_of_type_then_get_prod_vec(
    type_env: &TypeEnv,
    expect_type: &OptType,
    struct_vec: &StructVec
) -> Result<Option<ProdVec>, InferTypeRet> {
    // 解构 expect_type 并判断与 struct_vec 的相容性
    if let Some(expect_type) = expect_type {
        match destruct_namely_type(type_env, expect_type) {
            // 解构合法, 当且仅当由 t 解构出的 ProdType 的字段数和 vec 相等
            // 且对于二者 zip 后的每一对字段, 其名称相同
            // 且 vec 字段的类型可以被提升到 ProdType 字段的类型(如果 vec 字段类型存在的话)
            Some(Type::ProdType(prod_vec)) => (prod_vec.len() ==
                struct_vec.len() &&
                prod_vec
                    .iter()
                    .zip(struct_vec.iter())
                    .map(|((n, t), (v_n, v_t, _))| {
                        // 名称相等判断
                        n == v_n &&
                            // 类型相容判断
                            v_t.clone()
                                .map(|v_t| {
                                    v_t.can_lift_to(type_env, t)
                                })
                                .unwrap_or(true)
                    })
                    .all(id))
                .then_some(prod_vec)
                .wrap_ok(),

            Some(Type::SumType(sum_vec)) => sum_vec
                .into_iter()
                .map(|t| {
                    is_struct_vec_of_type_then_get_prod_vec(
                        type_env,
                        &t.wrap_some(),
                        struct_vec
                    )
                })
                .find(|x| matches!(x, Ok(Some(..))))
                .unwrap_or(
                    TypeMissMatch::of(
                        format!(
                            "{expect_type:?} <> type of Struct{struct_vec:?}"
                        )).wrap_quad_r()
                        .wrap_err()
                ),

            Some(t) => TypeMissMatch::of(format!(
                "{t:?} <> type of Struct{struct_vec:?}"
            )).wrap_quad_r()
                .wrap_err(),

            None => TypeMissMatch::of(format!(
                "{expect_type:?} not found in type env"
            )).wrap_quad_r()
                .wrap_err()
        }
    } else {
        None.wrap_ok()
    }
}
