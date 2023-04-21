#[macro_export]
macro_rules! has_type {
    ($e:expr) => {{
        use crate::infra::quad::Quad;
        Quad::L($e)
    }};
}

#[macro_export]
macro_rules! require_constraint {
    ($t:expr, $constraint:expr) => {{
        use crate::get_type::r#type::require_constraint::RequireConstraint;
        use crate::infra::quad::Quad;
        Quad::ML(RequireConstraint {
            r#type: $t,
            constraint: $constraint
        })
    }};
}

#[macro_export]
macro_rules! extend_constraint_then_require {
    ($t:expr, $l_c:expr, $r_c:expr) => {{
        use crate::get_type::r#type::type_miss_match::TypeMissMatch;
        use crate::require_constraint;
        match $l_c
            .clone()
            .extend_new($r_c.clone())
        {
            Some(constraint) => require_constraint!($t, constraint),
            None => TypeMissMatch::of_constraint(&$l_c, &$r_c).into()
        }
    }};
}

#[macro_export]
macro_rules! require_info {
    ($ref_name:expr) => {{
        use crate::get_type::r#type::require_info::RequireInfo;
        use crate::infra::quad::Quad;
        Quad::MR(RequireInfo {
            ref_name: $ref_name
        })
    }};
}

#[macro_export]
macro_rules! type_miss_match_pat {
    () => {
        crate::get_type::r#type::type_miss_match::TypeMissMatch { .. }
    };
}

#[macro_export]
macro_rules! single_constraint {
    ($ref_name:expr,$r#type:expr) => {{
        use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
        EnvRefConstraint::single($ref_name, $r#type)
    }};
}

#[macro_export]
macro_rules! empty_constraint {
    () => {{
        use crate::get_type::r#type::env_ref_constraint::EnvRefConstraint;
        EnvRefConstraint::empty()
    }};
}
