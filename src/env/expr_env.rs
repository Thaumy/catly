use crate::env::env_ref_src::EnvRefSrc;
use crate::env::type_constraint::TypeConstraint;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::{MaybeExpr, MaybeType};
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::lift_or_left;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{
    has_type,
    require_constraint,
    require_info,
    type_miss_match
};

type Item = (String, TypeConstraint, EnvRefSrc);

// 表达式环境
#[derive(Clone, Debug)]
pub struct ExprEnv<'t> {
    type_env: TypeEnv,
    prev_env: Option<&'t ExprEnv<'t>>,
    env: Vec<Item>
}

impl<'t> ExprEnv<'t> {
    pub fn new(type_env: TypeEnv, vec: Vec<Item>) -> ExprEnv<'t> {
        ExprEnv {
            type_env,
            prev_env: None,
            env: vec
        }
    }

    pub fn extend_vec_new(&self, vec: Vec<Item>) -> ExprEnv {
        ExprEnv {
            type_env: self.type_env.clone(),
            prev_env: Some(self),
            env: vec
        }
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
            // 环境引用不存在引用源, 引用名所对应的类型是类型约束的直接类型
            Some((tc, EnvRefSrc::NoSrc)) => match tc {
                TypeConstraint::Constraint(t) => has_type!(t),
                TypeConstraint::Free => return None
            },
            // 环境引用存在引用源
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
                            Some(t) =>
                                require_constraint!(t.clone(), vec![
                                    (ref_name.to_string(), t.clone())
                                ]),
                            // 不具备 hint, 为了防止无类型弃元信息传播, 修改错误信息
                            None =>
                                require_info!(ref_name.to_string()),
                        },
                    mr_r => mr_r
                },
            None =>
                return match self.prev_env {
                    Some(prev_env) =>
                        prev_env.get_type_with_hint(ref_name, hint),
                    None => None
                },
        }
        .some()
    }

    pub fn exist_ref(&self, ref_name: &str) -> bool {
        match self
            .env
            .iter()
            .rev()
            .any(|(n, ..)| n == ref_name)
        {
            true => true,
            false => match self.prev_env {
                Some(env) => env.exist_ref(ref_name),
                None => false
            }
        }
    }
}
