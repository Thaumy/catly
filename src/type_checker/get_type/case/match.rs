use crate::infra::alias::MaybeType;
use crate::parser::expr::Expr;
use crate::type_checker::get_type::r#type::GetTypeReturn;

pub fn case(
    expect_type: &MaybeType,
    match_expr: &Expr,
    vec: &Vec<(Expr, Expr)>
) -> GetTypeReturn {
    todo!()
}
/*                // TODO: 对 Case 常量的类型检查
                // TODO: 实施类型的反向约束
                Expr::Match(t, _, vec) => {
                    let iter = vec
                        .iter()
                        .map(|(_, then_expr)| {
                            get_type(type_env, expr_env, then_expr)
                        });
                    let vec: Vec<Type> =
                        maybe_fold_to!(iter, vec![], push, id)?;

                    match t {
                        Some(t) => {
                            let f = |acc: Type, then_expr_type: &Type| {
                                unify(type_env, &acc, &then_expr_type)
                            };
                            maybe_fold!(vec.iter(), t.clone(), f)
                        }
                        _ => {
                            let f = |acc: Type, t: &Type| {
                                unify(type_env, &acc, t)
                            };
                            maybe_reduce!(vec.iter(), f)
                        }
                    }
                }
*/
