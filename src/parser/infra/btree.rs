#[macro_export]
macro_rules! btree_map {
    ($($v:expr,)*) => ({
        use std::collections::BTreeSet;
        let mut bt = BTreeSet::new();
        $(
            bt.insert($v);
        )*
        bt
    })
}
