#[derive(Clone, Debug, PartialEq)]
pub enum EvalErr {
    EvalDiscard(String),
    EnvRefNotFound(String),
    NonExhaustiveMatch(String)
}
