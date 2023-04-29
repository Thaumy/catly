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
