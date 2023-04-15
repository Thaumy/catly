use crate::env::env_ref_src::EnvRefSrc;
use crate::env::type_constraint::TypeConstraint;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::{MaybeExpr, MaybeType};
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::lift_or_left;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{
    has_type,
    require_constraint,
    require_info,
    single_constraint,
    type_miss_match
};

type Item = (String, TypeConstraint, EnvRefSrc);

// 表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv<'t> {
    type_env: TypeEnv,
    prev_env: Option<&'t ExprEnv<'t>>,
    // TODO: 使用 Option 减少创建空环境时不必要的堆分配
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

    pub fn get_type_with_hint(
        &self,
        ref_name: &str,
        hint: &MaybeType
    ) -> Option<GetTypeReturn> {
        match self
            .env
            .iter()
            .rev()
            .find(|(n, ..)| n == ref_name)
            .map(|(_, tc, src)| (tc.clone(), src))
        {
            // 当前环境查找到引用名, 但不存在引用源
            Some((tc, EnvRefSrc::NoSrc)) => match tc {
                // 引用名所对应的类型是类型约束的直接类型
                TypeConstraint::Constraint(t) => has_type!(t),
                // 不存在类型约束, 缺乏类型信息
                // TODO: 返回类型的合理性或重构
                TypeConstraint::Free => return None
            },
            // 当前环境查找到引用名, 且存在引用源
            // 以约束为 hint 获取引用源类型, 并提升到 hint
            Some((tc, EnvRefSrc::Src(src_expr))) =>
                match get_type_with_hint(
                    &self.type_env,
                    self,
                    src_expr,
                    &tc.clone().into()
                ) {
                    Quad::L(src_expr_type) => match lift_or_left(
                        &self.type_env,
                        &src_expr_type,
                        &tc.into()
                    ) {
                        Some(t) => has_type!(t),
                        None => type_miss_match!()
                    },
                    Quad::ML(rc) => match lift_or_left(
                        &self.type_env,
                        &rc.r#type,
                        &tc.into()
                    ) {
                        Some(t) =>
                            require_constraint!(t, rc.constraint),
                        None => type_miss_match!()
                    },
                    // 如果引用源是无类型弃元
                    Quad::MR(ri) if ri.ref_name == "_" =>
                        match hint {
                            // 具备 hint, 可以将引用名约束到 hint, 传播该约束
                            Some(t) => require_constraint!(
                                t.clone(),
                                single_constraint!(
                                    ref_name.to_string(),
                                    t.clone()
                                )
                            ),
                            // 不具备 hint, 为了防止无类型弃元信息被捕获, 改写错误信息
                            None =>
                                require_info!(ref_name.to_string()),
                        },
                    // 无法处理其他情况
                    mr_r => mr_r
                },
            // 当前环境查找不到, 去外层环境查找
            None =>
                return match self.prev_env {
                    Some(prev_env) =>
                        prev_env.get_type_with_hint(ref_name, hint),
                    None => None
                },
        }
        .some()
    }

    pub fn get_type(&self, ref_name: &str) -> Option<GetTypeReturn> {
        self.get_type_with_hint(ref_name, &None)
    }

    pub fn get_expr(&self, ref_name: &str) -> Option<Expr> {
        let expr = self
            .env
            .iter()
            .rev()
            .find(|(n, ..)| n == ref_name)
            .map(|(.., t)| t.clone().into())
            .flatten(): Option<Expr>;

        match (expr, self.prev_env) {
            (Some(expr), _) => expr.some(),
            (None, Some(prev_env)) => prev_env.get_expr(ref_name),
            _ => None
        }
    }

    pub fn get_ref(&self, ref_name: &str) -> Option<Expr> {
        let expr = self
            .env
            .iter()
            .rev()
            .find(|(n, ..)| n == ref_name)
            .map(|(n, tc, _)| {
                Expr::EnvRef(tc.clone().into(), n.to_string())
            }): Option<Expr>;

        match (expr, self.prev_env) {
            (Some(expr), _) => expr.some(),
            (None, Some(prev_env)) => prev_env.get_expr(ref_name),
            _ => None
        }
    }

    pub fn exist_ref(&self, ref_name: &str) -> bool {
        let is_exist = self
            .env
            .iter()
            .rev()
            .any(|(n, ..)| n == ref_name);

        match (is_exist, self.prev_env) {
            (true, _) => true,
            (false, Some(prev_env)) => prev_env.exist_ref(ref_name),
            _ => false
        }
    }
}
