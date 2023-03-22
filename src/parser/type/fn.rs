use std::collections::BTreeSet;

use crate::parser::{BoxExt, Either, vec_get_head_tail_follow, VecExt};
use crate::parser::char::parse_char;
use crate::parser::keyword::Keyword;
use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;
use crate::parser::r#type::follow_pat::{FollowPat, parse_follow_pat};
use crate::parser::r#type::pat::Pat;

fn move_in(stack: &Vec<Pat>, head: Option<Either<char, Keyword>>) -> Pat {
    match head {
        Some(Either::L(c)) => match (&stack[..], c) {
            // CharSeq: [0-9a-zA-Z] -> Char
            ([.., Pat::CharSeq(_)], c) if parse_char(&c).is_some() =>
                Pat::Char(c),
            // [0-9a-zA-Z] -> Char
            (_, c) if parse_char(&c).is_some() =>
                Pat::Char(c),

            // ' ' -> Blank
            (_, ' ') => Pat::Blank,
            // '(' -> `(`
            (_, '(') => Pat::Mark('('),
            // ')' -> `)`
            (_, ')') => Pat::Mark(')'),

            // '-' -> `-`
            (_, '-') => Pat::Mark('-'),
            // '>' -> `>`
            (_, '>') => Pat::Mark('>'),

            // '{' -> `{`
            (_, '{') => Pat::Mark('{'),
            // '}' -> `}`
            (_, '}') => Pat::Mark('}'),
            // ',' -> `,`
            (_, ',') => Pat::Mark(','),

            // '|' -> `|`
            (_, '|') => Pat::Mark('|'),
            // '_' -> Discard
            (_, '_') => Pat::DiscardType,
            // ':' -> `:`
            (_, ':') => Pat::Mark(':'),

            // _ -> Err
            (_, c) => {
                println!("Invalid head Pat: {:?}", c);
                Pat::Err
            }
        }

        // _ -> Err
        Some(p) => {
            println!("Invalid head Pat: {:?}", p);
            Pat::Err
        }

        // ɛ -> End
        None => Pat::End,
    }
}

