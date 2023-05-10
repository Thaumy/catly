use crate::eval::std::bool::std_bool;
use crate::eval::std::builtin::std_builtin;
use crate::eval::std::fraction::std_fraction;
use crate::eval::std::int_list::std_int_list;

mod bool;
mod builtin;
mod fraction;
mod int_list;

pub const fn std_code() -> &'static str {
    concat!(
        std_builtin!(),
        std_bool!(),
        std_int_list!(),
        std_fraction!()
    )
}
