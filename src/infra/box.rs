pub trait Ext<T> {
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<T> Ext<T> for T where T: Sized {}
