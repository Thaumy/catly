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

#[macro_export]
macro_rules! btree_set {
    ($($v:expr,)*) => ({
        use std::collections::BTreeSet;
        let mut bt = BTreeSet::new();
        $(
            bt.insert($v);
        )*
        bt
    })
}
