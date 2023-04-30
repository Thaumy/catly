#[derive(Clone, Debug, PartialEq)]
pub enum Alphanum {
    Digit(u8),
    Lower(char),
    Upper(char)
}

pub fn parse_digit(x: &char) -> Option<u8> {
    let map = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    map.iter()
        .position(|c| c == x)
        .map(|d| d as u8)
}

pub fn parse_lower(x: &char) -> Option<char> {
    let map = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z'
    ];
    map.iter()
        .find(|c| c == &x)
        .copied()
}

pub fn parse_upper(x: &char) -> Option<char> {
    let map = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z'
    ];
    map.iter()
        .find(|c| c == &x)
        .copied()
}

pub fn parse_letter(x: &char) -> Option<Alphanum> {
    use crate::parser::alphanum::Alphanum::{Lower, Upper};

    let f1 = || parse_upper(x).map(|c| Upper(c));
    let f2 = || parse_lower(x).map(|c| Lower(c));

    f1().or_else(f2)
}

pub fn parse_alphanum(x: &char) -> Option<Alphanum> {
    use crate::parser::alphanum::Alphanum::Digit;

    let f1 = || parse_letter(x);
    let f2 = || parse_digit(x).map(|d| Digit(d));

    f1().or_else(f2)
}

#[test]
fn test_part1() {
    use crate::parser::alphanum::{parse_alphanum, Alphanum};

    assert_eq!(parse_alphanum(&'a'), Some(Alphanum::Lower('a')));
    assert_eq!(parse_alphanum(&'A'), Some(Alphanum::Upper('A')));
    assert_eq!(parse_alphanum(&'1'), Some(Alphanum::Digit(1)));

    assert_eq!(parse_alphanum(&'_'), None);
}
