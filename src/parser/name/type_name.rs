use crate::infra::option::AnyExt;
use crate::infra::str::str_get_head_tail;
use crate::parser::alphanum::{parse_alphanum, parse_upper};

#[derive(Debug, Clone, PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Upper(char),
    Alphanum(char),
    TypeName(String)
}

fn go(stack: &Pat, seq: &str) -> Option<String> {
    let (head, tail) = str_get_head_tail(seq);

    let move_in = match (stack, head) {
        // TypeName: [0-9a-zA-Z] -> Alphanum
        (Pat::TypeName(_), Some(c))
            if parse_alphanum(&c).is_some() =>
            Pat::Alphanum(c),
        // Start: [A-Z] -> Upper
        (Pat::Start, Some(c)) if parse_upper(&c).is_some() =>
            Pat::Upper(c),

        // ɛ -> End
        (_, None) => Pat::End,
        // _ -> Err
        (_, Some(c)) => {
            if cfg!(feature = "lr1_log") {
                let log = format!("Invalid head Pat: {c:?}");
                println!("{log}");
            }

            Pat::Err
        }
    };

    let reduced_stack = match (stack, move_in) {
        // Start Upper -> TypeName
        (Pat::Start, Pat::Upper(c)) => Pat::TypeName(c.to_string()),
        // TypeName Alphanum -> TypeName
        (Pat::TypeName(n), Pat::Alphanum(c)) =>
            Pat::TypeName(format!("{}{}", n, c)),

        // Success
        (Pat::TypeName(n), Pat::End) => return n.to_string().some(),

        // Can not parse
        (_, Pat::Err) => return None,
        // Can not reduce
        (a, b) => {
            if cfg!(feature = "lr1_log") {
                let log = format!("Reduction failed: {a:?}, {b:?}");
                println!("{log}");
            }

            return None;
        }
    };

    go(&reduced_stack, tail)
}

pub fn parse_type_name(seq: &str) -> Option<String> {
    go(&Pat::Start, seq)
}

#[test]
fn test_part1() {
    use crate::parser::name::type_name::parse_type_name;

    assert_eq!(parse_type_name("a1B2C3"), None);
    assert_eq!(parse_type_name("A1b2c3"), Some("A1b2c3".to_string()));
}
