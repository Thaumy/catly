use crate::infra::iter::IntoIteratorExt;
use crate::infra::option::AnyExt;
use crate::infra::vec::Ext;
use crate::parser::define::{parse_define, Define};

type In = crate::parser::preprocess::Out;

pub fn split_to_top_levels(seq: Vec<In>) -> Vec<Vec<In>> {
    type F =
        fn((Vec<Vec<In>>, Vec<In>), &In) -> (Vec<Vec<In>>, Vec<In>);
    let f: F = |(acc_a, mut acc_p), x| match x {
        In::Kw(kw) if kw.is_top_level() =>
            if acc_p.is_empty() {
                (acc_a, vec![In::Kw(kw.clone())])
            } else {
                // remove last blank for each def
                if let Some(In::Symbol(' ')) = acc_p.last() {
                    acc_p.pop();
                }
                (acc_a.chain_push(acc_p), vec![In::Kw(kw.clone())])
            },
        x => (acc_a, acc_p.chain_push(x.clone()))
    };
    let (acc_a, acc_p) = seq
        .iter()
        .fold((vec![], vec![]), f);

    if acc_p.is_empty() {
        acc_a
    } else {
        acc_a.chain_push(acc_p)
    }
}

pub fn parse_to_defines(seq: Vec<Vec<In>>) -> Option<Vec<Define>> {
    let iter: Vec<_> = seq
        .iter()
        .map(|vec| parse_define(vec.clone()))
        .collect();

    iter.maybe_fold(vec![], |acc, it| {
        let it = it.clone()?;
        acc.chain_push(it).some()
    })
}
