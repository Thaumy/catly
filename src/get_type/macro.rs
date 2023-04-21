#[macro_export]
macro_rules! type_miss_match_pat {
    () => {
        crate::get_type::r#type::type_miss_match::TypeMissMatch { .. }
    };
}
