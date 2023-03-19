use crate::parser::{VecExt, get_head_tail};
use crate::parser::preprocess::comment::Either::{*};
use crate::parser::preprocess::comment::Pat::{*};

#[derive(Clone)]
#[derive(Debug)]
pub enum Either<L, R> {
    L(L),
    R(R),
}

#[derive(Clone)]
enum Pat {
    CommentStart,
    CommentBody,
    Comment,
}

fn reduce_stack(stack: Vec<Either<char, Pat>>) -> Vec<Either<char, Pat>> {
    match stack[..] {
        // "# " -> CommentStart
        [.., L('#'), L(' ')] =>
            stack.reduce_to_new(2, R(CommentStart)),
        // CommentStart: (!\n) -> CommentBody
        [.., R(CommentStart), L(c)] if c != '\n' =>
            stack.reduce_to_new(1, R(CommentBody)),
        // CommentBody (!\n) -> CommentBody
        [.., R(CommentBody), L(c)] if c != '\n' =>
            stack.reduce_to_new(2, R(CommentBody)),
        // CommentStart '\n' -> Comment
        [.., R(CommentStart), L('\n')] =>
            stack.reduce_to_new(2, R(Comment)),
        // CommentStart CommentBody '\n' -> Comment
        [.., R(CommentStart), R(CommentBody), L('\n')] =>
            stack.reduce_to_new(3, R(Comment)),
        // CommentStart CommentBody End -> Comment
        [.., R(CommentStart), R(CommentBody)] =>
            stack.reduce_to_new(3, R(Comment)),

        _ => return stack
    }
}

fn go(stack: Vec<Either<char, Pat>>, tail: &str) -> Vec<Either<char, Pat>> {
    let (head, tail) = get_head_tail(tail);
    let move_in = match head {
        Some(c) => L(c),
        _ => return stack,
    };

    let stack = stack.push_to_new(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail)
}

pub fn preprocess_comment(seq: &str) -> String {
    let r = go(vec![], seq)
        .iter()
        .fold("".to_string(), |mut acc, p|
            match p {
                L(c) => {
                    acc.push(*c);
                    acc
                }
                _ => acc,
            },
        );
    println!("Comment pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::BoxExt;
    use crate::parser::expr::{Expr, parse_expr};
    use crate::parser::preprocess::comment::preprocess_comment;

    #[test]
    fn test_comment_pp_part1() {
        let seq =
            "match x with# Comment 123# Comment 123
\
             | 1 -> if a then b else c\
             | v -> a -> b -> add a b# Comment 123 Comment 123
\
             # Comment 123 Comment 123
\
             | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
                 { x = 123, y = c }# Comment 123 Comment 123
\
             | _ -> \
                match y with\
                | 1 -> ()# Comment 123 Comment 123
\
                # Comment 123 Comment 123
\
                | () -> \
                     # Comment 123 Comment 123
\
                     a -> b -> \
                       (# Comment 123#Comment 123
\
                       match z with\
                       | _ -> 114514# Comment 123 Comment 123
\
                       | a -> x -> y -> add () y\
                       # Comment 123 Comment 123
\
                       )\
                | _ -> baz# Comment 123 Comment 123";

        let r =
            "match x with\
             | 1 -> if a then b else c\
             | v -> a -> b -> add a b\
             | { a = _, b = { foo = _, bar = _ }, c = 3 } -> \
                 { x = 123, y = c }\
             | _ -> \
                match y with\
                | 1 -> ()\
                | () -> \
                     a -> b -> \
                       (\
                       match z with\
                       | _ -> 114514\
                       | a -> x -> y -> add () y\
                       )\
                | _ -> baz";

        assert_eq!(preprocess_comment(seq), r);
    }

    #[test]
    fn test_comment_pp_part2() {
        let seq =
            "# Comment 123# Comment 123
\
            let a = 123, # Comment 123 Comment 123
\
                 # Comment 123 Comment 123
\
                 b = \
                 let x = i -> j -> k, # Comment 123# Comment 123
\
                     y = a \
                 # Comment 123# Comment 123
\
                 in let z = () in a, \
                 d = neg 1 # Comment 123# Comment 123
\
             in \
                 # Comment 123 Comment 123
\
             let e = 6, k = () in # Comment 123 Comment 123
\
\
             let m = (), n = 4 in \
                 # Comment 123 Comment 123
\
             add () 456# Comment 123";

        let r =
            "let a = 123, \
                 b = \
                 let x = i -> j -> k, \
                     y = a \
                 in let z = () in a, \
                 d = neg 1 \
             in \
             let e = 6, k = () in \
             let m = (), n = 4 in \
             add () 456";

        assert_eq!(preprocess_comment(seq), r);
    }
}
