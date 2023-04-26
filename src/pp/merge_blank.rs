use crate::infra::either::Either;
use crate::infra::str::str_get_head_tail;
use crate::infra::vec::Ext;

fn any(c: char) -> AnyOrBlank { Either::L(c) }

fn blank() -> AnyOrBlank { Either::R(()) }

type AnyOrBlank = Either<char, ()>;

fn reduce_stack(mut stack: Vec<AnyOrBlank>) -> Vec<AnyOrBlank> {
    use crate::pp::merge_blank::Either::R;
    match &stack[..] {
        // Blank Blank -> Blank
        [.., R(()), R(())] => stack.reduce(2, blank()),
        _ => return stack
    }
    stack
}

fn go(mut stack: Vec<AnyOrBlank>, tail: &str) -> Vec<AnyOrBlank> {
    let (head, tail) = str_get_head_tail(tail);
    let move_in = match head {
        Some(' ' | '\t' | '\n' | '\r') => blank(),
        Some(c) => any(c),
        _ => return stack
    };

    stack.push(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail)
}

pub fn pp_merge_blank(seq: &str) -> String {
    let r =
        go(vec![], seq)
            .iter()
            .fold("".to_string(), |mut acc, c| {
                match c {
                    Either::L(c) => acc.push(*c),
                    _ => acc.push(' ')
                }
                acc
            });

    if cfg!(feature = "pp_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[pp]", "MergeBlank");
        println!("{log}");
    }

    r
}

#[cfg(test)]
mod tests {
    use crate::pp::merge_blank::pp_merge_blank;

    #[test]
    fn test_blank_pp() {
        let seq = "    \n    \r    \t    ";
        assert_eq!(pp_merge_blank(seq), " ");
        let seq = "    \n\t\n\r    \t\t\r\n\n    \t\n\r\r    ";
        assert_eq!(pp_merge_blank(seq), " ");
    }
}