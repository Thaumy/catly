mod expt;
mod fib;
mod filter;
mod find;
mod gcd;
mod map;
mod pi;

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
