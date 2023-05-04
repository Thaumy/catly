mod expt;
mod fib;
mod filter;
mod find;
mod gcd;
mod map;
mod pi;

pub fn get_std_code() -> String {
    "
        def neg: Int -> Int = _
        def add: Int -> Int -> Int = _
        def sub: Int -> Int -> Int = _
        def mul: Int -> Int -> Int = _
        def mod: Int -> Int -> Int = _
        def rem: Int -> Int -> Int = _

        def gt: Int -> Int -> Bool = _
        def eq: Int -> Int -> Bool = _
        def lt: Int -> Int -> Bool = _

        def not: Bool -> Bool = _
        def and: Bool -> Bool -> Bool = _
        def or: Bool -> Bool -> Bool = _

        type True = Int
        type False = Int
        type Bool = True | False

        def true = 1: True
        def false = 0: False

        type EmptyList = Unit
        type IntCons = { head: Int, tail: IntList }
        type IntList = IntCons | EmptyList

        def emptyList = (): EmptyList
        def intCons = h -> t -> { head = h, tail = t } : IntCons

        type Fraction = { n: Int, d: Int }

        def fraction = n -> d ->
            { n = n, d = d }: Fraction
        def int2F = i ->
            fraction i 1
    "
    .to_string()
}

#[allow(unused_macros)]
macro_rules! check_has_type {
    ($ret:expr, $t:expr) => {{
        use crate::infra::quad::Quad;
        match $ret {
            Quad::L((t, e)) => {
                let r = $t;
                assert_eq!(t, r);
                assert!(e.is_fully_typed());
            }
            _ => panic!()
        }
    }};
}
#[allow(unused_imports)]
pub(super) use check_has_type;

#[allow(unused_macros)]
macro_rules! check_req_constraint {
    ($ret:expr, $t:expr, $erc:expr) => {{
        use crate::infra::quad::Quad;
        match $ret {
            Quad::ML(rc) => {
                let r = $t;
                assert_eq!(rc.r#type, r);
                let erc = $erc;
                assert_eq!(rc.constraint, erc);
                assert!(rc.typed_expr.is_fully_typed());
            }
            _ => panic!()
        }
    }};
}
#[allow(unused_imports)]
pub(super) use check_req_constraint;
