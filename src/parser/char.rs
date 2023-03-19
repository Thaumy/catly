#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Char {
    Digit(u8),
    Lower(char),
    Upper(char),
}

pub fn parse_digit(x: &char) -> Option<u8> {
    let map = [
        '0', '1',
        '2', '3',
        '4', '5',
        '6', '7',
        '8', '9',
    ];
    map.iter().position(|c| c == x).map(|d| d as u8)
}

pub fn parse_lower(x: &char) -> Option<char> {
    let map = [
        'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x',
        'y', 'z'
    ];
    map.iter().find(|c| c == &x).copied()
}

pub fn parse_upper(x: &char) -> Option<char> {
    let map = [
        'A', 'B', 'C', 'D', 'E', 'F',
        'G', 'H', 'I', 'J', 'K', 'L',
        'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z'
    ];
    map.iter().find(|c| c == &x).copied()
}

pub fn parse_letter(x: &char) -> Option<Char> {
    use crate::parser::char::Char::{Lower, Upper};

    let f1 = || parse_upper(x).and_then(|c| Some(Upper(c)));
    let f2 = || parse_lower(x).and_then(|c| Some(Lower(c)));

    f1().or_else(f2)
}

// letter or digit
pub fn parse_char(x: &char) -> Option<Char> {
    use crate::parser::char::Char::Digit;

    let f1 = || parse_letter(x);
    let f2 = || parse_digit(x).and_then(|d| Some(Digit(d)));

    f1().or_else(f2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_keyword() {
        use crate::parser::char::{Char, parse_char};

        assert_eq!(parse_char(&'a'), Some(Char::Lower('a')));
        assert_eq!(parse_char(&'A'), Some(Char::Upper('A')));
        assert_eq!(parse_char(&'1'), Some(Char::Digit(1)));

        assert_eq!(parse_char(&'_'), None);
    }
}
