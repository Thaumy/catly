use crate::parser::r#type::Type;

pub fn lift(
    env: &Vec<(String, Type)>,
    base: &str,
    derive: &str,
) -> bool {
    println!("Uplift {:?} to {:?}", base, derive);

    match env
        .iter()
        .find(|(n, _)| n == derive)
        .map(|(_, t)| t)
    {
        // type Derive = Base
        Some(Type::TypeEnvRef(n))
        => n == base,

        // type Derive = .. | Base | ..
        Some(Type::SumType(s))
        => s
            .iter()
            .any(|t| match t {
                Type::TypeEnvRef(n) => n == base,
                _ => false
            }),

        _ => false
    }
}
