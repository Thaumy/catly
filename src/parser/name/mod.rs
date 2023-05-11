use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;

pub mod let_name;
pub mod type_name;

pub enum Name {
    LetName(String),
    TypeName(String)
}

pub fn parse_name(str: &str) -> Option<Name> {
    if str.starts_with(|c: char| c.is_uppercase()) {
        parse_type_name(str).map(Name::TypeName)
    } else {
        // _ -> LetName
        parse_let_name(str).map(Name::LetName)
    }
}
