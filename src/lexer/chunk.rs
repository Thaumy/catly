use crate::infra::IteratorExt;
use crate::infra::VecExt;
use crate::infra::WrapOption;
use crate::parser::alphanum::{
    parse_alphanum,
    parse_digit,
    parse_lower,
    parse_upper
};

#[derive(Debug, Clone, PartialEq)]
pub enum Out {
    Symbol(char),
    DigitChunk(String),
    LowerStartChunk(String),
    UpperStartChunk(String)
}

#[derive(Debug, Clone, PartialEq)]
enum Pat {
    Start,
    End,

    Symbol(char),

    Digit(char),
    DigitStart(char),
    DigitChunk(String), // Out::DigitStartChunk

    Alphanum(char),

    LowerStart(char),
    LowerStartChunk(String), // Out::LowerStartChunk

    UpperStart(char),
    UpperStartChunk(String) // Out::UpperStartChunk
}

fn move_in(stack: &[Pat], head: Option<char>) -> Pat {
    match head {
        Some(c) => match (stack, c) {
            // Char|Start: [0-9] -> DigitStart
            ([.., Pat::Symbol(_) | Pat::Start], c)
                if parse_digit(&c).is_some() =>
                Pat::DigitStart(c),
            // DigitStart|DigitStartChunk: [0-9] -> Digit
            ([.., Pat::DigitStart(_) | Pat::DigitChunk(_)], c)
                if parse_digit(&c).is_some() =>
                Pat::Digit(c),

            // Char|Start: [a-z] -> LowerStart
            ([.., Pat::Symbol(_) | Pat::Start], c)
                if parse_lower(&c).is_some() =>
                Pat::LowerStart(c),
            // LowerStart|LowerStartChunk: [0-9a-zA-Z] -> Alphanum
            (
                [.., Pat::LowerStart(_) | Pat::LowerStartChunk(_)],
                c
            ) if parse_alphanum(&c).is_some() => Pat::Alphanum(c),

            // Char|Start: [A-Z] -> UpperStart
            ([.., Pat::Symbol(_) | Pat::Start], c)
                if parse_upper(&c).is_some() =>
                Pat::UpperStart(c),
            // UpperStart|UpperStartChunk: [0-9a-zA-Z] -> Alphanum
            (
                [.., Pat::UpperStart(_) | Pat::UpperStartChunk(_)],
                c
            ) if parse_alphanum(&c).is_some() => Pat::Alphanum(c),

            // _ -> Char
            (_, c) => Pat::Symbol(c)
        },
        _ => Pat::End
    }
}

fn reduce_stack(
    mut stack: Vec<Pat>,
    follow: Option<char>
) -> Vec<Pat> {
    match (&stack[..], follow) {
        // DigitStart Digit -> DigitStartChunk
        ([.., Pat::DigitStart(a), Pat::Digit(b)], _) => {
            let top = Pat::DigitChunk(format!("{}{}", a, b));
            stack.reduce(2, top)
        }
        // LowerStart Alphanum -> LowerStartChunk
        ([.., Pat::LowerStart(a), Pat::Alphanum(b)], _) => {
            let top = Pat::LowerStartChunk(format!("{}{}", a, b));
            stack.reduce(2, top)
        }
        // UpperStart Alphanum -> UpperStartChunk
        ([.., Pat::UpperStart(a), Pat::Alphanum(b)], _) => {
            let top = Pat::UpperStartChunk(format!("{}{}", a, b));
            stack.reduce(2, top)
        }

        // DigitStartChunk Digit -> DigitStartChunk
        ([.., Pat::DigitChunk(c), Pat::Digit(d)], _) => {
            let top = Pat::DigitChunk(format!("{}{}", c, d));
            stack.reduce(2, top)
        }
        // LowerStartChunk Alphanum -> LowerStartChunk
        ([.., Pat::LowerStartChunk(c), Pat::Alphanum(a)], _) => {
            let top = Pat::LowerStartChunk(format!("{}{}", c, a));
            stack.reduce(2, top)
        }
        // UpperStartChunk Alphanum -> UpperStartChunk
        ([.., Pat::UpperStartChunk(c), Pat::Alphanum(a)], _) => {
            let top = Pat::UpperStartChunk(format!("{}{}", c, a));
            stack.reduce(2, top)
        }

        // DigitStart :!Digit -> DigitStartChunk
        ([.., Pat::DigitStart(c)], follow)
            if match follow {
                Some(c) => parse_digit(&c).is_none(),
                None => true
            } =>
            stack.reduce(1, Pat::DigitChunk(c.to_string())),
        // LowerStart :!Alphanum -> LowerStartChunk
        ([.., Pat::LowerStart(c)], follow)
            if match follow {
                Some(c) => parse_alphanum(&c).is_none(),
                None => true
            } =>
            stack.reduce(1, Pat::LowerStartChunk(c.to_string())),
        // UpperStart :!Alphanum -> UpperStartChunk
        ([.., Pat::UpperStart(c)], follow)
            if match follow {
                Some(c) => parse_alphanum(&c).is_none(),
                None => true
            } =>
            stack.reduce(1, Pat::UpperStartChunk(c.to_string())),

        _ => return stack
    };

    let reduced_stack = stack;

    if cfg!(feature = "parser_lr1_log") {
        let log = format!("Reduced: {reduced_stack:?}");
        println!("{log}");
    }

    reduce_stack(reduced_stack, follow)
}

fn go(mut stack: Vec<Pat>, tail: &str) -> Vec<Pat> {
    let (head, tail, follow) = tail
        .chars()
        .get_head_tail_follow();

    stack.push(move_in(&stack, head));

    if cfg!(feature = "parser_lr1_log") {
        let log = format!("Move in: {stack:?} follow: {follow:?}");
        println!("{log}");
    }

    let reduced_stack = reduce_stack(stack, follow);

    match reduced_stack[..] {
        [Pat::Start, .., Pat::End] => {
            let mut stack = reduced_stack;
            stack.remove(0); // remove Start
            stack.pop(); // remove End
            stack
        }
        _ => go(reduced_stack, tail.as_str())
    }
}

impl From<Pat> for Option<Out> {
    fn from(value: Pat) -> Self {
        match value {
            Pat::DigitChunk(c) => Out::DigitChunk(c),
            Pat::LowerStartChunk(c) => Out::LowerStartChunk(c),
            Pat::UpperStartChunk(c) => Out::UpperStartChunk(c),
            Pat::Symbol(s) => Out::Symbol(s),
            _ => return None
        }
        .wrap_some()
    }
}

pub fn lexer_chunk(seq: &str) -> Option<Vec<Out>> {
    let r = go(vec![Pat::Start], seq)
        .into_iter()
        .try_fold(vec![], |acc, p| {
            let it: Option<Out> = p.into();
            acc.chain_push(it?)
                .wrap_some()
        });

    #[cfg(feature = "lexer_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[lexer]", "Chunk");
        println!("{log}");
    }

    r
}

#[test]
fn test_part1() {
    use crate::lexer::chunk::{lexer_chunk, Out};

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
    ]
    .wrap_some();

    assert_eq!(lexer_chunk(seq), r);
}
