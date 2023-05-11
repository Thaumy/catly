use crate::parser::expr::pat::Pat;
use crate::parser::expr::In;

pub fn move_in(stack: &[Pat], head: Option<In>) -> Pat {
    match head {
        Some(o) => match (stack, o) {
            // .. -> LetName
            (_, In::LetName(n)) => Pat::LetName(None, n),
            // .. -> TypeName
            (_, In::TypeName(n)) => Pat::TypeName(n),
            // .. -> Kw
            (_, In::Kw(kw)) => Pat::Kw(kw),
            // .. -> Int
            (_, In::IntValue(i)) => Pat::Int(None, i),
            // .. -> Unit
            (_, In::UnitValue) => Pat::Unit(None),
            // .. -> Discard
            (_, In::DiscardValue) => Pat::Discard(None),

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
                ':' => Pat::Mark(':'), // type annotation usage

                // _ -> Err
                c => {
                    if cfg!(feature = "parser_lr1_log") {
                        let log = format!("Invalid head Pat: {c:?}");
                        println!("{log}");
                    }

                    Pat::Err
                }
            }
        },

        // ɛ -> End
        None => Pat::End
    }
}
