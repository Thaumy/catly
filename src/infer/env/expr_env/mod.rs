mod infer_type;
mod infer_type_with_hint;

use std::rc::Rc;

use crate::infer::env::r#type::env_ref_src::EnvRefSrc;
use crate::infer::env::r#type::type_constraint::TypeConstraint;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::parser::expr::r#type::{Expr, OptExpr};
use crate::parser::r#type::r#type::OptType;

pub type ExprEnvEntry = (String, TypeConstraint, EnvRefSrc);

// 编译时表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv {
    prev_env: Option<Rc<ExprEnv>>,
    entry: Option<(String, TypeConstraint, EnvRefSrc)>
}

impl ExprEnv {
    pub fn empty() -> ExprEnv {
        ExprEnv {
            prev_env: None,
            entry: None
        }
    }

    pub fn new(
        ref_name: impl Into<String>,
        tc: TypeConstraint,
        src: EnvRefSrc
    ) -> ExprEnv {
        let entry = (ref_name.into(), tc, src);

        let expr_env = ExprEnv {
            prev_env: None,
            entry: entry.wrap_some()
        };

        #[cfg(feature = "ct_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.entry
            );
            println!("{log}");
        }

        expr_env
    }

    // 通过将最近的非空环境作为上级环境, 能够提高查找效率
    // 因为环境不可变, 所以没有风险
    fn latest_none_empty_expr_env(self: &Rc<Self>) -> Rc<ExprEnv> {
        match (self.entry.is_none(), &self.prev_env) {
            (true, Some(prev_env)) =>
                prev_env.latest_none_empty_expr_env(),
            _ => self.clone()
        }
    }

    pub fn extend_vec_new(
        self: &Rc<Self>,
        env_vec: Vec<ExprEnvEntry>
    ) -> Rc<ExprEnv> {
        let expr_env = env_vec.into_iter().fold(
            self.clone(),
            |acc, (r_n, tc, src)| {
                ExprEnv {
                    prev_env: acc
                        .latest_none_empty_expr_env()
                        .wrap_some(),
                    entry: (r_n, tc, src).wrap_some()
                }
                .rc()
            }
        );

        #[cfg(feature = "ct_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.entry
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_new(
        self: &Rc<Self>,
        ref_name: impl Into<String>,
        r#type: OptType,
        src: OptExpr
    ) -> Rc<ExprEnv> {
        let tc = r#type
            .map(|t| t.into())
            .unwrap_or(TypeConstraint::Free);

        let expr_env = self.extend_vec_new(vec![(
            ref_name.into(),
            tc,
            src.into()
        )]);

        #[cfg(feature = "ct_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.entry
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_constraint_new(
        self: &Rc<Self>,
        constraint: EnvRefConstraint
    ) -> Rc<ExprEnv> {
        let vec = constraint
            .into_iter()
            .map(|(n, t)| (n, t.into(), EnvRefSrc::NoSrc))
            .collect();

        let expr_env = self.extend_vec_new(vec);

        #[cfg(feature = "ct_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[ct env]", "ExprEnv", expr_env.entry
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
        let entry =
            self.entry
                .as_ref()
                .and_then(|entry @ (n, ..)| {
                    (n == ref_name).then_some(entry)
                });

        match (entry, &self.prev_env) {
            (Some(entry), _) => entry.wrap_some(),
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
                EnvRefSrc::Src(expr) => expr.wrap_some(),
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
