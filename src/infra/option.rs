pub trait WrapOption
where
    Self: Sized
{
    #[inline]
    fn wrap_some(self) -> Option<Self> { Some(self) }
}

impl<T> WrapOption for T {}
