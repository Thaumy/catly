pub trait BoxAnyExt<T> {
    #[inline]
    fn boxed(self) -> Box<Self>
    where
        Self: Sized
    {
        Box::new(self)
    }
}

impl<T> BoxAnyExt<T> for T where T: Sized {}
