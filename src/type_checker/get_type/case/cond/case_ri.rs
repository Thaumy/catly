use crate::env::expr_env::ExprEnv;
use crate::env::type_env::TypeEnv;
use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::infra::r#box::Ext;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::type_checker::get_type::{get_type, get_type_with_hint};

pub fn case_ri(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    expect_type: &MaybeType,
    bool_expr: &Expr,
    else_expr: &Expr,
    then_expr: &Expr
) -> GetTypeReturn {
    let else_expr_type = match get_type_with_hint(
        type_env,
        expr_env,
        else_expr,
        &expect_type
    ) {
        Quad::L(t) => t,
        // 无需手机约束, 约束会在下次调用 get_type 时被自动处理
        Quad::ML(rc) => rc.r#type,
        mr_r => return mr_r.clone()
    };

    let expr = Expr::Cond(
        else_expr_type.some(),
        bool_expr.clone().boxed(),
        then_expr.clone().boxed(),
        else_expr.clone().boxed()
    );

    get_type(type_env, expr_env, &expr)
}
