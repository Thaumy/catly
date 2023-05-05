use crate::infer::env::expr_env::{ExprEnv, ExprEnvEntry};
use crate::infer::env::r#type::env_ref_src::EnvRefSrc;
use crate::infer::env::r#type::type_constraint::TypeConstraint;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::env_ref_constraint::EnvRefConstraint;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::option::OptionAnyExt;
use crate::infra::quad::Quad;
use crate::infra::result::ResultAnyExt;
use crate::infra::vec::VecExt;

enum EntryAction {
    // 表示已经推导完成的 def, 需要从待推导 def 列表中移出
    Remove(ExprEnvEntry, EnvRefConstraint),
    // 保留在待推导 def 列表中, 留作下一轮次推导
    Keep(ExprEnvEntry, EnvRefConstraint)
}

pub struct InferErr {
    pub info: String
}

impl InferErr {
    pub fn of(info: impl Into<String>) -> InferErr {
        InferErr { info: info.into() }
    }
}

pub fn infer_type_of_defs<'t>(
    type_env: TypeEnv,
    expr_env: ExprEnv,
    expr_env_vec: Vec<ExprEnvEntry>
) -> Result<Vec<ExprEnvEntry>, InferErr> {
    let expr_env = expr_env.extend_vec_new(expr_env_vec.clone());

    let (need_to_infer, inferred, constraint_acc) = expr_env_vec
        .clone()
        .into_iter()
        .map(|(n, tc, src)| match &src {
            EnvRefSrc::Src(src_expr) => match src_expr
                .clone()
                .with_opt_fallback_type(&tc.clone().into())
                .infer_type(&type_env, &expr_env)
            {
                Quad::L(typed_src_expr) => {
                    let entry = {
                        let tc = match tc {
                            TypeConstraint::Constraint(_) => tc,
                            TypeConstraint::Free =>
                                TypeConstraint::Constraint(
                                    typed_src_expr
                                        .unwrap_type_annot()
                                        .clone()
                                ),
                        };

                        (n, tc, EnvRefSrc::Src(typed_src_expr))
                    };

                    EntryAction::Remove(
                        entry,
                        EnvRefConstraint::empty()
                    )
                    .ok()
                }
                Quad::ML(rc) => {
                    let entry = {
                        let tc = match tc {
                            TypeConstraint::Constraint(_) => tc,
                            TypeConstraint::Free =>
                                TypeConstraint::Constraint(
                                    rc.typed_expr
                                        .unwrap_type_annot()
                                        .clone()
                                ),
                        };

                        (n, tc, EnvRefSrc::Src(rc.typed_expr))
                    };

                    EntryAction::Remove(entry, rc.constraint).ok()
                }
                Quad::MR(ri) => {
                    let entry = (n, tc, src);
                    EntryAction::Keep(entry, ri.constraint).ok()
                }
                Quad::R(e) => InferErr::of(e.info).err()
            },
            EnvRefSrc::NoSrc => match tc {
                TypeConstraint::Constraint(_) => {
                    let entry = (n, tc, EnvRefSrc::NoSrc);
                    EntryAction::Remove(
                        entry,
                        EnvRefConstraint::empty()
                    )
                    .ok()
                }
                TypeConstraint::Free => {
                    let entry = (n, tc, src);
                    EntryAction::Keep(
                        entry,
                        EnvRefConstraint::empty()
                    )
                    .ok()
                }
            }
        })
        .try_fold(
            (vec![], vec![], EnvRefConstraint::empty()),
            |(need_to_infer, inferred, constraint_acc), x| match x {
                Ok(action) => match action {
                    EntryAction::Keep(entry, c) => {
                        let constraint_acc = match constraint_acc
                            .extend_new(c.clone())
                        {
                            Some(c) => c.ok(),
                            None => InferErr::of(
                                TypeMissMatch::of_constraint(
                                    &constraint_acc,
                                    &c
                                )
                                .info
                            )
                            .err()
                        }?;

                        (
                            need_to_infer.chain_push(entry),
                            inferred,
                            constraint_acc
                        )
                            .ok()
                    }
                    EntryAction::Remove(entry, c) => {
                        let constraint_acc = match constraint_acc
                            .extend_new(c.clone())
                        {
                            Some(c) => c.ok(),
                            None => InferErr::of(
                                TypeMissMatch::of_constraint(
                                    &constraint_acc,
                                    &c
                                )
                                .info
                            )
                            .err()
                        }?;

                        (
                            need_to_infer,
                            inferred.chain_push(entry),
                            constraint_acc
                        )
                            .ok()
                    }
                },
                Err(e) => e.err()
            }
        )?;

    if need_to_infer.is_empty() {
        // 当没有 def 需要推导时
        // 此时 constraint_acc 一定为空, 因为不存在类型不确定的 def 可供约束
        inferred.ok()
    } else if constraint_acc.is_empty() {
        // 仍有 def 需要推导, 但本轮次并未产生新的约束
        InferErr::of("Need info to infer defs").err()
    } else {
        // 仍有 def 需要推导, 且本轮次产生了新的约束
        // 将已推导出类型的 def 和约束合并到环境, 进行下一轮推导

        let need_to_infer = need_to_infer
            .into_iter()
            // 对于推导产生的约束, 将约束作用于目标, 留给下一轮次推导使用
            .map(|(n, tc, src)| match tc {
                TypeConstraint::Free => {
                    let tc = match constraint_acc
                        .iter()
                        .find(|(inferred_n, t)| inferred_n == &&n)
                    {
                        Some((_, t)) => t.clone().into(),
                        None => tc
                    };
                    (n, tc, src)
                }
                // 推导也可能向环境中注入部分类型
                TypeConstraint::Constraint(ref t)
                    if t.is_partial() =>
                {
                    let tc = match constraint_acc
                        .iter()
                        .find(|(inferred_n, t)| inferred_n == &&n)
                    {
                        Some((_, t)) => t.clone().into(),
                        None => tc
                    };
                    (n, tc, src)
                }
                _ => (n, tc, src)
            })
            .collect();

        // 对于完成推导的 def, 去除其引用源以防止被再次推导
        let new_expr_env_vec = inferred
            .clone()
            .into_iter()
            .map(|(n, tc, _)| (n, tc, EnvRefSrc::NoSrc))
            .collect();

        let new_expr_env = ExprEnv::new(new_expr_env_vec);

        // 收集下一轮推导的结果, 与当前轮次的推导结果合并后返回
        let inferred_from_next_round = infer_type_of_defs(
            type_env,
            new_expr_env,
            need_to_infer
        )?;

        vec![inferred, inferred_from_next_round]
            .concat()
            .ok()
    }
}
