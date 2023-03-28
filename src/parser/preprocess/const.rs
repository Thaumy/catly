use crate::parser::infra::{vec_get_head_tail, VecExt};
use crate::parser::keyword::Keyword;
use crate::parser::value::int::parse_int;

type In = crate::parser::preprocess::keyword::Out;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Out {
    Symbol(char),
    LowerStartChunk(String),
    UpperStartChunk(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Pat {
    Start,
    End,
    Err,

    Symbol(char),
    DigitChunk(String),
    LowerStartChunk(String),
    UpperStartChunk(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue,
}

fn reduce_stack(stack: Vec<Pat>) -> Vec<Pat> {
    use crate::parser::keyword::Keyword::{*};

    let reduced_stack = match &stack[..] {
        // DigitChunk -> IntValue
        [.., Pat::DigitChunk(c)] => match parse_int(c) {
            Some(i) => stack.reduce_to_new(1, Pat::IntValue(i)),
            None => return vec![Pat::Err]
        }

        // '(' ')' -> UnitValue
        [.., Pat::Symbol('('), Pat::Symbol(')')] =>
            stack.reduce_to_new(2, Pat::UnitValue),

        // '_' -> DiscardValue
        [.., Pat::Symbol('_')] =>
            stack.reduce_to_new(1, Pat::DiscardValue),

        _ => return stack
    };

    println!("Reduce to: {:?}", reduced_stack);

    reduced_stack
}

impl From<In> for Pat {
    fn from(value: In) -> Self {
        match value {
            In::Symbol(c) => Self::Symbol(c),
            In::Kw(kw) => Self::Kw(kw),
            In::DigitChunk(c) => Self::DigitChunk(c),
            In::LowerStartChunk(c) => Self::LowerStartChunk(c),
            In::UpperStartChunk(c) => Self::UpperStartChunk(c),
        }
    }
}

impl From<Pat> for Option<Out> {
    fn from(value: Pat) -> Self {
        let r = match value {
            Pat::Symbol(c) => Out::Symbol(c.clone()),
            Pat::LowerStartChunk(c) => Out::LowerStartChunk(c.clone()),
            Pat::UpperStartChunk(c) => Out::UpperStartChunk(c.clone()),
            Pat::Kw(kw) => Out::Kw(kw.clone()),

            Pat::IntValue(i) => Out::IntValue(i.clone()),
            Pat::UnitValue => Out::UnitValue,
            Pat::DiscardValue => Out::DiscardValue,

            _ => return None
        };
        Some(r)
    }
}

fn go(stack: Vec<Pat>, tail: Vec<In>) -> Vec<Pat> {
    let (head, tail) = vec_get_head_tail(tail);
    let move_in = match head {
        Some(x) => x.into(),
        _ => return stack,
    };

    let stack = stack.push_to_new(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail)
}

pub fn preprocess_const(seq: Vec<In>) -> Option<Vec<Out>> {
    let r = go(vec![], seq)
        .iter()
        .fold(Some(vec![]), |acc, x|
            match (acc, Option::<Out>::from(x.clone())) {
                (Some(vec), Some(top)) => Some(vec.push_to_new(top)),
                _ => None
            },
        );
    println!("Const pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::keyword::Keyword;
    use crate::parser::preprocess::r#const::{Out, preprocess_const};

    type In = crate::parser::preprocess::keyword::Out;

    #[test]
    fn test_pp_const() {
        let seq = vec![
            In::Symbol('{'),
            In::Kw(Keyword::Type),
            In::LowerStartChunk("boob".to_string()),
            In::Kw(Keyword::Def),
            In::DigitChunk("8888".to_string()),
            In::Kw(Keyword::Let),
            In::Symbol('('),
            In::Symbol(')'),
            In::DigitChunk("123".to_string()),
            In::Kw(Keyword::Then),
            In::UpperStartChunk("Boob".to_string()),
            In::Kw(Keyword::Match),
            In::Symbol('_'),
            In::Kw(Keyword::With),
            In::Symbol(' '),
        ];
        let r = vec![
            Out::Symbol('{'),
            Out::Kw(Keyword::Type),
            Out::LowerStartChunk("boob".to_string()),
            Out::Kw(Keyword::Def),
            Out::IntValue(8888),
            Out::Kw(Keyword::Let),
            Out::UnitValue,
            Out::IntValue(123),
            Out::Kw(Keyword::Then),
            Out::UpperStartChunk("Boob".to_string()),
            Out::Kw(Keyword::Match),
            Out::DiscardValue,
            Out::Kw(Keyword::With),
            Out::Symbol(' '),
        ];
        let r = Some(r);

        assert_eq!(preprocess_const(seq), r);
    }
}
