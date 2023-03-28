use crate::parser::keyword::Keyword;
use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Out {
    Symbol(char),
    LetName(String),
    TypeName(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue,
}

impl From<In> for Option<Out> {
    fn from(value: In) -> Self {
        let r = match value {
            In::Symbol(c) => Out::Symbol(c),
            In::LowerStartChunk(c) => match parse_let_name(&c) {
                Some(n) => Out::LetName(n),
                None => return None
            },
            In::UpperStartChunk(c) => match parse_type_name(&c) {
                Some(n) => Out::TypeName(n),
                None => return None
            },
            In::Kw(kw) => Out::Kw(kw),
            In::IntValue(i) => Out::IntValue(i),
            In::UnitValue => Out::UnitValue,
            In::DiscardValue => Out::DiscardValue,
        };
        Some(r)
    }
}

type In = crate::parser::preprocess::r#const::Out;

pub fn preprocess_name(seq: &[In]) -> Option<Vec<Out>> {
    let r = seq
        .iter()
        .fold(Some(vec![]), |acc, x|
            match (acc, Option::<Out>::from(x.clone())) {
                (Some(mut vec), Some(o)) => {
                    vec.push(o);
                    Some(vec)
                }
                _ => None
            },
        );
    println!("Name pp out: {:?}", r);
    r
}

#[cfg(test)]
mod tests {
    use crate::parser::keyword::Keyword;
    use crate::parser::preprocess::name::{Out, preprocess_name};

    type In = crate::parser::preprocess::r#const::Out;

    #[test]
    fn test_pp_keyword() {
        let seq = vec![
            In::Symbol('{'),
            In::Kw(Keyword::Type),
            In::LowerStartChunk("boob".to_string()),
            In::Kw(Keyword::Def),
            In::IntValue(8888),
            In::Kw(Keyword::Let),
            In::UnitValue,
            In::IntValue(123),
            In::Kw(Keyword::Then),
            In::UpperStartChunk("Boob".to_string()),
            In::Kw(Keyword::Match),
            In::DiscardValue,
            In::Kw(Keyword::With),
            In::Symbol(' '),
        ];
        let r = vec![
            Out::Symbol('{'),
            Out::Kw(Keyword::Type),
            Out::LetName("boob".to_string()),
            Out::Kw(Keyword::Def),
            Out::IntValue(8888),
            Out::Kw(Keyword::Let),
            Out::UnitValue,
            Out::IntValue(123),
            Out::Kw(Keyword::Then),
            Out::TypeName("Boob".to_string()),
            Out::Kw(Keyword::Match),
            Out::DiscardValue,
            Out::Kw(Keyword::With),
            Out::Symbol(' '),
        ];
        let r = Some(r);

        assert_eq!(preprocess_name(&seq), r);
    }
}
