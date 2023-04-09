#[macro_export]
macro_rules! int_type {
    () => {{
        use crate::parser::r#type::Type;
        Type::TypeEnvRef("Int".to_string())
    }};
}

#[macro_export]
macro_rules! unit_type {
    () => {{
        use crate::parser::r#type::Type;
        Type::TypeEnvRef("Unit".to_string())
    }};
}

#[macro_export]
macro_rules! true_type {
    () => {{
        use crate::parser::r#type::Type;
        Type::TypeEnvRef("Int".to_string())
    }};
}

#[macro_export]
macro_rules! false_type {
    () => {{
        use crate::parser::r#type::Type;
        Type::TypeEnvRef("Int".to_string())
    }};
}

#[macro_export]
macro_rules! bool_type {
    () => {{
        use crate::btree_set;
        use crate::parser::r#type::Type;
        use crate::{false_type, true_type};
        Type::SumType(btree_set![true_type!(), false_type!(),])
    }};
}

#[macro_export]
macro_rules! has_type {
    ($e:expr) => {{
        use crate::infra::quad::Quad;
        Quad::L($e)
    }};
}

#[macro_export]
macro_rules! require_constraint {
    ($t:expr, $vec:expr) => {{
        use crate::infra::quad::Quad;
        use crate::type_checker::get_type::r#type::RequireConstraint;
        Quad::ML(RequireConstraint {
            r#type: $t,
            constraint: $vec
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
