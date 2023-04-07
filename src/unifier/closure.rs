use crate::infra::alias::MaybeType;
use crate::parser::r#type::Type;
use crate::unifier::can_lift;

pub fn lift(
    type_env: &Vec<(String, Type)>,
    i_t: &MaybeType,
    o_t: &MaybeType,
    derive: &Type
) -> bool {
    println!("Uplift {:?} -> {:?} to {:?}", i_t, o_t, derive);

    match derive {
        // Base
        Type::ClosureType(d_i_t, d_o_t) =>
        // 此处是类型信息的补全关系, 而非针对泛型的类型相容性判断
            (match (i_t, d_i_t) {
                (Some(i_t), Some(d_i_t)) =>
                    can_lift(type_env, i_t, d_i_t),
                (None, _) => true,
                _ => false
            }) && (match (o_t, d_o_t) {
                (Some(o_t), Some(d_o_t)) =>
                    can_lift(type_env, o_t, d_o_t),
                (None, _) => true,
                _ => false
            }),
        // T
        // where Base can be lifted to T
        Type::TypeEnvRef(ref_name) => type_env
            .iter()
            .rev()
            .find(|(n, t)| {
                n == ref_name && lift(type_env, i_t, o_t, t)
            })
            .is_some(),

        // .. | T | ..
        // where Base can be lifted to T
        Type::SumType(s) => s
            .iter()
            .rev()
            .any(|t| lift(type_env, i_t, o_t, t)),

        _ => false
    }
}
