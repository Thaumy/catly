use std::iter::IntoIterator;

pub trait IntoIteratorExt<'t, T: 't>
where
    Self: IntoIterator<Item = &'t T> + Sized
{
    fn maybe_fold<A, F>(self, acc: A, f: F) -> Option<A>
    where
        F: Fn(A, &T) -> Option<A>
    {
        self.into_iter()
            .fold(Some(acc), |acc, p| match acc {
                Some(acc) => f(acc, p),
                none => none
            })
    }

    fn maybe_reduce<F>(self, f: F) -> Option<T>
    where
        T: Clone,
        F: Fn(T, &T) -> Option<T>
    {
        let mut iter = self.into_iter();
        let first = iter.next().cloned()?;

        iter.maybe_fold(first, f)
    }
}

impl<'t, T: 't, I> IntoIteratorExt<'t, T> for I where
    I: IntoIterator<Item = &'t T>
{
}
