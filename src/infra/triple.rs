#[derive(Clone, Debug, PartialEq)]
pub enum Triple<L, M, R> {
    L(L),
    M(M),
    R(R)
}

pub trait AnyExt
where
    Self: Sized
{
    fn l<M, R>(self) -> Triple<Self, M, R> { Triple::L(self) }
    fn m<L, R>(self) -> Triple<L, Self, R> { Triple::M(self) }
    fn r<L, M>(self) -> Triple<L, M, Self> { Triple::R(self) }
}

impl<T> AnyExt for T {}
