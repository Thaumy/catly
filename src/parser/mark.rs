#[derive(Clone, Debug, PartialEq)]
pub enum Mark {
    Underline,
    Dash,
    RightAngleBracket,
    LeftPar,
    RightPar,
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Star,
    Comma,
    VerticalBar,
    Equal
}

pub fn parse_underline(x: &char) -> bool { x == &'_' }

pub fn parse_l_par(x: &char) -> bool { x == &'(' }

pub fn parse_r_par(x: &char) -> bool { x == &')' }

pub fn parse_l_curly_bracket(x: &char) -> bool { x == &'{' }

pub fn parse_r_curly_bracket(x: &char) -> bool { x == &'}' }

pub fn parse_colon(x: &char) -> bool { x == &':' }

pub fn parse_star(x: &char) -> bool { x == &'*' }

pub fn parse_comma(x: &char) -> bool { x == &',' }

pub fn parse_vertical_bar(x: &char) -> bool { x == &'|' }

pub fn parse_equal(x: &char) -> bool { x == &'=' }

pub fn parse_dash(x: &char) -> bool { x == &'-' }

pub fn parse_r_angle_bracket(x: &char) -> bool { x == &'>' }

pub fn parse_mark(x: &char) -> Option<Mark> {
    let r = match x {
        '_' => Mark::Underline,
        '(' => Mark::LeftPar,
        ')' => Mark::RightPar,
        '{' => Mark::LeftCurlyBracket,
        '}' => Mark::RightCurlyBracket,
        ':' => Mark::Colon,
        '*' => Mark::Star,
        ',' => Mark::Comma,
        '|' => Mark::VerticalBar,
        '=' => Mark::Equal,
        '-' => Mark::Dash,
        '>' => Mark::RightAngleBracket,
        _ => return None
    };
    Some(r)
}

#[cfg(test)]
mod tests {
    use crate::parser::mark::*;

    #[test]
    fn test_parse_mark_part1() {
        assert!(parse_underline(&'_'));
        assert!(parse_l_par(&'('));
        assert!(parse_r_par(&')'));
        assert!(parse_l_curly_bracket(&'{'));
        assert!(parse_r_curly_bracket(&'}'));
        assert!(parse_colon(&':'));
        assert!(parse_star(&'*'));
        assert!(parse_comma(&','));
        assert!(parse_vertical_bar(&'|'));
        assert!(parse_equal(&'='));
        assert!(parse_dash(&'-'));
        assert!(parse_r_angle_bracket(&'>'));

        assert!(!parse_underline(&'a'));
        assert!(!parse_l_par(&'a'));
        assert!(!parse_r_par(&'a'));
        assert!(!parse_l_curly_bracket(&'a'));
        assert!(!parse_r_curly_bracket(&'a'));
        assert!(!parse_colon(&'a'));
        assert!(!parse_star(&'a'));
        assert!(!parse_comma(&'a'));
        assert!(!parse_vertical_bar(&'a'));
        assert!(!parse_equal(&'a'));
        assert!(!parse_dash(&'a'));
        assert!(!parse_r_angle_bracket(&'a'));
    }

    #[test]
    fn test_parse_mark_part2() {
        use crate::parser::mark::{parse_mark, Mark};

        assert_eq!(parse_mark(&'_'), Some(Mark::Underline));
        assert_eq!(parse_mark(&'('), Some(Mark::LeftPar));
        assert_eq!(parse_mark(&')'), Some(Mark::RightPar));
        assert_eq!(parse_mark(&'{'), Some(Mark::LeftCurlyBracket));
        assert_eq!(parse_mark(&'}'), Some(Mark::RightCurlyBracket));
        assert_eq!(parse_mark(&':'), Some(Mark::Colon));
        assert_eq!(parse_mark(&'*'), Some(Mark::Star));
        assert_eq!(parse_mark(&','), Some(Mark::Comma));
        assert_eq!(parse_mark(&'|'), Some(Mark::VerticalBar));
        assert_eq!(parse_mark(&'='), Some(Mark::Equal));
        assert_eq!(parse_mark(&'-'), Some(Mark::Dash));
        assert_eq!(parse_mark(&'>'), Some(Mark::RightAngleBracket));

        assert_eq!(parse_mark(&'a'), None);
        assert_eq!(parse_mark(&'1'), None);
    }
}
