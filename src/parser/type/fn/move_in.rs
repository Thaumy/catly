use crate::lexer::Token;
use crate::parser::r#type::pat::Pat;

pub fn move_in(head: Option<Token>) -> Pat {
    match head {
        Some(o) => match o {
            // .. -> LetName
            Token::LetName(n) => Pat::LetName(None, n),
            // .. -> TypeName
            Token::TypeName(n) => Pat::TypeName(n),

            // .. -> Mark
            Token::Symbol(s) => match s {
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
