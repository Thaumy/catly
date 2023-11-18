use crate::infer::env::type_env::TypeEnv;
use crate::infer::infer_type::InferTypeRet;
use crate::infra::rc::RcAnyExt;
use crate::parser::expr::r#type::Expr;
use crate::parser::r#type::r#type::OptType;
use crate::parser::r#type::r#type::Type;

pub fn has_input_type<F>(
    type_env: &TypeEnv,
    expect_type: &OptType,
    output_expr_type: &Type,
    input_type: Type,
    typed_expr_cons: F
) -> InferTypeRet
where
    F: Fn(Type, Type) -> Expr
{
    let base = Type::ClosureType(
        input_type.clone().rc(),
        output_expr_type.clone().rc()
    );

    InferTypeRet::from_auto_lift(
        type_env,
        &base,
        expect_type,
        None,
        |t| typed_expr_cons(t, input_type.clone())
    )
}
