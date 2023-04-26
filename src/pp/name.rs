use crate::infra::option::AnyExt;
use crate::infra::vec::Ext;
use crate::parser::keyword::Keyword;
use crate::parser::name::let_name::parse_let_name;
use crate::parser::name::type_name::parse_type_name;

#[derive(Debug, Clone, PartialEq)]
pub enum Out {
    Symbol(char),
    LetName(String),
    TypeName(String),
    Kw(Keyword),

    IntValue(i64),
    UnitValue,
    DiscardValue
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
            In::DiscardValue => Out::DiscardValue
        };
        Some(r)
    }
}

type In = crate::pp::r#const::Out;

pub fn pp_name(seq: &[In]) -> Option<Vec<Out>> {
    let result = seq
        .iter()
        .try_fold(vec![], |acc, p| {
            let it = (p.clone().into(): Option<Out>)?;
            acc.chain_push(it).some()
        });

    if cfg!(feature = "pp_log") {
        let log = format!("{:8}{:>10} │ {result:?}", "[pp]", "Name");
        println!("{log}");
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::parser::keyword::Keyword;
    use crate::pp::name::{pp_name, Out};

    type In = crate::pp::r#const::Out;

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

        assert_eq!(pp_name(&seq), r);
    }
}
