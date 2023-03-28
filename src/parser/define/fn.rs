use crate::parser::alphanum::parse_alphanum;
use crate::parser::define::pat::Pat;
use crate::parser::expr::parse_expr;
use crate::parser::infra::{Either, vec_get_head_tail_follow, VecExt};
use crate::parser::keyword::Keyword;
use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;
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
                // ' ' -> Blank
                ' ' => Pat::Blank,
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

fn reduce_stack(stack: &Vec<Pat>, follow: Option<Out>) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], _) => {
            return vec![p.clone()];
        }

        // KwDef Blank LetName Blank `=` Blank -> TypeDefHead End
        ([..,
        Pat::Kw(Keyword::Type), Pat::Blank,
        Pat::TypeName(n), Pat::Blank, Pat::Mark('='), Pat::Blank], _
        ) => {
            let top = Pat::TypeDefHead(n.to_string());
            stack
                .reduce_to_new(6, top)
                .push_to_new(Pat::End)
        }

        // KwDef Blank LetName Blank `=` Blank -> ExprDefHead End
        ([..,
        Pat::Kw(Keyword::Def), Pat::Blank,
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

    reduce_stack(&reduced_stack, follow)
}

pub fn go(stack: &Vec<Pat>, seq: Vec<Out>) -> Pat {
    let (head, tail, follow) =
        vec_get_head_tail_follow(seq);

    let stack = stack.push_to_new(move_in(stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow);

    let reduced_stack = reduce_stack(&stack, follow.clone());

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
        _ => go(&reduced_stack, tail)
    }
}
