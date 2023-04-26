use crate::infra::option::AnyExt;
use crate::infra::str::str_get_head_tail;

#[derive(Copy, Debug, Clone, PartialEq)]
enum Pat {
    Start,
    End,
    Err,

    Int(i64),
    Digit(u8)
}

fn go(stack: Vec<Pat>, seq: &str) -> Option<i64> {
    let (head, tail) = str_get_head_tail(seq);

    let move_in = match head {
        // _ -> Digit|Err
        Some(c) => {
            match crate::parser::alphanum::parse_digit(&c) {
                // [0-9] -> Digit
                Some(d) => Pat::Digit(d),
                // ɛ -> Err
                None => {
                    if cfg!(feature = "lr1_log") {
                        let log = format!("Invalid head Pat: {c:?}");
                        println!("{log}");
                    }

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
            vec![Pat::Int(a as i64)]
        }
        // Int Digit -> Int
        ([Pat::Int(a)], Pat::Digit(b)) => {
            vec![Pat::Int(a * 10 + (b as i64))]
        }

        // Success
        ([Pat::Int(a)], Pat::End) => return a.clone().some(),

        // Can not parse
        (_, Pat::Err) => return None,
        // Can not reduce
        (_, b) => {
            if cfg!(feature = "lr1_log") {
                let log =
                    format!("Reduction failed: {stack:?}, {b:?}");
                println!("{log}");
            }

            return None;
        }
    };

    go(reduced_stack, tail)
}

pub fn parse_int(x: &str) -> Option<i64> { go(vec![Pat::Start], x) }

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
