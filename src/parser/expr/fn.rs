use crate::parser::alphanum::{parse_alphanum, parse_digit};
use crate::parser::expr::pat::Pat;
use crate::parser::follow_pat::{FollowPat, parse_follow_pat};
use crate::parser::infra::{BoxExt, Either, vec_get_head_tail_follow, VecExt};
use crate::parser::keyword::{Keyword};
use crate::parser::name::let_name::parse_let_name;
use crate::parser::value::int::parse_int;

fn move_in(stack: &Vec<Pat>, head: Option<Either<char, Keyword>>) -> Pat {
    match head {
        Some(Either::L(c)) => match (&stack[..], c) {
            // DigitSeq: [0-9] -> Digit
            ([.., Pat::DigitSeq(_)], c) if parse_digit(&c).is_some() =>
                Pat::Digit(c),
            // [0-9] -> Digit
            (_, c) if parse_digit(&c).is_some() =>
                Pat::Digit(c),

            // AlphanumSeq: [0-9a-zA-Z] -> Alphanum
            ([.., Pat::AlphanumSeq(_)], c) if parse_alphanum(&c).is_some() =>
                Pat::Alphanum(c),
            // [0-9a-zA-Z] -> Alphanum
            (_, c) if parse_alphanum(&c).is_some() =>
                Pat::Alphanum(c),

            // TypedExprHead: _ -> TypeSymbol
            // TypeSymbolSeq: _ -> TypeSymbolSeq

            // ' ' -> Blank
            (_, ' ') => Pat::Blank,
            // '(' -> `(`
            (_, '(') => Pat::Mark('('),
            // ')' -> `)`
            (_, ')') => Pat::Mark(')'),

            // '-' -> `-`
            (_, '-') => Pat::Mark('-'),
            // '>' -> `>`
            (_, '>') => Pat::Mark('>'),

            // '{' -> `{`
            (_, '{') => Pat::Mark('{'),
            // '}' -> `}`
            (_, '}') => Pat::Mark('}'),
            // '=' -> `=`
            (_, '=') => Pat::Mark('='),
            // ',' -> `,`
            (_, ',') => Pat::Mark(','),

            // '|' -> `|`
            (_, '|') => Pat::Mark('|'),
            // '_' -> Discard
            (_, '_') => Pat::Discard,

            // _ -> Err
            (_, c) => {
                println!("Invalid head Pat: {:?}", c);
                Pat::Err
            }
        }

        Some(Either::R(kw)) => kw.into(),

        // ɛ -> End
        None => Pat::End,
    }
}

fn reduce_stack(stack: &Vec<Pat>, follow_pat: &FollowPat) -> Vec<Pat> {
    let reduced_stack = match (&stack[..], follow_pat) {
        // Success
        ([Pat::Start, p, Pat::End], FollowPat::End) => return vec![p.clone()],

        // Expr `:` Blank -> TypedExprHead
        // TypedExprHead: TypeSymbolSeq :!Blank -> Expr
        /* TODO: 此产生式要求当 Type 后存在空白时:
                 x: A -> B ->
                 Expr: Type 必须被括号环绕:
                 (x: A -> B) ->
                 (x: A -> { x: Int }) ->
                 否则将无法归约 */
        // TypedExprHead Blank TypeName -> Expr

        // `(` `)` -> Unit
        ([.., Pat::Mark('('), Pat::Mark(')')], _) =>
            stack.reduce_to_new(2, Pat::Unit),

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

        // DigitSeq Digit -> DigitSeq
        ([.., Pat::DigitSeq(ds), Pat::Digit(d)], _) =>
            stack.reduce_to_new(2, Pat::DigitSeq(format!("{}{}", ds, d))),
        // DigitSeq :Digit -> DigitSeq
        ([.., Pat::DigitSeq(_)], FollowPat::Digit(_)) =>
            return stack.clone(),
        // DigitSeq :!Digit -> Int|Err
        ([.., Pat::DigitSeq(ds)], _) => {
            let top = match parse_int(ds) {
                Some(i) => Pat::Int(i),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }
        // Digit :Digit -> DigitSeq
        ([.., Pat::Digit(d)], FollowPat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::DigitSeq(d.to_string())),
        // Digit :!Digit -> Int|Err
        ([.., Pat::Digit(d)], _) => {
            let top = match parse_int(&d.to_string()) {
                Some(i) => Pat::Int(i),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }

        // AlphanumSeq Alphanum -> AlphanumSeq
        ([.., Pat::AlphanumSeq(cs), Pat::Alphanum(c)], _) =>
            stack.reduce_to_new(2, Pat::AlphanumSeq(format!("{}{}", cs, c))),
        // AlphanumSeq :Alphanum -> AlphanumSeq
        ([.., Pat::AlphanumSeq(_)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            return stack.clone(),
        // AlphanumSeq :!Alphanum-> LetName|TypeName|Err
        ([.., Pat::AlphanumSeq(cs)], _) => {
            let top = match parse_let_name(cs) {
                Some(n) => Pat::LetName(n),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }
        // Alphanum :Alphanum -> AlphanumSeq
        ([.., Pat::Alphanum(c)], FollowPat::Letter(_) | FollowPat::Digit(_)) =>
            stack.reduce_to_new(1, Pat::AlphanumSeq(c.to_string())),
        // Alphanum :!Alphanum -> LetName|TypeName|Err
        ([.., Pat::Alphanum(c)], _) => {
            let top = match parse_let_name(&c.to_string()) {
                Some(n) => Pat::LetName(n),
                None => Pat::Err
            };
            stack.reduce_to_new(1, top)
        }

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
        ([.., Pat::ClosurePara(n), p], follow_pat)
        if follow_pat.not_blank() && p.is_expr() => {
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
        p, Pat::Blank], FollowPat::Mark('}')
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
        p, Pat::Blank ], FollowPat::Mark('|')
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
        p  ], follow_pat
        )
        if follow_pat.not_blank() && p.is_expr() => {
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
        Pat::Case(case, then) ], follow_pat
        )
        if match follow_pat {
            FollowPat::Blank | FollowPat::Mark('|') => false,
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
        Pat::CaseSeq(vec) ], follow_pat
        )
        if match follow_pat {
            FollowPat::Blank | FollowPat::Mark('|') => false,
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
        p, Pat::Blank], FollowPat::Keyword(Keyword::In)
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
        p], follow_pat)
        if follow_pat.not_blank() && p.is_expr() => {
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
        p], follow_pat)
        if follow_pat.not_blank() && p.is_expr() => {
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

    reduce_stack(&reduced_stack, follow_pat)
}

pub fn go(stack: &Vec<Pat>, seq: Vec<Either<char, Keyword>>) -> Pat {
    let (head, tail, follow) =
        vec_get_head_tail_follow(seq);

    let follow_pat = parse_follow_pat(follow);

    let stack = stack.push_to_new(move_in(stack, head));
    println!("Move in result: {:?} follow: {:?}", stack, follow_pat);

    let reduced_stack = reduce_stack(&stack, &follow_pat);

    match (&reduced_stack[..], follow_pat) {
        ([p], FollowPat::End) => {
            let r = p.clone();
            println!("Success with: {:?}", r);
            return r;
        }
        _ => go(&reduced_stack, tail)
    }
}
