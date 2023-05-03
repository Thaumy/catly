mod case_ri;
mod case_t_rc;
mod r#fn;
#[cfg(test)]
mod test;

use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::case::r#match::case_ri::case_ri;
use crate::infer::infer_type::case::r#match::case_t_rc::case_t_rc;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;

pub fn case(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &OptType,
    target_expr: &Expr,
    case_vec: &Vec<(Expr, Expr)>
) -> InferTypeRet {
    match target_expr.infer_type(type_env, expr_env)? {
        // L 与 ML 同样只有是否需要传播对外界环境的约束的区别
        target_expr_type @ (Triple::L(_) | Triple::M(_)) => {
            let (target_expr_type, constraint_acc, typed_target_expr) =
                target_expr_type.unwrap_type_constraint_expr();

            let new_expr_env = expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_t_rc(
                type_env,
                &new_expr_env,
                target_expr,
                target_expr_type,
                expect_type,
                case_vec,
                typed_target_expr
            )?
            .with_constraint_acc(constraint_acc)
        }

        // TODO:
        // 考虑是否应该在存在 type_annot 时继续进行旁路类型推导
        // 因为上一轮推导产生的约束可能对推导成功有所帮助

        // 无法获取 target_expr 类型信息, 启用旁路类型推导
        // 同样, 为了防止内层环境对外层环境造成跨越优先级的约束, 仅当 target_expr 没有类型标注时才能启用旁路推导
        // 相关讨论参见 let case
        Triple::R(ri) if target_expr.is_no_type_annot() => {
            let constraint_acc = ri.constraint.clone();

            let new_expr_env = expr_env
                .extend_constraint_new(constraint_acc.clone());

            case_ri(
                type_env,
                &new_expr_env,
                ri,
                expect_type,
                target_expr,
                case_vec
            )?
            .with_constraint_acc(constraint_acc)
        }

        Triple::R(ri) => ri.into()
    }
}
