use crate::env::r#type::type_env::TypeEnv;
use crate::infra::option::AnyExt;
use crate::infra::r#box::Ext;
use crate::parser::r#type::r#type::Type;

pub fn lift_closure(
    type_env: &TypeEnv,
    i_t: &Type,
    o_t: &Type,
    derive: &Type
) -> Option<Type> {
    if derive.is_primitive() {
        return None;
    }

    match derive {
        // Base
        Type::ClosureType(d_i_t, d_o_t) => Type::ClosureType(
            i_t.lift_to(type_env, d_i_t)?
                .boxed(),
            o_t.lift_to(type_env, d_o_t)?
                .boxed()
        )
        .some(),

        // PartialClosureType
        // HACK:
        // 不允许 ClosureType 退化为 PartialClosureType
        // 此提升将会使 ClosureType 顶替掉 PartialClosureType, 这是安全行为, 因为后者的输出类型不可知
        Type::PartialClosureType(d_i_t) => Type::ClosureType(
            i_t.lift_to(type_env, d_i_t)?
                .boxed(),
            o_t.clone().boxed()
        )
        .some(),

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => {
            let base = Type::ClosureType(
                i_t.clone().boxed(),
                o_t.clone().boxed()
            );
            type_env
                .find_type(type_name)
                .and_then(|type_base| {
                    base.lift_to(type_env, &type_base)
                })
                .map(|_| derive.clone())
        }

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => {
            let base = Type::ClosureType(
                i_t.clone().boxed(),
                o_t.clone().boxed()
            );
            s.iter()
                .any(|t| {
                    base.lift_to(type_env, t)
                        .is_some()
                })
                .then(|| derive.clone())
        }

        _ => None
    }
}
