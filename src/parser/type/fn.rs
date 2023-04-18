use std::collections::BTreeSet;

use crate::infra::option::{AnyExt, FollowExt};
use crate::infra::r#box::Ext as BoxExt;
use crate::infra::vec::{vec_get_head_tail_follow, Ext};
use crate::parser::r#type::pat::Pat;

type In = crate::parser::preprocess::Out;

fn move_in(stack: &Vec<Pat>, head: Option<In>) -> Pat {
    match head {
        Some(o) => match (&stack[..], o) {
            // .. -> LetName
            (_, In::LetName(n)) => Pat::LetName(None, n),
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
                    println!("Invalid head Pat: {c:?}");
                    Pat::Err
                }
            },

            // _ -> Err
            (_, p) => {
                println!("Invalid head Pat: {p:?}");
                Pat::Err
            }
        },

        // ɛ -> End
        None => Pat::End
    }
}

fn reduce_stack(mut stack: Vec<Pat>, follow: Option<In>) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

        // LetName `:` Type :TypeEndPat -> LetName
        // where LetName is untyped
        ([.., Pat::LetName(None, n), Pat::Mark(':'), p], follow)
            if follow.is_type_end_pat() && p.is_type() =>
        {
            let top =
                Pat::LetName(p.clone().boxed().some(), n.to_string());
            stack.reduce(3, top)
        }

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce(2, Pat::Arrow),

        // `(` Type `)` -> Type
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _)
            if p.is_type() =>
            stack.reduce(3, p.clone()),

        // Type Arrow -> ClosureTypeHead
        ([.., p, Pat::Arrow], _) if p.is_type() => {
            let top = Pat::ClosureTypeHead(p.clone().boxed());
            stack.reduce(2, top)
        }
        // ClosureTypeHead Type :TypeEndPat -> ClosureType
        ([.., Pat::ClosureTypeHead(t), p], follow)
            if follow.is_type_end_pat() && p.is_type() =>
        {
            let top = Pat::ClosureType(t.clone(), p.clone().boxed());
            stack.reduce(2, top)
        }

        // SumType `|` SumType -> SumType
        (
            [.., Pat::SumType(l), Pat::Mark('|'), Pat::SumType(r)],
            _
        ) => {
            let mut set = BTreeSet::new();
            set.extend(l.clone());
            set.extend(r.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // Type `|` SumType -> SumType
        ([.., p, Pat::Mark('|'), Pat::SumType(vec)], _)
            if p.is_type() =>
        {
            let mut set = BTreeSet::new();
            set.extend(vec.clone());
            set.insert(p.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // SumType `|` Type -> SumType
        ([.., Pat::SumType(vec), Pat::Mark('|'), p], _)
            if p.is_type() =>
        {
            let mut set = BTreeSet::new();
            set.extend(vec.clone());
            set.insert(p.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // Type `|` Type -> SumType
        ([.., a, Pat::Mark('|'), b], _)
            if a.is_type() && b.is_type() =>
        {
            let mut set = BTreeSet::new();
            set.insert(a.clone());
            set.insert(b.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // LetName `,` LetName :`}`|`,` -> TypedLetNameSeq
        // where LetName is typed
        (
            [.., Pat::LetName(Some(a_t), a_n), Pat::Mark(','), Pat::LetName(Some(b_t), b_n)],
            Some(In::Symbol('}' | ','))
        ) => {
            let top = Pat::TypedLetNameSeq(vec![
                (a_n.clone(), *a_t.clone()),
                (b_n.clone(), *b_t.clone()),
            ]);
            stack.reduce(3, top)
        }
        // TypedLetNameSeq `,` LetName :`}`|`,` -> TypedLetNameSeq
        // where LetName is typed
        (
            [.., Pat::TypedLetNameSeq(seq), Pat::Mark(','), Pat::LetName(Some(t), n)],
            Some(In::Symbol('}' | ','))
        ) => {
            let top = Pat::TypedLetNameSeq(
                seq.push_to_new((n.clone(), *t.clone()))
            );
            stack.reduce(3, top)
        }
        // `{` TypedLetNameSeq `}` -> ProdType
        (
            [.., Pat::Mark('{'), Pat::TypedLetNameSeq(seq), Pat::Mark('}')],
            _
        ) => {
            let top = Pat::ProdType(seq.clone());
            stack.reduce(3, top)
        }
        // `{` TypedLetNameSeq `,` `}` -> ProdType
        (
            [.., Pat::Mark('{'), Pat::TypedLetNameSeq(seq), Pat::Mark(','), Pat::Mark('}')],
            _
        ) => {
            let top = Pat::ProdType(seq.clone());
            stack.reduce(4, top)
        }
        // `{` LetName `}` -> ProdType
        // where LetName is typed
        (
            [.., Pat::Mark('{'), Pat::LetName(Some(t), n), Pat::Mark('}')],
            _
        ) => {
            let top = Pat::ProdType(vec![(n.clone(), *t.clone())]);
            stack.reduce(3, top)
        }
        // `{` LetName `,` `}` -> ProdType
        // where LetName is typed
        (
            [.., Pat::Mark('{'), Pat::LetName(Some(t), n), Pat::Mark(','), Pat::Mark('}')],
            _
        ) => {
            let top = Pat::ProdType(vec![(n.clone(), *t.clone())]);
            stack.reduce(4, top)
        }

        // Can not parse
        ([.., Pat::Err], _) => return vec![Pat::Err],
        // Can not reduce
        ([.., Pat::End], _) => {
            println!("Reduction failed: {stack:?}");
            return vec![Pat::Err];
        }
        // keep move in
        _ => return stack
    };

    let reduced_stack = stack;

    // println!("Reduced: {reduced_stack:?}");

    reduce_stack(reduced_stack, follow)
}

pub fn go(mut stack: Vec<Pat>, seq: Vec<In>) -> Pat {
    let (head, tail, follow) = vec_get_head_tail_follow(seq);

    stack.push(move_in(&stack, head));
    // println!("Move in: {stack:?} follow: {follow:?}");

    let reduced_stack = reduce_stack(stack, follow.clone());

    match (&reduced_stack[..], follow) {
        ([p], None) => {
            let r = p.clone();

            //println!("Success with: {r:?}");

            r
        }
        _ => go(reduced_stack, tail)
    }
}
