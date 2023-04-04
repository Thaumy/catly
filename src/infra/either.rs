#[derive(Clone, Debug, PartialEq)]
pub enum Either<L, R> {
    L(L),
    R(R)
}
