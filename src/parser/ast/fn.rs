use crate::infra::option::OptionAnyExt;
use crate::infra::vec::VecExt;
use crate::lexer::Token;
use crate::parser::define::{parse_define, Define};

pub fn split_to_top_levels(seq: Vec<Token>) -> Vec<Vec<Token>> {
    let (acc_a, acc_p) = seq.into_iter().fold(
        (vec![], vec![]),
        |(acc_a, mut acc_p), x| match x {
            Token::Kw(kw) if kw.is_top_level() =>
                if acc_p.is_empty() {
                    (acc_a, vec![Token::Kw(kw)])
                } else {
                    // remove last blank for each def
                    if let Some(Token::Symbol(' ')) = acc_p.last() {
                        acc_p.pop();
                    }

                    (acc_a.chain_push(acc_p), vec![Token::Kw(kw)])
                },
            x => (acc_a, acc_p.chain_push(x))
        }
    );

    if acc_p.is_empty() {
        acc_a
    } else {
        acc_a.chain_push(acc_p)
    }
}

pub fn parse_to_defines(seq: Vec<Vec<Token>>) -> Option<Vec<Define>> {
    seq.into_iter()
        .map(parse_define)
        .try_fold(vec![], |acc, it| {
            let it = it?;
            acc.chain_push(it).some()
        })
}
