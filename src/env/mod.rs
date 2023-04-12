use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::parser::define::Define;

pub mod env_ref_src;
pub mod expr_env;
mod r#macro;
pub mod type_constraint;
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
            Define::ExprDef(n, et, ee) => {
                let tc = et.clone().into();
                eev.push((n.clone(), tc, ee.clone().into()));
                (tev, eev)
            }
        }
    );

    let type_env = TypeEnv::new(tev);
    let expr_env = ExprEnv::new(type_env.clone(), eev);

    (type_env.clone(), expr_env)
}
