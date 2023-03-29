pub fn slice_get_head_tail<T>(slice: &[T]) -> (Option<&T>, &[T]) {
    if slice.len() > 0 {
        (slice.first(), &slice[1..])
    } else {
        (slice.first(), &[])
    }
}

