#[derive(Clone, Debug, PartialEq)]
pub enum Triple<L, M, R> {
    L(L),
    M(M),
    R(R)
}

pub trait TripleAnyExt
where
    Self: Sized
{
    #[inline]
    fn tri_l<M, R>(self) -> Triple<Self, M, R> { Triple::L(self) }
    #[inline]
    fn tri_m<L, R>(self) -> Triple<L, Self, R> { Triple::M(self) }
    #[inline]
    fn tri_r<L, M>(self) -> Triple<L, M, Self> { Triple::R(self) }
}

impl<T> TripleAnyExt for T {}
