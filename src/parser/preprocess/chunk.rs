use crate::parser::alphanum::{parse_alphanum, parse_digit, parse_lower, parse_upper};
use crate::parser::infra::{str_get_head_tail_follow, VecExt};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Out {
    Symbol(char),
    DigitChunk(String),
    LowerStartChunk(String),
    UpperStartChunk(String),
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Pat {
    Start,
    End,

    Symbol(char),

    Digit(char),
    DigitStart(char),
    DigitChunk(String),//Out::DigitStartChunk

    Alphanum(char),

    LowerStart(char),
    LowerStartChunk(String),//Out::LowerStartChunk

    UpperStart(char),
    UpperStartChunk(String),//Out::UpperStartChunk
}

fn move_in(stack: &Vec<Pat>, head: Option<char>) -> Pat {
    match head {
        Some(c) => match (&stack[..], c) {
            // Char|Start: [0-9] -> DigitStart
            ([.., Pat::Symbol(_) | Pat::Start], c)
            if parse_digit(&c).is_some() => Pat::DigitStart(c),
            // DigitStart|DigitStartChunk: [0-9] -> Digit
            ([.., Pat::DigitStart(_) | Pat::DigitChunk(_)], c)
            if parse_digit(&c).is_some() => Pat::Digit(c),

            // Char|Start: [a-z] -> LowerStart
            ([.., Pat::Symbol(_) | Pat::Start], c)
            if parse_lower(&c).is_some() => Pat::LowerStart(c),
            // LowerStart|LowerStartChunk: [0-9a-zA-Z] -> Alphanum
            ([.., Pat::LowerStart(_) | Pat::LowerStartChunk(_)], c)
            if parse_alphanum(&c).is_some() => Pat::Alphanum(c),

            // Char|Start: [A-Z] -> UpperStart
            ([.., Pat::Symbol(_) | Pat::Start], c)
            if parse_upper(&c).is_some() => Pat::UpperStart(c),
            // UpperStart|UpperStartChunk: [0-9a-zA-Z] -> Alphanum
            ([.., Pat::UpperStart(_) | Pat::UpperStartChunk(_)], c)
            if parse_alphanum(&c).is_some() => Pat::Alphanum(c),

            // _ -> Char
            (_, c) => Pat::Symbol(c),
        }
        _ => Pat::End,
    }
}

fn reduce_stack(stack: Vec<Pat>, follow: Option<char>) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], follow) {
        // DigitStart Digit -> DigitStartChunk
        ([.., Pat::DigitStart(a), Pat::Digit(b)], _) => {
            let top = Pat::DigitChunk(format!("{}{}", a, b));
            stack.reduce_to_new(2, top)
        }
        // LowerStart Alphanum -> LowerStartChunk
        ([.., Pat::LowerStart(a), Pat::Alphanum(b)], _) => {
            let top = Pat::LowerStartChunk(format!("{}{}", a, b));
            stack.reduce_to_new(2, top)
        }
        // UpperStart Alphanum -> UpperStartChunk
        ([.., Pat::UpperStart(a), Pat::Alphanum(b)], _) => {
            let top = Pat::UpperStartChunk(format!("{}{}", a, b));
            stack.reduce_to_new(2, top)
        }

        // DigitStartChunk Digit -> DigitStartChunk
        ([.., Pat::DigitChunk(c), Pat::Digit(d)], _) => {
            let top = Pat::DigitChunk(format!("{}{}", c, d));
            stack.reduce_to_new(2, top)
        }
        // LowerStartChunk Alphanum -> LowerStartChunk
        ([.., Pat::LowerStartChunk(c), Pat::Alphanum(a)], _) => {
            let top = Pat::LowerStartChunk(format!("{}{}", c, a));
            stack.reduce_to_new(2, top)
        }
        // UpperStartChunk Alphanum -> UpperStartChunk
        ([.., Pat::UpperStartChunk(c), Pat::Alphanum(a)], _) => {
            let top = Pat::UpperStartChunk(format!("{}{}", c, a));
            stack.reduce_to_new(2, top)
        }

        // DigitStart :!Digit -> DigitStartChunk
        ([.., Pat::DigitStart(c)], follow)
        if match follow {
            Some(c) => parse_digit(&c).is_none(),
            None => true,
        } => stack.reduce_to_new(1, Pat::DigitChunk(c.to_string())),
        // LowerStart :!Alphanum -> LowerStartChunk
        ([.., Pat::LowerStart(c)], follow)
        if match follow {
            Some(c) => parse_alphanum(&c).is_none(),
            None => true,
        } => stack.reduce_to_new(1, Pat::LowerStartChunk(c.to_string())),
        // UpperStart :!Alphanum -> UpperStartChunk
        ([.., Pat::UpperStart(c)], follow)
        if match follow {
            Some(c) => parse_alphanum(&c).is_none(),
            None => true,
        } => stack.reduce_to_new(1, Pat::UpperStartChunk(c.to_string())),

        _ => return stack
    };

    println!("Reduce to: {:?}", reduced_stack);

    reduce_stack(reduced_stack, follow)
}

fn go(stack: Vec<Pat>, tail: &str) -> Vec<Pat> {
    let (head, tail, follow) = str_get_head_tail_follow(tail);

    let stack = stack.push_to_new(move_in(&stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow);
    let reduced_stack = reduce_stack(stack, follow);

    match reduced_stack[..] {
        [Pat::Start, .., Pat::End] => {
            let mut stack = reduced_stack.clone();
            stack.remove(0);// remove Start
            stack.pop();// remove End
            return stack;
        }
        _ => go(reduced_stack, tail)
    }
}

impl From<Pat> for Option<Out> {
    fn from(value: Pat) -> Self {
        let r = match value {
            Pat::DigitChunk(c) => Out::DigitChunk(c.to_string()),
            Pat::LowerStartChunk(c) => Out::LowerStartChunk(c.to_string()),
            Pat::UpperStartChunk(c) => Out::UpperStartChunk(c.to_string()),
            Pat::Symbol(s) => Out::Symbol(s.clone()),
            _ => return None
        };
        Some(r)
    }
}

pub fn preprocess_chunk(seq: &str) -> Option<Vec<Out>> {
    let vec = go(vec![Pat::Start], seq);
    let r = vec
        .iter()
        .fold(Some(vec![]), |mut acc, p|
            match (acc, Option::<Out>::from(p.clone())) {
                (Some(acc), Some(o)) =>
                    Some(acc.push_to_new(o)),
                _ => None
            },
        );
    println!("Chunk pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::preprocess::chunk::{Out, preprocess_chunk};

    #[test]
    fn test_pp_chunk() {
        let seq = "123 abc,Ab1c}233|foo";
        let r = vec![
            Out::DigitChunk("123".to_string()),
            Out::Symbol(' '),
            Out::LowerStartChunk("abc".to_string()),
            Out::Symbol(','),
            Out::UpperStartChunk("Ab1c".to_string()),
            Out::Symbol('}'),
            Out::DigitChunk("233".to_string()),
            Out::Symbol('|'),
            Out::LowerStartChunk("foo".to_string()),
        ];
        let r = Some(r);

        assert_eq!(preprocess_chunk(seq), r);
    }
}
