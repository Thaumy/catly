use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#fn::has_type;
use crate::infer_type::r#type::require_constraint::require_constraint;
use crate::infer_type::r#type::require_info::RequireInfo;
use crate::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infer_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;

// TODO: 外部环境约束同层传播完备性
pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    ref_name: &str
) -> GetTypeReturn {
    match expr_env.infer_type_with_hint(ref_name, expect_type) {
        // 约束到确切类型, 尝试提升
        Quad::L(t) =>
            match t.lift_to_or_left(type_env, expect_type) {
                Some(expect_type) => has_type(expect_type),
                None => TypeMissMatch::of_type(
                    &t,
                    &expect_type.clone().unwrap()
                )
                .into()
            },
        // 提升并传播约束
        Quad::ML(rc) => match rc
            .r#type
            .lift_to_or_left(type_env, expect_type)
        {
            Some(expect_type) =>
                require_constraint(expect_type, rc.constraint),
            // TODO: 在所有类似的地方都应用这种检查
            None =>
                if rc.r#type.is_partial_type() {
                    RequireInfo::of(ref_name, rc.constraint).into()
                } else {
                    TypeMissMatch::of_type(
                        &rc.r#type,
                        &expect_type.clone().unwrap()
                    )
                    .into()
                },
        },
        // 引用源类型信息不足或引用源类型不匹配
        // 引用源类型信息不足, 例如:
        // f -> a -> f a
        // env:
        // def f = _
        // def a = _
        // 无法推导出 a 的类型, 因为 a 的类型是自由的
        mr_r => mr_r
    }
}
