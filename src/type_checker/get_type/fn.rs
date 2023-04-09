use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt as OptAnyExt;
use crate::infra::quad::Quad;
use crate::infra::triple::AnyExt as TriAnyExt;
use crate::infra::vec::Ext;
use crate::parser::expr::Expr;
use crate::parser::r#type::Type;
use crate::type_checker::get_type::get_type;
use crate::type_checker::get_type::r#type::{
    ExprEnv,
    GetTypeReturn,
    TypeConstraint,
    TypeEnv
};
use crate::unifier::{can_lift, lift};
use crate::{
    bool_type,
    false_type,
    has_type,
    int_type,
    require_constraint,
    true_type,
    type_miss_match,
    unit_type
};

pub fn of_boolean_types(t: &Type) -> bool {
    t == &bool_type!() || t == &true_type!() || t == &false_type!()
}

// Lift l to r if r exist, then return lifting result
// Return l if r not exist
pub fn lift_or_left(
    type_env: &TypeEnv,
    l: &Type,
    r: &MaybeType
) -> Option<Type> {
    match r {
        Some(r) => lift(type_env, &l, &r),
        _ => Some(l.clone())
    }
}

pub fn with_constraint_lift_or_left(
    constraint: Vec<(String, Type)>,
    type_env: &TypeEnv,
    base: &Type,
    derive: &MaybeType
) -> GetTypeReturn {
    match lift_or_left(type_env, base, derive) {
        Some(t) =>
        // 按需传播
            if constraint.is_empty() {
                has_type!(t)
            } else {
                require_constraint!(t, constraint)
            },
        None => type_miss_match!()
    }
}

pub fn find_ref_type<'t>(
    expr_env: &'t ExprEnv,
    ref_name: &str
) -> Option<&'t TypeConstraint> {
    expr_env
        .iter()
        .rev()
        .find(|(n, _)| n == ref_name)
        .map(|(_, t)| t)
}

pub fn destruct_type_env_ref(
    type_env: &TypeEnv,
    r#type: &Type
) -> Option<Type> {
    match r#type {
        Type::TypeEnvRef(name) => {
            let base_type = type_env
                .iter()
                .rev()
                .find(|(n, _)| n == name)
                .map(|(_, t)| t)?;
            destruct_type_env_ref(type_env, base_type)
        }
        x => Some(x.clone())
    }
}

pub fn inject_to_new_expr_env(
    old_expr_env: &ExprEnv,
    name: &str,
    r#type: &MaybeType
) -> ExprEnv {
    old_expr_env.push_to_new((
        name.to_string(),
        r#type
            .as_ref()
            .map(|t| TypeConstraint::Constraint(t.clone()))
            .unwrap_or_else(|| TypeConstraint::Free)
    ))
}

pub fn inject_const_expr_destruction_to_new_expr_env(
    type_env: &TypeEnv,
    old_expr_env: &ExprEnv,
    expr: &Expr
) -> ExprEnv {
    todo!()
    /*    match expr {
            Expr::EnvRef(t, n) => {
                let t = t
                    .map(|t| TypeConstraint::Constraint(t))
                    .unwrap_or_else(TypeConstraint::Free);
                old_expr_env.push_to_new((n.to_string(), t))
            }
            Expr::Struct(_, vec) => {
                vec.iter().map(|(s, mt, e)| {
                    match mt {
                        Some(t) => (s, TypeConstraint::Constraint(t.clone())),
                        None => {
                            todo!()
                        }
                    }
                })
            }
            _ => todo!()
        }
    */
}
