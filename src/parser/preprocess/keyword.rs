use crate::parser::alphanum::parse_alphanum;
use crate::parser::infra::{vec_get_head_tail, vec_get_head_tail_follow, VecExt};
use crate::parser::keyword::Keyword;

type In = crate::parser::preprocess::chunk::Out;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Out {
    Symbol(char),
    DigitChunk(String),
    LowerStartChunk(String),
    UpperStartChunk(String),

    Kw(Keyword),
}

fn reduce_stack(stack: Vec<Out>) -> Vec<Out> {
    use crate::parser::keyword::Keyword::{*};

    let reduced_stack = match &stack[..] {
        // "type" -> Type
        [.., Out::LowerStartChunk(c)]
        if c == "type" => stack.reduce_to_new(1, Out::Kw(Type)),

        // "def" -> Def
        [.., Out::LowerStartChunk(c)]
        if c == "def" => stack.reduce_to_new(1, Out::Kw(Def)),

        // "let" -> Let
        [.., Out::LowerStartChunk(c)]
        if c == "let" => stack.reduce_to_new(1, Out::Kw(Let)),

        // "in" -> In
        [.., Out::LowerStartChunk(c)]
        if c == "in" => stack.reduce_to_new(1, Out::Kw(In)),

        // "if" -> If
        [.., Out::LowerStartChunk(c)]
        if c == "if" => stack.reduce_to_new(1, Out::Kw(If)),

        // "then" -> Then
        [.., Out::LowerStartChunk(c)]
        if c == "then" => stack.reduce_to_new(1, Out::Kw(Then)),

        // "else" -> Else
        [.., Out::LowerStartChunk(c)]
        if c == "else" => stack.reduce_to_new(1, Out::Kw(Else)),

        // "match" -> Match
        [.., Out::LowerStartChunk(c)]
        if c == "match" => stack.reduce_to_new(1, Out::Kw(Match)),

        // "with" -> With
        [.., Out::LowerStartChunk(c)]
        if c == "with" => stack.reduce_to_new(1, Out::Kw(With)),

        _ => return stack
    };

    println!("Reduce to: {:?}", reduced_stack);

    reduced_stack
}

impl From<In> for Out {
    fn from(value: In) -> Self {
        match value {
            In::DigitChunk(c) => Self::DigitChunk(c),
            In::LowerStartChunk(c) => Self::LowerStartChunk(c),
            In::UpperStartChunk(c) => Self::UpperStartChunk(c),
            In::Symbol(c) => Self::Symbol(c),
        }
    }
}

fn go(stack: Vec<Out>, tail: Vec<In>) -> Vec<Out> {
    let (head, tail) = vec_get_head_tail(tail);
    let move_in = match head {
        Some(x) => x.into(),
        _ => return stack,
    };

    let stack = stack.push_to_new(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail)
}

pub fn preprocess_keyword(seq: Vec<In>) -> Vec<Out> {
    let r = go(vec![], seq);
    println!("Keyword pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::keyword::Keyword;
    use crate::parser::preprocess::keyword::{Out, preprocess_keyword};

    type In = crate::parser::preprocess::chunk::Out;

    #[test]
    fn test_pp_keyword() {
        let seq = vec![
            In::Symbol('{'),
            In::LowerStartChunk("type".to_string()),
            In::LowerStartChunk("boob".to_string()),
            In::LowerStartChunk("def".to_string()),
            In::LowerStartChunk("let".to_string()),
            In::LowerStartChunk("in".to_string()),
            In::LowerStartChunk("if".to_string()),
            In::DigitChunk("123".to_string()),
            In::LowerStartChunk("then".to_string()),
            In::LowerStartChunk("else".to_string()),
            In::UpperStartChunk("Boob".to_string()),
            In::LowerStartChunk("match".to_string()),
            In::LowerStartChunk("with".to_string()),
            In::Symbol(' '),
        ];
        let r = vec![
            Out::Symbol('{'),
            Out::Kw(Keyword::Type),
            Out::LowerStartChunk("boob".to_string()),
            Out::Kw(Keyword::Def),
            Out::Kw(Keyword::Let),
            Out::Kw(Keyword::In),
            Out::Kw(Keyword::If),
            Out::DigitChunk("123".to_string()),
            Out::Kw(Keyword::Then),
            Out::Kw(Keyword::Else),
            Out::UpperStartChunk("Boob".to_string()),
            Out::Kw(Keyword::Match),
            Out::Kw(Keyword::With),
            Out::Symbol(' '),
        ];

        assert_eq!(preprocess_keyword(seq), r);
    }
}
