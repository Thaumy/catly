mod get_type_with_hint;

use crate::env::env_ref_src::EnvRefSrc;
use crate::env::type_constraint::TypeConstraint;
use crate::env::type_env::TypeEnv;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::{MaybeExpr, MaybeType};
use crate::infra::option::AnyExt;
use crate::parser::expr::r#type::Expr;

type Item = (String, TypeConstraint, EnvRefSrc);

// 表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv<'t> {
    type_env: TypeEnv,
    prev_env: Option<&'t ExprEnv<'t>>,
    env: Vec<Item>
}

impl<'t> ExprEnv<'t> {
    pub fn new(type_env: TypeEnv, env_vec: Vec<Item>) -> ExprEnv<'t> {
        let expr_env = ExprEnv {
            type_env,
            prev_env: None,
            env: env_vec
        };
        println!("New ExprEnv: {:?}", expr_env.env);
        expr_env
    }

    pub fn extend_vec_new(&self, env_vec: Vec<Item>) -> ExprEnv {
        let expr_env = ExprEnv {
            type_env: self.type_env.clone(),
            prev_env: self.some(),
            env: env_vec
        };
        println!("New ExprEnv: {:?}", expr_env.env);
        expr_env
    }

    pub fn extend_new(
        &self,
        ref_name: String,
        r#type: MaybeType,
        src: MaybeExpr
    ) -> ExprEnv {
        let tc = r#type
            .map(|t| t.into())
            .unwrap_or(TypeConstraint::Free);

        self.extend_vec_new(vec![(ref_name, tc, src.into())])
    }

    pub fn get_type(&self, ref_name: &str) -> GetTypeReturn {
        self.get_type_with_hint(ref_name, &None)
    }

    fn find_entry(&self, ref_name: &str) -> Option<&Item> {
        let entry = self
            .env
            .iter()
            .rev()
            .find(|(n, ..)| n == ref_name);

        match (entry, self.prev_env) {
            (Some(entry), _) => entry.some(),
            (None, Some(prev_env)) => prev_env.find_entry(ref_name),
            _ => None
        }
    }

    pub fn get_expr(&self, ref_name: &str) -> Option<&Expr> {
        self.find_entry(ref_name)
            .and_then(|(.., src)| match src {
                EnvRefSrc::Src(expr) => expr.some(),
                EnvRefSrc::NoSrc => None
            })
    }

    pub fn get_ref(&self, ref_name: &str) -> Option<Expr> {
        self.find_entry(ref_name)
            .map(|(n, tc, _)| {
                Expr::EnvRef(tc.clone().into(), n.to_string())
            })
    }

    pub fn exist_ref(&self, ref_name: &str) -> bool {
        self.find_entry(ref_name)
            .is_some()
    }
}
