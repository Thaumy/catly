use crate::parser::{VecExt, get_head_tail_follow, get_head_tail};
use crate::parser::char::parse_char;

#[derive(Clone)]
#[derive(Debug)]
enum Either<L, R> {
    L(L),
    R(R),
}

fn any(c: char) -> AnyOrBlank {
    Either::L(c)
}

fn blank() -> AnyOrBlank {
    Either::R(())
}

type AnyOrBlank = Either<char, ()>;

fn reduce_stack(stack: Vec<AnyOrBlank>) -> Vec<AnyOrBlank> {
    use crate::parser::preprocess::blank::Either::R;
    match &stack[..] {
        // Blank Blank -> Blank
        [.., R(()), R(())] => stack.reduce_to_new(2, blank()),
        _ => return stack
    }
}

fn go(stack: Vec<AnyOrBlank>, tail: &str) -> Vec<AnyOrBlank> {
    let (head, tail) = get_head_tail(tail);
    let move_in = match head {
        Some(' ' | '\t' | '\n' | '\r') => blank(),
        Some(c) => any(c),
        _ => return stack,
    };

    let stack = stack.push_to_new(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail)
}

pub fn preprocess_blank(seq: &str) -> String {
    let r = go(vec![], seq)
        .iter()
        .fold("".to_string(), |mut acc, c| {
            match c {
                Either::L(c) => acc.push(*c),
                _ => acc.push(' ')
            }
            acc
        });
    println!("Blank pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::preprocess::blank::preprocess_blank;

    #[test]
    fn test_blank_pp() {
        let seq = "    \n    \r    \t    ";
        assert_eq!(preprocess_blank(seq), " ");
        let seq = "    \n\t\n\r    \t\t\r\n\n    \t\n\r\r    ";
        assert_eq!(preprocess_blank(seq), " ");
    }
}
