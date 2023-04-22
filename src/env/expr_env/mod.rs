mod infer_type;
mod infer_type_with_hint;

use crate::env::r#type::env_ref_src::EnvRefSrc;
use crate::env::r#type::type_constraint::TypeConstraint;
use crate::env::r#type::type_env::TypeEnv;
use crate::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
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
        println!(
            "{:8}{:>10} │ {:?}",
            "[env]", "ExprEnv", expr_env.env
        );
        expr_env
    }

    // 通过将最近的非空环境作为上级环境, 能够提高查找效率
    // 因为环境不可变, 所以没有风险
    fn latest_none_empty_expr_env(&self) -> &ExprEnv {
        match (self.env.is_empty(), self.prev_env) {
            (true, Some(prev_env)) =>
                prev_env.latest_none_empty_expr_env(),
            _ => self
        }
    }

    pub fn extend_vec_new(&self, env_vec: Vec<Item>) -> ExprEnv {
        let expr_env = ExprEnv {
            type_env: self.type_env.clone(),
            prev_env: self
                .latest_none_empty_expr_env()
                .some(),
            env: env_vec
        };
        println!(
            "{:8}{:>10} │ {:?}",
            "[env]", "ExprEnv", expr_env.env
        );
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

        let expr_env =
            self.extend_vec_new(vec![(ref_name, tc, src.into())]);
        println!(
            "{:8}{:>10} │ {:?}",
            "[env]", "ExprEnv", expr_env.env
        );
        expr_env
    }

    pub fn extend_constraint_new(
        &self,
        constraint: EnvRefConstraint
    ) -> ExprEnv {
        let vec = constraint
            .iter()
            .map(|(n, t)| {
                (n.to_string(), t.clone().into(), EnvRefSrc::NoSrc)
            })
            .collect();

        let expr_env = self.extend_vec_new(vec);
        println!(
            "{:8}{:>10} │ {:?}",
            "[env]", "ExprEnv", expr_env.env
        );
        expr_env
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
