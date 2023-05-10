#[allow(unused_macros)]
macro_rules! std_builtin {
    () => {
        "
# compiler built-in

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
"
    };
}
#[allow(unused_imports)]
pub(super) use std_builtin;
