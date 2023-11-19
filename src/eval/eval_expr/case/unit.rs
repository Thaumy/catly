use crate::eval::eval_expr::EvalRet;
use crate::eval::Expr;
use crate::eval::Type;
use crate::infra::WrapResult;

pub fn case_unit(type_annot: &Type) -> EvalRet {
    Expr::Unit(type_annot.clone()).wrap_ok()
}

#[cfg(test)]
mod test {
    use crate::eval::env::ExprEnv;
    use crate::eval::env::TypeEnv;
    use crate::eval::eval_expr;
    use crate::eval::namely_type;
    use crate::eval::Expr;
    use crate::infra::WrapRc;
    use crate::infra::WrapResult;

    // (): Unit
    #[test]
    fn test_part1() {
        let type_env = TypeEnv::new(vec![]);
        let expr_env = ExprEnv::empty().wrap_rc();

        let expr = Expr::Unit(namely_type!("Unit"));
        let evaluated =
            eval_expr(&type_env, &expr_env, &expr.clone().wrap_rc());

        assert_eq!(evaluated, expr.wrap_ok());
    }
}
