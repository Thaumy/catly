use crate::infra::VecExt;
use crate::infra::WrapOption;
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
        match value {
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
        }
        .wrap_some()
    }
}

type In = crate::lexer::literal::Out;

pub fn lexer_name<S>(mut seq: S) -> Option<Vec<Out>>
where
    S: Iterator<Item = In>
{
    let r = seq.try_fold(vec![], |acc, p| {
        let it: Option<Out> = p.into();
        acc.chain_push(it?)
            .wrap_some()
    });

    #[cfg(feature = "lexer_log")]
    {
        let log = format!("{:8}{:>10} │ {r:?}", "[lexer]", "Name");
        println!("{log}");
    }

    r
}

#[test]
fn test_part1() {
    use crate::lexer::name::{lexer_name, Out};
    use crate::parser::keyword::Keyword;

    type In = crate::lexer::literal::Out;

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
    ]
    .wrap_some();

    assert_eq!(lexer_name(seq.into_iter()), r);
}
