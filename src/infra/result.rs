pub trait ResultAnyExt
where
    Self: Sized
{
    #[inline]
    fn ok<E>(self) -> Result<Self, E> { Ok(self) }
    #[inline]
    fn err<O>(self) -> Result<O, Self> { Err(self) }
}

impl<T> ResultAnyExt for T {}
