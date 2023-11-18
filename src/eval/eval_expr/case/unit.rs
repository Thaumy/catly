use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::Expr;
use crate::eval::r#type::Type;
use crate::infra::result::WrapResult;

pub fn case_unit(type_annot: &Type) -> EvalRet {
    Expr::Unit(type_annot.clone()).wrap_ok()
}

#[cfg(test)]
mod test {
    use crate::eval::env::expr_env::ExprEnv;
    use crate::eval::env::type_env::TypeEnv;
    use crate::eval::eval_expr::eval_expr;
    use crate::eval::r#macro::namely_type;
    use crate::eval::r#type::Expr;
    use crate::infra::rc::RcAnyExt;
    use crate::infra::result::WrapResult;

    // (): Unit
    #[test]
    fn test_part1() {
        let type_env = TypeEnv::new(vec![]);
        let expr_env = ExprEnv::empty().rc();

        let expr = Expr::Unit(namely_type!("Unit"));
        let evaluated =
            eval_expr(&type_env, &expr_env, &expr.clone().rc());

        assert_eq!(evaluated, expr.wrap_ok());
    }
}
