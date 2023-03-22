use crate::parser::{Either, get_head_tail_follow, VecExt};
use crate::parser::char::parse_char;
use crate::parser::keyword::Keyword;

fn reduce_stack(stack: Vec<Either<char, Keyword>>, follow: Option<char>) -> Vec<Either<char, Keyword>> {
    use crate::parser::preprocess::keyword::Either::{*};
    use crate::parser::keyword::Keyword::{*};

    match (&stack[..], follow) {
        // !(Letter|Digit): "type" :Blank -> Type
        ([.., L(c), L('t'), L('y'), L('p'), L('e')], Some(' '))
        if parse_char(c).is_none() =>
            stack.reduce_to_new(4, R(Type)),
        // Start: "type" :Blank -> Type
        ([L('t'), L('y'), L('p'), L('e')], Some(' ')) => vec![R(Type)],

        // !(Letter|Digit): "def" :Blank -> Def
        ([.., L(c), L('d'), L('e'), L('f')], Some(' '))
        if parse_char(c).is_none() =>
            stack.reduce_to_new(3, R(Def)),
        // Start: "def" :Blank -> Def
        ([L('d'), L('e'), L('f')], Some(' ')) => vec![R(Def)],

        // !(Letter|Digit): "let" :Blank -> Let
        ([.., L(c), L('l'), L('e'), L('t')], Some(' '))
        if parse_char(c).is_none() =>
            stack.reduce_to_new(3, R(Let)),
        // Start: "let" :Blank -> Let
        ([L('l'), L('e'), L('t')], Some(' ')) => vec![R(Let)],

        // Blank|`,`: "in" :Blank -> In
        ([.., L(' ' | ','), L('i'), L('n') ], Some(' ')) =>
            stack.reduce_to_new(2, R(In)),

        // !(Letter|Digit): "if" :Blank -> If
        ([.., L(c), L('i'), L('f')], Some(' '))
        if parse_char(c).is_none() =>
            stack.reduce_to_new(2, R(If)),
        // Start: "if" :Blank -> If
        ([L('i'), L('f')], Some(' ')) => vec![R(If)],

        // Blank: "then" :Blank -> Then
        ([.., L(' '), L('t'), L('h'), L('e'), L('n')], Some(' ')) =>
            stack.reduce_to_new(4, R(Then)),

        // Blank: "else" :Blank -> Else
        ([.., L(' '), L('e'), L('l'), L('s'), L('e') ], Some(' ')) =>
            stack.reduce_to_new(4, R(Else)),

        // !(Letter|Digit): "match" :Blank -> Match
        ([.., L(c), L('m'), L('a'), L('t'), L('c'), L('h')], Some(' '))
        if parse_char(c).is_none() =>
            stack.reduce_to_new(5, R(Match)),
        // Start: "match" :Blank -> Match
        ([L('m'), L('a'), L('t'), L('c'), L('h')], Some(' ')) => vec![R(Match)],

        // Blank: "with" :`|` -> With
        ([.., L(' '), L('w'), L('i'), L('t'), L('h')], Some('|')) =>
            stack.reduce_to_new(4, R(With)),

        _ => return stack
    }
}

fn go(stack: Vec<Either<char, Keyword>>, tail: &str) -> Vec<Either<char, Keyword>> {
    let (head, tail, follow) = get_head_tail_follow(tail);
    let move_in = match head {
        Some(c) => Either::L(c),
        _ => return stack,
    };

    let stack = stack.push_to_new(move_in);
    let reduced_stack = reduce_stack(stack, follow);
    go(reduced_stack, tail)
}

pub fn preprocess_keyword(seq: &str) -> Vec<Either<char, Keyword>> {
    let r = go(vec![], seq);
    println!("Keyword pp out: {:?}", r);
    r
}
