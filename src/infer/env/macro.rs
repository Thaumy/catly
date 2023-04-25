#[macro_export]
macro_rules! int_type {
    () => {{
        use crate::parser::r#type::r#type::Type;
        Type::NamelyType("Int".to_string())
    }};
}

#[macro_export]
macro_rules! unit_type {
    () => {{
        use crate::parser::r#type::r#type::Type;
        Type::NamelyType("Unit".to_string())
    }};
}

#[macro_export]
macro_rules! namely_type {
    ($str:expr) => {{
        use crate::parser::r#type::r#type::Type;
        Type::NamelyType($str.to_string())
    }};
}

#[macro_export]
macro_rules! closure_type {
    ($lhs:expr, $rhs:expr) => {{
        use crate::infra::r#box::Ext;
        use crate::parser::r#type::r#type::Type;
        Type::ClosureType($lhs.boxed(), $rhs.boxed())
    }};
}

#[macro_export]
macro_rules! prod_type {
    ($($types:expr),* $(,)?) => ({
        use crate::parser::r#type::r#type::Type;

        Type::ProdType(vec![
            $(($types),)*
        ])
    })
}

#[macro_export]
macro_rules! sum_type {
    ($($types:expr),* $(,)?) => ({
        use crate::{btree_set};
        use crate::parser::r#type::r#type::Type;

        Type::SumType(btree_set![
            $(($types),)*
        ])
    })
}

#[macro_export]
macro_rules! true_type {
    () => {{
        use crate::parser::r#type::r#type::Type;
        Type::NamelyType("True".to_string())
    }};
}

#[macro_export]
macro_rules! false_type {
    () => {{
        use crate::parser::r#type::r#type::Type;
        Type::NamelyType("False".to_string())
    }};
}

#[macro_export]
macro_rules! bool_type {
    () => {{
        use crate::parser::r#type::r#type::Type;
        Type::NamelyType("Bool".to_string())
    }};
}
