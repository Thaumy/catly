use crate::infra::alias::MaybeType;
use crate::infra::option::AnyExt;
use crate::infra::quad::Quad;
use crate::parser::expr::Expr;
use crate::type_checker::env::expr_env::ExprEnv;
use crate::type_checker::env::type_env::TypeEnv;
use crate::type_checker::get_type::get_type_with_hint;
use crate::type_checker::get_type::r#fn::{
    lift_or_left,
    with_constraint_lift_or_left
};
use crate::type_checker::get_type::r#type::GetTypeReturn;
use crate::{has_type, type_miss_match};

pub fn case_t_rc(
    type_env: &TypeEnv,
    expr_env: &ExprEnv,
    assign_expr_type: GetTypeReturn,
    expect_type: &MaybeType,
    assign_name: &str,
    assign_type: &MaybeType,
    scope_expr: &Expr
) -> GetTypeReturn {
    // 合并处理是为了节省代码量
    let (assign_expr_type, constraint) = match assign_expr_type {
        Quad::L(t) => (t, vec![]),
        // 需传播额外携带的约束
        Quad::ML(rc) => (rc.r#type, rc.constraint),
        x => panic!("Impossible assign_expr_type: {:?}", x)
    };

    // Lift assign_expr_type to assign_type
    let assign_type = match lift_or_left(
        type_env,
        &assign_expr_type,
        assign_type
    ) {
        None => return type_miss_match!(),
        Some(t) => t
    };

    // Env inject
    let expr_env = expr_env
        .extend_new(assign_name.to_string(), assign_type.some());

    // Hint scope_expr with expect_type and get scope_expr_type
    let scope_expr_type = get_type_with_hint(
        type_env,
        &expr_env,
        scope_expr,
        expect_type
    );

    match scope_expr_type {
        Quad::L(scope_expr_type) => match lift_or_left(
            type_env,
            &scope_expr_type,
            expect_type
        ) {
            Some(t) => has_type!(t),
            None => type_miss_match!()
        },
        // 由于 assign_type 存在, 所以此处的约束作用于外层环境, 传播之
        Quad::ML(rc) => with_constraint_lift_or_left(
            vec![constraint, rc.constraint].concat(),
            type_env,
            &rc.r#type,
            expect_type
        ),
        // 由于 scope_expr 已被 hint, 且环境已被尽力注入, 所以无法处理这些错误
        mr_r => mr_r
    }
}
