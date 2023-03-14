use crate::parser::char::{parse_char, parse_upper};
use crate::parser::{get_head_tail};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Upper(char),
    Char(char),
    TypeName(String),
}

fn go(stack: &Pat, seq: &str) -> Option<String> {
    let (head, tail) = get_head_tail(seq);

    let move_in = match (stack, head) {
        // TypeName: [0-9a-zA-Z] -> Char
        (Pat::TypeName(_), Some(c)) if parse_char(&c).is_some() =>
            Pat::Char(c),
        // Start: [A-Z] -> Upper
        (Pat::Start, Some(c)) if parse_upper(&c).is_some() =>
            Pat::Upper(c),

        // ɛ -> End
        (_, None) => Pat::End,
        // _ -> Err
        (_, Some(c)) => {
            println!("Invalid head Pat: {}", c);
            Pat::Err
        }
    };

    let reduced_stack = match (stack, move_in) {
        // Start Upper -> TypeName
        (Pat::Start, Pat::Upper(c)) =>
            Pat::TypeName(c.to_string()),
        // TypeName Char -> TypeName
        (Pat::TypeName(n), Pat::Char(c)) =>
            Pat::TypeName(format!("{}{}", n, c)),

        // Success
        (Pat::TypeName(n), Pat::End) => return Some(n.to_string()),

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

pub fn parse_type_name(seq: &str) -> Option<String> {
    go(&Pat::Start, seq)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_type_name() {
        use crate::parser::name::type_name::parse_type_name;

        assert_eq!(parse_type_name("a1B2C3"), None);
        assert_eq!(parse_type_name("A1b2c3"), Some("A1b2c3".to_string()));
    }
}
