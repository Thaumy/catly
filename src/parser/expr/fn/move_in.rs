use crate::lexer::Token;
use crate::parser::expr::pat::Pat;

pub fn move_in(stack: &[Pat], head: Option<Token>) -> Pat {
    match head {
        Some(o) => match (stack, o) {
            // .. -> LetName
            (_, Token::LetName(n)) => Pat::LetName(None, n),
            // .. -> TypeName
            (_, Token::TypeName(n)) => Pat::TypeName(n),
            // .. -> Kw
            (_, Token::Kw(kw)) => Pat::Kw(kw),
            // .. -> Int
            (_, Token::IntValue(i)) => Pat::Int(None, i),
            // .. -> Unit
            (_, Token::UnitValue) => Pat::Unit(None),
            // .. -> Discard
            (_, Token::DiscardValue) => Pat::Discard(None),

            // .. -> Mark
            (_, Token::Symbol(s)) => match s {
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
