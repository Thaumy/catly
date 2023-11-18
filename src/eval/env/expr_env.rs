use std::rc::Rc;

use crate::eval::Expr;
use crate::eval::Type;
use crate::infra::WrapOption;
use crate::infra::WrapRc;

// 某些表达式可能是递归定义的(常见于顶层环境和 Let)
// 对于这样的表达式, 其求值环境将具具备自引用结构
// 为了方便实现, 将具备自引用结构的环境引用设置为 None
// 通过在 None 时返回当前环境, 就能实现递归定义
pub type ExprEnvEntry =
    (String, Type, Option<Rc<Expr>>, Option<Rc<ExprEnv>>);

// 运行时表达式环境
#[derive(Clone, Debug, PartialEq)]
pub struct ExprEnv {
    prev_env: Option<Rc<ExprEnv>>,
    // 单一环境条目会使得顶层环境中平行函数的互递归定义成为不可能, 但仍有实现价值
    entry: Option<ExprEnvEntry>
}

fn source_env_ref(
    expr: Rc<Expr>,
    expr_env: Rc<ExprEnv>
) -> (Rc<Expr>, Rc<ExprEnv>) {
    match expr.as_ref() {
        Expr::EnvRef(_, ref_name) => {
            match expr_env.get_src_expr_and_env(ref_name.as_str()) {
                Some((expr, expr_env)) =>
                    source_env_ref(expr, expr_env),
                _ => (expr, expr_env)
            }
        }
        _ => (expr, expr_env)
    }
}

impl ExprEnv {
    pub fn empty() -> ExprEnv {
        ExprEnv {
            prev_env: None,
            entry: None
        }
    }

    // TODO:
    // 此处为逐层查找 env_ref
    // 可以设置穿透的访问链, 提高 env_ref 的检索效率
    pub fn new(
        ref_name: impl Into<String>,
        r#type: Type,
        src: Option<Rc<Expr>>,
        src_env: Option<Rc<ExprEnv>>
    ) -> ExprEnv {
        let (src, src_env) = match (src, src_env) {
            (Some(expr), Some(src_env))
                if matches!(expr.as_ref(), Expr::EnvRef(..)) =>
            {
                let (src, src_env) = source_env_ref(expr, src_env);
                (src.wrap_some(), src_env.wrap_some())
            }
            x => x
        };

        let entry = (ref_name.into(), r#type, src, src_env);

        let expr_env = ExprEnv {
            prev_env: None,
            entry: entry.wrap_some()
        };

        #[cfg(feature = "rt_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "ExprEnv", expr_env.entry
            );
            println!("{log}");
        }

        expr_env
    }

    fn latest_none_empty_expr_env(self: &Rc<Self>) -> Rc<ExprEnv> {
        match (self.entry.is_none(), &self.prev_env) {
            (true, Some(prev_env)) => prev_env
                .clone()
                .latest_none_empty_expr_env(),
            _ => self.clone()
        }
    }

    // TODO
    pub fn extend_new(
        self: &Rc<Self>,
        ref_name: impl Into<String>,
        r#type: Type,
        src: Option<Rc<Expr>>,
        src_env: Option<Rc<ExprEnv>>
    ) -> ExprEnv {
        let (src, src_env) = match (src, src_env) {
            (Some(expr), Some(src_env))
                if matches!(expr.as_ref(), Expr::EnvRef(..)) =>
            {
                let (src, src_env) = source_env_ref(expr, src_env);
                (src.wrap_some(), src_env.wrap_some())
            }
            x => x
        };

        let entry = (ref_name.into(), r#type, src, src_env);

        let expr_env = ExprEnv {
            prev_env: self
                .latest_none_empty_expr_env()
                .wrap_some(),
            entry: entry.wrap_some()
        };

        #[cfg(feature = "rt_env_log")]
        {
            let log = format!(
                "{:8}{:>10} │ {:?}",
                "[rt env]", "ExprEnv", expr_env.entry
            );
            println!("{log}");
        }

        expr_env
    }

    pub fn extend_vec_new(
        self: &Rc<Self>,
        expr_env_vec: Vec<ExprEnvEntry>
    ) -> Rc<ExprEnv> {
        expr_env_vec.into_iter().fold(
            self.clone(),
            |acc, (r_n, t, src, src_env)| {
                acc.extend_new(r_n.as_str(), t, src, src_env)
                    .wrap_rc()
            }
        )
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

    // TODO: Rc<Expr>
    pub fn get_src_expr_and_env<'s>(
        self: &Rc<Self>,
        ref_name: impl Into<&'s str>
    ) -> Option<(Rc<Expr>, Rc<ExprEnv>)> {
        self.find_entry(ref_name)
            .and_then(|(.., src, src_env)| {
                let src_env = match src_env {
                    Some(env) => env.clone(),
                    // 如果找不到源环境, 则说明该引用存在于顶层环境, 即当前环境
                    None => self.clone()
                };
                let src_expr = match src {
                    Some(expr) => expr.clone(),
                    None => return None
                };
                (src_expr, src_env).wrap_some()
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
                    None => Rc::new(self.clone())
                };
                (Expr::EnvRef(t.clone(), ref_name.clone()), src_env)
                    .wrap_some()
            })
    }
}
