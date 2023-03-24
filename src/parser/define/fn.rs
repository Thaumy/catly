use crate::parser::alphanum::parse_alphanum;
use crate::parser::define::pat::Pat;
use crate::parser::expr::parse_expr;
use crate::parser::follow_pat::{FollowPat, parse_follow_pat};
use crate::parser::infra::{Either, vec_get_head_tail_follow, VecExt};
use crate::parser::keyword::Keyword;
use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;
use crate::parser::r#type::parse_type;

fn move_in(stack: &Vec<Pat>, head: Option<Either<char, Keyword>>) -> Pat {
    match head {
        Some(Either::L(o)) => match (&stack[..], o) {
            // AlphanumSeq: [0-9a-zA-Z] -> Alphanum
            ([.., Pat::AlphanumSeq(_)], c) if parse_alphanum(&c).is_some() =>
                Pat::Alphanum(c),
            // [0-9a-zA-Z] -> Alphanum
            (_, c) if parse_alphanum(&c).is_some() =>
                Pat::Alphanum(c),

            // ' ' -> Blank
            (_, ' ') => Pat::Blank,
            // '=' -> `=`
            (_, '=') => Pat::Mark('='),

            // _ -> Err
            (_, c) => {
                println!("Invalid head Pat: {:?}", c);
                Pat::Err
            }
        }

        Some(Either::R(kw)) => Pat::Keyword(kw),

        // ɛ -> End
        None => Pat::End,
    }
}

fn reduce_stack(stack: &Vec<Pat>, follow_pat: &FollowPat) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], follow_pat) {
        // Success
        ([Pat::Start, p, Pat::End], _) => {
            return vec![p.clone()];
        }

        // AlphanumSeq Alphanum -> AlphanumSeq
        ([.., Pat::AlphanumSeq(cs), Pat::Alphanum(c)], _) =>
            stack.reduce_to_new(2, Pat::AlphanumSeq(format!("{}{}", cs, c))),
        // AlphanumSeq :Alphanum -> AlphanumSeq
        ([.., Pat::AlphanumSeq(_)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            return stack.clone(),
        // AlphanumSeq :!Alphanum-> TypeName|LetName|Err
        ([.., Pat::AlphanumSeq(cs)], _) => {
            let top = match parse_type_name(cs) {
                Some(n) => match &n[..] {
                    // override primitive types are not allowed
                    "Int" | "Unit" => Pat::Err,
                    // _ -> TypeName
                    n => Pat::TypeName(n.to_string()),
                }
                None => match parse_let_name(cs) {
                    Some(n) => Pat::LetName(n.to_string()),
                    None => Pat::Err
                }
            };
            stack.reduce_to_new(1, top)
        }
        // Alphanum :Alphanum -> AlphanumSeq
        ([.., Pat::Alphanum(c)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::AlphanumSeq(c.to_string())),
        // Alphanum :!Alphanum -> TypeName|LetName|Err
        ([.., Pat::Alphanum(c)], _) => {
            let top = match parse_type_name(c.to_string().as_str()) {
                // _ -> TypeName
                Some(n) => Pat::TypeName(n.to_string()),
                None => match parse_let_name(&c.to_string()) {
                    Some(n) => Pat::LetName(n.to_string()),
                    None => Pat::Err
                }
            };
            stack.reduce_to_new(1, top)
        }

        // KwDef Blank LetName Blank `=` Blank -> TypeDefHead End
        ([..,
        Pat::Keyword(Keyword::Type), Pat::Blank,
        Pat::TypeName(n), Pat::Blank, Pat::Mark('='), Pat::Blank], _
        ) => {
            let top = Pat::TypeDefHead(n.to_string());
            stack
                .reduce_to_new(6, top)
                .push_to_new(Pat::End)
        }

        // KwDef Blank LetName Blank `=` Blank -> ExprDefHead End
        ([..,
        Pat::Keyword(Keyword::Def), Pat::Blank,
        Pat::LetName(n), Pat::Blank, Pat::Mark('='), Pat::Blank], _
        ) => {
            let top = Pat::ExprDefHead(n.to_string());
            stack
                .reduce_to_new(6, top)
                .push_to_new(Pat::End)
        }

        // Can not parse
        ([.., Pat::Err], _) => return vec![Pat::Err],
        // Can not reduce
        ([.., Pat::End], _) => {
            println!("Reduction failed: {:?}", stack);
            return vec![Pat::Err];
        }
        // keep move in
        _ => return stack.clone()
    };

    println!("Reduce to: {:?}", reduced_stack);

    reduce_stack(&reduced_stack, follow_pat)
}

pub fn go(stack: &Vec<Pat>, seq: Vec<Either<char, Keyword>>) -> Pat {
    let (head, tail, follow) =
        vec_get_head_tail_follow(seq);

    let follow_pat = parse_follow_pat(follow);

    let stack = stack.push_to_new(move_in(stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow_pat);

    let reduced_stack = reduce_stack(&stack, &follow_pat);

    match (&reduced_stack[..], follow_pat) {
        ([p], _) => {
            let head = p.clone();

            let r = match head {
                Pat::TypeDefHead(n) => match parse_type(tail) {
                    Some(t) => Pat::TypeDef(n, t),
                    _ => Pat::Err
                },
                Pat::ExprDefHead(n) => match parse_expr(tail) {
                    Some(e) => Pat::ExprDef(n, e),
                    _ => Pat::Err
                },
                _ => Pat::Err
            };

            println!("Success with: {:?}", r);

            return r;
        }
        _ => go(&reduced_stack, tail)
    }
}
