pub trait VecExt<T> {
    fn push_to_new(&self, item: T) -> Vec<T>;
    fn chain_push(self, item: T) -> Vec<T>;
    fn reduce(&mut self, cost: u8, item: T);
}

impl<T> VecExt<T> for Vec<T>
where
    T: Clone
{
    #[inline]
    fn push_to_new(&self, item: T) -> Vec<T> {
        let b = self.clone();
        b.chain_push(item)
    }
    #[inline]
    fn chain_push(mut self, item: T) -> Vec<T> {
        self.push(item);
        self
    }
    #[inline]
    fn reduce(&mut self, cost: u8, item: T) {
        if cost == 0 {
            self.push(item);
        } else {
            self.pop();
            self.reduce(cost - 1, item);
        }
    }
}
