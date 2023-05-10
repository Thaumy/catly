use std::rc::Rc;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::r#type::env_ref_src::EnvRefSrc;
use crate::infer::env::r#type::type_constraint::TypeConstraint;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::require_constraint::require_constraint;
use crate::infer::infer_type::r#type::require_info::ReqInfo;
use crate::infra::option::OptionAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;

impl ExprEnv {
    pub fn infer_type<'s>(
        self: &Rc<Self>,
        type_env: &TypeEnv,
        ref_name: impl Into<&'s str> + Clone
    ) -> InferTypeRet {
        let ref_name = ref_name.into().to_string();

        let tc_and_src = self
            .find_entry(ref_name.as_str())
            .map(|(_, tc, src)| (tc, src));

        match tc_and_src {
            // 当前环境查找到引用名, 但不存在引用源
            Some((tc, EnvRefSrc::NoSrc)) => match tc {
                // 引用名所对应的类型是类型约束的直接类型
                TypeConstraint::Constraint(t) =>
                    InferTypeRet::has_type(Expr::EnvRef(
                        t.clone().some(),
                        ref_name
                    )),
                // 不存在类型约束
                TypeConstraint::Free =>
                    ReqInfo::of(ref_name, EnvRefConstraint::empty())
                        .into(),
            },

            // 当前环境查找到引用名, 且存在引用源
            Some((tc, EnvRefSrc::Src(src_expr))) => {
                // 为防止递归调用导致类型检查不能终止, 需要将引用名去源后注入环境
                let new_expr_env = self.extend_new(
                    ref_name.clone(),
                    tc.clone().into(),
                    None
                );

                match tc {
                    // 具备约束
                    TypeConstraint::Constraint(t) => match src_expr
                        .with_fallback_type(t)
                        .infer_type(type_env, &new_expr_env)?
                    {
                        Triple::L(typed_src_expr) =>
                            InferTypeRet::from_auto_lift(
                                type_env,
                                &typed_src_expr.unwrap_type_annot(),
                                &t.clone().some(),
                                None,
                                // 由于这里构建的是具备类型的 EnvRef, 所以不应该使用引用源返回
                                |t| {
                                    Expr::EnvRef(
                                        t.clone().some(),
                                        ref_name.clone()
                                    )
                                }
                            ),
                        Triple::M(rc) =>
                            InferTypeRet::from_auto_lift(
                                type_env,
                                &rc.typed_expr
                                    .unwrap_type_annot(),
                                &t.clone().some(),
                                rc.constraint.some(),
                                // 与上同理
                                |t| {
                                    Expr::EnvRef(
                                        t.clone().some(),
                                        ref_name.clone()
                                    )
                                }
                            ),
                        // 如果引用源是无类型弃元
                        Triple::R(ri) if ri.ref_name == "_" =>
                        // 为了防止无类型弃元信息被捕获, 改写错误信息
                            ri.new_ref_name(ref_name)
                                .into(),

                        ri => ri.into()
                    },

                    // 不具备约束
                    TypeConstraint::Free => match src_expr
                        .infer_type(type_env, &new_expr_env)?
                    {
                        Triple::L(typed_src_expr) =>
                            InferTypeRet::has_type(Expr::EnvRef(
                                typed_src_expr
                                    .unwrap_type_annot()
                                    .clone()
                                    .some(),
                                ref_name
                            )),
                        // 由于 ref_name 是 Free 的, 所以此时约束可能作用于 ref_name 本身
                        // 此时作用于 ref_name 的约束相当于 ref_name 的固有类型, 只需将其他约束按需传播
                        // 如果引用源是无类型弃元
                        Triple::M(rc) => require_constraint(
                            rc.typed_expr,
                            rc.constraint
                                .exclude_new(ref_name.as_str())
                        ),
                        Triple::R(ri) if ri.ref_name == "_" =>
                        // 为了防止无类型弃元信息被捕获, 改写错误信息
                            ri.new_ref_name(ref_name)
                                .into(),

                        ri => ri.into()
                    }
                }
            }

            // 缺乏推导信息
            None => ReqInfo::of(ref_name, EnvRefConstraint::empty())
                .into()
        }
    }
}
