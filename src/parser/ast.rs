use std::vec;

use crate::parser::define::{Define, parse_define};
use crate::parser::infra::Either;
use crate::parser::keyword::Keyword;

fn parse_ast(seq: Vec<Either<char, Keyword>>) -> Option<Vec<Define>> {
    let seq = {
        let (mut acc_a, acc_p) = seq
            .iter()
            .fold(
                (vec![], vec![]),
                |(mut acc_a, mut acc_p), x|
                    match x {
                        Either::R(kw) if kw.is_top_level() => {
                            if acc_p.is_empty() {
                                (acc_a, vec![Either::R(kw.clone())])
                            } else {
                                // remove last blank for each def
                                if let Some(Either::L(' ')) = acc_p.last() {
                                    acc_p.pop();
                                }
                                acc_a.push(acc_p);
                                (acc_a, vec![Either::R(kw.clone())])
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
