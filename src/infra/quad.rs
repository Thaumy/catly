#[derive(Clone, Debug, PartialEq)]
pub enum Quad<L, ML, MR, R> {
    L(L),
    ML(ML),
    MR(MR),
    R(R)
}

pub trait WrapQuad
where
    Self: Sized
{
    #[inline]
    fn wrap_quad_l<ML, MR, R>(self) -> Quad<Self, ML, MR, R> {
        Quad::L(self)
    }
    #[inline]
    fn wrap_quad_ml<L, MR, R>(self) -> Quad<L, Self, MR, R> {
        Quad::ML(self)
    }
    #[inline]
    fn wrap_quad_mr<L, ML, R>(self) -> Quad<L, ML, Self, R> {
        Quad::MR(self)
    }
    #[inline]
    fn wrap_quad_r<L, ML, MR>(self) -> Quad<L, ML, MR, Self> {
        Quad::R(self)
    }
}

impl<T> WrapQuad for T {}
