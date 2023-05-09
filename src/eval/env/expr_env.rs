use std::rc::Rc;

use crate::eval::r#type::expr::{Expr, OptExpr};
use crate::eval::r#type::r#type::Type;
use crate::infra::option::OptionAnyExt;

// 某些表达式可能是递归定义的(常见于顶层环境和 Let)
// 对于这样的表达式, 其求值环境将具具备自引用结构
// 为了方便实现, 将具备自引用结构的环境引用设置为 None
// 通过在 None 时返回当前环境, 就能实现递归定义
pub type ExprEnvEntry = (String, Type, OptExpr, Option<Rc<ExprEnv>>);

// 运行时表达式环境
#[derive(Clone, Debug, PartialEq)]
pub struct ExprEnv {
    prev_env: Option<Rc<ExprEnv>>,
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

    fn latest_none_empty_expr_env(&self) -> Rc<ExprEnv> {
        match (self.env.is_empty(), &self.prev_env) {
            (true, Some(prev_env)) =>
                prev_env.latest_none_empty_expr_env(),
            _ => Rc::new(self.clone())
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
        src_env: Rc<ExprEnv>
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

    pub fn extend_rec_new(
        &self,
        ref_name: impl Into<String>,
        r#type: Type,
        src_expr: Expr
    ) -> ExprEnv {
        let expr_env = self.extend_vec_new(vec![(
            ref_name.into(),
            r#type,
            src_expr.into(),
            None
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

    pub fn get_src_expr_and_env<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<(&Expr, Rc<ExprEnv>)> {
        self.find_entry(ref_name)
            .and_then(|(.., src, src_env)| {
                let src_env = match src_env {
                    Some(env) => env.clone(),
                    // 如果找不到源环境, 则说明该引用存在于顶层环境, 即当前环境
                    None => Rc::new(self.clone())
                };
                let src_expr = match src {
                    Some(expr) => expr,
                    None => return None
                };
                (src_expr, src_env).some()
            })
    }

    pub fn get_ref_expr_and_env<'s>(
        &self,
        ref_name: impl Into<&'s str>
    ) -> Option<(Expr, Rc<ExprEnv>)> {
        self.find_entry(ref_name)
            .and_then(|(ref_name, t, _, src_env)| {
                let src_env = match src_env {
                    Some(env) => env.clone(),
                    // 当前环境中存在递归定义的引用条目, 其求值环境应是当前环境
                    // 由于当前环境是从捕获环境中扩展而来的, 所以对于非递归的引用, 仍能从上一环境中得到
                    // TODO: 单一条目的环境块对于此实现至关重要
                    // 因为它让所有非递归引用都从上一环境中得到, 从而阻止了在当前环境中不正确的引用发现
                    // TODO: 顶层环境不具备单一环境块, 应对其重构
                    // 这种重构会使得顶层环境中平行函数的互递归定义成为不可能, 但仍有实现价值
                    None => Rc::new(self.clone())
                };
                (Expr::EnvRef(t.clone(), ref_name.clone()), src_env)
                    .some()
            })
    }
}
