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
        use crate::infra::quad::Quad;
        use crate::type_checker::get_type::r#type::RequireConstraint;
        Quad::ML(RequireConstraint {
            r#type: $t,
            constraint: $constraint
        })
    }};
}

#[macro_export]
macro_rules! require_info {
    ($ref_name:expr) => {{
        use crate::infra::quad::Quad;
        use crate::type_checker::get_type::r#type::RequireInfo;
        Quad::MR(RequireInfo {
            ref_name: $ref_name
        })
    }};
}

#[macro_export]
macro_rules! type_miss_match {
    () => {{
        use crate::infra::quad::Quad;
        use crate::type_checker::get_type::r#type::TypeMissMatch;
        Quad::R(TypeMissMatch {})
    }};
}
