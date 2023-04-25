mod expt;
mod fib;
mod filter;
mod find;
mod gcd;
mod map;
mod pi;

fn get_std_code() -> String {
    "
        type True = Int
        type False = Int
        type Bool = True | False

        def true = 1: True
        def false = 0: False

        def add: Int -> Int -> Int = _
        def sub: Int -> Int -> Int = _
        def mul: Int -> Int -> Int = _
        def rem: Int -> Int -> Int = _

        def gt: Int -> Int -> Bool = _
        def eq: Int -> Int -> Bool = _
        def lt: Int -> Int -> Bool = _

        type EmptyList = Unit
        type IntCons = { head: Int, tail: IntList }
        type IntList = IntCons | EmptyList

        def emptyList = (): EmptyList
        def intCons = h -> t -> { head = h, tail = t } : IntList
    "
    .to_string()
}
