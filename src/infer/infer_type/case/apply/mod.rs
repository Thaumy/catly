mod case_ri;
mod case_t_rc;
#[cfg(test)]
mod test;

use std::ops::Deref;
use std::rc::Rc;

use case_ri::case_ri;
use case_t_rc::case_t_rc;

use crate::infer::env::ExprEnv;
use crate::infer::env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infer::infer_type::TypeMissMatch;
use crate::infra::Triple;
use crate::infra::WrapOption;
use crate::infra::WrapRc;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::OptType;
use crate::parser::r#type::Type;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &Rc<ExprEnv>,
    expect_type: &OptType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> InferTypeRet {
    match lhs_expr.infer_type(type_env, expr_env)? {
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_lhs_expr, constraint_acc) =
                result.unwrap_expr_constraint();

            let lhs_expr_type = typed_lhs_expr.unwrap_type_annot();

            let (lhs_input_type, lhs_output_type) =
                if let Type::ClosureType(input_type, output_type) =
                    lhs_expr_type
                {
                    (
                        input_type.deref().clone(),
                        output_type.deref().clone()
                    )
                } else {
                    // lhs_expr_type must be ClosureType
                    // PartialClosureType is used for hint only
                    return TypeMissMatch::of(format!(
                        "{lhs_expr_type:?} <> ClosureType"
                    ))
                    .into();
                };

            // 注入获得 lhs_expr_type 时得到的约束到环境, 这些约束可能对取得 rhs_expr_type 有所帮助
            let new_expr_env = &expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_t_rc(
                type_env,
                new_expr_env,
                lhs_input_type,
                lhs_output_type,
                expect_type,
                rhs_expr,
                |type_annot, typed_rhs_expr| {
                    Expr::Apply(
                        type_annot.wrap_some(),
                        typed_lhs_expr
                            .clone()
                            .wrap_rc(),
                        typed_rhs_expr.wrap_rc()
                    )
                }
            )?
            .with_constraint_acc(constraint_acc)
        }

        // 使用 expect_type 和 rhs_expr_type 进行旁路推导
        // 仅在 lhs_expr 缺乏类型标注时进行处理
        Triple::R(ri) if lhs_expr.is_no_type_annot() => {
            let new_expr_env = &expr_env
                .extend_constraint_new(ri.constraint.clone());

            case_ri(
                type_env,
                new_expr_env,
                expect_type,
                lhs_expr,
                rhs_expr
            )?
            .with_constraint_acc(ri.constraint)
        }

        ri => ri.into()
    }
}
