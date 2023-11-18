use crate::infer::env::type_env::TypeEnv;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
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
                .rc(),
            o_t.lift_to(type_env, d_o_t)?
                .rc()
        )
        .wrap_some(),

        // PartialClosureType
        // HACK:
        // 不允许 ClosureType 退化为 PartialClosureType
        // 此提升将会使 ClosureType 顶替掉 PartialClosureType, 这是安全行为, 因为后者的输出类型不可知
        Type::PartialClosureType(d_i_t) => Type::ClosureType(
            i_t.lift_to(type_env, d_i_t)?
                .rc(),
            o_t.clone().rc()
        )
        .wrap_some(),

        // T
        // where Base can be lifted to T
        Type::NamelyType(type_name) => {
            let base =
                Type::ClosureType(i_t.clone().rc(), o_t.clone().rc());
            type_env
                .find_type(type_name.as_str())
                .and_then(|type_base| {
                    base.lift_to(type_env, &type_base)
                })
                .map(|_| derive.clone())
        }

        // .. | Base | ..
        Type::SumType(s) =>
            s.iter()
                .any(|t| {
                    &Type::ClosureType(
                        i_t.clone().rc(),
                        o_t.clone().rc()
                    ) == t
                })
                .then(|| derive.clone()),

        // 与 int case 同理
        // // .. | T | ..
        // // where Base can be lifted to T
        // Type::SumType(s) => s
        //     .iter()
        //     .any(|t| {
        //         Type::ClosureType(
        //             i_t.clone().rc(),
        //             o_t.clone().rc()
        //         )
        //         .can_lift_to(type_env, t)
        //     })
        //     .then(|| derive.clone()),
        _ => None
    }
}
