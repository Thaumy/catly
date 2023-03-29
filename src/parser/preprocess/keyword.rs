use crate::parser::keyword::Keyword;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Out {
    Symbol(char),
    DigitChunk(String),
    LowerStartChunk(String),
    UpperStartChunk(String),

    Kw(Keyword),
}

impl From<In> for Out {
    fn from(value: In) -> Self {
        match value {
            In::DigitChunk(c) => Self::DigitChunk(c),
            In::LowerStartChunk(c) => match c {
                // "type" -> Type
                c if c == "type" => Self::Kw(Keyword::Type),
                // "def" -> Def
                c if c == "def" => Self::Kw(Keyword::Def),
                // "let" -> Let
                c if c == "let" => Self::Kw(Keyword::Let),
                // "in" -> In
                c if c == "in" => Self::Kw(Keyword::In),
                // "if" -> If
                c if c == "if" => Self::Kw(Keyword::If),
                // "then" -> Then
                c if c == "then" => Self::Kw(Keyword::Then),
                // "else" -> Else
                c if c == "else" => Self::Kw(Keyword::Else),
                // "match" -> Match
                c if c == "match" => Self::Kw(Keyword::Match),
                // "with" -> With
                c if c == "with" => Self::Kw(Keyword::With),

                _ => Self::LowerStartChunk(c)
            },
            In::UpperStartChunk(c) => Self::UpperStartChunk(c),
            In::Symbol(c) => Self::Symbol(c),
        }
    }
}

type In = crate::parser::preprocess::chunk::Out;

pub fn pp_keyword(seq: &[In]) -> Vec<Out> {
    let r = seq
        .iter()
        .fold(vec![], |mut acc, x| {
            acc.push(Out::from(x.clone()));
            acc
        });
    println!("Keyword pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::keyword::Keyword;
    use crate::parser::preprocess::keyword::{Out, pp_keyword};

    type In = crate::parser::preprocess::chunk::Out;

    #[test]
    fn test_pp_keyword() {
        let seq = vec![
            In::Symbol('{'),
            In::LowerStartChunk("type".to_string()),
            In::LowerStartChunk("boob".to_string()),
            In::LowerStartChunk("def".to_string()),
            In::LowerStartChunk("let".to_string()),
            In::LowerStartChunk("in".to_string()),
            In::LowerStartChunk("if".to_string()),
            In::DigitChunk("123".to_string()),
            In::LowerStartChunk("then".to_string()),
            In::LowerStartChunk("else".to_string()),
            In::UpperStartChunk("Boob".to_string()),
            In::LowerStartChunk("match".to_string()),
            In::LowerStartChunk("with".to_string()),
            In::Symbol(' '),
        ];
        let r = vec![
            Out::Symbol('{'),
            Out::Kw(Keyword::Type),
            Out::LowerStartChunk("boob".to_string()),
            Out::Kw(Keyword::Def),
            Out::Kw(Keyword::Let),
            Out::Kw(Keyword::In),
            Out::Kw(Keyword::If),
            Out::DigitChunk("123".to_string()),
            Out::Kw(Keyword::Then),
            Out::Kw(Keyword::Else),
            Out::UpperStartChunk("Boob".to_string()),
            Out::Kw(Keyword::Match),
            Out::Kw(Keyword::With),
            Out::Symbol(' '),
        ];

        assert_eq!(pp_keyword(&seq), r);
    }
}
