use crate::parser::get_head_tail;
use crate::parser::char::{parse_char, parse_lower};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Lower(char),
    Char(char),
    LetName(String),
}

fn go(stack: &Pat, seq: &str) -> Option<String> {
    let (head, tail) = get_head_tail(seq);

    let move_in = match (&stack, head) {
        // LetName: [0-9a-zA-Z] -> Char
        (Pat::LetName(_), Some(c)) if parse_char(&c).is_some() =>
            Pat::Char(c),
        // Start: [a-z] -> Lower
        (Pat::Start, Some(c)) if parse_lower(&c).is_some() =>
            Pat::Lower(c),

        // ɛ -> End
        (_, None) => Pat::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head Pat: {}", c);
            Pat::Err
        }
    };

    let reduced_stack = match (stack, move_in) {
        // Start Lower -> LetName
        (Pat::Start, Pat::Lower(c)) =>
            Pat::LetName(c.to_string()),
        // LetName Char -> LetName
        (Pat::LetName(n), Pat::Char(c)) =>
            Pat::LetName(format!("{}{}", n, c)),

        // Success
        (Pat::LetName(n), Pat::End) => return Some(n.to_string()),

        // Can not parse
        (_, Pat::Err) => return None,
        // Can not reduce
        (a, b) => {
            println!("Reduction failed: {:?}, {:?}", a, b);
            return None;
        }
    };

    go(&reduced_stack, tail)
}

pub fn parse_let_name(seq: &str) -> Option<String> {
    go(&Pat::Start, seq)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_let_name() {
        use crate::parser::name::let_name::parse_let_name;

        assert_eq!(parse_let_name("a1B2C3"), Some("a1B2C3".to_string()));
        assert_eq!(parse_let_name("A1b2c3"), None);
    }
}
