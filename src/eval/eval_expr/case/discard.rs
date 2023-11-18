use crate::eval::eval_expr::EvalRet;
use crate::eval::EvalErr;
use crate::eval::Type;
use crate::infra::WrapResult;

pub fn case_discard(type_annot: &Type) -> EvalRet {
    EvalErr::EvalDiscard(format!("Trying to eval _:{type_annot:?}"))
        .wrap_err()
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use crate::eval::env::ExprEnv;
    use crate::eval::env::TypeEnv;
    use crate::eval::eval_expr::eval_expr;
    use crate::eval::namely_type;
    use crate::eval::Expr;
    use crate::infra::RcAnyExt;

    // _: Int
    #[test]
    fn test_part1() {
        let type_env = TypeEnv::new(vec![]);
        let expr_env = ExprEnv::empty().rc();

        let expr = Expr::Discard(namely_type!("Int"));
        let evaluated = eval_expr(&type_env, &expr_env, &expr.rc());

        assert_matches!(evaluated, Result::Err(..))
    }
}
