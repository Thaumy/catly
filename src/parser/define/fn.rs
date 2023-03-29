use crate::parser::define::pat::Pat;
use crate::parser::expr::parse_expr;
use crate::parser::infra::vec::{Ext, vec_get_head_tail_follow};
use crate::parser::keyword::Keyword;
use crate::parser::preprocess::Out;
use crate::parser::r#type::parse_type;

fn move_in(stack: &Vec<Pat>, head: Option<Out>) -> Pat {
    match head {
        Some(o) => match (&stack[..], o) {
            // .. -> LetName
            (_, Out::LetName(n)) => Pat::LetName(n),
            // .. -> TypeName
            (_, Out::TypeName(n)) => Pat::TypeName(n),
            // .. -> Kw
            (_, Out::Kw(kw)) => Pat::Kw(kw),

            // .. -> Mark
            (_, Out::Symbol(s)) => match s {
                // '=' -> `=`
                '=' => Pat::Mark('='),

                // _ -> Err
                c => {
                    println!("Invalid head Pat: {:?}", c);
                    Pat::Err
                }
            }

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

fn reduce_stack(mut stack: Vec<Pat>, follow: Option<Out>) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], _) => {
            return vec![p.clone()];
        }

        // KwDef Blank LetName Blank `=` Blank -> TypeDefHead End
        ([..,
        Pat::Kw(Keyword::Type),
        Pat::TypeName(n), Pat::Mark('=')], _
        ) => {
            let top = Pat::TypeDefHead(n.to_string());
            stack.reduce(3, top);
            stack.push(Pat::End)
        }

        // KwDef Blank LetName Blank `=` Blank -> ExprDefHead End
        ([..,
        Pat::Kw(Keyword::Def),
        Pat::LetName(n), Pat::Mark('=')], _
        ) => {
            let top = Pat::ExprDefHead(n.to_string());
            stack.reduce(3, top);
            stack.push(Pat::End)
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

pub fn go(mut stack: Vec<Pat>, seq: Vec<Out>) -> Pat {
    let (head, tail, follow) =
        vec_get_head_tail_follow(seq);

    stack.push(move_in(&stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow);

    let reduced_stack = reduce_stack(stack, follow.clone());

    match (&reduced_stack[..], follow) {
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
        _ => go(reduced_stack, tail)
    }
}
