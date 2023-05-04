use crate::infer::env::expr_env::ExprEnv;
use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::r#type::infer_type_ret::InferTypeRet;
use crate::infra::r#box::BoxAnyExt;
use crate::infra::triple::Triple;
use crate::parser::expr::r#type::Expr;

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    bool_expr: &Expr,
    else_expr: &Expr,
    then_expr: &Expr
) -> InferTypeRet {
    match else_expr.infer_type(type_env, expr_env)? {
        // 需要收集这些作用于外层环境的约束并传播, 因为它们可能对推导 then_expr_type 有所帮助
        result @ (Triple::L(_) | Triple::M(_)) => {
            let (typed_else_expr, constraint_acc) =
                result.unwrap_expr_constraint();

            let cond_expr = Expr::Cond(
                typed_else_expr
                    .get_type_annot()
                    .cloned(),
                bool_expr.clone().boxed(),
                then_expr.clone().boxed(),
                typed_else_expr
                    .clone()
                    .boxed()
            );

            let new_expr_env = expr_env
                .extend_constraint_new(constraint_acc.clone());

            cond_expr
                .infer_type(type_env, &new_expr_env)?
                .with_constraint_acc(constraint_acc)
        }

        Triple::R(ri) => ri.into()
    }
}
