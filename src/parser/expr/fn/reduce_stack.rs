use std::collections::BTreeSet;
use std::ops::Deref;

use crate::infra::option::OptionAnyExt;
use crate::infra::rc::RcAnyExt;
use crate::infra::vec::VecExt;
use crate::parser::expr::pat::{OptRcPat, Pat};
use crate::parser::expr::In;
use crate::parser::keyword::Keyword;
use crate::pp::FollowExt;

pub fn reduce_stack(
    mut stack: Vec<Pat>,
    follow: Option<In>
) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

        /* expression productions */

        // `(` Expr `)` -> Expr
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _)
        if p.is_expr() =>
            stack.reduce(3, p.clone()),

        // KwIf Expr KwThen Expr KwElse Expr ... -> Cond
        ([..,
        Pat::Kw(Keyword::If), a,
        Pat::Kw(Keyword::Then), b,
        Pat::Kw(Keyword::Else), c], follow
        )
        if follow.is_expr_end_pat()
            && a.is_expr()
            && b.is_expr()
            && c.is_expr()
        => stack.reduce(6, Pat::Cond(
            None,
            a.clone().rc(),
            b.clone().rc(),
            c.clone().rc(),
        )),

        // `-` `>` -> Arrow
        ([.., Pat::Mark('-'), Pat::Mark('>')], _) =>
            stack.reduce(2, Pat::Arrow),
        // Discard Arrow -> ClosureInput
        ([..,
        Pat::Discard(t), Pat::Arrow], _
        ) => {
            let top = Pat::ClosureInput(
                None,
                t.clone(),
            );
            stack.reduce(2, top)
        }
        // LetName Arrow -> ClosureInput
        ([..,
        Pat::LetName(t, n), Pat::Arrow], _
        ) => {
            let top = Pat::ClosureInput(
                n.to_string().some(),
                t.clone(),
            );
            stack.reduce(2, top)
        }
        // ClosureInput Expr :ExprEndPat -> Closure
        ([..,
        Pat::ClosureInput(n, t),
        p], follow
        )
        if follow.is_expr_end_pat() && p.is_expr() => {
            let top = Pat::Closure(
                None,
                n.clone(),
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(2, top)
        }

        // KwRec LetName `=` Expr `,` -> Assign
        ([..,
        Pat::Kw(Keyword::Rec),
        Pat::LetName(t, n), Pat::Mark('='),
        p, Pat::Mark(',')], _
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                true,
                n.clone(),
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(5, top)
        }
        // LetName `=` Expr `,` -> Assign
        ([..,
        Pat::LetName(t, n), Pat::Mark('='),
        p, Pat::Mark(',')], _
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                false,
                n.clone(),
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(4, top)
        }
        // LetName `=` Expr :`}`-> Assign
        ([..,
        Pat::LetName(t, n), Pat::Mark('='),
        p], Some(In::Symbol('}'))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                false,
                n.clone(),
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(3, top)
        }
        // Assign Assign -> AssignSeq
        ([..,
        Pat::Assign(a_r_a, a_n, a_t, a_v),
        Pat::Assign(b_r_a, b_n, b_t, b_v)], _
        ) => {
            let top = Pat::AssignSeq(vec![
                (a_r_a.clone(), a_n.to_string(), a_t.clone(), a_v.deref().clone()),
                (b_r_a.clone(), b_n.to_string(), b_t.clone(), b_v.deref().clone()),
            ]);
            stack.reduce(2, top)
        }
        // AssignSeq Assign -> AssignSeq
        ([..,
        Pat::AssignSeq(vec),
        Pat::Assign(r_a, n, t, v)], _
        ) => {
            let top = Pat::AssignSeq(vec.push_to_new(
                (r_a.clone(), n.clone(), t.clone(), v.deref().clone())
            ));
            stack.reduce(2, top)
        }
        // `{` AssignSeq `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::AssignSeq(seq), // no rec
        Pat::Mark('}')], _
        ) if seq.iter().all(|(r_a, ..)| !*r_a) => {
            let seq = seq
                .clone()
                .into_iter()
                .map(|(_, x, y, z)| (x, y, z))
                .collect();

            let top = Pat::Struct(
                None,
                seq,
            );
            stack.reduce(3, top)
        }
        // `{` Assign `}` -> Struct
        ([..,
        Pat::Mark('{'),
        Pat::Assign(false, n, t, v), // no rec
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(
                None,
                vec![(n.to_string(), t.clone(), v.deref().clone())],
            );
            stack.reduce(3, top)
        }

        // KwMatch Expr KwWith -> MatchHead
        ([..,
        Pat::Kw(Keyword::Match),
        p, Pat::Kw(Keyword::With)], _
        )
        if p.is_expr() => {
            let top = Pat::MatchHead(p.clone().rc());
            stack.reduce(3, top)
        }
        // `|` Expr :`-` -> CaseHead
        ([..,
        Pat::Mark('|'),
        p], Some(In::Symbol('-'))
        )
        if p.is_expr() => {
            let top = Pat::CaseHead(p.clone().rc());
            stack.reduce(2, top)
        }
        // CaseHead Arrow Expr :ExprEndPat -> Case
        ([..,
        Pat::CaseHead(e), Pat::Arrow,
        p  ], follow
        )
        // Case 的终结标记与一般的语言构造恰好相同
        if follow.is_expr_end_pat() && p.is_expr() => {
            let top = Pat::Case(
                e.clone(),
                p.clone().rc(),
            );
            stack.reduce(3, top)
        }
        // Case Case -> CaseSeq
        ([..,
        Pat::Case(a_case, a_then),
        Pat::Case(b_case, b_then) ], _
        ) => {
            let top = Pat::CaseSeq(vec![
                (a_case.deref().clone(), a_then.deref().clone()),
                (b_case.deref().clone(), b_then.deref().clone()),
            ]);
            stack.reduce(2, top)
        }
        // CaseSeq Case -> CaseSeq
        ([..,
        Pat::CaseSeq(vec),
        Pat::Case(case, then) ], _
        ) => {
            let top = Pat::CaseSeq(vec.push_to_new(
                (case.deref().clone(), then.deref().clone())
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
            Some(In::Symbol('|')) => false,
            _ => true,
        } => {
            let top = Pat::Match(
                None,
                h_e.clone(),
                vec![((case.deref().clone(), then.deref().clone()))],
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
            Some(In::Symbol('|')) => false,
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
                None,
                lhs.clone().rc(),
                rhs.clone().rc(),
            );
            stack.reduce(2, top)
        }

        // KwRec LetName `=` Expr :KwIn -> Assign
        ([..,
        Pat::Kw(Keyword::Rec),
        Pat::LetName(t, n), Pat::Mark('='),
        p], Some(In::Kw(Keyword::In))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                true,
                n.clone(),
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(4, top)
        }
        // LetName `=` Expr :KwIn -> Assign
        ([..,
        Pat::LetName(t, n), Pat::Mark('='),
        p], Some(In::Kw(Keyword::In))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                false,
                n.clone(),
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(3, top)
        }
        // KwLet AssignSeq KwIn Expr :ExprEndPat -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::AssignSeq(seq), Pat::Kw(Keyword::In),
        p], follow)
        if follow.is_expr_end_pat() && p.is_expr() => {
            type F = fn(Pat, (bool, String, OptRcPat, Pat)) -> Pat;
            let f: F = |acc, (r_a, n, t, e)|
                Pat::Let(
                    None,
                    r_a,// TODO
                    n.to_string(),
                    t.clone(),
                    e.clone().rc(),
                    acc.rc(),
                );
            let top = seq
                .clone()
                .into_iter()
                .rev()
                .fold(p.clone(), f);
            stack.reduce(4, top)
        }
        // KwLet Assign KwIn Expr :ExprEndPat -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::Assign(r_a, n, t, e), Pat::Kw(Keyword::In),
        p], follow)
        if follow.is_expr_end_pat() && p.is_expr() => {
            let top = Pat::Let(
                None,
                r_a.clone(),
                n.to_string(),
                t.clone(),
                e.clone(),
                p.clone().rc(),
            );
            stack.reduce(4, top)
        }

        /* type annotation productions */

        /* TODO:
        # 1
        类型标注的结合优先级低于 Apply

        # 2
        当类型标注发生于 Closure body 时, 类型会优先标注到 Closure body,
        需要对其合理性进行评估

        # 4
        SumType 与 Case 在存在归约冲突, 表现为:
        当 Case value 后存在类型标注时:
        | a: Int ->

        需要显式限定类型标注的优先级, 从而使其归约终结:
        | (a: Int) ->

        # 5
        ClosureType 与 Closure, Case 在归约上存在二义性, 表现为:

        当 Type 后存在 `->` 时:
        x: A -> <closure_body>
        f: A -> B -> <closure_body>
        | x: A -> <case_then>
        | g: A -> B -> <case_then>

        Type 必须由括号显示终结:
        (x: A) -> <closure_body>
        (f: A -> B) -> <closure_body>
        | (x: A) -> <case_then>
        | (g: A -> B) -> <case_then>

        否则类型标注将无法终结
        重新设计 类型归约的终止模式 以解决该问题 */

        // Expr `:` -> TypedExprHead
        ([.., p, Pat::Mark(':')], _) if p.is_expr() =>
            stack.reduce(2, Pat::TypedExprHead(p.clone().rc())),

        // TypedExprHead Type :TypeEndPat -> Expr
        ([.., Pat::TypedExprHead(e), p], follow)
        if follow.is_type_end_pat() && p.is_type() =>
            match e.deref().clone().with_type(p.clone())
            {
                Some(e) => stack.reduce(2, e),
                _ => return vec![Pat::Err]
            }

        // `(` Type `)` -> Type
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) if p.is_type() =>
            stack.reduce(3, p.clone()),

        // Type Arrow -> ClosureTypeHead
        ([.., p, Pat::Arrow, ], _)
        if p.is_type() => {
            let top = Pat::ClosureTypeHead(p.clone().rc());
            stack.reduce(2, top)
        }
        // ClosureTypeHead Type :TypeEndPat -> ClosureType
        ([.., Pat::ClosureTypeHead(t), p], follow)
        if follow.is_type_end_pat() && p.is_type() => {
            let top = Pat::ClosureType(
                t.clone(),
                p.clone().rc(),
            );
            stack.reduce(2, top)
        }

        // SumType `|` SumType -> SumType
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
        // Type `|` SumType -> SumType
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
        // SumType `|` Type -> SumType
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
        // Type `|` Type -> SumType
        ([.., a, Pat::Mark('|'), b], _)
        if a.is_type() && b.is_type() => {
            let mut set = BTreeSet::new();
            set.insert(a.clone());
            set.insert(b.clone());

            let top = Pat::SumType(set);
            stack.reduce(3, top)
        }
        // LetName `,` LetName :`}`|`,` -> TypedLetNameSeq
        // where LetName is typed
        ([..,
        Pat::LetName(Some(a_t), a_n), Pat::Mark(','),
        Pat::LetName(Some(b_t), b_n),
        ], Some(In::Symbol('}' | ','))
        ) => {
            let top = Pat::TypedLetNameSeq(vec![
                (a_n.clone(), a_t.deref().clone()),
                (b_n.clone(), b_t.deref().clone()),
            ]);
            stack.reduce(3, top)
        }
        // TypedLetNameSeq `,` LetName :`}`|`,` -> TypedLetNameSeq
        // where LetName is typed
        ([..,
        Pat::TypedLetNameSeq(seq), Pat::Mark(','),
        Pat::LetName(Some(t), n),
        ], Some(In::Symbol('}' | ','))
        ) => {
            let top = Pat::TypedLetNameSeq(seq.push_to_new(
                (n.clone(), t.deref().clone())
            ));
            stack.reduce(3, top)
        }
        // `{` TypedLetNameSeq `}` -> ProdType
        ([..,
        Pat::Mark('{'),
        Pat::TypedLetNameSeq(seq),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::ProdType(seq.clone());
            stack.reduce(3, top)
        }
        // `{` TypedLetNameSeq `,` `}` -> ProdType
        ([..,
        Pat::Mark('{'),
        Pat::TypedLetNameSeq(seq),
        Pat::Mark(','), Pat::Mark('}')], _
        ) => {
            let top = Pat::ProdType(seq.clone());
            stack.reduce(4, top)
        }
        // `{` LetName `}` -> ProdType
        // where LetName is typed
        ([..,
        Pat::Mark('{'),
        Pat::LetName(Some(t), n),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::ProdType(vec![
                (n.clone(), t.deref().clone())
            ]);
            stack.reduce(3, top)
        }
        // `{` LetName `,` `}` -> ProdType
        // where LetName is typed
        ([..,
        Pat::Mark('{'),
        Pat::LetName(Some(t), n),
        Pat::Mark(','), Pat::Mark('}')], _
        ) => {
            let top = Pat::ProdType(vec![
                (n.clone(), t.deref().clone())
            ]);
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
