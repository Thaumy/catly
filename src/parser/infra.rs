use crate::parser::expr::Expr;
use crate::parser::r#type::Type;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Either<L, R> {
    L(L),
    R(R),
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

pub fn str_get_head_tail(seq: &str) -> (Option<char>, &str) {
    let mut chars = seq.chars();
    let head = chars.next();
    let tail = chars.as_str();
    (head, tail)
}

pub fn str_get_head_tail_follow(seq: &str) -> (Option<char>, &str, Option<char>) {
    let (head, tail) = str_get_head_tail(seq);
    let (follow, _) = str_get_head_tail(tail);
    (head, tail, follow)
}

pub trait VecExt<T> {
    fn reduce_to_new(&self, cost: u8, item: T) -> Vec<T>;
    fn push_to_new(&self, item: T) -> Vec<T>;
}

impl<T> VecExt<T> for Vec<T> where T: Clone {
    fn reduce_to_new(&self, cost: u8, item: T) -> Vec<T> {
        let mut b = self.clone();
        for _ in 0..cost {
            b.pop();
        }
        b.push(item);
        b
    }
    fn push_to_new(&self, item: T) -> Vec<T> {
        let mut b = self.clone();
        b.push(item);
        b
    }
}

pub trait BoxExt<T> {
    fn boxed(self) -> Box<Self> where Self: Sized {
        Box::new(self)
    }
}

impl<T> BoxExt<T> for T where T: Sized {}

pub type MaybeType = Option<Type>;
pub type MaybeExpr = Option<Expr>;
