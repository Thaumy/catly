use std::ops::Deref;
use std::rc::Rc;

use crate::eval::env::ExprEnv;
use crate::eval::env::TypeEnv;
use crate::eval::eval_expr;
use crate::eval::EvalErr;
use crate::eval::Expr;
use crate::eval::PrimitiveOp;
use crate::eval::Type;
use crate::infra::WrapRc;
use crate::infra::WrapResult;
use crate::infra::{Either, WrapEither};

pub fn source_lhs_expr_to_closure(
    type_env: &'_ TypeEnv,
    expr_env: &Rc<ExprEnv>,
    lhs_expr: &Rc<Expr>
) -> Result<
    Either<
        (Option<String>, Type, Rc<Expr>, Rc<ExprEnv>),
        (PrimitiveOp, Rc<ExprEnv>)
    >,
    EvalErr
> {
    match lhs_expr.as_ref() {
        Expr::Closure(
            _,
            input_name,
            input_type,
            output_expr,
            eval_env
        ) => (
            input_name.clone(),
            input_type.clone(),
            output_expr.clone(),
            // 如果 Closure 是 Apply 的直接左操作数, 那么它一定还没捕获环境
            // 它将在这里使用当前的环境作为求值环境
            // 否则, Closure 将捕获到其他环境, 并将其用作求值环境
            eval_env
                .clone()
                .unwrap_or(expr_env.clone())
        )
            .wrap_l()
            .wrap_ok(),

        Expr::PrimitiveOp(_, op, lhs_eval_env) => (
            op.deref().clone(),
            // 此处, PrimitiveOp 通过 Apply 捕获环境(当还没有左操作数时)
            // 当 PrimitiveOp 具备左操作数时, 环境已被捕获, 该环境将被用作求值环境
            lhs_eval_env
                .clone()
                .unwrap_or(expr_env.clone())
        )
            .wrap_r()
            .wrap_ok(),

        // 由于现在 Closure 和 PrimitiveOp 会捕获环境
        // 所以可以对 lhs_expr 进行自由求值
        _ => source_lhs_expr_to_closure(
            type_env,
            expr_env,
            &eval_expr(type_env, expr_env, lhs_expr)?.wrap_rc()
        )
    }
}
