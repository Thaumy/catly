use std::vec;

use crate::parser::define::{Define, parse_define};
use crate::parser::preprocess::Out;

fn parse_ast(seq: Vec<Out>) -> Option<Vec<Define>> {
    let seq = {
        let (mut acc_a, acc_p) = seq
            .iter()
            .fold(
                (vec![], vec![]),
                |(mut acc_a, mut acc_p), x|
                    match x {
                        Out::Kw(kw) if kw.is_top_level() => {
                            if acc_p.is_empty() {
                                (acc_a, vec![Out::Kw(kw.clone())])
                            } else {
                                // remove last blank for each def
                                if let Some(Out::Symbol(' ')) = acc_p.last() {
                                    acc_p.pop();
                                }
                                acc_a.push(acc_p);
                                (acc_a, vec![Out::Kw(kw.clone())])
                            }
                        }
                        x => {
                            acc_p.push(x.clone());
                            (acc_a, acc_p)
                        }
                    },
            );

        if acc_p.is_empty() {
            acc_a
        } else {
            acc_a.push(acc_p);
            acc_a
        }
    };

    let r: Option<Vec<Define>> = seq
        .iter()
        .map(|vec| parse_define(vec.clone()))
        .fold(
            Some(vec![]),
            |acc, x|
                match (acc, x) {
                    (Some(mut acc), Some(ast)) => {
                        acc.push(ast);
                        Some(acc)
                    }
                    _ => None,
                },
        );

    println!("Success with: {:?}", r);
    r
}

mod test;