fn reduce_stack(stack: &Vec<Pat>, follow_pat: &FollowPat) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], follow_pat) {
        // Success
        ([Pat::Start, p, Pat::End], FollowPat::End) => return vec![p.clone()],

        // `(` Type `)` -> Type
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) if p.is_type() =>
            stack.reduce_to_new(3, p.clone()),

        // "_" -> DiscardType
        ([.., Pat::Mark('_')], _) =>
            stack.reduce_to_new(1, Pat::DiscardType),

        // CharSeq Char -> CharSeq
        ([.., Pat::CharSeq(cs), Pat::Char(c)], _) =>
            stack.reduce_to_new(2, Pat::CharSeq(format!("{}{}", cs, c))),
        // CharSeq :Char -> CharSeq
        ([.., Pat::CharSeq(_)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            return stack.clone(),
        // CharSeq :!Char-> TypeName|LetName|Err
        ([.., Pat::CharSeq(cs)], _) => {
            let top = match parse_type_name(cs) {
                Some(n) => match &n[..] {
                    // "Unit" -> UnitType
                    "Unit" => Pat::UnitType,
                    // "Int" -> IntType
                    "Int" => Pat::IntType,
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
        // Char :Char -> CharSeq
        ([.., Pat::Char(c)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::CharSeq(c.to_string())),
        // Char :!Char -> TypeName|LetName|Err
        ([.., Pat::Char(c)], _) => {
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

        // Type Blank Type -> TypeApply
        ([.., lhs, Pat::Blank, rhs], _)
        if lhs.is_type() && rhs.is_type() => {
            let top = Pat::TypeApply(
                lhs.clone().boxed(),
                rhs.clone().boxed(),
            );
            stack.reduce_to_new(3, top)
        }

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce_to_new(2, Pat::Arrow),
        // TypeName Blank Arrow Blank -> TypeClosurePara
        ([.., Pat::TypeName(n), Pat::Blank, Pat::Arrow, Pat::Blank], _) => {
            let top = Pat::TypeClosurePara(n.to_string());
            stack.reduce_to_new(4, top)
        }
        // TypeClosurePara Type -> TypeClosure
        ([.., Pat::TypeClosurePara(n), p], follow_pat)
        if follow_pat.not_blank() && p.is_type() => {
            let top = Pat::TypeClosure(
                n.to_string(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(2, top)
        }

        // SumType Blank `|` Blank SumType -> SumType
        ([..,
        Pat::SumType(l), Pat::Blank, Pat::Mark('|'), Pat::Blank,
        Pat::SumType(r)], _
        ) => {
            let mut set = BTreeSet::new();
            set.extend(l.clone());
            set.extend(r.clone());

            let top = Pat::SumType(set);
            stack.reduce_to_new(5, top)
        }
        // Type Blank `|` Blank SumType -> SumType
        ([..,
        p, Pat::Blank, Pat::Mark('|'), Pat::Blank,
        Pat::SumType(vec)], _
        )
        if p.is_type() => {
            let mut set = BTreeSet::new();
            set.extend(vec.clone());
            set.insert(p.clone());

            let top = Pat::SumType(set);
            stack.reduce_to_new(5, top)
        }
        // SumType Blank `|` Blank Type -> SumType
        ([..,
        Pat::SumType(vec), Pat::Blank, Pat::Mark('|'), Pat::Blank,
        p], _
        )
        if p.is_type() => {
            let mut set = BTreeSet::new();
            set.extend(vec.clone());
            set.insert(p.clone());

            let top = Pat::SumType(set);
            stack.reduce_to_new(5, top)
        }
        // Type Blank `|` Blank Type -> SumType
        ([.., a, Pat::Blank, Pat::Mark('|'), Pat::Blank, b], _)
        if a.is_type() && b.is_type() => {
            let mut set = BTreeSet::new();
            set.insert(a.clone());
            set.insert(b.clone());

            let top = Pat::SumType(set);
            stack.reduce_to_new(5, top)
        }

        // Blank LetName `:` Blank Type `,` -> LetNameWithType
        ([..,
        Pat::Blank,
        Pat::LetName(n), Pat::Mark(':'), Pat::Blank,
        p, Pat::Mark(',')], _
        )
        if p.is_type() => {
            let top = Pat::LetNameWithType(
                n.to_string(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(6, top)
        }
        // Blank LetName `:` Blank Type Blank :`}` -> LetNameWithType
        ([..,
        Pat::Blank,
        Pat::LetName(n), Pat::Mark(':'), Pat::Blank,
        p, Pat::Blank], FollowPat::Mark('}')
        )
        if p.is_type() => {
            let top = Pat::LetNameWithType(
                n.to_string(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(6, top)
        }
        // `{` LetNameWithType `}` -> ProductType
        ([..,
        Pat::Mark('{'),
        Pat::LetNameWithType(n, t),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::ProductType(vec![
                (n.clone(), *t.clone())
            ]);
            stack.reduce_to_new(3, top)
        }
        // LetNameWithType LetNameWithType -> LetNameWithTypeSeq
        ([..,
        Pat::LetNameWithType(a_n, a_t),
        Pat::LetNameWithType(b_n, b_t),
        ], _
        ) => {
            let top = Pat::LetNameWithTypeSeq(vec![
                (a_n.clone(), *a_t.clone()),
                (b_n.clone(), *b_t.clone()),
            ]);
            stack.reduce_to_new(2, top)
        }
        // LetNameWithTypeSeq LetNameWithType -> LetNameWithTypeSeq
        ([..,
        Pat::LetNameWithTypeSeq(seq),
        Pat::LetNameWithType(n, t),
        ], _
        ) => {
            let top = Pat::LetNameWithTypeSeq(seq.push_to_new(
                (n.clone(), *t.clone())
            ));
            stack.reduce_to_new(2, top)
        }
        // "{ " LetNameWithTypeSeq " }" -> ProductType
        ([..,
        Pat::Mark('{'),
        Pat::LetNameWithTypeSeq(seq),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::ProductType(seq.clone());
            stack.reduce_to_new(3, top)
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
        ([p], FollowPat::End) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(&reduced_stack, tail)
    }
}