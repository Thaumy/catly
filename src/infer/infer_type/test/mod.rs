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
        def div: Int -> Int -> Int = _
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
        def intCons = h -> t -> { head = h, tail = t }: IntCons

        type Fraction = { n: Int, d: Int }

        def gcd = a -> b ->
            if eq b 0 then
                a
            else
                gcd b (rem a b)

        def fraction = n -> d ->
            if or (gt n 1000) (gt d 1000) then
                let
                    g = gcd n d
                in
                    { n = div n g, d = div d g }: Fraction
            else
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
            Quad::L(e) => {
                assert!(e.is_fully_typed());

                let e_t = e.unwrap_type_annot();
                assert_eq!(e_t, &$t);
            }
            _ => panic!("{:?}", $ret)
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
                assert!(rc.typed_expr.is_fully_typed());

                let e_t = rc
                    .typed_expr
                    .unwrap_type_annot();
                assert_eq!(e_t, &$t);
                assert_eq!(rc.constraint, $erc);
            }
            _ => panic!("{:?}", $ret)
        }
    }};
}
#[allow(unused_imports)]
pub(super) use check_req_constraint;
