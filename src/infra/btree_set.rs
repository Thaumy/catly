use std::collections::BTreeSet;

pub trait Ext<T> {
    fn chain_insert(self, item: T) -> BTreeSet<T>;
}

impl<T> Ext<T> for BTreeSet<T>
where
    T: Ord
{
    fn chain_insert(mut self, item: T) -> BTreeSet<T> {
        self.insert(item);
        self
    }
}

#[macro_export]
macro_rules! btree_set {
    () => ({
        use std::collections::BTreeSet;
        BTreeSet::new()
    });
    ($($v:expr),* $(,)*) => ({
        use std::collections::BTreeSet;
        let mut bt = BTreeSet::new();
        $(
            bt.insert($v);
        )*
        bt
    })
}
