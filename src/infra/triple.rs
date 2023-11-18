#[derive(Clone, Debug, PartialEq)]
pub enum Triple<L, M, R> {
    L(L),
    M(M),
    R(R)
}

pub trait WrapTriple
where
    Self: Sized
{
    #[inline]
    fn wrap_tri_l<M, R>(self) -> Triple<Self, M, R> {
        Triple::L(self)
    }
    #[inline]
    fn wrap_tri_m<L, R>(self) -> Triple<L, Self, R> {
        Triple::M(self)
    }
    #[inline]
    fn wrap_tri_r<L, M>(self) -> Triple<L, M, Self> {
        Triple::R(self)
    }
}

impl<T> WrapTriple for T {}
