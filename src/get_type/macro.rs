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
        use crate::get_type::r#type::RequireConstraint;
        use crate::infra::quad::Quad;
        Quad::ML(RequireConstraint {
            r#type: $t,
            constraint: $constraint
        })
    }};
}

#[macro_export]
macro_rules! require_info {
    ($ref_name:expr) => {{
        use crate::get_type::r#type::RequireInfo;
        use crate::infra::quad::Quad;
        Quad::MR(RequireInfo {
            ref_name: $ref_name
        })
    }};
}

#[macro_export]
macro_rules! type_miss_match {
    ($info:expr) => {{
        use crate::get_type::r#type::TypeMissMatch;
        use crate::infra::quad::Quad;
        Quad::R(TypeMissMatch { info: $info })
    }};
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
