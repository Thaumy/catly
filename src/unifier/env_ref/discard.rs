use crate::parser::r#type::Type;

pub fn lift(
    env: &Vec<(String, Type)>,
    derive: &Type,
) -> bool {
    println!("Uplift Discard to {:?}", derive);
    true
}
