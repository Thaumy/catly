use crate::infra::either::Either;
use crate::infra::iter::IteratorExt;
use crate::infra::vec::VecExt;
use crate::pp::rm_comment::Either::*;
use crate::pp::rm_comment::Pat::*;

#[derive(Clone)]
enum Pat {
    CommentStart,
    CommentBody,
    Comment
}

fn reduce_stack(
    mut stack: Vec<Either<char, Pat>>
) -> Vec<Either<char, Pat>> {
    match stack[..] {
        // "# " -> CommentStart
        [.., L('#'), L(' ')] => stack.reduce(2, R(CommentStart)),
        // CommentStart: (!\n) -> CommentBody
        [.., R(CommentStart), L(c)] if c != '\n' =>
            stack.reduce(1, R(CommentBody)),
        // CommentBody (!\n) -> CommentBody
        [.., R(CommentBody), L(c)] if c != '\n' =>
            stack.reduce(2, R(CommentBody)),
        // CommentStart '\n' -> Comment
        [.., R(CommentStart), L('\n')] => stack.reduce(2, R(Comment)),
        // CommentStart CommentBody '\n' -> Comment
        [.., R(CommentStart), R(CommentBody), L('\n')] =>
            stack.reduce(3, R(Comment)),
        // CommentStart CommentBody End -> Comment
        [.., R(CommentStart), R(CommentBody)] =>
            stack.reduce(3, R(Comment)),

        _ => return stack
    }

    stack
}

fn go(
    mut stack: Vec<Either<char, Pat>>,
    tail: &str
) -> Vec<Either<char, Pat>> {
    let (head, tail) = tail.chars().get_head_tail();
    let move_in = match head {
        Some(c) => L(c),
        _ => return stack
    };

    stack.push(move_in);
    let reduced_stack = reduce_stack(stack);
    go(reduced_stack, tail.as_str())
}

pub fn pp_rm_comment(seq: &str) -> String {
    let r = go(vec![], seq)
        .into_iter()
        .fold(vec![], |acc, p| match p {
            L(c) => acc.chain_push(c),
            R(_) => acc
        })
        .into_iter()
        .collect();

    #[cfg(feature = "pp_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[pp]", "Comment");
        println!("{log}");
    }

    r
}

#[test]
fn test_part1() {
    use crate::pp::rm_comment;
    let seq = "match x with# Comment 123# Comment 123
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

    let r = "match x with\
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

    assert_eq!(rm_comment(seq), r);
}

#[test]
fn test_part2() {
    use crate::pp::rm_comment;

    let seq = "# Comment 123# Comment 123
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

    let r = "let a = 123, \
                 b = \
                 let x = i -> j -> k, \
                     y = a \
                 in let z = () in a, \
                 d = neg 1 \
             in \
             let e = 6, k = () in \
             let m = (), n = 4 in \
             add () 456";

    assert_eq!(rm_comment(seq), r);
}
