use std::collections::BTreeSet;
use crate::parser::infra::option::Ext as OptExt;

use crate::parser::infra::r#box::Ext as BoxExt;
use crate::parser::infra::vec::{Ext, vec_get_head_tail_follow};
use crate::parser::r#type::pat::Pat;

type In = crate::parser::preprocess::Out;

fn move_in(stack: &Vec<Pat>, head: Option<In>) -> Pat {
    match head {
        Some(o) => match (&stack[..], o) {
            // .. -> LetName
            (_, In::LetName(n)) => Pat::LetName(n),
            // .. -> TypeName
            (_, In::TypeName(n)) => Pat::TypeName(n),

            // .. -> Mark
            (_, In::Symbol(s)) => match s {
                // '(' -> `(`
                '(' => Pat::Mark('('),
                // ')' -> `)`
                ')' => Pat::Mark(')'),

                // '-' -> `-`
                '-' => Pat::Mark('-'),
                // '>' -> `>`
                '>' => Pat::Mark('>'),

                // '{' -> `{`
                '{' => Pat::Mark('{'),
                // '}' -> `}`
                '}' => Pat::Mark('}'),
                // ',' -> `,`
                ',' => Pat::Mark(','),

                // '|' -> `|`
                '|' => Pat::Mark('|'),
                // ':' -> `:`
                ':' => Pat::Mark(':'),

                // _ -> Err
                c => {
                    println!("Invalid head Pat: {:?}", c);
                    Pat::Err
                }
            },

            // _ -> Err
            (_, p) => {
                println!("Invalid head Pat: {:?}", p);
                Pat::Err
            }
        }

        // ɛ -> End
        None => Pat::End,
    }
}

fn reduce_stack(mut stack: Vec<Pat>, follow: Option<In>) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

        // `(` Type `)` -> Type
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) if p.is_type() =>
            stack.reduce(3, p.clone()),

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce(2, Pat::Arrow),
        // Type Blank Arrow Blank -> ClosureTypeHead
        ([.., p, Pat::Arrow, ], _)
        if p.is_type() => {
            let top = Pat::ClosureTypeHead(p.clone().boxed());
            stack.reduce(2, top)
        }
        // ClosureTypeHead Type :!Blank -> ClosureType
        ([.., Pat::ClosureTypeHead(t), p], follow)
        if follow.is_end_pat() && p.is_type() => {
            let top = Pat::ClosureType(
                t.clone(),
                p.clone().boxed(),
            );
            stack.reduce(2, top)
        }

        // SumType Blank `|` Blank SumType -> SumType
        ([..,
        Pat::SumType(l), Pat::Mark('|'),
        Pat::SumType(r)], _
        ) => {
            let mut set = BTreeSet::new();
            set.extend(l.clone());
            set.extend(r.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // Type Blank `|` Blank SumType -> SumType
        ([..,
        p, Pat::Mark('|'),
        Pat::SumType(vec)], _
        )
        if p.is_type() => {
            let mut set = BTreeSet::new();
            set.extend(vec.clone());
            set.insert(p.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // SumType Blank `|` Blank Type -> SumType
        ([..,
        Pat::SumType(vec), Pat::Mark('|'),
        p], _
        )
        if p.is_type() => {
            let mut set = BTreeSet::new();
            set.extend(vec.clone());
            set.insert(p.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // Type Blank `|` Blank Type -> SumType
        ([.., a, Pat::Mark('|'), b], _)
        if a.is_type() && b.is_type() => {
            let mut set = BTreeSet::new();
            set.insert(a.clone());
            set.insert(b.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }

        // Blank LetName `:` Blank Type `,` -> LetNameWithType
        ([..,
        Pat::LetName(n), Pat::Mark(':'),
        p, Pat::Mark(',')], _
        )
        if p.is_type() => {
            let top = Pat::LetNameWithType(
                n.to_string(),
                p.clone().boxed(),
            );
            stack.reduce(4, top)
        }
        // Blank LetName `:` Blank Type Blank :`}` -> LetNameWithType
        ([..,
        Pat::LetName(n), Pat::Mark(':'),
        p], Some(In::Symbol('}'))
        )
        if p.is_type() => {
            let top = Pat::LetNameWithType(
                n.to_string(),
                p.clone().boxed(),
            );
            stack.reduce(3, top)
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
            stack.reduce(3, top)
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
            stack.reduce(2, top)
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
            stack.reduce(2, top)
        }
        // "{ " LetNameWithTypeSeq " }" -> ProductType
        ([..,
        Pat::Mark('{'),
        Pat::LetNameWithTypeSeq(seq),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::ProductType(seq.clone());
            stack.reduce(3, top)
        }

        // Can not parse
        ([.., Pat::Err], _) => return vec![Pat::Err],
        // Can not reduce
        ([.., Pat::End], _) => {
            println!("Reduction failed: {:?}", stack);
            return vec![Pat::Err];
        }
        // keep move in
        _ => return stack
    };

    let reduced_stack = stack;

    println!("Reduce to: {:?}", reduced_stack);

    reduce_stack(reduced_stack, follow)
}

pub fn go(mut stack: Vec<Pat>, seq: Vec<In>) -> Pat {
    let (head, tail, follow) =
        vec_get_head_tail_follow(seq);

    stack.push(move_in(&stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow);

    let reduced_stack = reduce_stack(stack, follow.clone());

    match (&reduced_stack[..], follow) {
        ([p], None) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(reduced_stack, tail)
    }
}
