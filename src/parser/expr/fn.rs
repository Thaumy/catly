use crate::parser::alphanum::{parse_alphanum, parse_digit};
use crate::parser::expr::pat::Pat;
use crate::parser::infra::{BoxExt, Either, vec_get_head_tail_follow, VecExt};
use crate::parser::keyword::{Keyword};
use crate::parser::name::let_name::parse_let_name;
use crate::parser::preprocess::Out;
use crate::parser::value::int::parse_int;

fn move_in(stack: &Vec<Pat>, head: Option<Out>) -> Pat {
    match head {
        Some(o) => match (&stack[..], o) {
            // .. -> LetName
            (_, Out::LetName(n)) => Pat::LetName(n),
            // .. -> Kw
            (_, Out::Kw(kw)) => Pat::Kw(kw),
            // .. -> Int
            (_, Out::IntValue(i)) => Pat::Int(i),
            // .. -> Unit
            (_, Out::UnitValue) => Pat::Unit,
            // .. -> Discard
            (_, Out::DiscardValue) => Pat::Discard,

            // TypedExprHead: _ -> TypeSymbol
            // TypeSymbolSeq: _ -> TypeSymbolSeq

            // .. -> Mark
            (_, Out::Symbol(s)) => match s {
                // ' ' -> Blank
                ' ' => Pat::Blank,
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
                // '=' -> `=`
                '=' => Pat::Mark('='),
                // ',' -> `,`
                ',' => Pat::Mark(','),

                // '|' -> `|`
                '|' => Pat::Mark('|'),

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
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

        // Expr `:` Blank -> TypedExprHead
        // TypedExprHead: TypeSymbolSeq :!Blank -> Expr
        /* TODO: 此产生式要求当 Type 后存在空白时:
                 x: A -> B ->
                 Expr: Type 必须被括号环绕:
                 (x: A -> B) ->
                 (x: A -> { x: Int }) ->
                 否则将无法归约 */
        // TypedExprHead Blank TypeName -> Expr

        // `(` Expr `)` -> Expr
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) if p.is_expr() =>
            stack.reduce_to_new(3, p.clone()),

        // KwIf Blank Expr Blank KwThen Blank Expr Blank KwElse Blank Expr -> Cond
        ([..,
        Pat::Kw(Keyword::If), Pat::Blank, a, Pat::Blank,
        Pat::Kw(Keyword::Then), Pat::Blank, b, Pat::Blank,
        Pat::Kw(Keyword::Else), Pat::Blank, c
        ], _)
        if a.is_expr() && b.is_expr() && c.is_expr() =>
            stack.reduce_to_new(11, Pat::Cond(
                None,
                a.clone().boxed(),
                b.clone().boxed(),
                c.clone().boxed(),
            )),

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce_to_new(2, Pat::Arrow),
        // LetName Blank Arrow Blank -> ClosurePara
        ([.., Pat::LetName(n), Pat::Blank, Pat::Arrow, Pat::Blank], _) => {
            let top = Pat::ClosurePara(n.to_string());
            stack.reduce_to_new(4, top)
        }
        // ClosurePara Expr :!Blank -> Closure
        /* TODO: 此产生式要求当 Closure 具备如下形式时:
                 x -> y -> z }
                 Closure 必须被括号环绕:
                 (x -> y -> z) }
                 否则将无法归约 */
        ([.., Pat::ClosurePara(n), p], follow)
        if match follow {
            Some(Out::Symbol(' ')) => false,
            _ => true,
        } && p.is_expr() => {
            let top = Pat::Closure(
                None,
                n.to_string(),
                None,
                p.clone().boxed(),
            );
            stack.reduce_to_new(2, top)
        }

        // Blank LetName Blank `=` Blank Expr `,` -> Assign
        ([.., Pat::Blank,
        Pat::LetName(n), Pat::Blank, Pat::Mark('='), Pat::Blank,
        p, Pat::Mark(',')], _
        )
        if p.is_expr() => {
            let top = Pat::Assign(n.clone(), p.clone().boxed());
            stack.reduce_to_new(7, top)
        }
        // Blank LetName Blank `=` Blank Expr Blank :`}`-> Assign
        ([.., Pat::Blank,
        Pat::LetName(n), Pat::Blank, Pat::Mark('='), Pat::Blank,
        p, Pat::Blank], Some(Out::Symbol('}'))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                n.clone(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(7, top)
        }
        // Assign Assign -> AssignSeq
        ([..,
        Pat::Assign(a_n, a_v),
        Pat::Assign(b_n, b_v)], _
        ) => {
            let top = Pat::AssignSeq(vec![
                (a_n.to_string(), *a_v.clone()),
                (b_n.to_string(), *b_v.clone()),
            ]);
            stack.reduce_to_new(2, top)
        }
        // AssignSeq Assign -> AssignSeq
        ([..,
        Pat::AssignSeq(a_seq),
        Pat::Assign(n, v)], _
        ) => {
            let top = Pat::AssignSeq(
                a_seq.push_to_new((n.clone(), *v.clone()))
            );
            stack.reduce_to_new(2, top)
        }
        // `{` AssignSeq `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::AssignSeq(a_seq),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(a_seq.clone());
            stack.reduce_to_new(3, top)
        }
        // `{` Assign `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::Assign(n, v),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(vec![(n.to_string(), *v.clone())]);
            stack.reduce_to_new(3, top)
        }

        // KwMatch Blank Expr Blank KwWith Blank -> MatchHead
        ([..,
        Pat::Kw(Keyword::Match), Pat::Blank,
        p, Pat::Blank, Pat::Kw(Keyword::With), Pat::Blank], _
        )
        if p.is_expr() => {
            let top = Pat::MatchHead(p.clone().boxed());
            stack.reduce_to_new(6, top)
        }
        // `|` Blank Expr Blank Arrow -> CaseHead
        ([..,
        Pat::Mark('|'), Pat::Blank,
        p, Pat::Blank, Pat::Arrow], _
        )
        if p.is_expr() => {
            let top = Pat::CaseHead(p.clone().boxed());
            stack.reduce_to_new(5, top)
        }
        // CaseHead Blank Expr Blank :VerticalBar -> Case
        ([..,
        Pat::CaseHead(e), Pat::Blank,
        p, Pat::Blank ], Some(Out::Symbol('|'))
        )
        if p.is_expr() => {
            let top = Pat::Case(
                e.clone(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(4, top)
        }
        // CaseHead Blank Expr :!Blank -> Case
        ([..,
        Pat::CaseHead(e), Pat::Blank,
        p  ], follow
        )
        if match follow {
            Some(Out::Symbol(' ')) => false,
            _ => true,
        } && p.is_expr() => {
            let top = Pat::Case(
                e.clone(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(3, top)
        }
        // Case Case -> CaseSeq
        ([..,
        Pat::Case(a_case, a_then),
        Pat::Case(b_case, b_then) ], _
        ) => {
            let top = Pat::CaseSeq(vec![
                (*a_case.clone(), *a_then.clone()),
                (*b_case.clone(), *b_then.clone()),
            ]);
            stack.reduce_to_new(2, top)
        }
        // CaseSeq Case -> CaseSeq
        ([..,
        Pat::CaseSeq(vec),
        Pat::Case(case, then) ], _
        ) => {
            let top = Pat::CaseSeq(
                vec.push_to_new((*case.clone(), *then.clone()))
            );
            stack.reduce_to_new(2, top)
        }
        // MatchHead Case :!(Blank|`|`) -> Match
        ([..,
        Pat::MatchHead(h_e),
        Pat::Case(case, then) ], follow
        )
        if match follow {
            Some(Out::Symbol(' ')) | Some(Out::Symbol('|')) => false,
            _ => true
        } => {
            let top = Pat::Match(
                None,
                h_e.clone(),
                vec![((*case.clone(), *then.clone()))],
            );
            stack.reduce_to_new(2, top)
        }
        // MatchHead CaseSeq :!(Blank|`|`) -> Match
        ([..,
        Pat::MatchHead(h_e),
        Pat::CaseSeq(vec) ], follow
        )
        if match follow {
            Some(Out::Symbol(' ')) | Some(Out::Symbol('|')) => false,
            _ => true
        } => {
            let top = Pat::Match(
                None,
                h_e.clone(),
                vec.clone(),
            );
            stack.reduce_to_new(2, top)
        }

        // Expr Blank Expr -> Apply
        ([.., lhs, Pat::Blank, rhs], _)
        if lhs.is_expr() && rhs.is_expr() => {
            let top = Pat::Apply(
                lhs.clone().boxed(),
                rhs.clone().boxed(),
            );
            stack.reduce_to_new(3, top)
        }

        // Blank LetName Blank `=` Blank Expr Blank :KwIn -> Assign
        ([.., Pat::Blank,
        Pat::LetName(n), Pat::Blank, Pat::Mark('='), Pat::Blank,
        p, Pat::Blank], Some(Out::Kw(Keyword::In))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                n.clone(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(7, top)
        }
        // KwLet AssignSeq KwIn Blank Expr :!Blank -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::AssignSeq(a_seq), Pat::Kw(Keyword::In), Pat::Blank,
        p], follow)
        if match follow {
            Some(Out::Symbol(' ')) => false,
            _ => true,
        } && p.is_expr() => {
            type F = fn(Pat, &(String, Pat)) -> Pat;
            let f: F = |acc, (n, e)| Pat::Let(
                None,
                n.to_string(),
                None,
                e.clone().boxed(),
                acc.boxed(),
            );
            let top = a_seq
                .iter()
                .rev()
                .fold(p.clone(), f);
            stack.reduce_to_new(5, top)
        }
        // KwLet Assign KwIn Blank Expr :!Blank -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::Assign(n, e), Pat::Kw(Keyword::In), Pat::Blank,
        p], follow)
        if match follow {
            Some(Out::Symbol(' ')) => false,
            _ => true,
        } && p.is_expr() => {
            let top = Pat::Let(
                None,
                n.to_string(),
                None,
                *e.clone().boxed(),
                p.clone().boxed(),
            );
            stack.reduce_to_new(5, top)
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
        ([p], None) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(&reduced_stack, tail)
    }
}
