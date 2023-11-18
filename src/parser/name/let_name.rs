use crate::infra::IteratorExt;
use crate::infra::WrapOption;
use crate::parser::alphanum::{parse_alphanum, parse_lower};

#[derive(Debug, Clone, PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Lower(char),
    Alphanum(char),
    LetName(String)
}

fn go(stack: &Pat, seq: &str) -> Option<String> {
    let (head, tail) = seq.chars().get_head_tail();

    let move_in = match (&stack, head) {
        // LetName: [0-9a-zA-Z] -> Alphanum
        (Pat::LetName(_), Some(c))
            if parse_alphanum(&c).is_some() =>
            Pat::Alphanum(c),
        // Start: [a-z] -> Lower
        (Pat::Start, Some(c)) if parse_lower(&c).is_some() =>
            Pat::Lower(c),

        // ɛ -> End
        (_, None) => Pat::End,
        // _ -> Err
        (_, Some(c)) => {
            if cfg!(feature = "parser_lr1_log") {
                let log = format!("Invalid head Pat: {c:?}");
                println!("{log}");
            }

            Pat::Err
        }
    };

    let reduced_stack = match (stack, move_in) {
        // Start Lower -> LetName
        (Pat::Start, Pat::Lower(c)) => Pat::LetName(c.to_string()),
        // LetName Alphanum -> LetName
        (Pat::LetName(n), Pat::Alphanum(c)) =>
            Pat::LetName(format!("{}{}", n, c)),

        // Success
        (Pat::LetName(n), Pat::End) =>
            return n.to_string().wrap_some(),

        // Can not parse
        (_, Pat::Err) => return None,
        // Can not reduce
        (a, b) => {
            if cfg!(feature = "parser_lr1_log") {
                let log = format!("Reduction failed: {a:?}, {b:?}");
                println!("{log}");
            }

            return None;
        }
    };

    go(&reduced_stack, tail.as_str())
}

pub fn parse_let_name(seq: &str) -> Option<String> {
    go(&Pat::Start, seq)
}

#[test]
fn test_part1() {
    use crate::parser::name::parse_let_name;

    assert_eq!(parse_let_name("a1B2C3"), Some("a1B2C3".to_string()));
    assert_eq!(parse_let_name("A1b2c3"), None);
}
