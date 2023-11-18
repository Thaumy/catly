#[derive(Clone, Debug, PartialEq)]
pub enum Either<L, R> {
    L(L),
    R(R)
}

pub trait WrapEither
where
    Self: Sized
{
    #[inline]
    fn wrap_l<R>(self) -> Either<Self, R> { Either::L(self) }
    #[inline]
    fn wrap_r<L>(self) -> Either<L, Self> { Either::R(self) }
}

impl<T> WrapEither for T {}
