pub fn str_get_head_tail(seq: &str) -> (Option<char>, &str) {
    let mut chars = seq.chars();
    let head = chars.next();
    let tail = chars.as_str();
    (head, tail)
}

pub fn str_get_head_tail_follow(seq: &str) -> (Option<char>, &str, Option<char>) {
    let (head, tail) = str_get_head_tail(seq);
    let (follow, _) = str_get_head_tail(tail);
    (head, tail, follow)
}

