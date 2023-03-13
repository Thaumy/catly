use crate::parser::char::{parse_char, parse_lower};
use crate::parser::{get_head_tail};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pattern {
    Start,
    End,
    Err,

    Lower(char),
    Char(char),
    LetName(String),
}

fn go(stack: &Pattern, seq: &str) -> Option<String> {
    let (head, tail) = get_head_tail(seq);

    let move_in = match (&stack, head) {
        // LetName: [0-9a-zA-Z] -> Char
        (Pattern::LetName(_), Some(c)) if parse_char(&c).is_some() =>
            Pattern::Char(c),
        // Start: [a-z] -> Lower
        (Pattern::Start, Some(c)) if parse_lower(&c).is_some() =>
            Pattern::Lower(c),

        // ɛ -> End
        (_, None) => Pattern::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head pattern: {}", c);
            Pattern::Err
        }
    };

    let reduced_stack = match (stack, move_in) {
        // Start Lower -> LetName
        (Pattern::Start, Pattern::Lower(c)) =>
            Pattern::LetName(c.to_string()),
        // LetName Char -> LetName
        (Pattern::LetName(n), Pattern::Char(c)) =>
            Pattern::LetName(format!("{}{}", n, c)),

        // Success
        (Pattern::LetName(n), Pattern::End) => return Some(n.to_string()),

        // Can not parse
        (_, Pattern::Err) => return None,
        // Can not reduce
        (a, b) => {
            println!("Reduction failed: {:?}, {:?}", a, b);
            return None;
        }
    };

    go(&reduced_stack, tail)
}

pub fn parse_let_name(seq: &str) -> Option<String> {
    go(&Pattern::Start, seq)
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
