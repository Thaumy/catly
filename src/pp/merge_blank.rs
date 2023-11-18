use crate::infra::IteratorExt;
use crate::infra::VecExt;

fn any(c: char) -> AnyOrBlank { Some(c) }

fn blank() -> AnyOrBlank { None }

type AnyOrBlank = Option<char>;

fn reduce_stack(mut stack: Vec<AnyOrBlank>) -> Vec<AnyOrBlank> {
    match &stack[..] {
        // Blank Blank -> Blank
        [.., None, None] => stack.reduce(2, blank()),
        _ => return stack
    }
    stack
}

fn go(mut stack: Vec<AnyOrBlank>, tail: &str) -> Vec<AnyOrBlank> {
    let (head, tail) = tail.chars().get_head_tail();
    let move_in = match head {
        Some(' ' | '\t' | '\n' | '\r') => blank(),
        Some(c) => any(c),
        _ => return stack
    };

    stack.push(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail.as_str())
}

pub fn pp_merge_blank(seq: &str) -> String {
    let r = go(vec![], seq)
        .iter()
        .map(|c| c.unwrap_or(' '))
        .collect();

    #[cfg(feature = "pp_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[pp]", "MergeBlank");
        println!("{log}");
    }

    r
}

#[test]
fn test_part1() {
    let seq = "    \n    \r    \t    ";
    assert_eq!(pp_merge_blank(seq), " ");

    let seq = "    \n\t\n\r    \t\t\r\n\n    \t\n\r\r    ";
    assert_eq!(pp_merge_blank(seq), " ");
}
