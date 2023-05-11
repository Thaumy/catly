use crate::parser::r#type::pat::Pat;
use crate::parser::r#type::In;

pub fn move_in(head: Option<In>) -> Pat {
    match head {
        Some(o) => match o {
            // .. -> LetName
            In::LetName(n) => Pat::LetName(None, n),
            // .. -> TypeName
            In::TypeName(n) => Pat::TypeName(n),

            // .. -> Mark
            In::Symbol(s) => match s {
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
                // ',' -> `,`
                ',' => Pat::Mark(','),

                // '|' -> `|`
                '|' => Pat::Mark('|'),
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
            p => {
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
