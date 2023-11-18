pub trait WrapResult
where
    Self: Sized
{
    #[inline]
    fn wrap_ok<E>(self) -> Result<Self, E> { Ok(self) }
    #[inline]
    fn wrap_err<O>(self) -> Result<O, Self> { Err(self) }
}

impl<T> WrapResult for T {}
