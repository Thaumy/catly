use std::rc::Rc;

pub trait RcAnyExt<T> {
    #[inline]
    fn rc(self) -> Rc<Self>
    where
        Self: Sized
    {
        Rc::new(self)
    }
}

impl<T> RcAnyExt<T> for T where T: Sized {}
