use crate::infra::str::str_get_head_tail;

#[derive(Copy, Debug, Clone, PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Int(u64),
    Digit(u8)
}

//TODO: handle int overflow
fn go(stack: Vec<Pat>, seq: &str) -> Option<u64> {
    let (head, tail) = str_get_head_tail(seq);

    let move_in = match head {
        // _ -> Digit|Err
        Some(c) => {
            match crate::parser::alphanum::parse_digit(&c) {
                // [0-9] -> Digit
                Some(d) => Pat::Digit(d),
                // ɛ -> Err
                None => {
                    println!("Invalid head Pat: {:?}", c);
                    Pat::Err
                }
            }
        }
        // ɛ -> End
        None => Pat::End
    };

    let reduced_stack = match (&stack[..], move_in) {
        // Start Digit -> Int
        ([Pat::Start], Pat::Digit(a)) => {
            vec![Pat::Int(a as u64)]
        }
        // Int Digit -> Int
        ([Pat::Int(a)], Pat::Digit(b)) => {
            vec![Pat::Int(a * 10 + (b as u64))]
        }

        // Success
        ([Pat::Int(a)], Pat::End) => return Some(*a),

        // Can not parse
        (_, Pat::Err) => return None,
        // Can not reduce
        (_, b) => {
            println!("Reduction failed: {:?}, {:?}", stack, b);
            return None;
        }
    };

    go(reduced_stack, tail)
}

pub fn parse_int(x: &str) -> Option<u64> { go(vec![Pat::Start], x) }

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_int() {
        use crate::parser::value::int::parse_int;

        assert_eq!(parse_int("abc"), None);
        assert_eq!(parse_int("1abc"), None);
        assert_eq!(parse_int("12345678"), Some(12345678));
    }
}
