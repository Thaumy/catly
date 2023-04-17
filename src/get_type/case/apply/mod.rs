mod case_ri;
mod case_t_rc;

use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::get_type::case::apply::case_ri::case_ri;
use crate::get_type::case::apply::case_t_rc::case_t_rc;
use crate::get_type::get_type;
use crate::get_type::r#type::GetTypeReturn;
use crate::infra::alias::MaybeType;
use crate::infra::quad::Quad;
use crate::parser::expr::r#type::Expr;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    lhs_expr: &Expr,
    rhs_expr: &Expr
) -> GetTypeReturn {
    let lhs_expr_type = get_type(type_env, expr_env, &lhs_expr);

    match lhs_expr_type {
        Quad::L(_) | Quad::ML(_) => case_t_rc(
            type_env,
            expr_env,
            lhs_expr_type,
            expect_type,
            rhs_expr
        ),

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
