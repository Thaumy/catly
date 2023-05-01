#[derive(Clone, Debug, PartialEq)]
pub enum Either<L, R> {
    L(L),
    R(R)
}

pub trait EitherAnyExt
where
    Self: Sized
{
    #[inline]
    fn l<R>(self) -> Either<Self, R> { Either::L(self) }
    #[inline]
    fn r<L>(self) -> Either<L, Self> { Either::R(self) }
}

impl<T> EitherAnyExt for T {}
