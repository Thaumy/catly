use crate::maybe_fold;
use crate::parser::define::{Define, parse_define};
use crate::parser::infra::r#fn::id;

type In = crate::parser::preprocess::Out;

pub fn split_to_top_levels(seq: Vec<In>) -> Vec<Vec<In>> {
    type F = fn((Vec<Vec<In>>, Vec<In>), &In) -> (Vec<Vec<In>>, Vec<In>);
    let f: F = |(mut acc_a, mut acc_p), x|
        match x {
            In::Kw(kw) if kw.is_top_level() => {
                if acc_p.is_empty() {
                    (acc_a, vec![In::Kw(kw.clone())])
                } else {
                    // remove last blank for each def
                    if let Some(In::Symbol(' ')) = acc_p.last() {
                        acc_p.pop();
                    }
                    acc_a.push(acc_p);
                    (acc_a, vec![In::Kw(kw.clone())])
                }
            }
            x => {
                acc_p.push(x.clone());
                (acc_a, acc_p)
            }
        };
    let (mut acc_a, acc_p) = seq
        .iter()
        .fold((vec![], vec![]), f);

    if acc_p.is_empty() {
        acc_a
    } else {
        acc_a.push(acc_p);
        acc_a
    }
}

pub fn parse_to_defines(seq: Vec<Vec<In>>) -> Option<Vec<Define>> {
    let iter = seq
        .iter()
        .map(|vec| parse_define(vec.clone()));

    maybe_fold!(
        iter,
        vec![],
        push,
        id
    )
}