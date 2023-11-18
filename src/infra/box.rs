pub trait WrapBox<T> {
    #[inline]
    fn wrap_box(self) -> Box<Self>
    where
        Self: Sized
    {
        Box::new(self)
    }
}

impl<T> WrapBox<T> for T where T: Sized {}
