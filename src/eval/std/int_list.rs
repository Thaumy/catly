#[allow(unused_macros)]
macro_rules! std_int_list {
    () => {
        "
type EmptyList = Unit
type IntCons = { head: Int, tail: IntList }
type IntList = IntCons | EmptyList

def emptyList = (): EmptyList
def intCons = h -> t -> { head = h, tail = t }: IntCons
"
    };
}
#[allow(unused_imports)]
pub(super) use std_int_list;
