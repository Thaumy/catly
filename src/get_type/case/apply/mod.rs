mod case_ri;
mod case_t_rc;

use std::ops::Deref;

use crate::env::expr_env::ExprEnv;
use crate::env::r#type::type_env::TypeEnv;
use crate::get_type::case::apply::case_ri::case_ri;
use crate::get_type::case::apply::case_t_rc::case_t_rc;
use crate::get_type::get_type;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::Type;
use crate::{empty_constraint, type_miss_match};

// TODO: 外部环境约束同层传播完备性
pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> GetTypeReturn {
    let lhs_expr_type = get_type(type_env, expr_env, &lhs_expr);

    match lhs_expr_type {
        Quad::L(_) | Quad::ML(_) => {
            let (lhs_expr_type, constraint_acc) = match lhs_expr_type
            {
                Quad::L(lhs_type) => (lhs_type, empty_constraint!()),
                Quad::ML(rc) => (rc.r#type, rc.constraint),
                _ => panic!(
                    "Impossible lhs_expr_type: {lhs_expr_type:?}"
                )
            };

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
                    return type_miss_match!(format!(
                        "{lhs_expr_type:?} <> ClosureType"
                    ));
                };

            // TODO: 相似用例检查
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
        Quad::MR(require_info) if lhs_expr.is_no_type_annot() =>
            case_ri(
                type_env,
                expr_env,
                require_info,
                expect_type,
                &lhs_expr,
                rhs_expr
            ),

        mr_r => mr_r
    }
}
