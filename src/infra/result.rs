pub trait AnyExt
where
    Self: Sized
{
    fn ok<E>(self) -> Result<Self, E> { Ok(self) }
    fn err<O>(self) -> Result<O, Self> { Err(self) }
}

impl<T> AnyExt for T {}
