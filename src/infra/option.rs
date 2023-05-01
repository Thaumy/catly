pub trait OptionAnyExt
where
    Self: Sized
{
    #[inline]
    fn some(self) -> Option<Self> { Some(self) }
}

impl<T> OptionAnyExt for T {}
