use crate::infra::option::OptionAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::define::{parse_define, Define};

type In = crate::pp::Out;

pub fn split_to_top_levels(seq: Vec<In>) -> Vec<Vec<In>> {
    let (acc_a, acc_p) =
        seq.iter()
            .fold(
                (vec![], vec![]),
                |(acc_a, mut acc_p), x| match x {
                    In::Kw(kw) if kw.is_top_level() =>
                        if acc_p.is_empty() {
                            (acc_a, vec![In::Kw(kw.clone())])
                        } else {
                            // remove last blank for each def
                            if let Some(In::Symbol(' ')) =
                                acc_p.last()
                            {
                                acc_p.pop();
                            }

                            (acc_a.chain_push(acc_p), vec![In::Kw(
                                kw.clone()
                            )])
                        },
                    x => (acc_a, acc_p.chain_push(x.clone()))
                }
            );

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

    iter.iter()
        .try_fold(vec![], |acc, it| {
            let it = it.clone()?;
            acc.chain_push(it).some()
        })
}
