use crate::parser::char::{parse_digit, parse_letter};
use crate::parser::Either;
use crate::parser::keyword::Keyword;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum FollowPat {
    End,
    Blank,
    Digit(char),
    Letter(char),
    Mark(char),
    Keyword(Keyword),
}

impl FollowPat {
    pub(crate) fn not_blank(&self) -> bool {
        match self {
            FollowPat::Blank => false,
            _ => true
        }
    }
}

pub fn parse_follow_pat(follow: Option<Either<char, Keyword>>) -> FollowPat {
    match follow {
        Some(e) => match e {
            Either::L(' ') => FollowPat::Blank,
            Either::L(c) if parse_digit(&c).is_some() => FollowPat::Digit(c),
            Either::L(c) if parse_letter(&c).is_some() => FollowPat::Letter(c),
            Either::L(c) => FollowPat::Mark(c),
            Either::R(kw) => FollowPat::Keyword(kw)
        }
        _ => FollowPat::End,
    }
}

