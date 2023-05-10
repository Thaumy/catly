use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::env::expr_env::{ExprEnv, ExprEnvEntry};
use crate::eval::env::type_env::{TypeEnv, TypeEnvEntry};
use crate::eval::r#type::expr::OptExpr;
use crate::eval::r#type::r#type::OptType;
use crate::infer::env::def_map_to_env_vec as def_map_to_ct_env_vec;
use crate::infer::env::def_vec_to_def_map;
use crate::infer::env::expr_env::ExprEnv as CtExprEnv;
use crate::infer::env::expr_env::ExprEnvEntry as CtExprEnvEntry;
use crate::infer::env::r#type::type_constraint::TypeConstraint;
use crate::infer::env::type_env::TypeEnv as CtTypeEnv;
use crate::infer::infer_type_of_defs::{
    infer_type_of_defs,
    InferErr
};
use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::result::ResultAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::ast::parse_ast;
use crate::parser::expr::r#type::{
    Expr as CtExpr,
    OptExpr as OptCtExpr
};
use crate::parser::r#type::r#type::{
    OptType as OptCtType,
    Type as CtType
};
use crate::pp::preprocess;

pub mod expr_env;
pub mod type_env;

fn ct_expr_env_vec_to_rt_expr_env_vec(
    ct_type_env: CtTypeEnv,
    ct_expr_env_vec: Vec<CtExprEnvEntry>
) -> Result<Vec<ExprEnvEntry>, InferErr> {
    let inferred_defs = infer_type_of_defs(
        ct_type_env,
        &CtExprEnv::empty().rc(),
        ct_expr_env_vec
    )?;

    inferred_defs
        .into_iter()
        .map(|(n, tc, src)| match tc {
            TypeConstraint::Constraint(t) => {
                let t = OptType::from(t.clone()).expect(&format!(
                    "Impossible env ref type: {t:?}"
                ));
                let src: OptExpr = OptCtExpr::from(src.clone())
                    .expect(&format!(
                        "Impossible env ref src: {src:?}"
                    ))
                    .into();
                let src = src.map(|x| x.rc());

                (n, t, src, None)
            }
            _ => panic!("Impossible type constraint: {tc:?}")
        })
        .collect::<Vec<ExprEnvEntry>>()
        .ok()
}

fn def_map_to_env_vec(
    type_def_map: HashMap<String, CtType>,
    expr_def_map: HashMap<String, (OptCtType, CtExpr)>
) -> Result<(Vec<TypeEnvEntry>, Vec<ExprEnvEntry>), InferErr> {
    let (ct_type_env_vec, ct_expr_env_vec) =
        def_map_to_ct_env_vec(type_def_map, expr_def_map);

    let rt_type_env_vec: Vec<TypeEnvEntry> =
        ct_type_env_vec
            .clone()
            .into_iter()
            .map(|(n, t)| (n, t.into()))
            .try_fold(vec![], |acc, x| match x {
                (n, Some(t)) => acc.chain_push((n, t)).ok(),
                x => InferErr::of(format!("Invalid type def: {x:?}"))
                    .err()
            })?;

    let rt_expr_env_vec = ct_expr_env_vec_to_rt_expr_env_vec(
        CtTypeEnv::new(ct_type_env_vec),
        ct_expr_env_vec
    )?;

    (rt_type_env_vec, rt_expr_env_vec).ok()
}

pub fn parse_to_env<'t>(
    seq: &str
) -> Option<(TypeEnv<'t>, Rc<ExprEnv>)> {
    let seq = preprocess(&seq)?;
    let def_vec = parse_ast(seq)?;

    let (type_def_map, expr_def_map) = def_vec_to_def_map(def_vec)?;
    let (type_env_vec, expr_env_vec) =
        def_map_to_env_vec(type_def_map, expr_def_map).ok()?;

    let type_env = TypeEnv::new(type_env_vec);
    let expr_env = ExprEnv::empty()
        .rc()
        .extend_vec_new(expr_env_vec);

    (type_env, expr_env).some()
}
