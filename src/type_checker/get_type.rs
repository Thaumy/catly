use crate::{maybe_fold, maybe_fold2, maybe_reduce};
use crate::parser::expr::Expr;
use crate::parser::infra::option::AnyExt;
use crate::parser::infra::r#fn::id;
use crate::parser::infra::vec::Ext;
use crate::parser::r#type::Type;
use crate::unifier::unify;

macro_rules! int_type {
    () => {
        Type::TypeEnvRef("Int".to_string())
    }
}

macro_rules! unit_type {
    () => {
        Type::TypeEnvRef("Unit".to_string())
    }
}

macro_rules! discard_type {
    () => {
        Type::TypeEnvRef("Discard".to_string())
    }
}


macro_rules! bool_type {
    () => {
        Type::TypeEnvRef("Discard".to_string())
    }
}

macro_rules! true_type {
    () => {
        Type::TypeEnvRef("Discard".to_string())
    }
}

macro_rules! false_type {
    () => {
        Type::TypeEnvRef("Discard".to_string())
    }
}

pub fn get_type(
    type_env: &Vec<(String, Type)>,
    expr_env: &Vec<(String, Type)>,
    expr: &Expr,
) -> Option<Type> {
    match expr {
        Expr::Int(t, _) => t
            .clone()
            .and_then(|t| unify(type_env, &int_type!(), &t))
            .or_else(|| int_type!().some()),

        Expr::Unit(t) => t
            .clone()
            .and_then(|t| unify(type_env, &unit_type!(), &t))
            .or_else(|| unit_type!().some()),

        Expr::Discard(t) => t
            .clone()
            .or_else(|| discard_type!().some()),

        // TODO: 实施类型的反向约束
        Expr::Let(
            t,
            assign_name,
            assign_type,
            assign_expr,
            e
        ) => {
            let assign_expr_type = get_type(type_env, expr_env, assign_expr)?;
            let assign_type = assign_type.clone()
                .and_then(|t|
                    unify(type_env, &t, &assign_expr_type)
                ).or_else(|| assign_expr_type.some())?;

            let expr_env = expr_env.push_to_new(
                (assign_name.to_string(), assign_type)
            );
            let e_t = get_type(type_env, &expr_env, e)?;

            t.clone()
                .and_then(|t| unify(type_env, &t, &e_t))
                .or_else(|| e_t.some())
        }

        Expr::Cond(
            t,
            bool_expr,
            true_expr,
            false_expr
        ) => match get_type(type_env, expr_env, bool_expr) {
            Some(bool_e_t)
            // Only Boolean types will be allowed
            if bool_e_t == bool_type!()
                || bool_e_t == true_type!()
                || bool_e_t == false_type!()
            => {
                let ture_expr_type = get_type(type_env, expr_env, true_expr)?;
                let false_expr_type = get_type(type_env, expr_env, false_expr)?;
                match t.clone() {
                    Some(t) => unify(type_env, &ture_expr_type, &t)
                        .and_then(|t| unify(type_env, &false_expr_type, &t)),
                    // TODO: 合一两种类型到最近的 SumType 定义
                    _ => unify(type_env, &ture_expr_type, &false_expr_type)
                }
            }
            _ => None
        }

        // TODO: 实施类型的反向约束
        Expr::Closure(
            t,
            input_name,
            input_type,
            output_expr
        ) => {
            match (input_name, input_type) {
                (Some(input_name), Some(input_type)) => {
                    let expr_env = expr_env.push_to_new(
                        (input_name.to_string(), input_type.clone())
                    );
                    let output_expr_type = get_type(type_env, &expr_env, output_expr)?;

                    t.clone()
                        .and_then(|t| unify(type_env, &output_expr_type, &t))
                        .or_else(|| output_expr_type.some())
                }
                _ => None
            }
        }

        Expr::EnvRef(t, ref_name) => {
            // 直接获取环境类型，不再进行推导
            let ref_type = expr_env
                .iter()
                .rev()
                .find(|(n, _)| n == ref_name)
                .map(|(_, t)| t)?;

            t.clone()
                .and_then(|t| unify(type_env, ref_type, &t))
                .or_else(|| ref_type.clone().some())
        }

        Expr::Struct(t, vec) => {
            let iter = vec
                .iter()
                .map(|(n, t, e)|
                    (n, t, get_type(type_env, expr_env, e))
                )
                .map(|(n, t, e_t)| {
                    let e_t = e_t?;
                    let unified_type = t.clone()
                        .and_then(|t| unify(type_env, &e_t, &t))
                        .or_else(|| e_t.some())?;
                    Some((n.to_string(), unified_type))
                });
            let vec = maybe_fold!(
                iter,
                vec![],
                push,
                id
            )?;
            let prod_type = Type::ProdType(vec);

            t.clone()
                .and_then(|t| unify(type_env, &t, &prod_type))
                .or_else(|| prod_type.some())
        }

        // TODO: 对 Case 常量的类型检查
        // TODO: 实施类型的反向约束
        Expr::Match(t, _, vec) => {
            let iter = vec
                .iter()
                .map(|(_, then_expr)| {
                    get_type(type_env, expr_env, then_expr)
                });
            let vec: Vec<Type> = maybe_fold!(
                iter,
                vec![],
                push,
                id
            )?;

            match t {
                Some(t) => {
                    let f = |acc: Type, then_expr_type: &Type| {
                        unify(type_env, &acc, &then_expr_type)
                    };
                    maybe_fold2!(
                        vec.iter(),
                        t.clone(),
                        f
                    )
                }
                _ => {
                    let f = |acc: Type, t: &Type| {
                        unify(type_env, &acc, t)
                    };
                    maybe_reduce!(
                        vec.iter(),
                        f
                    )
                }
            }
        }

        _ => Type::TypeEnvRef("".to_string()).some()
    }
}
