use crate::infra::option::OptionAnyExt;
use crate::infra::slice::slice_get_head_tail;
use crate::infra::vec::Ext;
use crate::parser::keyword::Keyword;
use crate::parser::value::int::parse_int;

#[derive(Debug, Clone, PartialEq)]
pub enum Out {
    Symbol(char),
    LowerStartChunk(String),
    UpperStartChunk(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue
}

#[derive(Debug, Clone, PartialEq)]
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
    DiscardValue
}

fn reduce_stack(mut stack: Vec<Pat>) -> Vec<Pat> {
    match &stack[..] {
        // DigitChunk -> IntValue
        [.., Pat::DigitChunk(c)] => match parse_int(c) {
            Some(i) => stack.reduce(1, Pat::IntValue(i)),
            None => return vec![Pat::Err]
        },

        // '(' ')' -> UnitValue
        [.., Pat::Symbol('('), Pat::Symbol(')')] =>
            stack.reduce(2, Pat::UnitValue),

        // '_' -> DiscardValue
        [.., Pat::Symbol('_')] => stack.reduce(1, Pat::DiscardValue),

        _ => return stack
    };
    let reduced_stack = stack;

    if cfg!(feature = "lr1_log") {
        let log = format!("Reduced: {reduced_stack:?}");
        println!("{log}");
    }

    reduced_stack
}

impl From<In> for Pat {
    fn from(value: In) -> Self {
        match value {
            In::Symbol(c) => Self::Symbol(c),
            In::Kw(kw) => Self::Kw(kw),
            In::DigitChunk(c) => Self::DigitChunk(c),
            In::LowerStartChunk(c) => Self::LowerStartChunk(c),
            In::UpperStartChunk(c) => Self::UpperStartChunk(c)
        }
    }
}

impl From<Pat> for Option<Out> {
    fn from(value: Pat) -> Self {
        match value {
            Pat::Symbol(c) => Out::Symbol(c.clone()),
            Pat::LowerStartChunk(c) =>
                Out::LowerStartChunk(c.clone()),
            Pat::UpperStartChunk(c) =>
                Out::UpperStartChunk(c.clone()),
            Pat::Kw(kw) => Out::Kw(kw.clone()),

            Pat::IntValue(i) => Out::IntValue(i.clone()),
            Pat::UnitValue => Out::UnitValue,
            Pat::DiscardValue => Out::DiscardValue,

            _ => return None
        }
        .some()
    }
}

fn go(mut stack: Vec<Pat>, tail: &[In]) -> Vec<Pat> {
    let (head, tail) = slice_get_head_tail(tail);
    let move_in = match head {
        Some(x) => x.clone().into(),
        _ => return stack
    };

    stack.push(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail)
}

type In = crate::pp::keyword::Out;

pub fn pp_const(seq: &[In]) -> Option<Vec<Out>> {
    let r = go(vec![], seq)
        .iter()
        .try_fold(vec![], |acc, p| {
            let it: Option<Out> = p.clone().into();
            acc.chain_push(it?).some()
        });

    if cfg!(feature = "pp_log") {
        let log = format!("{:8}{:>10} │ {r:?}", "[pp]", "Const");
        println!("{log}");
    }

    r
}

#[test]
fn test_part1() {
    use crate::parser::keyword::Keyword;
    use crate::pp::r#const::{pp_const, Out};

    type In = crate::pp::keyword::Out;

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
    ]
    .some();

    assert_eq!(pp_const(&seq), r);
}
