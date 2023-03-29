use crate::parser::expr::pat::Pat;
use crate::parser::infra::option::Ext as OptExt;
use crate::parser::infra::r#box::Ext as BoxExt;
use crate::parser::infra::vec::{Ext, vec_get_head_tail_follow};
use crate::parser::keyword::Keyword;
use crate::parser::preprocess::Out;

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

fn reduce_stack(mut stack: Vec<Pat>, follow: Option<Out>) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

        // Expr `:` -> TypedExprHead
        // TypedExprHead: TypeSymbolSeq :EndPat -> Expr
        /* TODO: 此产生式要求当 Type 后存在空白时:
                 x: A -> B ->
                 Expr: Type 必须被括号环绕:
                 (x: A -> B) ->
                 (x: A -> { x: Int }) ->
                 否则将无法归约 */
        // TypedExprHead TypeName -> Expr

        // `(` Expr `)` -> Expr
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) if p.is_expr() =>
            stack.reduce(3, p.clone()),

        // KwIf Expr KwThen Expr KwElse Expr ... -> Cond
        ([..,
        Pat::Kw(Keyword::If), a,
        Pat::Kw(Keyword::Then), b,
        Pat::Kw(Keyword::Else), c
        ], follow)
        if follow.is_end_pat() && a.is_expr() && b.is_expr() && c.is_expr() =>
            stack.reduce(6, Pat::Cond(
                None,
                a.clone().boxed(),
                b.clone().boxed(),
                c.clone().boxed(),
            )),

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce(2, Pat::Arrow),
        // LetName Arrow -> ClosurePara
        ([.., Pat::LetName(n), Pat::Arrow], _) => {
            let top = Pat::ClosurePara(n.to_string());
            stack.reduce(2, top)
        }
        // ClosurePara Expr :EndPat -> Closure
        ([.., Pat::ClosurePara(n), p], follow)
        if follow.is_end_pat() && p.is_expr() => {
            let top = Pat::Closure(
                None,
                n.to_string(),
                None,
                p.clone().boxed(),
            );
            stack.reduce(2, top)
        }

        // LetName `=` Expr `,` -> Assign
        ([..,
        Pat::LetName(n), Pat::Mark('='),
        p, Pat::Mark(',')], _
        )
        if p.is_expr() => {
            let top = Pat::Assign(n.clone(), p.clone().boxed());
            stack.reduce(4, top)
        }
        // LetName `=` Expr :`}`-> Assign
        ([..,
        Pat::LetName(n), Pat::Mark('='),
        p], Some(Out::Symbol('}'))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                n.clone(),
                p.clone().boxed(),
            );
            stack.reduce(3, top)
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
            stack.reduce(2, top)
        }
        // AssignSeq Assign -> AssignSeq
        ([..,
        Pat::AssignSeq(vec),
        Pat::Assign(n, v)], _
        ) => {
            let top = Pat::AssignSeq(vec.push_to_new(
                (n.clone(), *v.clone())
            ));
            stack.reduce(2, top)
        }
        // `{` AssignSeq `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::AssignSeq(a_seq),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(a_seq.clone());
            stack.reduce(3, top)
        }
        // `{` Assign `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::Assign(n, v),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(vec![(n.to_string(), *v.clone())]);
            stack.reduce(3, top)
        }

        // KwMatch Expr KwWith -> MatchHead
        ([..,
        Pat::Kw(Keyword::Match),
        p, Pat::Kw(Keyword::With)], _
        )
        if p.is_expr() => {
            let top = Pat::MatchHead(p.clone().boxed());
            stack.reduce(3, top)
        }
        // `|` Expr -> CaseHead
        ([..,
        Pat::Mark('|'),
        p], _
        )
        if p.is_expr() => {
            let top = Pat::CaseHead(p.clone().boxed());
            stack.reduce(2, top)
        }
        // CaseHead Arrow Expr :EndPat -> Case
        ([..,
        Pat::CaseHead(e), Pat::Arrow,
        p  ], follow
        )
        // Case 的终结标记与一般的语言构造恰好相同
        if follow.is_end_pat() && p.is_expr() => {
            let top = Pat::Case(
                e.clone(),
                p.clone().boxed(),
            );
            stack.reduce(3, top)
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
            stack.reduce(2, top)
        }
        // CaseSeq Case -> CaseSeq
        ([..,
        Pat::CaseSeq(vec),
        Pat::Case(case, then) ], _
        ) => {
            let top = Pat::CaseSeq(vec.push_to_new(
                (*case.clone(), *then.clone())
            ));
            stack.reduce(2, top)
        }
        // MatchHead Case :!`|` -> Match
        ([..,
        Pat::MatchHead(h_e),
        Pat::Case(case, then) ], follow
        )
        if match follow {
            // 存在后继 Case 时拒绝归约
            Some(Out::Symbol('|')) => false,
            _ => true,
        } => {
            let top = Pat::Match(
                None,
                h_e.clone(),
                vec![((*case.clone(), *then.clone()))],
            );
            stack.reduce(2, top)
        }
        // MatchHead CaseSeq :!`|` -> Match
        ([..,
        Pat::MatchHead(h_e),
        Pat::CaseSeq(vec) ], follow
        )
        if match follow {
            // 存在后继 Case 时拒绝归约
            Some(Out::Symbol('|')) => false,
            _ => true,
        } => {
            let top = Pat::Match(
                None,
                h_e.clone(),
                vec.clone(),
            );
            stack.reduce(2, top)
        }

        // Expr Expr -> Apply
        ([.., lhs, rhs], _)
        if lhs.is_expr() && rhs.is_expr() => {
            let top = Pat::Apply(
                lhs.clone().boxed(),
                rhs.clone().boxed(),
            );
            stack.reduce(2, top)
        }

        // LetName `=` Expr :KwIn -> Assign
        ([..,
        Pat::LetName(n), Pat::Mark('='),
        p], Some(Out::Kw(Keyword::In))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                n.clone(),
                p.clone().boxed(),
            );
            stack.reduce(3, top)
        }
        // KwLet AssignSeq KwIn Expr :EndPat -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::AssignSeq(a_seq), Pat::Kw(Keyword::In),
        p], follow)
        if follow.is_end_pat() && p.is_expr() => {
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
            stack.reduce(4, top)
        }
        // KwLet Assign KwIn Expr :EndPat -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::Assign(n, e), Pat::Kw(Keyword::In),
        p], follow)
        if follow.is_end_pat() && p.is_expr() => {
            let top = Pat::Let(
                None,
                n.to_string(),
                None,
                *e.clone().boxed(),
                p.clone().boxed(),
            );
            stack.reduce(4, top)
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
        ([p], None) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(reduced_stack, tail)
    }
}
