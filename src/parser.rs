mod char;
mod keyword;
mod mark;
mod value;
mod name;
mod expr;

pub fn get_head_tail(seq: &str) -> (Option<char>, &str) {
    let mut chars = seq.chars();
    let head = chars.next();
    let tail = chars.as_str();
    (head, tail)
}

pub fn get_head_tail_next(seq: &str) -> (Option<char>, &str, Option<char>) {
    let (head, tail) = get_head_tail(seq);
    let (next, _) = get_head_tail(tail);
    (head, tail, next)
}
