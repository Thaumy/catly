mod move_in;
mod reduce_stack;

use crate::infra::IteratorExt;
use crate::lexer::Token;
use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::r#fn::move_in::move_in;
use crate::parser::r#type::r#fn::reduce_stack::reduce_stack;

pub fn go<S>(mut stack: Vec<Pat>, seq: S) -> Pat
where
    S: Iterator<Item = Token> + Clone
{
    let (head, tail, follow) = seq.get_head_tail_follow();

    stack.push(move_in(head));

    if cfg!(feature = "parser_lr1_log") {
        let log = format!("Move in: {stack:?} follow: {follow:?}");
        println!("{log}");
    }

    let reduced_stack = reduce_stack(stack, follow.clone());

    match (&reduced_stack[..], follow) {
        ([p], None) => {
            let r = p.clone();

            if cfg!(feature = "parser_lr1_log") {
                let log = format!("Success with: {r:?}");
                println!("{log}");
            }

            r
        }
        _ => go(reduced_stack, tail)
    }
}
