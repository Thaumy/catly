mod infer_type;
mod infer_type_with_hint;

use crate::infer::env::r#type::env_ref_src::EnvRefSrc;
use crate::infer::env::r#type::type_constraint::TypeConstraint;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infra::option::OptionAnyExt;
use crate::parser::expr::r#type::{Expr, OptExpr};
use crate::parser::r#type::r#type::OptType;

pub type ExprEnvEntry = (String, TypeConstraint, EnvRefSrc);

// TODO: 重构到单一环境条目
// 编译时表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv<'t> {
    prev_env: Option<&'t ExprEnv<'t>>,
    env: Vec<ExprEnvEntry>
}

impl<'t> ExprEnv<'t> {
    pub fn empty() -> ExprEnv<'t> { Self::new(vec![]) }

    pub fn new(env_vec: Vec<ExprEnvEntry>) -> ExprEnv<'t> {
        let expr_env = ExprEnv {
            prev_env: None,
            env: env_vec
        };

        if cfg!(feature = "ct_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

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

    pub fn extend_vec_new(
        &self,
        env_vec: Vec<ExprEnvEntry>
    ) -> ExprEnv {
        let expr_env = ExprEnv {
            prev_env: self
                .latest_none_empty_expr_env()
                .some(),
            env: env_vec
        };

        if cfg!(feature = "ct_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_new(
        &self,
        ref_name: impl Into<String>,
        r#type: OptType,
        src: OptExpr
    ) -> ExprEnv {
        let tc = r#type
            .map(|t| t.into())
            .unwrap_or(TypeConstraint::Free);

        let expr_env = self.extend_vec_new(vec![(
            ref_name.into(),
            tc,
            src.into()
        )]);

        if cfg!(feature = "ct_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_constraint_new(
        &self,
        constraint: EnvRefConstraint
    ) -> ExprEnv {
        let vec = constraint
            .into_iter()
            .map(|(n, t)| (n, t.into(), EnvRefSrc::NoSrc))
            .collect();

        let expr_env = self.extend_vec_new(vec);

        if cfg!(feature = "ct_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

        expr_env
    }

    fn find_entry<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<&ExprEnvEntry> {
        let ref_name = ref_name.into();
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

    pub fn get_expr<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<&Expr> {
        self.find_entry(ref_name)
            .and_then(|(.., src)| match src {
                EnvRefSrc::Src(expr) => expr.some(),
                EnvRefSrc::NoSrc => None
            })
    }

    pub fn get_ref<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<Expr> {
        self.find_entry(ref_name)
            .map(|(n, tc, _)| {
                Expr::EnvRef(tc.clone().into(), n.to_string())
            })
    }

    pub fn exist_ref<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> bool {
        self.find_entry(ref_name)
            .is_some()
    }
}
