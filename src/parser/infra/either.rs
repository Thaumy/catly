#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Either<L, R> {
    L(L),
    R(R),
}

