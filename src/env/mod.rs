use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::infra::vec::Ext;
use crate::parser::define::Define;

pub mod expr_env;
pub mod r#macro;
pub mod r#type;

pub fn from_defines<'t>(
    defines: Vec<Define>
) -> (TypeEnv, ExprEnv<'t>) {
    let (tev, eev) = defines.iter().fold(
        (vec![], vec![]),
        |(tev, eev), define| match define {
            Define::TypeDef(n, t) =>
                (tev.chain_push((n.clone(), t.clone())), eev),
            Define::ExprDef(n, et, ee) => {
                let tc = et.clone().into();
                (
                    tev,
                    eev.chain_push((
                        n.clone(),
                        tc,
                        ee.clone().into()
                    ))
                )
            }
        }
    );

    let type_env = TypeEnv::new(tev);
    let expr_env = ExprEnv::new(type_env.clone(), eev);

    (type_env.clone(), expr_env)
}
