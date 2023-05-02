mod move_in;
mod reduce_stack;

use crate::infra::iter::IteratorExt;
use crate::parser::expr::pat::Pat;
use crate::parser::expr::r#fn::move_in::move_in;
use crate::parser::expr::r#fn::reduce_stack::reduce_stack;
use crate::parser::expr::In;

pub fn go<'t, S>(mut stack: Vec<Pat>, seq: S) -> Pat
where
    S: Iterator<Item = &'t In> + Clone
{
    let (head, tail, follow) = seq.get_head_tail_follow();

    stack.push(move_in(&stack, head.cloned()));

    if cfg!(feature = "parser_lr1_log") {
        let log = format!("Move in: {stack:?} follow: {follow:?}");
        println!("{log}");
    }

    let reduced_stack = reduce_stack(stack, follow.cloned());

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
