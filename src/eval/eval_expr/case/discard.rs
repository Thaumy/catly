use crate::eval::eval_expr::EvalRet;
use crate::eval::r#type::eval_err::EvalErr;
use crate::eval::r#type::r#type::Type;
use crate::infra::result::ResultAnyExt;

pub fn case_discard(type_annot: &Type) -> EvalRet {
    EvalErr::EvalDiscard(format!("Trying to eval _:{type_annot:?}"))
        .err()
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use crate::eval::env::expr_env::ExprEnv;
    use crate::eval::env::type_env::TypeEnv;
    use crate::eval::eval_expr::eval_expr;
    use crate::eval::r#macro::namely_type;
    use crate::eval::r#type::expr::Expr;
    use crate::infra::rc::RcAnyExt;

    // _: Int
    #[test]
    fn test_part1() {
        let type_env = TypeEnv::new(vec![]);
        let expr_env = ExprEnv::empty().rc();

        let expr = Expr::Discard(namely_type!("Int"));
        let evaluated = eval_expr(&type_env, expr_env, &expr);

        assert_matches!(evaluated, Result::Err(..))
    }
}
