use std::rc::Rc;

pub trait WrapRc<T> {
    #[inline]
    fn wrap_rc(self) -> Rc<Self>
    where
        Self: Sized
    {
        Rc::new(self)
    }
}

impl<T> WrapRc<T> for T where T: Sized {}
