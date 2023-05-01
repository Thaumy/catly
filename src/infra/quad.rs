#[derive(Clone, Debug, PartialEq)]
pub enum Quad<L, ML, MR, R> {
    L(L),
    ML(ML),
    MR(MR),
    R(R)
}

pub trait QuadAnyExt
where
    Self: Sized
{
    #[inline]
    fn quad_l<ML, MR, R>(self) -> Quad<Self, ML, MR, R> {
        Quad::L(self)
    }
    #[inline]
    fn quad_ml<L, MR, R>(self) -> Quad<L, Self, MR, R> {
        Quad::ML(self)
    }
    #[inline]
    fn quad_mr<L, ML, R>(self) -> Quad<L, ML, Self, R> {
        Quad::MR(self)
    }
    #[inline]
    fn quad_r<L, ML, MR>(self) -> Quad<L, ML, MR, Self> {
        Quad::R(self)
    }
}

impl<T> QuadAnyExt for T {}
