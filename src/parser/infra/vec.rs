pub trait Ext<T> {
    fn reduce(&mut self, cost: u8, item: T);
    fn push_to_new(&self, item: T) -> Vec<T>;
}

impl<T> Ext<T> for Vec<T> where T: Clone {
    fn reduce(&mut self, cost: u8, item: T) {
        for _ in 0..cost {
            self.pop();
        }
        self.push(item);
    }
    fn push_to_new(&self, item: T) -> Vec<T> {
        let mut b = self.clone();
        b.push(item);
        b
    }
}

pub fn vec_get_head_tail<T>(vec: Vec<T>) -> (Option<T>, Vec<T>)
    where T: Clone
{
    let mut iter = vec.iter();
    let head = iter.next().cloned();
    (head, iter.map(|x| x.clone()).collect())
}

pub fn vec_get_head_tail_follow<T>(vec: Vec<T>) -> (Option<T>, Vec<T>, Option<T>)
    where T: Clone
{
    let (head, tail) = vec_get_head_tail(vec);
    let (follow, _) = vec_get_head_tail(tail.clone());
    (head, tail, follow)
}

