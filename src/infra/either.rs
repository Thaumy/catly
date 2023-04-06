#[derive(Clone, Debug, PartialEq)]
pub enum Either<L, R> {
    L(L),
    R(R)
}

pub trait AnyExt
where
    Self: Sized
{
    fn l<R>(self) -> Either<Self, R> { Either::L(self) }
    fn r<L>(self) -> Either<L, Self> { Either::R(self) }
}

impl<T> AnyExt for T {}
