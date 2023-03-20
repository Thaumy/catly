use crate::parser::char::{parse_digit, parse_letter};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum FollowPat {
    End,
    Blank,
    Digit(char),
    Letter(char),
    Mark(char),
}

impl FollowPat {
    pub(crate) fn not_blank(&self) -> bool {
        match self {
            FollowPat::Blank => false,
            _ => true
        }
    }
}

pub fn parse_follow_pat(follow: Option<char>) -> FollowPat {
    match follow {
        Some(' ') => FollowPat::Blank,
        Some(c) if parse_digit(&c).is_some() => FollowPat::Digit(c),
        Some(c) if parse_letter(&c).is_some() => FollowPat::Letter(c),
        Some(c) => FollowPat::Mark(c),
        _ => FollowPat::End,
    }
}

