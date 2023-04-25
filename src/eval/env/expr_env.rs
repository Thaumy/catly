use crate::eval::env::type_env::TypeEnv;
use crate::eval::r#type::expr::Expr;
use crate::eval::r#type::r#type::Type;
use crate::infra::option::AnyExt;

pub type EnvEntry = (String, Type, Expr);

// 运行时表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv<'t> {
    type_env: TypeEnv,
    prev_env: Option<&'t ExprEnv<'t>>,
    env: Vec<EnvEntry>
}

impl<'t> ExprEnv<'t> {
    pub fn empty(type_env: TypeEnv) -> ExprEnv<'t> {
        Self::new(type_env, vec![])
    }

    pub fn new(
        type_env: TypeEnv,
        env_vec: Vec<EnvEntry>
    ) -> ExprEnv<'t> {
        let expr_env = ExprEnv {
            type_env,
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

    fn latest_none_empty_expr_env(&self) -> &ExprEnv {
        match (self.env.is_empty(), self.prev_env) {
            (true, Some(prev_env)) =>
                prev_env.latest_none_empty_expr_env(),
            _ => self
        }
    }

    pub fn extend_vec_new(&self, env_vec: Vec<EnvEntry>) -> ExprEnv {
        let expr_env = ExprEnv {
            type_env: self.type_env.clone(),
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
        src: Expr
    ) -> ExprEnv {
        let expr_env = self.extend_vec_new(vec![(
            ref_name.into(),
            r#type,
            src.into()
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

    fn find_entry<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<&EnvEntry> {
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
            .map(|(.., src)| src)
    }

    pub fn get_ref<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<Expr> {
        self.find_entry(ref_name)
            .map(|(n, t, _)| Expr::EnvRef(t.clone(), n.to_string()))
    }
}
