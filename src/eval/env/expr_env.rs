use crate::eval::r#type::expr::{Expr, OptExpr};
use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;
use crate::infra::r#box::BoxAnyExt;

pub type ExprEnvEntry = (String, Type, OptExpr, Option<ExprEnv>);

// 运行时表达式环境
#[derive(Clone, Debug, PartialEq)]
pub struct ExprEnv {
    prev_env: Option<Box<ExprEnv>>,
    env: Vec<ExprEnvEntry>
}

impl ExprEnv {
    pub fn empty() -> ExprEnv { Self::new(vec![]) }

    pub fn new(env_vec: Vec<ExprEnvEntry>) -> ExprEnv {
        let expr_env = ExprEnv {
            prev_env: None,
            env: env_vec
        };

        if cfg!(feature = "rt_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

        expr_env
    }

    fn latest_none_empty_expr_env(&self) -> Box<ExprEnv> {
        match (self.env.is_empty(), &self.prev_env) {
            (true, Some(prev_env)) =>
                prev_env.latest_none_empty_expr_env(),
            _ => Box::new(self.clone())
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

        if cfg!(feature = "rt_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_new(
        &self,
        ref_name: impl Into<String>,
        r#type: Type,
        src_expr: Expr,
        src_env: ExprEnv
    ) -> ExprEnv {
        let expr_env = self.extend_vec_new(vec![(
            ref_name.into(),
            r#type,
            src_expr.into(),
            src_env.some()
        )]);

        if cfg!(feature = "rt_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "ExprEnv", expr_env.env
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_heap_new(
        self,
        ref_name: impl Into<String>,
        r#type: Type,
        src_expr: Expr,
        src_env: ExprEnv
    ) -> ExprEnv {
        let expr_env = ExprEnv {
            prev_env: self.boxed().some(),
            env: vec![(
                ref_name.into(),
                r#type,
                src_expr.some(),
                src_env.some()
            )]
        };

        if cfg!(feature = "rt_env_log") {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "ExprEnv", expr_env.env
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

        match (entry, &self.prev_env) {
            (Some(entry), _) => entry.some(),
            (None, Some(prev_env)) => prev_env.find_entry(ref_name),
            _ => None
        }
    }

    pub fn get_expr_and_env<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<(&Expr, &ExprEnv)> {
        self.find_entry(ref_name)
            .and_then(|(.., src, src_env)| {
                let src_env = match src_env {
                    Some(env) => env,
                    // 如果找不到源环境, 则说明该引用存在于顶层环境, 即当前环境
                    None => self
                };
                let src_expr = match src {
                    Some(expr) => expr,
                    None => return None
                };
                (src_expr, src_env).some()
            })
    }
}
