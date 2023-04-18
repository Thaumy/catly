use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::parser::r#type::r#type::Type;
use crate::{
    bool_type,
    closure_type,
    false_type,
    int_type,
    sum_type,
    true_type
};

pub fn gen_builtin_type_env_vec() -> Vec<(String, Type)> {
    vec![
        ("True".to_string(), int_type!()),
        ("False".to_string(), int_type!()),
        ("Bool".to_string(), sum_type![true_type!(), false_type!()]),
    ]
}

pub fn gen_builtin_expr_env_vec(
) -> Vec<(String, TypeConstraint, EnvRefSrc)> {
    let fn_int_int = closure_type!(
        int_type!(),
        closure_type!(int_type!(), int_type!())
    );
    let fn_int_int_int =
        closure_type!(int_type!(), fn_int_int.clone());
    let fn_bool_bool = closure_type!(bool_type!(), bool_type!());
    let fn_bool_bool_bool =
        closure_type!(int_type!(), fn_bool_bool.clone());
    vec![
        (
            "add".to_string(),
            TypeConstraint::Constraint(fn_int_int_int.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "sub".to_string(),
            TypeConstraint::Constraint(fn_int_int_int.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "neg".to_string(),
            TypeConstraint::Constraint(fn_int_int.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "mul".to_string(),
            TypeConstraint::Constraint(fn_int_int_int.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "mod".to_string(),
            TypeConstraint::Constraint(fn_int_int_int.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "and".to_string(),
            TypeConstraint::Constraint(fn_bool_bool_bool.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "or".to_string(),
            TypeConstraint::Constraint(fn_bool_bool_bool.clone()),
            EnvRefSrc::NoSrc
        ),
        (
            "not".to_string(),
            TypeConstraint::Constraint(fn_bool_bool.clone()),
            EnvRefSrc::NoSrc
        ),
        ("eq".to_string(), TypeConstraint::Free, EnvRefSrc::NoSrc),
    ]
}
