use std::collections::BTreeSet;
use crate::parser::expr::pat::Pat;
use crate::parser::infra::alias::MaybeType;
use crate::parser::infra::option::Ext as OptExt;
use crate::parser::infra::r#box::Ext as BoxExt;
use crate::parser::infra::vec::{Ext, vec_get_head_tail_follow};
use crate::parser::keyword::Keyword;

type In = crate::parser::preprocess::Out;

fn move_in(stack: &Vec<Pat>, head: Option<In>) -> Pat {
    match head {
        Some(o) => match (&stack[..], o) {
            // .. -> LetName
            (_, In::LetName(n)) => Pat::LetName(n, None),
            // .. -> Kw
            (_, In::Kw(kw)) => Pat::Kw(kw),
            // .. -> Int
            (_, In::IntValue(i)) => Pat::Int(i, None),
            // .. -> Unit
            (_, In::UnitValue) => Pat::Unit(None),
            // .. -> Discard
            (_, In::DiscardValue) => Pat::Discard,

            // TypedExprHead: _ -> TypeSymbol
            // TypeSymbolSeq: _ -> TypeSymbolSeq

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
                // '=' -> `=`
                '=' => Pat::Mark('='),
                // ',' -> `,`
                ',' => Pat::Mark(','),

                // '|' -> `|`
                '|' => Pat::Mark('|'),

                // ':' -> `:`
                ':' => Pat::Mark(':'),// type annotation usage

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

fn reduce_stack(mut stack: Vec<Pat>, follow: Option<In>) -> Vec<Pat> {
    match (&stack[..], &follow) {
        // Success
        ([Pat::Start, p, Pat::End], None) => return vec![p.clone()],

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
        ([.., Pat::LetName(n, None), Pat::Arrow], _) => {
            let top = Pat::ClosurePara(n.to_string(), None);
            stack.reduce(2, top)
        }
        // ClosurePara Expr :EndPat -> Closure
        ([.., Pat::ClosurePara(n, None), p], follow)
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
        Pat::LetName(n, None), Pat::Mark('='),
        p, Pat::Mark(',')], _
        )
        if p.is_expr() => {
            let top = Pat::Assign(n.clone(), None, p.clone().boxed());
            stack.reduce(4, top)
        }
        // LetName `=` Expr :`}`-> Assign
        ([..,
        Pat::LetName(n, None), Pat::Mark('='),
        p], Some(In::Symbol('}'))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                n.clone(),
                None,
                p.clone().boxed(),
            );
            stack.reduce(3, top)
        }
        // Assign Assign -> AssignSeq
        ([..,
        Pat::Assign(a_n, _, a_v),
        Pat::Assign(b_n, _, b_v)], _
        ) => {
            let top = Pat::AssignSeq(vec![
                (a_n.to_string(), None, *a_v.clone()),
                (b_n.to_string(), None, *b_v.clone()),
            ]);
            stack.reduce(2, top)
        }
        // AssignSeq Assign -> AssignSeq
        ([..,
        Pat::AssignSeq(vec),
        Pat::Assign(n, _, v)], _
        ) => {
            let top = Pat::AssignSeq(vec.push_to_new(
                (n.clone(), None, *v.clone())
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
        Pat::Assign(n, _, v),
        Pat::Mark('}')], _
        ) => {
            let top = Pat::Struct(vec![(n.to_string(), None, *v.clone())]);
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
            Some(In::Symbol('|')) => false,
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
                lhs.clone().boxed(),
                rhs.clone().boxed(),
            );
            stack.reduce(2, top)
        }

        // LetName `=` Expr :KwIn -> Assign
        ([..,
        Pat::LetName(n, None), Pat::Mark('='),
        p], Some(In::Kw(Keyword::In))
        )
        if p.is_expr() => {
            let top = Pat::Assign(
                n.clone(),
                None,
                p.clone().boxed(),
            );
            stack.reduce(3, top)
        }
        // KwLet AssignSeq KwIn Expr :EndPat -> Let
        ([.., Pat::Kw(Keyword::Let),
        Pat::AssignSeq(a_seq), Pat::Kw(Keyword::In),
        p], follow)
        if follow.is_end_pat() && p.is_expr() => {
            type F = fn(Pat, &(String, MaybeType, Pat)) -> Pat;
            let f: F = |acc, (n, _, e)| Pat::Let(
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
        Pat::Assign(n, _, e), Pat::Kw(Keyword::In),
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

        /* type annotation productions */

        /* TODO: ClosureType 与 Closure, Case 在归约上存在二义性, 表现为:
                 当 Type 后存在 `->` 时:
                 x: A -> <closure_body>
                 f: A -> B -> <closure_body>
                 | x: A -> <case_then>
                 | g: A -> B -> <case_then>
                 Type 必须由括号显示终结:
                 x: (A) -> <closure_body>
                 f: (A -> B) -> <closure_body>
                 | x: (A) -> <case_then>
                 | g: (A -> B) -> <case_then>
                 或:
                 (x: A) -> <closure_body>
                 (f: A -> B) -> <closure_body>
                 | (x: A) -> <case_then>
                 | (g: A -> B) -> <case_then>
                 否则类型标注将无法终结
                 重新设计 类型归约的终止模式 以解决该问题 */

        // Expr `:` -> TypedExprHead
        ([.., p, Pat::Mark(':')], _) if p.is_expr() =>
            stack.reduce(2, Pat::TypedExprHead(p.clone().boxed())),

        // TypedExprHead Type :EndPat -> Expr
        ([.., Pat::TypedExprHead(e), p], follow)
        if follow.is_end_pat() && p.is_type() =>
            stack.reduce(2, *e.clone()),

        // `(` Type `)` -> Type
        ([.., Pat::Mark('('), p, Pat::Mark(')')], _) if p.is_type() =>
            stack.reduce(3, p.clone()),

        // Type Arrow -> ClosureTypeHead
        ([.., p, Pat::Arrow, ], _)
        if p.is_type() => {
            let top = Pat::ClosureTypeHead(p.clone().boxed());
            stack.reduce(2, top)
        }
        // ClosureTypeHead Type :EndPat -> ClosureType
        ([.., Pat::ClosureTypeHead(t), p], follow)
        if follow.is_end_pat() && p.is_type() => {
            let top = Pat::ClosureType(
                t.clone(),
                p.clone().boxed(),
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

        // LetName `:` Type `,` -> LetNameWithType
        ([..,
        Pat::LetName(n, _), Pat::Mark(':'),
        p, Pat::Mark(',')], _
        )
        if p.is_type() => {
            let top = Pat::LetNameWithType(
                n.to_string(),
                p.clone().boxed(),
            );
            stack.reduce(4, top)
        }
        // LetName `:` Type :`}` -> LetNameWithType
        ([..,
        Pat::LetName(n, _), Pat::Mark(':'),
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
