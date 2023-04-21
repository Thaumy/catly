use std::iter::IntoIterator;

pub fn maybe_fold<'t, I, T: 't, A, F>(
    iter: I,
    acc: A,
    f: F
) -> Option<A>
where
    I: IntoIterator<Item = &'t T>,
    F: Fn(A, &T) -> Option<A>
{
    iter.into_iter()
        .fold(Some(acc), |acc, p| match acc {
            Some(acc) => f(acc, p),
            none => none
        })
}

pub fn maybe_reduce<'t, I, T: 't, F>(iter: I, f: F) -> Option<T>
where
    I: IntoIterator<Item = &'t T>,
    T: Clone,
    F: Fn(T, &T) -> Option<T>
{
    let mut iter = iter.into_iter();
    let first = iter.next().cloned()?;

    maybe_fold(iter, first, f)
}
