use std::collections::BTreeSet;
use std::ops::Deref;

use crate::infra::option::WrapOption;
use crate::infra::rc::RcAnyExt;
use crate::infra::vec::VecExt;
use crate::lexer::{FollowExt, Token};
use crate::parser::r#type::pat::Pat;

pub fn reduce_stack(
    mut stack: Vec<Pat>,
    follow: Option<Token>
) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

        // LetName `:` Type :TypeEndPat -> LetName
        // where LetName is untyped
        ([.., Pat::LetName(None, n), Pat::Mark(':'), p], follow)
            if follow.is_type_end_pat() && p.is_type() =>
        {
            let top = Pat::LetName(
                p.clone().rc().wrap_some(),
                n.to_string()
            );
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
            let top = Pat::ClosureTypeHead(p.clone().rc());
            stack.reduce(2, top)
        }
        // ClosureTypeHead Type :TypeEndPat -> ClosureType
        ([.., Pat::ClosureTypeHead(t), p], follow)
            if follow.is_type_end_pat() && p.is_type() =>
        {
            let top = Pat::ClosureType(t.clone(), p.clone().rc());
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
            Some(Token::Symbol('}' | ','))
        ) => {
            let top = Pat::TypedLetNameSeq(vec![
                (a_n.clone(), a_t.deref().clone()),
                (b_n.clone(), b_t.deref().clone()),
            ]);
            stack.reduce(3, top)
        }
        // TypedLetNameSeq `,` LetName :`}`|`,` -> TypedLetNameSeq
        // where LetName is typed
        (
            [.., Pat::TypedLetNameSeq(seq), Pat::Mark(','), Pat::LetName(Some(t), n)],
            Some(Token::Symbol('}' | ','))
        ) => {
            let top = Pat::TypedLetNameSeq(
                seq.push_to_new((n.clone(), t.deref().clone()))
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
            let top =
                Pat::ProdType(vec![(n.clone(), t.deref().clone())]);
            stack.reduce(3, top)
        }
        // `{` LetName `,` `}` -> ProdType
        // where LetName is typed
        (
            [.., Pat::Mark('{'), Pat::LetName(Some(t), n), Pat::Mark(','), Pat::Mark('}')],
            _
        ) => {
            let top =
                Pat::ProdType(vec![(n.clone(), t.deref().clone())]);
            stack.reduce(4, top)
        }

        // Can not parse
        ([.., Pat::Err], _) => return vec![Pat::Err],
        // Can not reduce
        ([.., Pat::End], _) => {
            if cfg!(feature = "parser_lr1_log") {
                let log = format!("Reduction failed: {stack:?}");
                println!("{log}");
            }

            return vec![Pat::Err];
        }
        // keep move in
        _ => return stack
    };

    let reduced_stack = stack;

    if cfg!(feature = "parser_lr1_log") {
        let log = format!("Reduced: {reduced_stack:?}");
        println!("{log}");
    }

    reduce_stack(reduced_stack, follow)
}
