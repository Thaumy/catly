#[allow(unused_macros)]
macro_rules! std_bool {
    () => {
        "
type True = Int
type False = Int
type Bool = True | False

def true = 1: True
def false = 0: False
"
    };
}
#[allow(unused_imports)]
pub(super) use std_bool;
