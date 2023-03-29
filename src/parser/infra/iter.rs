#[macro_export]
macro_rules! maybe_fold {
    ($iter:expr, $acc:expr, $do:ident, $f:expr) => {
        $iter.fold(Some($acc), |acc, p|
            match (acc, $f(p)) {
                (Some(mut vec), Some(v)) => {
                    vec.$do(v);
                    Some(vec)
                }
                _ => None
            },
        )
    }
}
