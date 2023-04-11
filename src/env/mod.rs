use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::parser::define::Define;
use crate::type_checker::r#type::TypeConstraint;

pub mod expr_env;
pub mod type_env;

pub fn from_defines<'t>(
    defines: Vec<Define>
) -> (TypeEnv, ExprEnv<'t>) {
    let (tev, eev) = defines.iter().fold(
        (vec![], vec![]),
        |(mut tev, mut eev), define| match define {
            Define::TypeDef(n, t) => {
                tev.push((n.clone(), t.clone()));
                (tev, eev)
            }
            Define::ExprDef(n, et, _) => {
                let tc = match et {
                    Some(t) => TypeConstraint::Constraint(t.clone()),
                    None => TypeConstraint::Free
                };
                eev.push((n.clone(), tc));
                (tev, eev)
            }
        }
    );

    (TypeEnv::new(tev), ExprEnv::new(eev))
}
