use crate::infra::iter::IteratorExt;
use crate::infra::option::OptionAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::define::pat::Pat;
use crate::parser::expr::parse_expr;
use crate::parser::keyword::Keyword;
use crate::parser::r#type::parse_type;

type In = crate::pp::Out;

fn move_in(stack: &Vec<Pat>, head: Option<In>) -> Pat {
    match head {
        Some(o) => match (&stack[..], o) {
            // (KwDef LetName `:`): _ -> AnyInSeq
            // where LetName is untyped
            (
                [.., Pat::Kw(Keyword::Def), Pat::LetName(None, _), Pat::Mark(':')],
                x
            ) => Pat::AnyInSeq(vec![x]),
            // AnyInSeq: _ -> AnyIn
            ([.., Pat::AnyInSeq(_)], x) => Pat::AnyIn(x),

            // .. -> LetName
            (_, In::LetName(n)) => Pat::LetName(None, n),
            // .. -> TypeName
            (_, In::TypeName(n)) => Pat::TypeName(n),
            // .. -> Kw
            (_, In::Kw(kw)) => Pat::Kw(kw),

            // .. -> Mark
            (_, In::Symbol(s)) => match s {
                // '=' -> `=`
                '=' => Pat::Mark('='),
                // ':' -> `:`
                ':' => Pat::Mark(':'),

                // _ -> Err
                c => {
                    if cfg!(feature = "parser_lr1_log") {
                        let log = format!("Invalid head Pat: {c:?}");
                        println!("{log}");
                    }

                    Pat::Err
                }
            },

            // _ -> Err
            (_, p) => {
                if cfg!(feature = "parser_lr1_log") {
                    let log = format!("Invalid head Pat: {p:?}");
                    println!("{log}");
                }

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
        ([Pat::Start, p, Pat::End], _) => {
            return vec![p.clone()];
        }

        // KwType TypeName `=` -> TypeDefHead End
        (
            [.., Pat::Kw(Keyword::Type), Pat::TypeName(n), Pat::Mark('=')],
            _
        ) => {
            let top = Pat::TypeDefHead(n.to_string());
            stack.reduce(3, top);
            stack.push(Pat::End)
        }

        // AnyInSeq AnyIn -> AnyInSeq
        ([.., Pat::AnyInSeq(seq), Pat::AnyIn(x)], _) => {
            let seq = seq.push_to_new(x.clone());
            let top = Pat::AnyInSeq(seq);
            stack.reduce(2, top);
        }
        // AnyInSeq :`=` -> Type
        ([.., Pat::AnyInSeq(seq)], Some(In::Symbol('='))) =>
            match parse_type(seq.clone().into_iter()) {
                Some(t) => stack.reduce(1, Pat::Type(t)),
                None => return vec![Pat::Err]
            },
        // KwDef: LetName `:` Type -> LetName
        // where LetName is untyped
        (
            [.., Pat::Kw(Keyword::Def), Pat::LetName(None, n), Pat::Mark(':'), Pat::Type(t)],
            _
        ) => {
            let top = Pat::LetName(t.clone().some(), n.clone());
            stack.reduce(3, top);
        }

        // KwDef LetName `=` -> ExprDefHead End
        (
            [.., Pat::Kw(Keyword::Def), Pat::LetName(t, n), Pat::Mark('=')],
            _
        ) => {
            let top = Pat::ExprDefHead(t.clone(), n.to_string());
            stack.reduce(3, top);
            stack.push(Pat::End)
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

pub fn go<S>(mut stack: Vec<Pat>, seq: S) -> Pat
where
    S: Iterator<Item = In> + Clone
{
    let (head, tail, follow) = seq.get_head_tail_follow();

    stack.push(move_in(&stack, head));

    if cfg!(feature = "parser_lr1_log") {
        let log = format!("Move in: {stack:?} follow: {follow:?}");
        println!("{log}");
    }

    let reduced_stack = reduce_stack(stack, follow.clone());

    match (&reduced_stack[..], follow) {
        ([p], _) => {
            let head = p.clone();

            let r = match head {
                Pat::TypeDefHead(n) => parse_type(tail)
                    .map(|t| Pat::TypeDef(n, t))
                    .unwrap_or(Pat::Err),
                Pat::ExprDefHead(t, n) => parse_expr(tail)
                    .map(|e| Pat::ExprDef(n, t, e))
                    .unwrap_or(Pat::Err),
                _ => Pat::Err
            };

            if cfg!(feature = "parser_lr1_log") {
                let log = format!("Success with: {r:?}");
                println!("{log}");
            }

            r
        }
        _ => go(reduced_stack, tail)
    }
}
