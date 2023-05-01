pub trait IteratorExt<T>
where
    Self: Sized
{
    fn get_head_tail(self) -> (Option<T>, Self);
    fn get_head_tail_follow(self) -> (Option<T>, Self, Option<T>)
    where
        Self: Clone;
}

impl<T, I> IteratorExt<T> for I
where
    I: Iterator<Item = T>
{
    fn get_head_tail(mut self) -> (Option<T>, Self) {
        let head = self.next();
        (head, self)
    }

    fn get_head_tail_follow(self) -> (Option<T>, Self, Option<T>)
    where
        Self: Clone
    {
        let (head, tail) = self.get_head_tail();
        let (follow, _) = tail.clone().get_head_tail();
        (head, tail, follow)
    }
}
