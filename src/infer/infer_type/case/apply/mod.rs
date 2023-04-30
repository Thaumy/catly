mod case_ri;
mod case_t_rc;
#[cfg(test)]
mod test;

use std::ops::Deref;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::apply::case_ri::case_ri;
use crate::infer::infer_type::case::apply::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infer::infer_type::r#type::type_miss_match::TypeMissMatch;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> InferTypeRet {
    match lhs_expr.infer_type(type_env, expr_env) {
        lhs_expr_type @ (Quad::L(_) | Quad::ML(_)) => {
            let (lhs_expr_type, constraint_acc) =
                lhs_expr_type.unwrap_type_and_constraint();

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
                    return TypeMissMatch::of(&format!(
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
                constraint_acc,
                expect_type,
                rhs_expr
            )
        }

        // 使用 expect_type 和 rhs_expr_type 进行旁路推导
        // 仅在 lhs_expr 缺乏类型标注时进行处理
        Quad::MR(ri) if lhs_expr.is_no_type_annot() => {
            let new_expr_env = &expr_env
                .extend_constraint_new(ri.constraint.clone());

            case_ri(
                type_env,
                new_expr_env,
                ri.constraint,
                expect_type,
                &lhs_expr,
                rhs_expr
            )
        }

        mr_r => mr_r
    }
}
