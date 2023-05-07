#[allow(unused_macros)]
macro_rules! int_type {
    () => {{
        use crate::eval::r#type::r#type::Type;
        Type::NamelyType("Int".to_string())
    }};
}
#[allow(unused_imports)]
pub(crate) use int_type;

#[allow(unused_macros)]
macro_rules! unit_type {
    () => {{
        use crate::eval::r#type::r#type::Type;
        Type::NamelyType("Unit".to_string())
    }};
}
#[allow(unused_imports)]
pub(crate) use unit_type;

#[allow(unused_macros)]
macro_rules! namely_type {
    ($str:expr) => {{
        use crate::eval::r#type::r#type::Type;
        Type::NamelyType($str.to_string())
    }};
}
#[allow(unused_imports)]
pub(crate) use namely_type;

#[allow(unused_macros)]
macro_rules! closure_type {
    ($lhs:expr, $rhs:expr) => {{
        use crate::eval::r#type::r#type::Type;
        use crate::infra::rc::RcAnyExt;

        Type::ClosureType($lhs.rc(), $rhs.rc())
    }};
}
#[allow(unused_imports)]
pub(crate) use closure_type;

#[allow(unused_macros)]
macro_rules! prod_type {
    ($($types:expr),* $(,)?) => ({
        use crate::eval::r#type::r#type::Type;

        Type::ProdType(vec![
            $(($types),)*
        ])
    })
}
#[allow(unused_imports)]
pub(crate) use prod_type;

#[allow(unused_macros)]
macro_rules! sum_type {
    ($($types:expr),* $(,)?) => ({
        use crate::{btree_set};
        use crate::eval::r#type::r#type::Type;

        Type::SumType(btree_set![
            $(($types),)*
        ])
    })
}
#[allow(unused_imports)]
pub(crate) use sum_type;

#[allow(unused_macros)]
macro_rules! true_type {
    () => {{
        use crate::eval::r#type::r#type::Type;
        Type::NamelyType("True".to_string())
    }};
}
#[allow(unused_imports)]
pub(crate) use true_type;

#[allow(unused_macros)]
macro_rules! false_type {
    () => {{
        use crate::eval::r#type::r#type::Type;
        Type::NamelyType("False".to_string())
    }};
}
#[allow(unused_imports)]
pub(crate) use false_type;

#[allow(unused_macros)]
macro_rules! bool_type {
    () => {{
        use crate::eval::r#type::r#type::Type;
        Type::NamelyType("Bool".to_string())
    }};
}
#[allow(unused_imports)]
pub(crate) use bool_type;
